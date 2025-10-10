# SDK Types Module

## Overview

Complete Rust translation of TypeScript SDK types from `wallet-toolbox`.

**TypeScript Source:** `src/sdk/types.ts` (215 lines)

**Rust Implementation:**
- `src/sdk/types.rs` (520+ lines)
- `src/sdk/types_tests.rs` (540+ lines with 47 tests)

## Status

✅ **COMPLETE** - All types implemented with functional parity

### Implemented Types

#### 1. **OutPoint** - Transaction Output Reference
```rust
pub struct OutPoint {
    pub txid: String,
    pub vout: u32,
}
```
- ✅ Identifies unique transaction outputs
- ✅ String formatting "txid:vout"
- ✅ Parsing from string format
- ✅ Full serialization support
- ✅ Hash and equality traits

#### 2. **Chain** - Network Identifier
```rust
pub enum Chain {
    Main,
    Test,
}
```
- ✅ Serializes to "main"/"test"
- ✅ Display trait implementation
- ✅ String parsing with validation

#### 3. **ProvenTxReqStatus** - Transaction Request Status (13 variants)
```rust
pub enum ProvenTxReqStatus {
    Sending, Unsent, Nosend, Unknown, Nonfinal,
    Unprocessed, Unmined, Callback, Unconfirmed,
    Completed, Invalid, DoubleSpend, Unfail,
}
```
- ✅ Terminal vs non-terminal classification
- ✅ Helper methods: `is_terminal()`, `is_non_terminal()`
- ✅ Static arrays: `terminal_statuses()`, `non_terminal_statuses()`
- ✅ Comprehensive documentation matching TypeScript comments

#### 4. **TransactionStatus** - Transaction Status (9 variants)
```rust
pub enum TransactionStatus {
    Completed, Failed, Unprocessed, Sending,
    Unproven, Unsigned, Nosend, Nonfinal, Unfail,
}
```
- ✅ All status variants
- ✅ Display and serialization

#### 5. **Paged** - Pagination Parameters
```rust
pub struct Paged {
    pub limit: u32,
    pub offset: Option<u32>,
}
```
- ✅ Optional offset field
- ✅ Builder methods: `new()`, `with_offset()`
- ✅ Skips serializing None offset

#### 6. **KeyPair** - Public/Private Key Pair
```rust
pub struct KeyPair {
    pub private_key: String,
    pub public_key: String,
}
```
- ✅ CamelCase field names in JSON (privateKey, publicKey)
- ✅ Snake_case in Rust (private_key, public_key)

#### 7. **StorageIdentity** - Storage Identification
```rust
pub struct StorageIdentity {
    pub storage_identity_key: String,
    pub storage_name: String,
}
```
- ✅ CamelCase JSON serialization
- ✅ Constructor method

#### 8. **EntityTimeStamp** - Entity Timestamps
```rust
pub struct EntityTimeStamp {
    pub created_at: String,
    pub updated_at: String,
}
```
- ✅ ISO 8601 datetime strings
- ✅ `now()` method using chrono
- ✅ Custom timestamp constructor

#### 9. **WalletBalance** - Balance Information
```rust
pub struct WalletBalance {
    pub total: u64,
    pub utxos: Vec<UtxoBalance>,
}
```
- ✅ Total satoshis
- ✅ UTXO breakdown
- ✅ `empty()` constructor

#### 10. **ReqHistoryNote** - Request History Tracking
```rust
pub struct ReqHistoryNote {
    pub when: Option<String>,
    pub what: String,
    pub extra: HashMap<String, serde_json::Value>,
}
```
- ✅ Flexible extra fields via HashMap
- ✅ Flatten serialization (extra fields at top level)
- ✅ Builder pattern with `with_field()`
- ✅ Optional timestamp

### Special Operation Constants

All 6 special operation identifiers implemented:

#### ListOutputs Operations
- ✅ `SPEC_OP_WALLET_BALANCE` - Get wallet balance
- ✅ `SPEC_OP_INVALID_CHANGE` - Find invalid change outputs
- ✅ `SPEC_OP_SET_WALLET_CHANGE_PARAMS` - Update change params

#### ListActions Operations
- ✅ `SPEC_OP_NO_SEND_ACTIONS` - Process nosend actions
- ✅ `SPEC_OP_FAILED_ACTIONS` - Process failed actions

#### CreateAction Operations
- ✅ `SPEC_OP_THROW_REVIEW_ACTIONS` - Test review actions error

### Helper Functions

- ✅ `is_list_outputs_spec_op()` - Check if basket is special operation
- ✅ `is_list_actions_spec_op()` - Check if label is special operation
- ✅ `is_create_action_spec_op()` - Check if label is special operation

## Test Coverage

**47 Tests - All Passing ✅**

### Test Breakdown

1. **OutPoint Tests** (6 tests)
   - Construction, formatting, parsing
   - Error handling for invalid formats
   - Serialization and equality

2. **Chain Tests** (5 tests)
   - String conversion (both directions)
   - Display trait
   - Serialization/deserialization
   - Case sensitivity validation

3. **ProvenTxReqStatus Tests** (8 tests)
   - Terminal/non-terminal classification
   - Status arrays
   - String conversion
   - Serialization for all 13 variants

4. **TransactionStatus Tests** (3 tests)
   - String conversion
   - Display trait
   - Serialization

5. **Paged Tests** (4 tests)
   - Construction with/without offset
   - Serialization
   - Optional field handling

6. **KeyPair Tests** (3 tests)
   - Construction
   - Field name mapping (camelCase JSON)
   - Serialization

7. **StorageIdentity Tests** (2 tests)
   - Construction
   - Serialization with field names

8. **EntityTimeStamp Tests** (3 tests)
   - Current timestamp generation
   - Custom timestamps
   - Serialization

9. **WalletBalance Tests** (3 tests)
   - Empty balance
   - Balance with UTXOs
   - Serialization

10. **ReqHistoryNote Tests** (5 tests)
    - Basic construction
    - Timestamp handling
    - Extra fields builder
    - Flatten serialization
    - Serialization round-trip

11. **Special Operations Tests** (5 tests)
    - Constant value verification
    - Helper function testing
    - Category exclusivity

## Usage Examples

### OutPoint
```rust
use wallet_core::sdk::types::OutPoint;

// Create
let outpoint = OutPoint::new("abc123", 0);

// Format as string
let s = outpoint.to_string_format(); // "abc123:0"

// Parse from string
let parsed = OutPoint::from_string_format("abc123:0").unwrap();
```

### Chain
```rust
use wallet_core::sdk::types::Chain;

let chain = Chain::Main;
println!("{}", chain); // "main"

let parsed = Chain::from_str("test").unwrap();
```

### Status Checking
```rust
use wallet_core::sdk::types::ProvenTxReqStatus;

let status = ProvenTxReqStatus::Completed;
assert!(status.is_terminal());

let terminals = ProvenTxReqStatus::terminal_statuses();
```

### Pagination
```rust
use wallet_core::sdk::types::Paged;

let page1 = Paged::new(20);
let page2 = Paged::with_offset(20, 40);
```

### History Notes with Extra Fields
```rust
use wallet_core::sdk::types::ReqHistoryNote;

let note = ReqHistoryNote::new("Transaction sent")
    .with_field("status", serde_json::json!("success"))
    .with_field("txid", serde_json::json!("abc123"))
    .with_field("amount", serde_json::json!(1000));
```

### Special Operations
```rust
use wallet_core::sdk::types::*;

if is_list_outputs_spec_op(basket_name) {
    // Handle special operation
}

if is_list_actions_spec_op(label) {
    // Handle special operation
}
```

## API Parity with TypeScript

### Type Mapping

| TypeScript | Rust | Notes |
|-----------|------|-------|
| `interface OutPoint` | `struct OutPoint` | ✅ Complete |
| `type Chain = 'main' \| 'test'` | `enum Chain` | ✅ Serializes to strings |
| `type ProvenTxReqStatus` | `enum ProvenTxReqStatus` | ✅ 13 variants |
| `type TransactionStatus` | `enum TransactionStatus` | ✅ 9 variants |
| `interface Paged` | `struct Paged` | ✅ Optional offset |
| `interface KeyPair` | `struct KeyPair` | ✅ CamelCase JSON |
| `interface StorageIdentity` | `struct StorageIdentity` | ✅ CamelCase JSON |
| `interface EntityTimeStamp` | `struct EntityTimeStamp` | ✅ ISO 8601 strings |
| `interface WalletBalance` | `struct WalletBalance` | ✅ Complete |
| `type ReqHistoryNote` | `struct ReqHistoryNote` | ✅ Flatten extra fields |
| `const specOp*` | `const SPEC_OP_*` | ✅ All 6 constants |
| `function is*SpecOp()` | `fn is_*_spec_op()` | ✅ All 3 helpers |

### Serialization Compatibility

All types serialize to JSON format compatible with TypeScript:

```rust
// Rust
let kp = KeyPair::new("priv", "pub");
serde_json::to_string(&kp).unwrap();
// Output: {"privateKey":"priv","publicKey":"pub"}

// Matches TypeScript JSON.stringify(kp)
```

### Constants Verification

All special operation constants match TypeScript exactly (SHA-256 hashes):
- ✅ Verified against TypeScript source
- ✅ Test ensures no typos or changes

## Integration

This module is re-exported from `wallet-core::sdk`:

```rust
use wallet_core::sdk::{
    Chain, OutPoint, ProvenTxReqStatus, 
    TransactionStatus, Paged, ReqHistoryNote
};
```

Used by:
- Storage layer (OutPoint, TransactionStatus)
- Wallet operations (Chain, Paged)
- History tracking (ReqHistoryNote)
- Special operations (constants and helpers)

## Dependencies

- `serde` - Serialization
- `serde_json` - JSON support and Value type
- `chrono` - Timestamp generation

## Build & Test

```bash
# Run all SDK tests (errors + types)
cargo test --package wallet-core --lib sdk

# Run just types tests
cargo test --package wallet-core --lib types_tests

# With output
cargo test --package wallet-core --lib types_tests -- --nocapture
```

## Verification

All tests pass ✅
```
test result: ok. 47 passed; 0 failed; 0 ignored; 0 measured
```

Total SDK tests: 70 (23 errors + 47 types) ✅

Zero compilation warnings ✅

Perfect TypeScript parity ✅

## Next Steps

This types module provides the foundation for:
- Storage schema definitions (using OutPoint, TransactionStatus)
- Wallet operations (using Chain, Paged)
- Service integrations (using status enums)
- History tracking (using ReqHistoryNote)

## Implementation Notes

### Design Decisions

1. **Enum for statuses**: TypeScript union types map to Rust enums with serde rename for exact JSON compatibility

2. **String timestamps**: Used ISO 8601 strings instead of chrono types for serialization simplicity

3. **Flexible history notes**: HashMap with flatten for arbitrary extra fields, matching TypeScript behavior

4. **Constants as &str**: Special operation identifiers as string constants for compile-time validation

5. **Builder patterns**: Optional methods like `with_offset()`, `with_field()` for ergonomic construction

### Testing Philosophy

- Test construction, serialization, and behavior
- Verify exact TypeScript compatibility (field names, values)
- Test error cases (invalid parsing)
- Ensure constant values match exactly
- Test helper function logic and exclusivity

## References

- TypeScript source: `wallet-toolbox/src/sdk/types.ts`
- Translation plan: `../../TRANSLATION_PLAN.md`
- Mapping document: `../../docs/mapping.md`
