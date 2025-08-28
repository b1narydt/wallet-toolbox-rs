//! Storage traits and models (placeholder)
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod schema;
pub mod methods;
pub mod sync;

/// Unified error for storage operations
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
    #[error("io error: {0}")]
    Io(String),
    #[error("db error: {0}")]
    Db(String),
    #[error("invalid argument: {0}")]
    InvalidArg(&'static str),
    #[error("not found")]
    NotFound,
}

/// Placeholder types that will be replaced with concrete SDK/domain types during translation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction; // maps to @bsv/sdk Transaction

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beef; // maps to @bsv/sdk Beef

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubKeyHex(String);

/// Results and argument shapes (to be expanded per TS definitions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListActionsResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOutputsResult;

/// Reader capabilities
pub trait StorageReader {
    fn get_transaction(&self, _txid: &str) -> Result<Option<Transaction>, StorageError> {
        Err(StorageError::NotImplemented("get_transaction"))
    }
    fn list_actions(&self) -> Result<ListActionsResult, StorageError> {
        Err(StorageError::NotImplemented("list_actions"))
    }
    fn list_outputs(&self) -> Result<ListOutputsResult, StorageError> {
        Err(StorageError::NotImplemented("list_outputs"))
    }
}

/// Writer capabilities
pub trait StorageWriter {
    fn put_transaction(&mut self, _tx: &Transaction) -> Result<(), StorageError> {
        Err(StorageError::NotImplemented("put_transaction"))
    }
}

/// Combined reader/writer
pub trait StorageReaderWriter: StorageReader + StorageWriter {}

/// The top-level provider interface analogous to TS StorageProvider
pub trait WalletStorageProvider: StorageReaderWriter {
    fn is_dirty(&self) -> bool { false }
    fn commission_satoshis(&self) -> u64 { 0 }
    fn commission_pubkey_hex(&self) -> Option<PubKeyHex> { None }
}
