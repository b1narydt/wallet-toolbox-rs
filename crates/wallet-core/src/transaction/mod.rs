//! Bitcoin Transaction Implementation
//!
//! Pure Rust implementation of Bitcoin SV transactions for signing and verification.
//! This provides the performance benefits of Rust for cryptographic operations.
//!
//! **Reference**: TypeScript @bsv/sdk Transaction class
//!
//! ## Overview
//!
//! This module implements the core Bitcoin transaction primitives:
//! - Transaction structure (inputs, outputs, version, lockTime)
//! - Transaction serialization
//! - Txid calculation (double SHA-256)
//! - Sighash calculation for signing
//! - Script operations
//!
//! ## Design Philosophy
//!
//! Implemented in pure Rust using:
//! - `secp256k1` for ECDSA signatures (performance-critical)
//! - `sha2` for SHA-256 hashing
//! - Native Rust for all serialization/deserialization
//!
//! This avoids FFI calls to TypeScript for crypto operations, maintaining
//! the performance benefits of Rust.

pub mod outpoint;
pub mod tx_input;
pub mod tx_output;
pub mod transaction;
pub mod sighash;
pub mod script;

pub use outpoint::OutPoint;
pub use tx_input::TxInput;
pub use tx_output::TxOutput;
pub use transaction::Transaction;
pub use sighash::{SigHash, SigHashType};
pub use script::Script;

/// Transaction error types
#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("invalid transaction format: {0}")]
    InvalidFormat(String),
    
    #[error("serialization error: {0}")]
    Serialization(String),
    
    #[error("invalid script: {0}")]
    InvalidScript(String),
    
    #[error("signing error: {0}")]
    Signing(String),
    
    #[error("invalid signature: {0}")]
    InvalidSignature(String),
}

pub type TransactionResult<T> = Result<T, TransactionError>;
