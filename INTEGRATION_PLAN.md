# Integration Plan: Transaction & PushDrop Wiring

**Created**: January 8, 2025 - 18:19 CST  
**Goal**: Wire up Transaction::from_beef() and PushDrop to complete token finding

---

## 🎯 Current Situation

### ✅ What's Complete
1. **Token Finding Structure** - All 5 functions have complete logic flow with TODOs
   - `find_protocol_token()` - 4 TODO call sites
   - `find_basket_token()` - 4 TODO call sites  
   - `find_certificate_token()` - 4 TODO call sites
   - `find_spending_token()` - 4 TODO call sites
   - Each function has exact TS line references

2. **Rust Transaction Module** - Basic structure exists
   - Location: `crates/wallet-core/src/transaction/transaction.rs`
   - Has `Transaction` struct with version, inputs, outputs, lock_time
   - Has serialization methods
   - ❌ **MISSING**: `from_beef()` method

3. **Rust BEEF Module** - Placeholder implementation
   - Location: `crates/wallet-core/src/beef/mod.rs`
   - Has `Beef` struct with find_txid(), merge methods
   - ❌ **MISSING**: `from_binary()` - Critical for parsing BEEF
   - ❌ **MISSING**: Transaction extraction from BEEF

4. **PushDrop**
   - ❌ **NOT FOUND** in Rust codebase
   - Need to check if available in ts-sdk or implement

---

## 📋 What We Need to Wire Up

### **Option 1: Pure Rust Implementation** ⭐ **RECOMMENDED**

Implement the missing pieces directly in Rust for best performance and type safety.

#### **Step 1: Implement Transaction::from_beef()** (2-3 hours)

**Location**: `crates/wallet-core/src/transaction/transaction.rs`

**What's Needed**:
```rust
impl Transaction {
    /// Parse a transaction from BEEF by txid
    ///
    /// Reference: TS Transaction.fromBEEF() 
    pub fn from_beef(beef: &serde_json::Value, txid: &str) -> TransactionResult<Self> {
        // 1. Extract BEEF binary data from JSON
        // 2. Parse BEEF structure (BRC-62 format)
        // 3. Find transaction by txid
        // 4. Deserialize transaction bytes
        // 5. Return Transaction
        
        // For now, can use beef module's Beef::from_binary()
        // or parse directly from result["BEEF"] field
    }
    
    /// Alternative: Parse from BEEF binary
    pub fn from_beef_binary(beef_bytes: &[u8], txid: &str) -> TransactionResult<Self> {
        // Parse BEEF binary format
        // Find transaction
        // Return Transaction
    }
}
```

**Implementation Steps**:
1. ✅ Parse BEEF binary from JSON value (base64 or hex)
2. ✅ Implement BEEF binary deserialization (BRC-62)
3. ✅ Find transaction by txid in BEEF.txs array
4. ✅ Deserialize transaction bytes to Transaction struct
5. ✅ Test with sample BEEF data

**Dependencies**:
- Binary parsing utilities (Reader/Writer)
- Transaction deserialization from bytes
- Txid calculation (double SHA-256)

#### **Step 2: Implement PushDrop** (3-4 hours)

**Location**: `crates/wallet-core/src/utility/pushdrop.rs` (new file)

**What's Needed**:
```rust
/// PushDrop script utilities
///
/// Reference: TS PushDrop class from @bsv/sdk
pub struct PushDrop {
    /// Decoded fields from PushDrop script
    pub fields: Vec<Vec<u8>>,
    
    /// Locking public key (if present)
    pub locking_public_key: Option<Vec<u8>>,
}

impl PushDrop {
    /// Decode a PushDrop locking script
    ///
    /// Reference: TS PushDrop.decode()
    pub fn decode(locking_script: &[u8]) -> Result<Self, PushDropError> {
        // 1. Parse Bitcoin script opcodes
        // 2. Extract OP_RETURN data fields
        // 3. Extract locking public key
        // 4. Return decoded PushDrop
    }
    
    /// Create a PushDrop locking script
    ///
    /// Reference: TS PushDrop.lock()
    pub fn lock(fields: &[Vec<u8>], locking_public_key: &[u8]) -> Result<Vec<u8>, PushDropError> {
        // 1. Create OP_RETURN script
        // 2. Push all fields
        // 3. Add locking public key
        // 4. Return script bytes
    }
    
    /// Create an unlocking script for PushDrop
    ///
    /// Reference: TS PushDrop.unlock()
    pub fn unlock(signature: &[u8]) -> Result<Vec<u8>, PushDropError> {
        // 1. Create unlocking script with signature
        // 2. Return script bytes
    }
}
```

**Implementation Steps**:
1. ✅ Implement Bitcoin script parser (opcodes)
2. ✅ Implement PushDrop.decode() to extract fields
3. ✅ Implement PushDrop.lock() to create scripts
4. ✅ Implement PushDrop.unlock() for unlocking
5. ✅ Test with sample PushDrop scripts

**Dependencies**:
- Script parsing utilities
- Opcode definitions (OP_RETURN, OP_PUSH, etc.)

#### **Step 3: Wire Up Token Finding** (30 min)

**Location**: `permission_validation.rs`

Simply uncomment and adjust the TODO code blocks in all 4 functions:

```rust
// Current (TODO):
// let tx = Transaction::from_beef(&result["BEEF"], txid)?;

// After:
let tx = Transaction::from_beef(&result["BEEF"], txid)?;
let locking_script = &tx.outputs[output_index].locking_script;

let decoded = PushDrop::decode(locking_script)?;
if decoded.fields.len() < 6 { // Or 3, 2 depending on function
    continue;
}

// Access fields
let domain_raw = &decoded.fields[0];
// ... etc
```

Apply to:
- [ ] find_protocol_token() - lines 162-243
- [ ] find_basket_token() - lines 329-385
- [ ] find_certificate_token() - lines 483-566
- [ ] find_spending_token() - lines 643-691

#### **Step 4: Wire Up Token Creation** (1 hour)

**Location**: `token_management.rs`

Uncomment TODO blocks in:
- [ ] create_permission_on_chain() - PushDrop.lock() call
- [ ] renew_permission_on_chain() - PushDrop.unlock() call
- [ ] coalesce_permission_tokens() - Multiple inputs handling

---

### **Option 2: Use TypeScript SDK via FFI/WASM** (Alternative)

If pure Rust is too time-consuming, we could:

1. **Compile ts-sdk to WASM**
   - Use wasm-bindgen to create Rust bindings
   - Call Transaction.fromBEEF() and PushDrop from WASM
   - Pros: Reuses existing tested code
   - Cons: Runtime dependency, performance overhead

2. **Create FFI Bridge**
   - Use Node.js via neon or napi-rs
   - Call ts-sdk functions from Rust
   - Pros: Full access to ts-sdk
   - Cons: Requires Node.js runtime

---

## 🎯 Recommended Approach

### **Phase 1: Implement Minimum Viable Integration** (4-5 hours)

**Priority**: Get token finding working ASAP

1. **Implement Transaction::from_beef()** (2 hours)
   - Simple parsing to extract transaction from BEEF JSON
   - Can skip full BEEF verification for now
   - Focus on `result["BEEF"]` field extraction

2. **Implement PushDrop.decode()** (2 hours)
   - Focus only on decode (reading)
   - Lock/unlock can wait for token creation phase

3. **Wire up all 4 find_*_token() functions** (30 min)
   - Uncomment TODO blocks
   - Test with mock data

4. **Test end-to-end** (30 min)
   - Create sample BEEF with PushDrop token
   - Verify token finding works

### **Phase 2: Complete Token Creation** (2-3 hours)

5. **Implement PushDrop.lock()** (1 hour)
6. **Implement PushDrop.unlock()** (1 hour)
7. **Wire up token_management.rs** (1 hour)

---

## 📁 Files to Create/Modify

### **New Files** (create these):
1. `crates/wallet-core/src/utility/pushdrop.rs`
   - PushDrop struct and methods
   - ~300 lines

2. `crates/wallet-core/src/utility/script.rs` (if needed)
   - Bitcoin script parsing
   - ~200 lines

### **Modify Files**:
1. `crates/wallet-core/src/transaction/transaction.rs`
   - Add `from_beef()` method
   - +50 lines

2. `crates/wallet-core/src/beef/mod.rs`
   - Implement `from_binary()` or simple JSON extraction
   - +100 lines

3. `crates/wallet-core/src/managers/wallet_permissions_manager/permission_validation.rs`
   - Uncomment 4 TODO blocks
   - Changes in 4 functions

4. `crates/wallet-core/src/managers/wallet_permissions_manager/token_management.rs`
   - Uncomment 3 TODO blocks
   - Changes in 3 functions

5. `crates/wallet-core/src/utility/mod.rs`
   - Add `pub mod pushdrop;`

---

## ✅ Success Criteria

**Phase 1 Complete When**:
- [ ] Can parse Transaction from BEEF JSON
- [ ] Can decode PushDrop script to extract fields
- [ ] All 4 find_*_token() functions compile without TODOs
- [ ] Can decrypt and validate token fields
- [ ] GREEN BUILD maintained

**Phase 2 Complete When**:
- [ ] Can create PushDrop locking scripts
- [ ] Can create PushDrop unlocking scripts
- [ ] Token creation functions work
- [ ] Token renewal functions work
- [ ] Integration tests pass

---

## 🚀 Next Immediate Action

**Recommended Start**: Implement Transaction::from_beef()

**Why**: This unblocks all 4 find_*_token() functions immediately

**Steps**:
1. Check ts-sdk for Transaction.fromBEEF() implementation
2. Understand BEEF JSON format from list_outputs()
3. Implement simple parsing in Rust
4. Test with sample data

**Time**: 2 hours to MVP, 4 hours to production-ready

---

**Question for User**: 

Would you like me to:
1. **Start with Pure Rust** - Implement Transaction::from_beef() now?
2. **Check ts-sdk first** - Look at TypeScript implementation for reference?
3. **Create FFI/WASM bridge** - Use ts-sdk directly?

I recommend **Option 1** (Pure Rust) for best long-term maintainability and performance.
