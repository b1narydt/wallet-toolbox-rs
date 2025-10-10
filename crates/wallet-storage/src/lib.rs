//! Storage traits and types for wallet operations
//!
//! Translates TypeScript WalletStorage interfaces to Rust traits.
//! Reference: wallet-toolbox/src/sdk/WalletStorage.interfaces.ts

use async_trait::async_trait;
use thiserror::Error;

pub mod schema;
pub mod methods;
pub mod sync;
pub mod types;

// Re-export commonly used types
pub use schema::tables::*;
pub use types::*;

/// Unified error for storage operations
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
    
    #[error("io error: {0}")]
    Io(String),
    
    #[error("database error: {0}")]
    Database(String),
    
    #[error("invalid argument: {0}")]
    InvalidArg(String),
    
    #[error("not found: {0}")]
    NotFound(String),
    
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("conflict: {0}")]
    Conflict(String),
}

pub type StorageResult<T> = Result<T, StorageError>;

/// Reader capabilities - read-only storage operations
///
/// Matches TypeScript `WalletStorageReader` interface
#[async_trait]
pub trait WalletStorageReader: Send + Sync {
    /// Check if storage is available
    fn is_available(&self) -> bool;
    
    /// Get storage settings
    fn get_settings(&self) -> &TableSettings;
    
    /// Find certificates with filters
    async fn find_certificates_auth(
        &self,
        auth: &AuthId,
        args: &FindCertificatesArgs,
    ) -> StorageResult<Vec<TableCertificate>>;
    
    /// Find output baskets
    async fn find_output_baskets_auth(
        &self,
        auth: &AuthId,
        args: &FindOutputBasketsArgs,
    ) -> StorageResult<Vec<TableOutputBasket>>;
    
    /// Find outputs with filters
    async fn find_outputs_auth(
        &self,
        auth: &AuthId,
        args: &FindOutputsArgs,
    ) -> StorageResult<Vec<TableOutput>>;
    
    /// Find proven transaction requests
    async fn find_proven_tx_reqs(
        &self,
        args: &FindProvenTxReqsArgs,
    ) -> StorageResult<Vec<TableProvenTxReq>>;
}

/// Writer capabilities - write operations on storage
///
/// Matches TypeScript `WalletStorageWriter` interface
#[async_trait]
pub trait WalletStorageWriter: WalletStorageReader {
    /// Initialize or verify storage is available
    async fn make_available(&mut self) -> StorageResult<TableSettings>;
    
    /// Migrate storage to new identity
    async fn migrate(
        &mut self,
        storage_name: &str,
        storage_identity_key: &str,
    ) -> StorageResult<String>;
    
    /// Destroy storage (dangerous operation)
    async fn destroy(&mut self) -> StorageResult<()>;
    
    /// Find or create user by identity key
    async fn find_or_insert_user(
        &mut self,
        identity_key: &str,
    ) -> StorageResult<FindOrInsertUserResult>;
    
    /// Insert or update a certificate
    async fn insert_certificate_auth(
        &mut self,
        auth: &AuthId,
        certificate: &TableCertificate,
    ) -> StorageResult<i64>;
}

/// Sync capabilities - synchronization between storage providers
///
/// Matches TypeScript `WalletStorageSync` interface
#[async_trait]
pub trait WalletStorageSync: WalletStorageWriter {
    /// Find or create sync state
    async fn find_or_insert_sync_state_auth(
        &mut self,
        auth: &AuthId,
        storage_identity_key: &str,
        storage_name: &str,
    ) -> StorageResult<FindOrInsertSyncStateResult>;
    
    /// Set active storage for user
    async fn set_active(
        &mut self,
        auth: &AuthId,
        new_active_storage_identity_key: &str,
    ) -> StorageResult<i64>;
}

/// Full storage provider interface
///
/// Matches TypeScript `WalletStorageProvider` interface
#[async_trait]
pub trait WalletStorageProvider: WalletStorageSync {
    /// Check if this is a storage provider (vs manager)
    fn is_storage_provider(&self) -> bool {
        true
    }
    
    // ============================================================================
    // Transaction Creation Methods (createAction requirements)
    // ============================================================================
    
    /// Count available change inputs in basket
    /// Reference: StorageKnex.ts line 1034
    async fn count_change_inputs(
        &self,
        user_id: i64,
        basket_id: i64,
        exclude_sending: bool,
    ) -> StorageResult<i64>;
    
    /// Allocate a change input for transaction funding
    /// Reference: StorageKnex.ts line 1049
    async fn allocate_change_input(
        &mut self,
        user_id: i64,
        basket_id: i64,
        target_satoshis: i64,
        exact_satoshis: Option<i64>,
        exclude_sending: bool,
        transaction_id: i64,
    ) -> StorageResult<Option<TableOutput>>;
    
    /// Verify transaction is known and valid
    /// Reference: StorageProvider.ts line 436
    async fn verify_known_valid_transaction(&self, txid: &str) -> StorageResult<bool>;
    
    /// Get proven or raw transaction
    /// Reference: StorageKnex.ts line 82
    async fn get_proven_or_raw_tx(&self, txid: &str) -> StorageResult<ProvenOrRawTx>;
    
    /// Get raw tx of known valid transaction
    /// Reference: StorageKnex.ts line 111
    async fn get_raw_tx_of_known_valid_transaction(
        &self,
        txid: &str,
        offset: Option<usize>,
        length: Option<usize>,
    ) -> StorageResult<Option<Vec<u8>>>;
    
    /// Find transactions with filters
    /// Reference: signAction.ts line 42, StorageReaderWriter.ts
    async fn find_transactions(
        &self,
        user_id: i64,
        reference: Option<&str>,
        status: Option<crate::TransactionStatus>,
    ) -> StorageResult<Vec<TableTransaction>>;
    
    /// Find outputs by transaction (as inputs or outputs)
    /// Reference: signAction.ts lines 62-75
    async fn find_outputs_by_transaction(
        &self,
        user_id: i64,
        transaction_id: i64,
        is_input: bool, // true = spent_by, false = transaction_id
    ) -> StorageResult<Vec<TableOutput>>;
    
    /// Insert transaction
    /// Reference: StorageReaderWriter.ts (via insertTransaction)
    async fn insert_transaction(&mut self, tx: &TableTransaction) -> StorageResult<i64>;
    
    /// Update transaction satoshis
    /// Reference: createAction.ts line 129
    async fn update_transaction(&mut self, transaction_id: i64, satoshis: i64) -> StorageResult<()>;
    
    /// Update transaction status
    /// Reference: signAction.ts line 188
    async fn update_transaction_status(&mut self, transaction_id: i64, status: TransactionStatus) -> StorageResult<()>;
    
    /// Update transaction txid
    /// Reference: signAction.ts line 189
    async fn update_transaction_txid(&mut self, transaction_id: i64, txid: &str) -> StorageResult<()>;
    
    /// Update transaction raw transaction bytes
    /// Reference: signAction.ts line 190
    async fn update_transaction_raw_tx(&mut self, transaction_id: i64, raw_tx: &[u8]) -> StorageResult<()>;
    
    /// Insert output
    /// Reference: StorageReaderWriter.ts
    async fn insert_output(&mut self, output: &TableOutput) -> StorageResult<i64>;
    
    /// Update output
    /// Reference: StorageReaderWriter.ts
    async fn update_output(&mut self, output_id: i64, updates: &OutputUpdates) -> StorageResult<()>;
    
    /// Insert commission
    /// Reference: createAction.ts line 329
    async fn insert_commission(&mut self, commission: &TableCommission) -> StorageResult<i64>;
    
    /// Find or insert output basket
    /// Reference: StorageReaderWriter.ts line 206
    async fn find_or_insert_output_basket(&mut self, user_id: i64, name: &str) -> StorageResult<TableOutputBasket>;
    
    /// Find or insert output tag
    /// Reference: StorageReaderWriter.ts line 291
    async fn find_or_insert_output_tag(&mut self, user_id: i64, tag: &str) -> StorageResult<TableOutputTag>;
    
    /// Find or insert output tag map
    /// Reference: StorageReaderWriter.ts line 319
    async fn find_or_insert_output_tag_map(&mut self, output_id: i64, output_tag_id: i64) -> StorageResult<()>;
    
    /// Find or insert transaction label
    /// Reference: StorageReaderWriter.ts line 236
    async fn find_or_insert_tx_label(&mut self, user_id: i64, label: &str) -> StorageResult<TableTxLabel>;
    
    /// Find or insert transaction label map
    /// Reference: StorageReaderWriter.ts line 264
    async fn find_or_insert_tx_label_map(&mut self, transaction_id: i64, tx_label_id: i64) -> StorageResult<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_error() {
        let err = StorageError::NotFound("test".to_string());
        assert!(err.to_string().contains("not found"));
    }
}
