# wallet-toolbox-rs Architecture

**Last Updated**: January 8, 2025 - 18:50 CST  
**Architecture**: Hybrid Frontend (ts-sdk) + Backend (Rust)

---

## 🏗️ System Architecture

### **Frontend Layer (TypeScript with ts-sdk)**
**Location**: Browser/Client application  
**Language**: TypeScript  
**SDK**: @bsv/sdk (ts-sdk)

**Responsibilities**:
- ✅ Transaction building and parsing
- ✅ BEEF (Background Evaluation Extended Format) parsing
- ✅ PushDrop script encoding/decoding
- ✅ UI/UX and user interactions
- ✅ Calling Rust backend for crypto operations
- ✅ Storing results in wallet storage (via backend)

**Why**: ts-sdk already has mature, tested implementations of all Bitcoin protocol parsing

---

### **Backend Layer (Rust wallet-toolbox)**
**Location**: Native service or WASM module  
**Language**: Rust  
**Purpose**: Heavy computation and cryptographic operations

**Responsibilities**:
- ✅ **Cryptographic Operations**:
  - Transaction signing (ECDSA, Schnorr)
  - Key derivation (BRC-42, BRC-43)
  - Field encryption/decryption (for permission tokens)
  - Hash computations
  
- ✅ **Storage Management**:
  - Wallet database (SQLite/PostgreSQL)
  - Output tracking (UTXO management)
  - Action history
  - Certificate storage
  
- ✅ **Permission Validation** (Logic only, no parsing):
  - Check if permission exists in storage
  - Validate expiry timestamps
  - Enforce spending limits
  - Track monthly spending
  
- ✅ **Performance-Critical Operations**:
  - Bulk signing operations
  - Large transaction handling
  - Concurrent request processing

**Why**: Rust excels at crypto, performance, and type safety - but we don't need to reimplement Bitcoin protocol parsing

---

## 🔄 Data Flow

### **Creating a Permission Token**

```
Frontend (ts-sdk):
1. User requests permission
2. Build token fields (domain, expiry, etc.)
3. Encode PushDrop script with fields
4. Build transaction with PushDrop output
   │
   └──► Backend (Rust):
        5. Sign transaction with private key
        6. Encrypt sensitive fields
        7. Store output metadata in database
        8. Return signed transaction
           │
           └──► Frontend:
                9. Broadcast transaction to network
                10. Update UI
```

### **Finding a Permission Token**

```
Frontend:
1. User needs to verify permission
2. Call backend: "Do I have protocol X permission?"
   │
   └──► Backend (Rust):
        3. Query storage: list_outputs(basket, tags)
        4. Check stored metadata (fields already decoded)
        5. Decrypt sensitive fields
        6. Validate expiry, counterparty, etc.
        7. Return: { hasPermission: true, token: {...} }
           │
           └──► Frontend:
                8. Proceed with authorized action
                9. Update UI
```

**Key Insight**: Backend never needs to parse BEEF or PushDrop - it works with **already-parsed data in storage**!

---

## 📦 Module Responsibilities

### **WalletPermissionsManager (Rust)**

**What It Does**:
- ✅ Query storage for permission tokens (list_outputs)
- ✅ Decrypt token field data
- ✅ Validate permissions (expiry, counterparty, spending limits)
- ✅ Track spending per originator per month (list_actions)
- ✅ Manage permission lifecycle (callbacks)

**What It Doesn't Do**:
- ❌ Parse BEEF binary format
- ❌ Parse Bitcoin transactions
- ❌ Decode PushDrop scripts
- ❌ Build locking scripts

**Why**: All parsing happens in frontend with ts-sdk before data reaches Rust backend

---

### **Storage Layer (Rust)**

**Stores**:
```rust
// Output metadata (parsed by frontend, stored by backend)
{
    "outpoint": "txid.vout",
    "satoshis": 1,
    "lockingScript": "hex string",  // Raw script
    "basket": "admin protocol-permission",
    "tags": ["originator domain.com", "privileged true", ...],
    
    // Custom metadata (parsed PushDrop fields)
    "customInstructions": {
        "fields": [
            { "encrypted": true, "data": "base64..." },  // domain
            { "encrypted": true, "data": "base64..." },  // expiry
            // ... more fields
        ],
        "lockingPublicKey": "hex..."
    }
}
```

**Key**: Fields are already extracted by frontend before storage!

---

### **WalletSigner (Rust)**

**What It Does**:
- ✅ Sign transaction inputs with derived keys
- ✅ Handle different signature types (SIGHASH flags)
- ✅ Batch signing operations
- ✅ Hardware wallet integration (future)

**What It Doesn't Do**:
- ❌ Build transactions (frontend does this)
- ❌ Parse scripts (frontend does this)

---

### **Key Derivation (Rust)**

**What It Does**:
- ✅ BRC-42: Key derivation from protocol/counterparty
- ✅ BRC-43: Invoice number derivation
- ✅ Child key generation
- ✅ Public key derivation

**Performance**: Rust is 10-100x faster than TypeScript for these operations

---

## 🔌 API Surface

### **Rust Backend Exposes (JSON-RPC or similar)**:

```typescript
// Signing
signTransaction(tx: Transaction, inputs: Input[]): Promise<Transaction>

// Key derivation
deriveKey(protocol: string, counterparty: string): Promise<PublicKey>

// Encryption
encryptField(data: Uint8Array, originator: string): Promise<Uint8Array>
decryptField(encrypted: Uint8Array, originator: string): Promise<Uint8Array>

// Permission checking (no parsing!)
checkPermission(request: PermissionRequest): Promise<PermissionResult>

// Storage operations
listOutputs(args: ListOutputsArgs): Promise<ListOutputsResult>
listActions(args: ListActionsArgs): Promise<ListActionsResult>
createAction(args: CreateActionArgs): Promise<CreateActionResult>
```

### **Frontend (ts-sdk) Responsibilities**:

```typescript
// Transaction building
const tx = new Transaction()
tx.addOutput(new TxOutput({
  satoshis: 1,
  lockingScript: PushDrop.lock(fields, pubkey)  // ← ts-sdk
}))

// BEEF handling
const beef = await BEEF.fromBinary(beefData)  // ← ts-sdk
const tx = beef.findTx(txid)

// PushDrop
const decoded = PushDrop.decode(script)  // ← ts-sdk
const fields = decoded.fields

// Call backend for signing
const signed = await backend.signTransaction(tx, inputs)

// Call backend for storage
await backend.createAction({
  outputs: [{
    satoshis: 1,
    lockingScript: lockingScript.toHex(),
    customInstructions: {
      fields: fields.map(f => ({ data: f.toBase64() }))
    }
  }]
})
```

---

## ✅ Simplified Implementation

### **What We DON'T Need to Implement in Rust**:

1. ❌ `Transaction::from_beef()` - frontend parses
2. ❌ `PushDrop::decode()` - frontend parses
3. ❌ `PushDrop::lock()` - frontend creates
4. ❌ BEEF binary parser - frontend handles
5. ❌ Script parser - frontend handles

**Time Saved**: ~15-20 hours of implementation + testing

### **What We DO Need in Rust**:

1. ✅ Storage queries (already working!)
2. ✅ Field encryption/decryption
3. ✅ Permission validation logic
4. ✅ Signing operations (already working!)
5. ✅ Key derivation (already working!)

**Implementation Time**: ~2-3 hours to wire up encryption + validation

---

## 🎯 Updated TODO Resolution

### **In permission_validation.rs**:

**Before (with TODOs)**:
```rust
// TODO: Implement Transaction::from_beef()
// TODO: Implement PushDrop::decode()
// TODO: decrypt fields
```

**After (simplified)**:
```rust
// Fields are already in storage metadata
let custom = output["customInstructions"].as_object()?;
let fields = custom["fields"].as_array()?;

// Decrypt each field
let domain = decrypt_field(&fields[0])?;
let expiry = decrypt_field(&fields[1])?;
// etc...

// Validate
if domain != originator { continue; }
if is_expired(expiry) { continue; }

// Return token
Ok(Some(PermissionToken { ... }))
```

**Lines of Code**: ~50 per function instead of ~200+

---

## 🚀 Benefits of This Architecture

1. **Leverage Existing Code**: ts-sdk is mature and tested
2. **Performance Where It Matters**: Rust for crypto, TS for parsing
3. **Maintainability**: Don't duplicate Bitcoin protocol logic
4. **Faster Development**: ~15 hours saved
5. **Type Safety**: Strong types in both layers
6. **Flexibility**: Can swap frontend framework, backend stays same

---

## 📋 Next Steps

See **[`SIMPLIFIED_INTEGRATION.md`](SIMPLIFIED_INTEGRATION.md)** for implementation guide.

**Summary**:
1. Keep all parsing in frontend (ts-sdk)
2. Store parsed data in backend
3. Backend works with stored metadata
4. Wire up encryption/decryption only
5. Remove all BEEF/PushDrop TODOs

**Time to Complete**: 2-3 hours (vs 15-20 hours for full parsing)
