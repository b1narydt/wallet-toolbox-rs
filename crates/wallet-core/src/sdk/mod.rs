// SDK module stubs mirroring TS structure

pub mod action;
pub mod action_list;
pub mod action_process;
pub mod errors;
pub mod types;
pub mod validation;
pub mod validation_args;
pub mod wallet_interface;

#[cfg(test)]
#[path = "types_tests.rs"]
mod types_tests;

#[derive(Debug, Default)]
pub struct PrivilegedKeyManager;

pub mod index {}

// Re-export commonly used items
pub use action::*;
pub use action_list::*;
pub use action_process::*;
pub use errors::{WalletError, WalletResult, WalletNetwork};
pub use types::{
    Chain, OutPoint, ProvenTxReqStatus, TransactionStatus, Paged, ReqHistoryNote,
    StorageProvidedBy, SyncStatus,
};
pub use validation::*;
pub use validation_args::*;
pub use wallet_interface::*;

// Ensure error module is accessible as both 'error' and 'errors'
pub mod error {
    pub use super::errors::*;
}
