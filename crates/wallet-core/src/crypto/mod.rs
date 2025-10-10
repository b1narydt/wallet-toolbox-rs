//! Cryptographic Operations
//!
//! Pure Rust cryptographic primitives for Bitcoin transactions.
//! Uses secp256k1 for ECDSA signing (high performance).
//!
//! **Reference**: TypeScript bsv-sdk cryptographic operations

pub mod signing;
pub mod keys;
pub mod symmetric;

pub use signing::{sign_ecdsa, verify_signature as verify_ecdsa, sha256, double_sha256, hmac_sha256, verify_hmac_sha256};
pub use keys::{derive_public_key, KeyDerivationError};
pub use symmetric::{encrypt_with_aes_gcm, decrypt_with_aes_gcm};
