# signAction Implementation Plan

**Created**: January 7, 2025  
**Status**: 🔄 **Phase 1 Complete** - Scaffolding & Infrastructure  
**Next**: Phase 2 - Transaction Building

---

## Overview

The `signAction` function signs an unsigned transaction created by `createAction`, generating unlocking scripts for each input and preparing the transaction for broadcast.

### TypeScript Reference
**File**: `src/storage/methods/signAction.ts`  
**Estimated Lines**: ~300-400 lines  
**Complexity**: High (requires BSV SDK, key derivation, cryptography)

---

## Current Status

### ✅ Phase 1 Complete (Scaffolding)

**File**: `crates/wallet-core/src/methods/sign_action.rs` (348 lines)

#### Implemented
- [x] Module structure with 7-step process
- [x] Main `sign_action` function signature
- [x] All helper function signatures (stubs)
- [x] `StorageSignActionResult` type
- [x] Error handling infrastructure
- [x] **4/4 initial tests passing** ✅

#### Tests Passing
```
test methods::sign_action::tests::test_validate_transaction_status_unsigned ... ok
test methods::sign_action::tests::test_validate_transaction_status_wrong ... ok
test methods::sign_action::tests::test_validate_transaction_status_sending ... ok
test methods::sign_action::tests::test_validate_transaction_status_nosend ... ok
```

#### Function Stubs Created
1. ✅ `find_transaction_by_reference` - Lookup by reference
2. ✅ `validate_transaction_status` - Status checking (COMPLETE)
3. ✅ `load_transaction_inputs` - Input loading
4. ✅ `load_transaction_outputs` - Output loading
5. ✅ `build_and_sign_transaction` - Core signing logic
6. ✅ `update_signed_transaction` - Storage update
7. ✅ `handle_broadcast` - Broadcast preparation

---

## Implementation Roadmap

### Phase 2: Transaction Building (3-4 hours)

#### Dependencies Needed
- BSV SDK Transaction type (from rs-sdk or bsv crate)
- Script building utilities
- Serialization/deserialization

#### Tasks
1. **Implement `build_and_sign_transaction`**
   - Create BSV Transaction structure
   - Add inputs from TableOutput data
   - Add outputs from TableOutput data
   - Set version and lockTime
   - Calculate preliminary txid

2. **Implement storage queries**
   - `find_transaction_by_reference` using storage.find_transactions
   - `load_transaction_inputs` using storage.find_outputs (spentBy)
   - `load_transaction_outputs` using storage.find_outputs (transactionId)

3. **Add transaction building tests**
   - Test transaction structure creation
   - Test input/output addition
   - Test version/lockTime handling

**Estimated Complexity**: Medium  
**Blockers**: Need BSV SDK Transaction type

---

### Phase 3: Key Derivation (4-5 hours)

#### Dependencies Needed
- BRC-42 protocol ID derivation
- BRC-43 sender/recipient derivation
- ECDH key operations
- PrivateKey/PublicKey types

#### Tasks
1. **Create key derivation module**
   - File: `crates/wallet-core/src/keys/derivation.rs`
   - Implement BRC-42 protocol ID derivation
   - Implement BRC-43 invoice number derivation
   - Implement public key derivation

2. **Integrate with signAction**
   - Derive keys for each input
   - Handle different derivation types (storage vs user)
   - Cache derived keys for performance

3. **Add derivation tests**
   - Test BRC-42 derivation
   - Test BRC-43 derivation
   - Test key caching

**Estimated Complexity**: High  
**Blockers**: Need BRC specs, curve operations

**Reference**:
- BRC-42: https://github.com/bitcoin-sv/BRCs/blob/master/key-derivation/0042.md
- BRC-43: https://github.com/bitcoin-sv/BRCs/blob/master/key-derivation/0043.md

---

### Phase 4: Signature Generation (3-4 hours)

#### Dependencies Needed
- ECDSA signing
- Signature serialization (DER format)
- Sighash calculation
- Transaction serialization

#### Tasks
1. **Implement signing logic**
   - Calculate sighash for each input
   - Generate ECDSA signature
   - Serialize to DER format
   - Add sighash type byte

2. **Build unlocking scripts**
   - P2PKH: `<sig> <pubkey>`
   - Custom scripts from spends parameter
   - Handle sequence numbers

3. **Add signing tests**
   - Test signature generation
   - Test sighash calculation
   - Test DER serialization
   - Test P2PKH unlocking scripts

**Estimated Complexity**: High  
**Blockers**: Need ECDSA library, sighash implementation

---

### Phase 5: Storage Integration (2-3 hours)

#### Tasks
1. **Implement `update_signed_transaction`**
   - Update transaction status (Sending or Nosend)
   - Store txid
   - Store rawTx bytes
   - Mark outputs as spent

2. **Implement `handle_broadcast`**
   - Process sendWith transactions
   - Mark for broadcast if not noSend
   - Return SendWithResult list

3. **Add storage tests**
   - Test transaction status update
   - Test txid storage
   - Test rawTx storage
   - Test output spent marking

**Estimated Complexity**: Medium  
**Blockers**: Storage provider implementation

---

### Phase 6: Integration Testing (2-3 hours)

#### Tasks
1. **Create end-to-end tests**
   - Full createAction → signAction flow
   - Multiple inputs/outputs
   - Different providedBy scenarios
   - SendWith handling

2. **Create mock storage provider**
   - In-memory transaction storage
   - In-memory output storage
   - Query support

3. **Add edge case tests**
   - Empty spends (auto-generate)
   - Custom unlocking scripts
   - NoSend transactions
   - SendWith transactions

**Estimated Complexity**: Medium  
**Blockers**: Mock storage provider

---

## TypeScript Reference Mapping

### Process Flow (TS → Rust)

#### 1. Validate Arguments (TS lines 30-40)
```typescript
// TS: Validate reference and spends
const { reference, spends, options } = args;
validateReference(reference);
validateSpends(spends);
```

```rust
// Rust: Validation in sign_action function
pub async fn sign_action(
    storage: &mut dyn WalletStorageProvider,
    auth: &AuthId,
    vargs: ValidSignActionArgs,
) -> Result<StorageProcessActionResults, StorageError>
```

**Status**: ✅ Type validation handled by ValidSignActionArgs

---

#### 2. Retrieve Transaction (TS lines 42-55)
```typescript
// TS: Find transaction by reference
const tx = await storage.findTransactions({
    userId,
    reference,
    limit: 1
});

if (tx.status !== 'unsigned') {
    throw new Error('Transaction must be unsigned');
}
```

```rust
// Rust: find_transaction_by_reference + validate_transaction_status
let transaction = find_transaction_by_reference(
    storage,
    user_id,
    &vargs.reference,
).await?;

validate_transaction_status(&transaction)?;
```

**Status**: ✅ Scaffolded, needs implementation

---

#### 3. Load Inputs/Outputs (TS lines 62-75)
```typescript
// TS: Load transaction inputs and outputs
const inputs = await storage.findOutputs({
    userId,
    spentBy: transaction.transactionId
});

const outputs = await storage.findOutputs({
    userId,
    transactionId: transaction.transactionId
});
```

```rust
// Rust: load_transaction_inputs + load_transaction_outputs
let inputs = load_transaction_inputs(storage, transaction.transaction_id).await?;
let outputs = load_transaction_outputs(storage, transaction.transaction_id).await?;
```

**Status**: ✅ Scaffolded, needs implementation

---

#### 4. Build Transaction (TS lines 77-120)
```typescript
// TS: Build BSV transaction
const tx = new Transaction();
tx.version = transaction.version || 1;
tx.lockTime = transaction.lockTime || 0;

// Add inputs
for (const input of inputs) {
    tx.addInput({
        sourceTXID: input.txid,
        sourceOutputIndex: input.vout,
        sequence: spends[input.vin]?.sequenceNumber || 0xFFFFFFFF,
    });
}

// Add outputs
for (const output of outputs) {
    tx.addOutput({
        satoshis: output.satoshis,
        lockingScript: Script.fromHex(output.lockingScript),
    });
}
```

```rust
// Rust: build_and_sign_transaction (transaction building part)
async fn build_and_sign_transaction(
    storage: &dyn WalletStorageProvider,
    user_id: i64,
    transaction: &TableTransaction,
    inputs: &[TableOutput],
    outputs: &[TableOutput],
    spends: &HashMap<u32, SignActionSpend>,
) -> Result<SignedTransaction, StorageError>
```

**Status**: ⏳ TODO - Needs BSV SDK Transaction type

---

#### 5. Sign Inputs (TS lines 122-180)
```typescript
// TS: Sign each input
for (let vin = 0; vin < inputs.length; vin++) {
    const input = inputs[vin];
    const spend = spends[vin];
    
    // Derive key
    const key = deriveKey(input.derivationPrefix, input.derivationSuffix);
    
    // Generate signature
    const sighash = tx.sighash(vin, input.lockingScript);
    const signature = key.sign(sighash);
    
    // Build unlocking script
    if (spend?.unlockingScript) {
        tx.inputs[vin].setScript(spend.unlockingScript);
    } else {
        // Auto-generate P2PKH script
        tx.inputs[vin].setScript(
            new Script()
                .writeBuffer(signature.toDER())
                .writeBuffer(key.publicKey.toBuffer())
        );
    }
}
```

```rust
// Rust: build_and_sign_transaction (signing part)
// TODO: Implement with:
// 1. Key derivation (BRC-42/43)
// 2. Sighash calculation
// 3. ECDSA signature
// 4. Script building
```

**Status**: ⏳ TODO - Needs key derivation + signing infrastructure

---

#### 6. Update Storage (TS lines 182-200)
```typescript
// TS: Store signed transaction
await storage.updateTransaction(transaction.transactionId, {
    status: options.noSend ? 'nosend' : 'signed',
    txid: tx.id('hex'),
    rawTx: tx.toBuffer(),
});

// Mark outputs as spent
for (const input of inputs) {
    await storage.updateOutput(input.outputId, {
        spendable: false,
        spentBy: transaction.transactionId,
    });
}
```

```rust
// Rust: update_signed_transaction
async fn update_signed_transaction(
    storage: &mut dyn WalletStorageProvider,
    transaction_id: i64,
    txid: &str,
    raw_tx: &[u8],
    is_no_send: bool,
) -> Result<(), StorageError>
```

**Status**: ✅ Scaffolded, needs implementation

---

#### 7. Broadcast Handling (TS lines 202-220)
```typescript
// TS: Handle sendWith transactions
const sendWithResults = [];
if (!options.noSend && options.sendWith.length > 0) {
    for (const txid of options.sendWith) {
        const result = await broadcast(txid);
        sendWithResults.push(result);
    }
}

return {
    txid: tx.id('hex'),
    rawTx: options.returnTXIDOnly ? undefined : tx.toBuffer(),
    sendWithResults,
};
```

```rust
// Rust: handle_broadcast
async fn handle_broadcast(
    storage: &dyn WalletStorageProvider,
    signed_tx: &SignedTransaction,
    vargs: &ValidSignActionArgs,
) -> Result<Vec<SendWithResult>, StorageError>
```

**Status**: ✅ Scaffolded, needs implementation

---

## Dependencies Matrix

### External Crates Needed

| Dependency | Purpose | Status | Priority |
|------------|---------|--------|----------|
| `bsv` or `rs-sdk` | Transaction building, signing | ⏳ TODO | High |
| `secp256k1` | ECDSA signatures | ⏳ TODO | High |
| `sha2` | Hash functions | ✅ Available | High |
| `ripemd` | RIPEMD-160 for addresses | ⏳ TODO | Medium |
| `base58` | Address encoding | ⏳ TODO | Medium |

### Internal Modules Needed

| Module | Purpose | Status | Priority |
|--------|---------|--------|----------|
| `keys/derivation` | BRC-42/43 key derivation | ⏳ TODO | High |
| `script/builder` | Script construction | ⏳ TODO | High |
| `transaction/sighash` | Sighash calculation | ⏳ TODO | High |
| `transaction/serialize` | Transaction serialization | ⏳ TODO | High |

---

## Test Strategy

### Unit Tests (Per Function)

1. **validate_transaction_status** - ✅ 4/4 tests passing
   - ✅ Unsigned transaction (valid)
   - ✅ Completed transaction (invalid)
   - ✅ Sending transaction (invalid)
   - ✅ Nosend transaction (invalid)

2. **find_transaction_by_reference** - ⏳ 0/3 tests
   - [ ] Valid reference
   - [ ] Invalid reference
   - [ ] Multiple matches (should error)

3. **load_transaction_inputs** - ⏳ 0/2 tests
   - [ ] Valid transaction ID
   - [ ] Empty inputs

4. **load_transaction_outputs** - ⏳ 0/2 tests
   - [ ] Valid transaction ID
   - [ ] Empty outputs

5. **build_and_sign_transaction** - ⏳ 0/5 tests
   - [ ] P2PKH inputs (auto-generated unlocking scripts)
   - [ ] Custom unlocking scripts
   - [ ] Multiple inputs
   - [ ] Version and lockTime handling
   - [ ] Txid calculation

6. **update_signed_transaction** - ⏳ 0/3 tests
   - [ ] Update to Sending status
   - [ ] Update to Nosend status
   - [ ] Store rawTx

7. **handle_broadcast** - ⏳ 0/2 tests
   - [ ] SendWith processing
   - [ ] NoSend mode

**Target**: 20+ unit tests

---

### Integration Tests

1. **Full createAction → signAction flow** - ⏳ TODO
   - Create transaction with createAction
   - Sign with signAction
   - Verify txid matches
   - Verify rawTx is valid

2. **Multiple inputs/outputs** - ⏳ TODO
   - Test with 5+ inputs
   - Test with 10+ outputs
   - Verify all signatures valid

3. **Different providedBy scenarios** - ⏳ TODO
   - Storage-provided inputs
   - User-provided inputs
   - Mixed inputs

4. **SendWith transactions** - ⏳ TODO
   - Sign with sendWith
   - Verify broadcast preparation

**Target**: 10+ integration tests

---

## Implementation Priority

### Week 1 (Current)
- [x] Phase 1: Scaffolding ✅
- [ ] Phase 2: Transaction Building
- [ ] Basic tests for transaction building

### Week 2
- [ ] Phase 3: Key Derivation
- [ ] Phase 4: Signature Generation
- [ ] Unit tests for signing

### Week 3
- [ ] Phase 5: Storage Integration
- [ ] Phase 6: Integration Testing
- [ ] Full end-to-end tests

---

## Success Criteria

### Phase Complete When:
- [ ] All 7 helper functions implemented
- [ ] 20+ unit tests passing
- [ ] 10+ integration tests passing
- [ ] Full createAction → signAction → broadcast flow works
- [ ] Perfect TypeScript parity verified
- [ ] Zero compilation errors
- [ ] Zero unsafe code

---

## Next Session Tasks

**Immediate Priority**: Phase 2 - Transaction Building

1. **Add storage query methods to trait**
   - `find_transactions` with reference filter
   - Enhance `find_outputs` to support spentBy filter

2. **Implement transaction queries**
   - `find_transaction_by_reference`
   - `load_transaction_inputs`
   - `load_transaction_outputs`

3. **Research BSV SDK options**
   - Check rs-sdk for Transaction type
   - Check bsv crate for Transaction type
   - Consider implementing minimal Transaction struct if needed

4. **Add 5+ new tests**
   - Transaction lookup tests
   - Input/output loading tests
   - Basic validation tests

**Estimated Time**: 3-4 hours for Phase 2 completion

---

## Notes

- signAction is naturally paired with createAction
- Both require similar storage operations
- Key derivation (BRC-42/43) is reusable for other methods
- Transaction building infrastructure will support other operations
- Signature generation is core Bitcoin functionality

**Current Status**: 🟢 On track, scaffolding complete, ready for implementation

