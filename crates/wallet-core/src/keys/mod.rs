//! BRC-42/43 Key Derivation
//!
//! Implementation of BRC-42 (BSV Key Derivation Scheme) and BRC-43
//! (Security Levels, Protocol IDs, Key IDs and Counterparties).
//!
//! **References**:
//! - BRC-42: https://github.com/bitcoin-sv/BRCs/blob/master/key-derivation/0042.md
//! - BRC-43: https://github.com/bitcoin-sv/BRCs/blob/master/key-derivation/0043.md

pub mod brc42;
pub mod brc43;
pub mod derivation;
pub mod key_deriver;

pub use brc42::{derive_child_private_key, derive_child_public_key, compute_shared_secret};
pub use brc43::{InvoiceNumber, SecurityLevel, normalize_protocol_id};
pub use derivation::{derive_key_from_output, KeyDerivationContext};
pub use key_deriver::KeyDeriver;

use crate::sdk::errors::{WalletError, WalletResult};
use sha2::{Sha256, Digest};

/// Key pair (private + public key)
///
/// Reference: TypeScript KeyPair from @bsv/sdk
#[derive(Debug, Clone)]
pub struct KeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

/// Derive an encryption key using BRC-42/43
///
/// Uses protocol ID, key ID, and optionally counterparty to derive a 32-byte
/// encryption key suitable for AES-256.
///
/// # Arguments
///
/// * `root_key` - Wallet's root private key (32 bytes)
/// * `protocol_id` - Protocol identifier (e.g., ["encryption"])
/// * `key_id` - Specific key identifier
/// * `counterparty` - Optional counterparty public key
/// * `privileged` - Whether to use privileged derivation
///
/// # Returns
///
/// 32-byte encryption key
pub fn derive_encryption_key(
    root_key: &[u8],
    protocol_id: &[String],
    key_id: &str,
    counterparty: Option<&str>,
    _privileged: bool,
) -> WalletResult<Vec<u8>> {
    if root_key.len() != 32 {
        return Err(WalletError::invalid_parameter(
            "root_key",
            "Must be 32 bytes"
        ));
    }
    
    // Build derivation path from protocol_id + key_id
    let mut derivation_path = protocol_id.join("/");
    if !derivation_path.is_empty() {
        derivation_path.push('/');
    }
    derivation_path.push_str(key_id);
    
    // Add counterparty if provided
    if let Some(cp) = counterparty {
        derivation_path.push('/');
        derivation_path.push_str(cp);
    }
    
    // For encryption keys, we use HMAC-based derivation which is more appropriate
    // for symmetric encryption than the asymmetric BRC-42 child key derivation
    use hmac::{Hmac, Mac};
    type HmacSha256 = Hmac<Sha256>;
    
    // Use HMAC with root key as the key and derivation path as the message
    let mut mac = HmacSha256::new_from_slice(root_key)
        .map_err(|e| WalletError::invalid_operation(&format!("HMAC init failed: {}", e)))?;
    mac.update(derivation_path.as_bytes());
    let result = mac.finalize();
    let encryption_key = result.into_bytes().to_vec();
    
    Ok(encryption_key)
}
