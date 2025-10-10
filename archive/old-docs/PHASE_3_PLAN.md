# Phase 3: Core Wallet Implementation Plan

**Version**: 1.5.0 (Phase 3 Start)  
**Start Date**: 2025-01-06  
**Target Duration**: Weeks 5-7 (3 weeks)  
**Current Status**: Phase 2 Complete (100%), Starting Phase 3

## Phase 3 Overview

Implement core wallet business logic by translating TypeScript wallet methods to Rust. This phase focuses on transaction building, key derivation, output management, and action processing.

**Reference**: `@wallet-toolbox/src/` (all TypeScript implementation)

---

## 3.1 SDK Module Interfaces (Week 5)

### Core SDK Types
**Reference**: `src/sdk/` TypeScript modules

#### Priority 1: Action Interfaces
- [ ] `ValidCreateActionInput` 
- [ ] `ValidCreateActionOutput`
- [ ] `CreateActionResult`
- [ ] `InternalizeActionResult`
- [ ] `ListActionsArgs` / `ListActionsResult`
- [ ] `AbortActionArgs`

#### Priority 2: Transaction Building
- [ ] `StorageCreateActionResult`
- [ ] `StorageProcessActionArgs`
- [ ] `SignActionArgs` / `SignActionResult`
- [ ] Transaction input/output builders

#### Priority 3: Certificate Management
- [ ] Certificate creation interfaces
- [ ] Field management
- [ ] Verification types

**Files to Create**:
```
wallet-core/src/sdk/
├── action.rs           (Action types & interfaces)
├── transaction.rs      (Transaction building types)
├── certificate.rs      (Certificate interfaces)
└── mod.rs             (Re-exports)
```

---

## 3.2 Transaction Building (Week 5-6)

### Core Transaction Methods
**Reference**: `src/methods/createAction.ts`, `signAction.ts`

#### createAction Implementation
- [ ] Input validation and selection
- [ ] Output creation and validation
- [ ] Derivation key management
- [ ] Fee calculation
- [ ] Change output handling
- [ ] `noSendChange` support

**Key Functions**:
```rust
async fn create_action(
    storage: &impl WalletStorageProvider,
    args: ValidCreateActionArgs,
) -> Result<CreateActionResult, WalletError>

async fn validate_create_action_inputs(
    inputs: &[ValidCreateActionInput],
) -> Result<(), WalletError>

async fn validate_create_action_outputs(
    outputs: &[ValidCreateActionOutput],
) -> Result<(), WalletError>
```

#### signAction Implementation
- [ ] Transaction signing
- [ ] Unlocking script generation
- [ ] Broadcast preparation
- [ ] Delayed broadcast handling

**Key Functions**:
```rust
async fn sign_action(
    storage: &mut impl WalletStorageProvider,
    args: SignActionArgs,
) -> Result<SignActionResult, WalletError>

async fn build_unlocking_scripts(
    inputs: &[StorageInput],
    tx: &Transaction,
) -> Result<Vec<UnlockingScript>, WalletError>
```

**Files to Create**:
```
wallet-core/src/methods/
├── create_action.rs      (Action creation logic)
├── sign_action.rs        (Transaction signing)
├── validate_action.rs    (Input/output validation)
└── mod.rs               (Method re-exports)
```

---

## 3.3 Key Derivation (Week 6)

### Derivation Protocol Implementation
**Reference**: BRC-42, BRC-43, `src/methods/` derivation logic

#### Key Derivation Methods
- [ ] Protocol/key ID derivation
- [ ] Counterparty key derivation
- [ ] Invoice number generation
- [ ] Public key derivation for outputs

**Key Functions**:
```rust
fn derive_key(
    root_key: &PrivateKey,
    protocol_id: &[u8; 2],
    key_id: &str,
    counterparty: Option<&PublicKey>,
    invoice_number: Option<&str>,
) -> Result<PrivateKey, WalletError>

fn derive_public_key(
    sender_identity: &PublicKey,
    recipient_identity: &PublicKey,
    invoice_number: &str,
) -> Result<PublicKey, WalletError>
```

**Files to Create**:
```
wallet-core/src/derivation/
├── brc42.rs             (Protocol ID derivation)
├── brc43.rs             (Sender/recipient derivation)
├── invoice.rs           (Invoice number generation)
└── mod.rs              (Derivation re-exports)
```

---

## 3.4 Output Management (Week 6-7)

### Output Selection & Management
**Reference**: `src/methods/` output methods

#### listOutputs Implementation
- [ ] Output filtering by basket
- [ ] Spendable output selection
- [ ] Transaction status integration
- [ ] Pagination support

**Key Functions**:
```rust
async fn list_outputs(
    storage: &impl WalletStorageReader,
    auth: &AuthId,
    args: ValidListOutputsArgs,
) -> Result<ListOutputsResult, WalletError>

async fn select_outputs_for_payment(
    storage: &impl WalletStorageReader,
    user_id: i64,
    amount: i64,
    basket: Option<&str>,
) -> Result<Vec<TableOutput>, WalletError>
```

#### Basket Management
- [ ] Create/delete baskets
- [ ] Manage desired UTXO counts
- [ ] Balance tracking

**Files to Create**:
```
wallet-core/src/methods/
├── list_outputs.rs       (Output listing & filtering)
├── basket_ops.rs         (Basket operations)
└── output_selection.rs   (Coin selection algorithms)
```

---

## 3.5 Action Processing (Week 7)

### Action Lifecycle Methods
**Reference**: `src/methods/internalizeAction.ts`, `processAction.ts`

#### internalizeAction Implementation
- [ ] Parse incoming transactions
- [ ] Match outputs to user
- [ ] Update storage state
- [ ] Handle proven transactions

**Key Functions**:
```rust
async fn internalize_action(
    storage: &mut impl WalletStorageProvider,
    args: InternalizeActionArgs,
) -> Result<InternalizeActionResult, WalletError>

async fn process_incoming_outputs(
    storage: &mut impl WalletStorageWriter,
    tx: &Transaction,
    outputs: &[IncomingOutput],
) -> Result<(), WalletError>
```

#### processAction Implementation
- [ ] Transaction validation
- [ ] Broadcast coordination
- [ ] Status updates
- [ ] Error handling

**Files to Create**:
```
wallet-core/src/methods/
├── internalize_action.rs  (Incoming tx processing)
├── process_action.rs      (Action state machine)
└── abort_action.rs        (Action cancellation)
```

---

## 3.6 Certificate Management (Week 7)

### Certificate Operations
**Reference**: `src/methods/` certificate methods

#### Certificate CRUD
- [ ] Create certificates
- [ ] Add/update fields
- [ ] Certificate verification
- [ ] Revocation handling

**Key Functions**:
```rust
async fn create_certificate(
    storage: &mut impl WalletStorageWriter,
    auth: &AuthId,
    cert: CertificateInput,
) -> Result<TableCertificate, WalletError>

async fn verify_certificate(
    storage: &impl WalletStorageReader,
    cert_id: i64,
) -> Result<bool, WalletError>
```

**Files to Create**:
```
wallet-core/src/methods/
├── certificate_ops.rs     (Certificate CRUD)
└── certificate_verify.rs  (Verification logic)
```

---

## Testing Strategy

### Unit Tests (Target: 150+ tests)
- [ ] Action creation validation (20 tests)
- [ ] Transaction building (25 tests)
- [ ] Key derivation (20 tests)
- [ ] Output selection (25 tests)
- [ ] Internalization (20 tests)
- [ ] Certificate operations (15 tests)
- [ ] Error handling (25 tests)

### Integration Tests (Target: 30+ tests)
- [ ] End-to-end action flow (10 tests)
- [ ] Multi-party transactions (5 tests)
- [ ] Certificate issuance flow (5 tests)
- [ ] Broadcast & confirmation (10 tests)

**Test Files**:
```
wallet-core/src/methods/tests/
├── action_tests.rs
├── transaction_tests.rs
├── derivation_tests.rs
├── output_tests.rs
└── integration_tests.rs
```

---

## Dependencies to Add

### wallet-core/Cargo.toml
```toml
[dependencies]
# BSV SDK (when available)
# bsv-sdk = "1.6"

# Cryptography
secp256k1 = { version = "0.28", features = ["rand", "recovery"] }
sha2 = "0.10"
ripemd = "0.1"

# Encoding
base64 = "0.22"
hex = "0.4"

# Async
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"

# Error handling
thiserror = "1"
anyhow = "1"

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Existing
wallet-storage = { path = "../wallet-storage" }
```

---

## Success Criteria

**Phase 3 Complete When:**
- ✅ All SDK interfaces defined (40+ types)
- ✅ createAction fully implemented & tested
- ✅ signAction fully implemented & tested
- ✅ Key derivation (BRC-42/43) working
- ✅ Output selection algorithms working
- ✅ internalizeAction handling incoming tx
- ✅ Certificate creation & management
- ✅ 180+ tests passing (150 unit + 30 integration)
- ✅ Zero compilation errors
- ✅ Functional parity with @wallet-toolbox core methods

---

## Progress Tracking

Track completion in `STATUS.md`:
```
Phase 3: Core Wallet (Week 5-7) - X% DONE
├── [x] SDK interfaces (40+ types)
├── [ ] createAction implementation
├── [ ] signAction implementation  
├── [ ] Key derivation (BRC-42/43)
├── [ ] Output management
├── [ ] internalizeAction
├── [ ] Certificate operations
└── [ ] 180+ tests passing
```

---

## Next: Phase 4 Preview

After Phase 3, we'll implement:
- Broadcaster service (transaction relay)
- Chain tracker (block monitoring)
- Overlay services integration
- ARC/TAAL/WhatsOnChain adapters

**Estimated Completion**: End of Week 7 (2025-01-20)
