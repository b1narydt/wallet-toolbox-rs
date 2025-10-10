//! Signature Operations
//!
//! Create and verify ECDSA signatures using wallet-derived keys.
//! Reference: wallet-toolbox SDK createSignature/verifySignature methods

use crate::crypto::signing::{sign_ecdsa, verify_signature as verify_sig_crypto, sha256};
use crate::keys::key_deriver::KeyDeriver;
use crate::sdk::{
    CreateSignatureArgs, CreateSignatureResult, VerifySignatureArgs, VerifySignatureResult,
    WalletError, WalletResult,
};

/// Create an ECDSA signature using a wallet-derived key
///
/// Derives a private key using the protocol ID, key ID, and counterparty,
/// then creates an ECDSA signature of the provided data (or hash).
///
/// # Arguments
/// * `args` - Signature creation arguments
/// * `key_deriver` - Key derivation service
///
/// # Returns
/// DER-encoded ECDSA signature
///
/// Reference: TypeScript `createSignature()` in SDK
pub async fn create_signature(
    args: &CreateSignatureArgs,
    key_deriver: &dyn KeyDeriver,
) -> WalletResult<CreateSignatureResult> {
    // Get the data to sign
    let hash_to_sign = if let Some(ref hash) = args.hash_to_directly_sign {
        // Use provided hash directly
        if hash.len() != 32 {
            return Err(WalletError::invalid_parameter(
                "hashToDirectlySign",
                "Hash must be exactly 32 bytes",
            ));
        }
        hash.clone()
    } else if let Some(ref data) = args.data {
        // Hash the data with SHA-256
        sha256(data)
    } else {
        return Err(WalletError::invalid_parameter(
            "data or hashToDirectlySign",
            "Either data or hashToDirectlySign must be provided",
        ));
    };
    
    // Derive the signing key
    let counterparty = args.counterparty.as_deref().unwrap_or("self");
    
    let derived_key = key_deriver
        .derive_key(
            &args.protocol_id,
            &args.key_id,
            counterparty,
        )
        .await
        .map_err(|e| WalletError::internal(format!("Key derivation failed: {}", e)))?;
    
    // Ensure key is exactly 32 bytes
    if derived_key.len() != 32 {
        return Err(WalletError::internal("Derived key must be 32 bytes"));
    }
    
    let key_array: [u8; 32] = derived_key
        .try_into()
        .map_err(|_| WalletError::internal("Key conversion failed"))?;
    
    // Create ECDSA signature (with default sighash type 0x01)
    let signature = sign_ecdsa(&hash_to_sign, &key_array, 0x01)
        .map_err(|e| WalletError::internal(format!("Signature creation failed: {}", e)))?;
    
    Ok(CreateSignatureResult { signature })
}

/// Verify an ECDSA signature using a wallet-derived public key
///
/// Derives the public key and verifies the signature matches.
///
/// # Arguments  
/// * `args` - Signature verification arguments
/// * `key_deriver` - Key derivation service
///
/// # Returns
/// `{ valid: true }` on success, error on failure
///
/// Reference: TypeScript `verifySignature()` in SDK
pub async fn verify_signature(
    args: &VerifySignatureArgs,
    key_deriver: &dyn KeyDeriver,
) -> WalletResult<VerifySignatureResult> {
    // Get the hash to verify
    let hash_to_verify = if let Some(ref hash) = args.hash_to_directly_verify {
        if hash.len() != 32 {
            return Err(WalletError::invalid_parameter(
                "hashToDirectlyVerify",
                "Hash must be exactly 32 bytes",
            ));
        }
        hash.clone()
    } else if let Some(ref data) = args.data {
        sha256(data)
    } else {
        return Err(WalletError::invalid_parameter(
            "data or hashToDirectlyVerify",
            "Either data or hashToDirectlyVerify must be provided",
        ));
    };
    
    // Derive the public key
    let counterparty = args.counterparty.as_deref().unwrap_or("self");
    let for_self = args.for_self.unwrap_or(true);
    
    let public_key = key_deriver
        .derive_public_key(
            &args.protocol_id,
            &args.key_id,
            counterparty,
            for_self,
        )
        .await
        .map_err(|e| WalletError::internal(format!("Public key derivation failed: {}", e)))?;
    
    // Verify signature
    let valid = verify_sig_crypto(&hash_to_verify, &args.signature, &public_key)
        .map_err(|e| WalletError::internal(format!("Signature verification failed: {}", e)))?;
    
    if !valid {
        return Err(WalletError::invalid_parameter(
            "signature",
            "Signature verification failed",
        ));
    }
    
    Ok(VerifySignatureResult { valid: true })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use crate::crypto::keys::derive_public_key;
    
    // Mock key deriver for testing
    struct MockKeyDeriver {
        private_key: Vec<u8>,
    }
    
    impl MockKeyDeriver {
        fn new() -> Self {
            Self {
                private_key: vec![0x42; 32],
            }
        }
    }
    
    #[async_trait]
    impl KeyDeriver for MockKeyDeriver {
        async fn derive_key(
            &self,
            _protocol_id: &(u8, String),
            _key_id: &str,
            _counterparty: &str,
        ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(self.private_key.clone())
        }
        
        async fn derive_public_key(
            &self,
            _protocol_id: &(u8, String),
            _key_id: &str,
            _counterparty: &str,
            _for_self: bool,
        ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
            let key_array: [u8; 32] = self.private_key.clone().try_into().unwrap();
            let pubkey = derive_public_key(&key_array, true)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            Ok(pubkey)
        }
    }
    
    #[tokio::test]
    async fn test_create_signature_basic() {
        let args = CreateSignatureArgs {
            protocol_id: (0, "test".to_string()),
            key_id: "key1".to_string(),
            data: Some(vec![1, 2, 3, 4]),
            hash_to_directly_sign: None,
            counterparty: None,
            privileged: None,
            privileged_reason: None,
        };
        
        let deriver = MockKeyDeriver::new();
        let result = create_signature(&args, &deriver).await.unwrap();
        
        // Signature should be DER-encoded (typically 70-72 bytes)
        assert!(result.signature.len() >= 70 && result.signature.len() <= 73);
    }
    
    #[tokio::test]
    async fn test_verify_signature_valid() {
        let data = vec![1, 2, 3, 4];
        
        let create_args = CreateSignatureArgs {
            protocol_id: (0, "test".to_string()),
            key_id: "key1".to_string(),
            data: Some(data.clone()),
            hash_to_directly_sign: None,
            counterparty: None,
            privileged: None,
            privileged_reason: None,
        };
        
        let deriver = MockKeyDeriver::new();
        let created = create_signature(&create_args, &deriver).await.unwrap();
        
        // Verify the signature
        let verify_args = VerifySignatureArgs {
            protocol_id: create_args.protocol_id.clone(),
            key_id: create_args.key_id.clone(),
            data: Some(data),
            hash_to_directly_verify: None,
            signature: created.signature,
            for_self: Some(true),
            counterparty: None,
            privileged: None,
            privileged_reason: None,
        };
        
        let result = verify_signature(&verify_args, &deriver).await.unwrap();
        assert!(result.valid);
    }
    
    #[tokio::test]
    async fn test_verify_signature_invalid() {
        let verify_args = VerifySignatureArgs {
            protocol_id: (0, "test".to_string()),
            key_id: "key1".to_string(),
            data: Some(vec![1, 2, 3, 4]),
            hash_to_directly_verify: None,
            signature: vec![0xFF; 71], // Invalid signature
            for_self: Some(true),
            counterparty: None,
            privileged: None,
            privileged_reason: None,
        };
        
        let deriver = MockKeyDeriver::new();
        let result = verify_signature(&verify_args, &deriver).await;
        
        // Should fail
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_signature_with_direct_hash() {
        let hash = vec![0x12; 32];
        
        let args = CreateSignatureArgs {
            protocol_id: (0, "test".to_string()),
            key_id: "key1".to_string(),
            data: None,
            hash_to_directly_sign: Some(hash.clone()),
            counterparty: None,
            privileged: None,
            privileged_reason: None,
        };
        
        let deriver = MockKeyDeriver::new();
        let result = create_signature(&args, &deriver).await.unwrap();
        
        assert!(result.signature.len() >= 70);
    }
    
    #[tokio::test]
    async fn test_signature_deterministic() {
        let args = CreateSignatureArgs {
            protocol_id: (0, "test".to_string()),
            key_id: "key1".to_string(),
            data: Some(vec![1, 2, 3, 4]),
            hash_to_directly_sign: None,
            counterparty: None,
            privileged: None,
            privileged_reason: None,
        };
        
        let deriver = MockKeyDeriver::new();
        let sig1 = create_signature(&args, &deriver).await.unwrap();
        let sig2 = create_signature(&args, &deriver).await.unwrap();
        
        // ECDSA with RFC 6979 should be deterministic
        assert_eq!(sig1.signature, sig2.signature);
    }
}
