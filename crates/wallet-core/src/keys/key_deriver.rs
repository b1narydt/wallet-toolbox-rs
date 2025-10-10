//! Key Deriver Trait
//!
//! Trait for deriving keys from protocol/keyID/counterparty combinations.
//! Used by wallet methods to derive cryptographic keys.

use async_trait::async_trait;

/// Trait for deriving wallet keys
///
/// Implementations should derive keys using BRC-42/BRC-43 or similar schemes.
#[async_trait]
pub trait KeyDeriver: Send + Sync {
    /// Derive a private key
    ///
    /// # Arguments
    /// * `protocol_id` - Protocol identifier tuple (security_level, protocol_name)
    /// * `key_id` - Key identifier string
    /// * `counterparty` - Counterparty identifier ("self", "anyone", or pubkey)
    ///
    /// # Returns
    /// 32-byte private key
    async fn derive_key(
        &self,
        protocol_id: &(u8, String),
        key_id: &str,
        counterparty: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Derive a public key
    ///
    /// # Arguments
    /// * `protocol_id` - Protocol identifier tuple
    /// * `key_id` - Key identifier string
    /// * `counterparty` - Counterparty identifier
    /// * `for_self` - Whether deriving for self (true) or counterparty (false)
    ///
    /// # Returns
    /// 33-byte compressed public key
    async fn derive_public_key(
        &self,
        protocol_id: &(u8, String),
        key_id: &str,
        counterparty: &str,
        for_self: bool,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>;
}
