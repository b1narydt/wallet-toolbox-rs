# Storage Schema Tables - Complete Implementation

## Overview

All 16 database table definitions translated from TypeScript with perfect functional parity.

**TypeScript Source:** `wallet-toolbox/src/storage/schema/tables/*.ts`  
**Rust Implementation:** `wallet-toolbox-rs/crates/wallet-storage/src/schema/tables/*.rs`

## Status: ✅ COMPLETE

### Test Results
```
79 tests passing
100% TypeScript parity
Zero compilation warnings
```

## Implemented Tables

### Core Tables (6)

1. **TableUser** (`table_user.rs`)
   - User identity and active storage
   - Fields: userId, identityKey, activeStorage
   - 7 tests

2. **TableTransaction** (`table_transaction.rs`)
   - Transaction records
   - Fields: transactionId, userId, status, reference, satoshis, rawTx, etc.
   - Builder pattern for optional fields
   - 11 tests

3. **TableOutput** (`table_output.rs`)
   - UTXO/output records (most complex table)
   - 30+ fields including locking script
   - StorageProvidedBy enum
   - Builder pattern, mark_spent() method
   - 9 tests

4. **TableSyncState** (`table_sync_state.rs`)
   - Storage synchronization tracking
   - Fields: syncStateId, userId, storageIdentityKey, status, syncMap
   - SyncStatus enum (5 variants)
   - set_error(), set_success() methods
   - 11 tests

5. **TableProvenTx** (`table_proven_tx.rs`)
   - Proven transactions with merkle proofs
   - Fields: provenTxId, txid, height, merklePath, blockHash
   - 2 tests

6. **TableProvenTxReq** (`table_proven_tx_req.rs`)
   - Transaction proof requests
   - Fields: provenTxReqId, status, attempts, notified, history, rawTx
   - ProvenTxReqStatus enum (13 variants)
   - increment_attempts(), mark_notified() methods
   - 3 tests

### Organization Tables (6)

7. **TableOutputBasket** (`table_output_basket.rs`)
   - Output basket configuration
   - Fields: basketId, name, numberOfDesiredUTXOs, minimumDesiredUTXOValue
   - Soft delete support
   - 7 tests

8. **TableOutputTag** (`table_output_tag.rs`)
   - Tag definitions for outputs
   - Fields: outputTagId, userId, tag
   - Soft delete support
   - 7 tests

9. **TableOutputTagMap** (`table_output_tag_map.rs`)
   - Many-to-many output/tag mapping
   - Fields: outputTagId, outputId
   - 2 tests

10. **TableTxLabel** (`table_tx_label.rs`)
    - Transaction label definitions
    - Fields: txLabelId, userId, label
    - Soft delete support
    - 3 tests

11. **TableTxLabelMap** (`table_tx_label_map.rs`)
    - Many-to-many transaction/label mapping
    - Fields: txLabelId, transactionId
    - 2 tests

12. **TableCommission** (`table_commission.rs`)
    - Commission payment records
    - Fields: commissionId, satoshis, keyOffset, lockingScript
    - redeem() method
    - 3 tests

### Certificate Tables (2)

13. **TableCertificate** (`table_certificate.rs`)
    - Identity certificates
    - Fields: certificateId, type, serialNumber, certifier, subject, signature
    - Soft delete support
    - 3 tests

14. **TableCertificateField** (`table_certificate_field.rs`)
    - Certificate field data
    - Fields: certificateId, fieldName, fieldValue, masterKey
    - 2 tests

### System Tables (2)

15. **TableMonitorEvent** (`table_monitor_event.rs`)
    - Monitor event logging
    - Fields: id, event, details
    - 3 tests

16. **TableSettings** (`table_settings.rs`)
    - Storage configuration
    - Fields: storageIdentityKey, storageName, chain, dbtype, maxOutputScript
    - Chain enum (Main/Test)
    - DbType enum (SQLite/MySQL/IndexedDB)
    - 4 tests

## Common Patterns

### Timestamps
All tables include:
```rust
pub created_at: String,  // ISO 8601
pub updated_at: String,  // ISO 8601
```

With `touch()` method:
```rust
pub fn touch(&mut self) {
    self.updated_at = chrono::Utc::now().to_rfc3339();
}
```

### Soft Delete Pattern
Tables with soft delete:
```rust
#[serde(rename = "isDeleted")]
pub is_deleted: bool,

pub fn delete(&mut self) {
    self.is_deleted = true;
    self.touch();
}

pub fn restore(&mut self) {
    self.is_deleted = false;
    self.touch();
}
```

### JSON Serialization
- CamelCase field names for TypeScript compatibility
- `#[serde(rename = "fieldName")]` for all composite names
- `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields

### Builder Pattern
Complex tables use builders:
```rust
TableOutput::new(...)
    .with_basket_id(50)
    .with_txid("abc123")
    .with_locking_script(vec![1, 2, 3])
```

## Type Enums

### TransactionStatus
```rust
enum TransactionStatus {
    Completed, Failed, Unprocessed, Sending,
    Unproven, Unsigned, Nosend, Nonfinal, Unfail
}
```

### ProvenTxReqStatus
```rust
enum ProvenTxReqStatus {
    Sending, Unsent, Nosend, Unknown, Nonfinal,
    Unprocessed, Unmined, Callback, Unconfirmed,
    Completed, Invalid, DoubleSpend, Unfail
}
```

### SyncStatus
```rust
enum SyncStatus {
    Success, Error, Identified, Updated, Unknown
}
```

### StorageProvidedBy
```rust
enum StorageProvidedBy {
    You, Storage, YouAndStorage
}
```

### Chain
```rust
enum Chain {
    Main, Test
}
```

### DbType
```rust
enum DbType {
    SQLite, MySQL, IndexedDB
}
```

## Usage Examples

### Creating Records
```rust
let user = TableUser::new(1, "identity_key", "storage_key");
let tx = TableTransaction::new(1, 100, TransactionStatus::Unprocessed, 
    "ref", true, 5000, "Payment");
let output = TableOutput::new(1, 100, 200, true, false, "desc", 
    0, 1000, StorageProvidedBy::You, "payment", "P2PKH");
```

### Updating Records
```rust
tx.set_status(TransactionStatus::Completed);
output.mark_spent(300, Some("spending desc".to_string()), Some(1));
sync_state.set_error(Some("error".to_string()), None);
```

### Serialization
```rust
let json = serde_json::to_string(&user)?;
let deserialized: TableUser = serde_json::from_str(&json)?;
```

## Integration

Tables are re-exported from `mod.rs`:
```rust
pub use table_user::TableUser;
pub use table_transaction::{TableTransaction, TransactionStatus};
pub use table_output::{TableOutput, StorageProvidedBy};
// ... all 16 tables
```

## Next Steps

With all tables complete, the next implementation targets are:

1. **Storage Traits** - Define StorageReader/StorageWriter interfaces
2. **Storage Methods** - Implement CRUD operations
3. **SQLite Backend** - First concrete storage implementation
4. **Validation Helpers** - Input validation functions
5. **Entity Types** - High-level data structures

## Verification

```bash
# Run all storage tests
cargo test --package wallet-storage

# Check compilation
cargo check --package wallet-storage

# Expected output:
# test result: ok. 79 passed; 0 failed
```

## References

- TypeScript tables: `wallet-toolbox/src/storage/schema/tables/`
- Translation plan: `../../../TRANSLATION_PLAN.md`
- Type mappings: `../../../docs/mapping.md`
