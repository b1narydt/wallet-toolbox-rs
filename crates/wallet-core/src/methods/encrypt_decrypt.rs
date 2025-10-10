//! Wallet Encryption and Decryption Methods
//!
//! Reference: TS `wallet.encrypt` and `wallet.decrypt` from @bsv/sdk
//!
//! Implements BRC-42/43 key derivation with AES-256-GCM encryption

use crate::sdk::errors::{WalletError, WalletResult};
use crate::keys::derive_encryption_key;
use crate::crypto::{decrypt_with_aes_gcm, encrypt_with_aes_gcm};
use serde::{Deserialize, Serialize};

/// Arguments for wallet encryption
///
/// Reference: TS WalletEncryptArgs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptArgs {
    /// Data to encrypt (base64 encoded)
    pub plaintext: String,
    
    /// Protocol ID for key derivation
    pub protocol_id: Vec<String>,
    
    /// Key ID for key derivation
    pub key_id: String,
    
    /// Counterparty public key (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<String>,
    
    /// Whether this uses privileged key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
}

/// Result from wallet encryption
///
/// Reference: TS WalletEncryptResult
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptResult {
    /// Encrypted ciphertext (base64 encoded)
    pub ciphertext: String,
}

/// Arguments for wallet decryption
///
/// Reference: TS WalletDecryptArgs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecryptArgs {
    /// Data to decrypt (base64 encoded)
    pub ciphertext: String,
    
    /// Protocol ID for key derivation
    pub protocol_id: Vec<String>,
    
    /// Key ID for key derivation
    pub key_id: String,
    
    /// Counterparty public key (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<String>,
    
    /// Whether this uses privileged key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
}

/// Result from wallet decryption
///
/// Reference: TS WalletDecryptResult
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecryptResult {
    /// Decrypted plaintext (base64 encoded)
    pub plaintext: String,
}

/// Encrypt data using wallet-derived keys
///
/// Reference: TS wallet.encrypt (Wallet.ts)
///
/// Uses BRC-42/43 key derivation to create encryption keys, then
/// encrypts the data using AES-256-GCM.
///
/// # Arguments
///
/// * `root_key` - Wallet's root private key (32 bytes)
/// * `args` - Encryption parameters
///
/// # Returns
///
/// Encrypted ciphertext
pub fn encrypt(root_key: &[u8], args: EncryptArgs) -> WalletResult<EncryptResult> {
    // Validate root key
    if root_key.len() != 32 {
        return Err(WalletError::invalid_parameter(
            "root_key",
            "Must be 32 bytes"
        ));
    }
    
    // Decode plaintext from base64
    let plaintext = base64::decode(&args.plaintext)
        .map_err(|e| WalletError::invalid_parameter("plaintext", &format!("Invalid base64: {}", e)))?;
    
    // Derive encryption key using BRC-42/43
    let encryption_key = derive_encryption_key(
        root_key,
        &args.protocol_id,
        &args.key_id,
        args.counterparty.as_deref(),
        args.privileged.unwrap_or(false),
    )?;
    
    // Encrypt using AES-256-GCM
    let ciphertext_bytes = encrypt_with_aes_gcm(&plaintext, &encryption_key)?;
    
    // Encode ciphertext to base64
    let ciphertext = base64::encode(&ciphertext_bytes);
    
    Ok(EncryptResult { ciphertext })
}

/// Decrypt data using wallet-derived keys
///
/// Reference: TS wallet.decrypt (Wallet.ts)
///
/// Uses BRC-42/43 key derivation to create decryption keys, then
/// decrypts the data using AES-256-GCM.
///
/// # Arguments
///
/// * `root_key` - Wallet's root private key (32 bytes)
/// * `args` - Decryption parameters
///
/// # Returns
///
/// Decrypted plaintext
pub fn decrypt(root_key: &[u8], args: DecryptArgs) -> WalletResult<DecryptResult> {
    // Validate root key
    if root_key.len() != 32 {
        return Err(WalletError::invalid_parameter(
            "root_key",
            "Must be 32 bytes"
        ));
    }
    
    // Decode ciphertext from base64
    let ciphertext = base64::decode(&args.ciphertext)
        .map_err(|e| WalletError::invalid_parameter("ciphertext", &format!("Invalid base64: {}", e)))?;
    
    // Derive decryption key using BRC-42/43
    let decryption_key = derive_encryption_key(
        root_key,
        &args.protocol_id,
        &args.key_id,
        args.counterparty.as_deref(),
        args.privileged.unwrap_or(false),
    )?;
    
    // Decrypt using AES-256-GCM
    let plaintext_bytes = decrypt_with_aes_gcm(&ciphertext, &decryption_key)?;
    
    // Encode plaintext to base64
    let plaintext = base64::encode(&plaintext_bytes);
    
    Ok(DecryptResult { plaintext })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let root_key = vec![1u8; 32];
        let plaintext_data = b"Hello, World!";
        let plaintext_b64 = base64::encode(plaintext_data);
        
        let encrypt_args = EncryptArgs {
            plaintext: plaintext_b64.clone(),
            protocol_id: vec!["test".to_string()],
            key_id: "encryption".to_string(),
            counterparty: None,
            privileged: Some(false),
        };
        
        // Encrypt
        let encrypt_result = encrypt(&root_key, encrypt_args).unwrap();
        assert!(!encrypt_result.ciphertext.is_empty());
        assert_ne!(encrypt_result.ciphertext, plaintext_b64);
        
        // Decrypt
        let decrypt_args = DecryptArgs {
            ciphertext: encrypt_result.ciphertext,
            protocol_id: vec!["test".to_string()],
            key_id: "encryption".to_string(),
            counterparty: None,
            privileged: Some(false),
        };
        
        let decrypt_result = decrypt(&root_key, decrypt_args).unwrap();
        assert_eq!(decrypt_result.plaintext, plaintext_b64);
    }
    
    #[test]
    fn test_encrypt_with_different_keys_produces_different_ciphertext() {
        let root_key = vec![1u8; 32];
        let plaintext = base64::encode(b"Test data");
        
        let args1 = EncryptArgs {
            plaintext: plaintext.clone(),
            protocol_id: vec!["test".to_string()],
            key_id: "key1".to_string(),
            counterparty: None,
            privileged: Some(false),
        };
        
        let args2 = EncryptArgs {
            plaintext: plaintext.clone(),
            protocol_id: vec!["test".to_string()],
            key_id: "key2".to_string(),
            counterparty: None,
            privileged: Some(false),
        };
        
        let result1 = encrypt(&root_key, args1).unwrap();
        let result2 = encrypt(&root_key, args2).unwrap();
        
        assert_ne!(result1.ciphertext, result2.ciphertext);
    }
    
    #[test]
    fn test_decrypt_with_wrong_key_fails() {
        let root_key = vec![1u8; 32];
        let plaintext = base64::encode(b"Test data");
        
        let encrypt_args = EncryptArgs {
            plaintext,
            protocol_id: vec!["test".to_string()],
            key_id: "key1".to_string(),
            counterparty: None,
            privileged: Some(false),
        };
        
        let encrypt_result = encrypt(&root_key, encrypt_args).unwrap();
        
        // Try to decrypt with different key_id
        let decrypt_args = DecryptArgs {
            ciphertext: encrypt_result.ciphertext,
            protocol_id: vec!["test".to_string()],
            key_id: "key2".to_string(), // Wrong key!
            counterparty: None,
            privileged: Some(false),
        };
        
        let result = decrypt(&root_key, decrypt_args);
        assert!(result.is_err());
    }
}
