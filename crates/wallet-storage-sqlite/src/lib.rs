//! SQLite storage backend for wallet operations
//!
//! Provides a rusqlite-based implementation of the WalletStorage traits.
//! Translates TypeScript StorageKnex to Rust.
//! Reference: wallet-toolbox/src/storage/StorageKnex.ts

pub mod migrations;
pub mod storage_sqlite;
pub mod transaction_ops;
pub mod output_ops;
pub mod proven_tx_ops;
pub mod basket_tag_label_ops;
pub mod cert_commission_ops;

pub use storage_sqlite::StorageSqlite;

// Re-export commonly used types
pub use wallet_storage::*;

pub fn init() {}
