//! HMAC Operations
//!
//! Create and verify HMAC signatures using wallet-derived keys.
//! Reference: wallet-toolbox SDK createHmac/verifyHmac methods

use crate::crypto::signing::{hmac_sha256, verify_hmac_sha256};
use crate::keys::key_deriver::KeyDeriver;
use crate::sdk::{
    CreateHmacArgs, CreateHmacResult, VerifyHmacArgs, VerifyHmacResult, WalletError,
    WalletResult,
};

/// Create an HMAC using a wallet-derived key
///
/// Derives a key using the protocol ID, key ID, and counterparty,
/// then creates an HMAC-SHA256 of the provided data.
///
/// # Arguments
/// * `args` - HMAC creation arguments (protocol, key ID, data, counterparty)
/// * `key_deriver` - Key derivation service
///
/// # Returns
/// HMAC bytes (32 bytes)
///
/// Reference: TypeScript `createHmac()` in SDK
pub async fn create_hmac(
    args: &CreateHmacArgs,
    key_deriver: &dyn KeyDeriver,
) -> WalletResult<CreateHmacResult> {
    // Derive the HMAC key using protocol ID, key ID, and counterparty
    let counterparty = args.counterparty.as_deref().unwrap_or("self");
    
    let derived_key = key_deriver
        .derive_key(
            &args.protocol_id,
            &args.key_id,
            counterparty,
        )
        .await
        .map_err(|e| WalletError::internal(format!("Key derivation failed: {}", e)))?;
    
    // Create HMAC using the derived key
    let hmac = hmac_sha256(&derived_key, &args.data);
    
    Ok(CreateHmacResult { hmac })
}

/// Verify an HMAC using a wallet-derived key
///
/// Derives the same key and verifies the HMAC matches.
///
/// # Arguments  
/// * `args` - HMAC verification arguments (protocol, key ID, data, hmac, counterparty)
/// * `key_deriver` - Key derivation service
///
/// # Returns
/// `{ valid: true }` on success, error on failure
///
/// Reference: TypeScript `verifyHmac()` in SDK
pub async fn verify_hmac(
    args: &VerifyHmacArgs,
    key_deriver: &dyn KeyDeriver,
) -> WalletResult<VerifyHmacResult> {
    // Derive the same HMAC key
    let counterparty = args.counterparty.as_deref().unwrap_or("self");
    
    let derived_key = key_deriver
        .derive_key(
            &args.protocol_id,
            &args.key_id,
            counterparty,
        )
        .await
        .map_err(|e| WalletError::internal(format!("Key derivation failed: {}", e)))?;
    
    // Verify HMAC
    let valid = verify_hmac_sha256(&derived_key, &args.data, &args.hmac);
    
    if !valid {
        return Err(WalletError::invalid_parameter(
            "hmac",
            "HMAC verification failed",
        ));
    }
    
    Ok(VerifyHmacResult { valid: true })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    
    // Mock key deriver for testing
    struct MockKeyDeriver;
    
    #[async_trait]
    impl KeyDeriver for MockKeyDeriver {
        async fn derive_key(
            &self,
            _protocol_id: &(u8, String),
            _key_id: &str,
            _counterparty: &str,
        ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
            // Return a fixed test key
            Ok(vec![0x42; 32])
        }
        
        async fn derive_public_key(
            &self,
            _protocol_id: &(u8, String),
            _key_id: &str,
            _counterparty: &str,
            _for_self: bool,
        ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(vec![0x03; 33])
        }
    }
    
    #[tokio::test]
    async fn test_create_hmac_basic() {
        let args = CreateHmacArgs {
            protocol_id: (0, "test".to_string()),
            key_id: "key1".to_string(),
            data: vec![1, 2, 3, 4],
            counterparty: None,
            privileged: None,
            privileged_reason: None,
        };
        
        let deriver = MockKeyDeriver;
        let result = create_hmac(&args, &deriver).await.unwrap();
        
        // HMAC should be 32 bytes
        assert_eq!(result.hmac.len(), 32);
    }
    
    #[tokio::test]
    async fn test_verify_hmac_valid() {
        let args = CreateHmacArgs {
            protocol_id: (0, "test".to_string()),
            key_id: "key1".to_string(),
            data: vec![1, 2, 3, 4],
            counterparty: None,
            privileged: None,
            privileged_reason: None,
        };
        
        let deriver = MockKeyDeriver;
        let created = create_hmac(&args, &deriver).await.unwrap();
        
        // Verify the HMAC
        let verify_args = VerifyHmacArgs {
            protocol_id: args.protocol_id.clone(),
            key_id: args.key_id.clone(),
            data: args.data.clone(),
            hmac: created.hmac,
            counterparty: None,
            privileged: None,
            privileged_reason: None,
        };
        
        let result = verify_hmac(&verify_args, &deriver).await.unwrap();
        assert!(result.valid);
    }
    
    #[tokio::test]
    async fn test_verify_hmac_invalid() {
        let verify_args = VerifyHmacArgs {
            protocol_id: (0, "test".to_string()),
            key_id: "key1".to_string(),
            data: vec![1, 2, 3, 4],
            hmac: vec![0xFF; 32], // Wrong HMAC
            counterparty: None,
            privileged: None,
            privileged_reason: None,
        };
        
        let deriver = MockKeyDeriver;
        let result = verify_hmac(&verify_args, &deriver).await;
        
        // Should fail
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_hmac_deterministic() {
        let args = CreateHmacArgs {
            protocol_id: (0, "test".to_string()),
            key_id: "key1".to_string(),
            data: vec![1, 2, 3, 4],
            counterparty: None,
            privileged: None,
            privileged_reason: None,
        };
        
        let deriver = MockKeyDeriver;
        let hmac1 = create_hmac(&args, &deriver).await.unwrap();
        let hmac2 = create_hmac(&args, &deriver).await.unwrap();
        
        // Should produce the same HMAC
        assert_eq!(hmac1.hmac, hmac2.hmac);
    }
}
