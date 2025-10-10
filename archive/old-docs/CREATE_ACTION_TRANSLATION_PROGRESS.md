# createAction Translation Progress

**Status**: Phase 3.2 In Progress  
**TypeScript Source**: `wallet-toolbox/src/storage/methods/createAction.ts` (979 lines)  
**Rust Target**: `wallet-toolbox-rs/crates/wallet-core/src/methods/create_action.rs`  
**Started**: 2025-01-06

## Overview

Translating the core transaction building function from TypeScript to Rust with **strict functional parity**. This is one of the most complex functions in the wallet, handling transaction creation, input/output validation, change allocation, and BEEF construction.

## Function Breakdown

### Main Function: `createAction` (TS lines 59-150, ~90 lines)
- **Status**: Structure defined, implementation in progress
- **Orchestrates**: All 14 steps of transaction creation
- **Returns**: `StorageCreateActionResult`

### Helper Functions (Total: ~890 lines)

#### 1. validateRequiredInputs (TS lines 557-658, ~100 lines)
- **Status**: Stub ready for implementation
- **Purpose**: Validate all inputs have valid proofs, parse scripts/satoshis
- **Key Logic**:
  - Check inputBEEF for proofs
  - Verify trustSelf='known' conditions
  - Parse locking scripts and satoshis
  - Build BEEF structure
  - Handle txidOnly entries
  - Verify BEEF validity with ChainTracker
- **Returns**: `(storageBeef, beef, xinputs)`
- **Dependencies**: 
  - BEEF handling (BSV SDK)
  - storage.verifyKnownValidTransaction()
  - storage.findOutputs()
  - storage.getProvenOrRawTx()

#### 2. validateRequiredOutputs (TS lines 496-534, ~38 lines)
- **Status**: Stub ready for implementation
- **Purpose**: Validate outputs and add storage commission
- **Key Logic**:
  - Assign vout numbers to each output
  - Add storage service charge if configured
  - Create XValidCreateActionOutput records
- **Returns**: `Vec<XValidCreateActionOutput>`
- **Dependencies**:
  - createStorageServiceChargeScript() (TS lines 973-978)
  - lockScriptWithKeyOffsetFromPubKey() (TS lines 958-971)
  - offsetPubKey() (TS lines 944-956)

#### 3. createNewTxRecord (TS lines 441-472, ~31 lines)
- **Status**: Stub ready for implementation
- **Purpose**: Insert transaction record with labels
- **Key Logic**:
  - Generate random reference ID (12 bytes base64)
  - Insert transaction with status='unsigned'
  - Create/link transaction labels
- **Returns**: `TableTransaction`
- **Dependencies**:
  - storage.insertTransaction()
  - storage.findOrInsertTxLabel()
  - storage.findOrInsertTxLabelMap()
  - randomBytesBase64() helper

#### 4. validateNoSendChange (TS lines 680-718, ~38 lines)
- **Status**: Stub ready for implementation
- **Purpose**: Validate noSendChange outpoints
- **Key Logic**:
  - Check if isNoSend flag set
  - Validate each noSendChange outpoint exists
  - Verify output is spendable change in correct basket
  - Check for duplicates
- **Returns**: `Vec<TableOutput>`
- **Dependencies**:
  - storage.findOutputs()

#### 5. fundNewTransactionSdk (TS lines 720-888, ~168 lines)
- **Status**: Stub ready for implementation
- **Purpose**: Fund transaction by allocating change
- **Key Logic**:
  - Call generateChangeSdk() with params
  - Allocate noSendChange first
  - Allocate additional change as needed
  - Lock allocated outputs (spendable=false, spentBy=transactionId)
  - Generate random derivation prefix/suffix
  - Create change output records
  - Handle maxPossibleSatoshis adjustment
- **Returns**: `FundingResult { allocatedChange, changeOutputs, derivationPrefix, maxPossibleSatoshisAdjustment }`
- **Dependencies**:
  - generateChangeSdk() (separate file: generateChange.ts)
  - storage.allocateChangeInput()
  - storage.updateOutput()
  - Random() / randomDerivation() helpers

#### 6. createNewOutputs (TS lines 297-439, ~142 lines)
- **Status**: Stub ready for implementation
- **Purpose**: Insert all outputs into database
- **Key Logic**:
  - Lookup/create baskets for outputs
  - Lookup/create tags for outputs
  - Create output records for user outputs
  - Handle service-charge (commission) outputs
  - Insert change outputs
  - Randomize output order if requested (shuffle vout assignments)
  - Link outputs to baskets and tags
  - Build StorageCreateTransactionOutput results
- **Returns**: `OutputCreationResult { outputs, changeVouts }`
- **Dependencies**:
  - storage.findOrInsertOutputBasket()
  - storage.findOrInsertOutputTag()
  - storage.insertOutput()
  - storage.insertCommission()
  - storage.findOrInsertOutputTagMap()
  - makeDefaultOutput() helper

#### 7. createNewInputs (TS lines 207-295, ~88 lines)
- **Status**: Stub ready for implementation
- **Purpose**: Build input specifications and mark outputs spent
- **Key Logic**:
  - Combine user xinputs + allocated change
  - Mark user outputs as spent (spendable=false, spentBy=transactionId)
  - Check double-spend conditions
  - Get source transactions if includeAllSourceTransactions
  - Build StorageCreateTransactionInput for each
  - Set unlockingScriptLength (107 for P2PKH change)
  - Set providedBy ('you', 'storage', 'you-and-storage')
- **Returns**: `Vec<StorageCreateTransactionInput>`
- **Dependencies**:
  - storage.updateOutput()
  - storage.findOutputs() (for double-spend check)
  - storage.getRawTxOfKnownValidTransaction()

#### 8. mergeAllocatedChangeBeefs (TS lines 903-926, ~23 lines)
- **Status**: Stub ready for implementation
- **Purpose**: Merge BEEFs from allocated change
- **Key Logic**:
  - For each allocated change output
  - If not in beef and not in knownTxids
  - Call storage.getBeefForTransaction()
  - Merge to main beef
  - Call trimInputBeef() to handle returnTXIDOnly
- **Returns**: `Option<Vec<u8>>` (trimmed BEEF or None)
- **Dependencies**:
  - storage.getBeefForTransaction()
  - trimInputBeef() (TS lines 895-901)
  - BEEF.findTxid(), BEEF.makeTxidOnly()

### Supporting Helper Functions

#### makeDefaultOutput (TS lines 177-205, ~28 lines)
- **Purpose**: Create a TableOutput with default values
- **Returns**: `TableOutput`

#### createStorageServiceChargeScript (TS lines 973-978, ~5 lines)
- **Purpose**: Create locking script for storage commission
- **Calls**: lockScriptWithKeyOffsetFromPubKey()
- **Returns**: `{ script: String, keyOffset: String }`

#### lockScriptWithKeyOffsetFromPubKey (TS lines 958-971, ~13 lines)
- **Purpose**: Generate P2PKH script with key offset
- **Calls**: offsetPubKey()
- **Returns**: `{ script: String, keyOffset: String }`

#### offsetPubKey (TS lines 944-956, ~12 lines)
- **Purpose**: Apply key offset to public key
- **Uses**: Curve cryptography (ECDH)
- **Returns**: `{ offsetPubKey: String, keyOffset: String }`

#### trimInputBeef (TS lines 895-901, ~6 lines)
- **Purpose**: Convert known txids to txidOnly in BEEF
- **Returns**: `Option<Vec<u8>>`

## Storage Methods Required

These need to be available on `WalletStorageProvider` trait:

### Read Operations
- [x] `findOutputs(partial, trx?)` - Find outputs by criteria
- [x] `findOutputBaskets(partial, trx?)` - Find baskets by criteria
- [ ] `countChangeInputs(userId, basketId, excludeSending)` - Count available change
- [ ] `verifyKnownValidTransaction(txid)` - Check if txid is known valid
- [ ] `getProvenOrRawTx(txid, trx?)` - Get transaction data with proof
- [ ] `getRawTxOfKnownValidTransaction(txid, offset?, length?, trx?)` - Get raw tx bytes

### Write Operations
- [ ] `insertTransaction(tx)` - Create transaction record
- [ ] `updateTransaction(transactionId, updates)` - Update transaction fields
- [ ] `insertOutput(output)` - Create output record
- [ ] `updateOutput(outputId, updates, trx?)` - Update output fields
- [ ] `insertCommission(commission)` - Create commission record
- [ ] `findOrInsertOutputBasket(userId, name)` - Get or create basket
- [ ] `findOrInsertOutputTag(userId, name)` - Get or create tag
- [ ] `findOrInsertOutputTagMap(outputId, tagId)` - Link output to tag
- [ ] `findOrInsertTxLabel(userId, label)` - Get or create label
- [ ] `findOrInsertTxLabelMap(transactionId, labelId)` - Link tx to label
- [ ] `allocateChangeInput(userId, basketId, targetSatoshis, exactSatoshis?, excludeSending, transactionId)` - Select and lock change

### External Operations
- [ ] `getBeefForTransaction(txid, options)` - Get BEEF for a transaction
- [ ] `getServices().getChainTracker()` - Get chain tracker for BEEF verification

## Dependencies Needed

### BSV SDK (External)
- `Beef` class - BEEF creation/validation/merging
- `Script` class - Script parsing and manipulation
- `Transaction` class - Transaction parsing
- `PublicKey`, `PrivateKey` - Cryptography
- `Curve` - ECDH operations
- `P2PKH` - P2PKH script generation
- `Utils` - Base64 encoding

### Internal Helpers
- `randomBytesBase64(length)` - Generate random base64 string
- `Random(count)` - Generate random bytes array
- `sha256Hash(data)` - SHA256 hashing
- `asArray(data)` - Convert to byte array
- `asString(data)` - Convert to string

## Implementation Strategy

### Phase 1: Infrastructure (Current)
1. ✅ Define all types (XValidCreateActionInput, XValidCreateActionOutput, Context)
2. ✅ Update SDK types (ValidCreateActionArgs fields)
3. 🔄 Add required storage trait methods
4. ⏳ Add BSV SDK placeholders/stubs

### Phase 2: Helper Functions
1. ⏳ Implement validateRequiredOutputs (simplest, no BEEF)
2. ⏳ Implement validateNoSendChange (simple validation)
3. ⏳ Implement createNewTxRecord (database insert)
4. ⏳ Implement makeDefaultOutput and crypto helpers
5. ⏳ Implement createNewOutputs (complex but no BEEF)
6. ⏳ Implement createNewInputs (complex but no BEEF)

### Phase 3: BEEF Handling
1. ⏳ Implement validateRequiredInputs (most complex, BEEF heavy)
2. ⏳ Implement mergeAllocatedChangeBeefs (BEEF merging)

### Phase 4: Change Allocation
1. ⏳ Review/implement generateChangeSdk (separate file)
2. ⏳ Implement fundNewTransactionSdk (uses generateChange)

### Phase 5: Integration
1. ⏳ Complete main createAction orchestration
2. ⏳ Add comprehensive tests
3. ⏳ Verify functional parity

## Testing Strategy

### Unit Tests (Per Function)
- validateRequiredOutputs: 5+ tests
- validateNoSendChange: 5+ tests
- createNewTxRecord: 5+ tests
- createNewOutputs: 10+ tests (complex)
- createNewInputs: 10+ tests (complex)
- validateRequiredInputs: 15+ tests (most complex)
- fundNewTransactionSdk: 15+ tests (complex)
- mergeAllocatedChangeBeefs: 5+ tests

### Integration Tests
- End-to-end transaction creation: 10+ tests
- BEEF validation scenarios: 10+ tests
- Change allocation scenarios: 10+ tests
- Error handling: 10+ tests

**Target**: 100+ tests for createAction alone

## Progress Tracking

- **Types/Infrastructure**: 30% (types defined, storage methods TBD)
- **Helper Functions**: 0% (8 functions to implement)
- **Main Function**: 10% (structure defined)
- **Tests**: 0% (TBD after implementation)
- **Overall**: ~10% Complete

## Next Actions

1. Add storage trait methods to WalletStorageProvider
2. Implement validateRequiredOutputs (simplest helper)
3. Implement validateNoSendChange
4. Implement createNewTxRecord
5. Continue with remaining helpers
6. Integrate into main createAction
7. Add comprehensive test suite

## Notes

- **BEEF handling** is the most complex part - will require careful BSV SDK integration
- **Change allocation** (generateChangeSdk) is already stubbed in separate file
- **Crypto operations** (offsetPubKey, etc.) need BSV SDK curve operations
- **Random generation** needs deterministic testing support (randomVals array)
- **Transaction validation** will be added in signAction (Phase 3.2)

## References

- TypeScript Source: `wallet-toolbox/src/storage/methods/createAction.ts`
- Generate Change: `wallet-toolbox/src/storage/methods/generateChange.ts`
- BEEF Spec: BRC-62
- Key Derivation: BRC-42, BRC-43
