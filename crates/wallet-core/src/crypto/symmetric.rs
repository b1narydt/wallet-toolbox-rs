//! Symmetric Encryption Operations
//!
//! AES-256-GCM encryption and decryption for wallet data

use crate::sdk::errors::{WalletError, WalletResult};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use rand::RngCore;

/// Encrypt data using AES-256-GCM
///
/// # Arguments
///
/// * `plaintext` - Data to encrypt
/// * `key` - 32-byte encryption key
///
/// # Returns
///
/// Encrypted data: [12-byte nonce][ciphertext][16-byte tag]
pub fn encrypt_with_aes_gcm(plaintext: &[u8], key: &[u8]) -> WalletResult<Vec<u8>> {
    if key.len() != 32 {
        return Err(WalletError::invalid_parameter(
            "key",
            "AES-256 requires 32-byte key"
        ));
    }
    
    // Create cipher
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| WalletError::invalid_operation(&format!("Failed to create cipher: {}", e)))?;
    
    // Generate random nonce (96 bits / 12 bytes for GCM)
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    // Encrypt
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| WalletError::invalid_operation(&format!("Encryption failed: {}", e)))?;
    
    // Combine: nonce || ciphertext (ciphertext includes auth tag)
    let mut result = Vec::with_capacity(12 + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    
    Ok(result)
}

/// Decrypt data using AES-256-GCM
///
/// # Arguments
///
/// * `ciphertext` - Encrypted data: [12-byte nonce][ciphertext][16-byte tag]
/// * `key` - 32-byte decryption key
///
/// # Returns
///
/// Decrypted plaintext
pub fn decrypt_with_aes_gcm(ciphertext: &[u8], key: &[u8]) -> WalletResult<Vec<u8>> {
    if key.len() != 32 {
        return Err(WalletError::invalid_parameter(
            "key",
            "AES-256 requires 32-byte key"
        ));
    }
    
    if ciphertext.len() < 12 + 16 {
        return Err(WalletError::invalid_parameter(
            "ciphertext",
            "Too short (need at least 28 bytes for nonce + tag)"
        ));
    }
    
    // Create cipher
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| WalletError::invalid_operation(&format!("Failed to create cipher: {}", e)))?;
    
    // Extract nonce (first 12 bytes)
    let nonce = Nonce::from_slice(&ciphertext[0..12]);
    
    // Extract ciphertext + tag (rest of data)
    let encrypted_data = &ciphertext[12..];
    
    // Decrypt
    let plaintext = cipher
        .decrypt(nonce, encrypted_data)
        .map_err(|e| WalletError::invalid_operation(&format!("Decryption failed (wrong key or corrupted data): {}", e)))?;
    
    Ok(plaintext)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = [1u8; 32];
        let plaintext = b"Hello, World!";
        
        let ciphertext = encrypt_with_aes_gcm(plaintext, &key).unwrap();
        assert!(ciphertext.len() >= plaintext.len() + 12 + 16);
        
        let decrypted = decrypt_with_aes_gcm(&ciphertext, &key).unwrap();
        assert_eq!(&decrypted[..], plaintext);
    }
    
    #[test]
    fn test_decrypt_with_wrong_key_fails() {
        let key1 = [1u8; 32];
        let key2 = [2u8; 32];
        let plaintext = b"Secret message";
        
        let ciphertext = encrypt_with_aes_gcm(plaintext, &key1).unwrap();
        let result = decrypt_with_aes_gcm(&ciphertext, &key2);
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_decrypt_corrupted_data_fails() {
        let key = [1u8; 32];
        let plaintext = b"Test data";
        
        let mut ciphertext = encrypt_with_aes_gcm(plaintext, &key).unwrap();
        
        // Corrupt the ciphertext
        ciphertext[20] ^= 0xFF;
        
        let result = decrypt_with_aes_gcm(&ciphertext, &key);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_each_encryption_produces_different_ciphertext() {
        let key = [1u8; 32];
        let plaintext = b"Same message";
        
        let ciphertext1 = encrypt_with_aes_gcm(plaintext, &key).unwrap();
        let ciphertext2 = encrypt_with_aes_gcm(plaintext, &key).unwrap();
        
        // Different nonces mean different ciphertexts
        assert_ne!(ciphertext1, ciphertext2);
        
        // But both decrypt to same plaintext
        let decrypted1 = decrypt_with_aes_gcm(&ciphertext1, &key).unwrap();
        let decrypted2 = decrypt_with_aes_gcm(&ciphertext2, &key).unwrap();
        assert_eq!(decrypted1, decrypted2);
        assert_eq!(&decrypted1[..], plaintext);
    }
}
