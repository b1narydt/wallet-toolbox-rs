//! Wallet-Toolbox Key Derivation Integration
//!
//! Integrates BRC-42/43 with wallet-toolbox's storage format.
//! Derives private keys from TableOutput records for transaction signing.

use super::{brc42, brc43::InvoiceNumber};
use wallet_storage::TableOutput;

/// Key derivation context
///
/// Contains the master keys needed for derivation
#[derive(Debug, Clone)]
pub struct KeyDerivationContext {
    /// Recipient's master private key (wallet's master key)
    pub master_private_key: Vec<u8>,
}

/// Key derivation errors
#[derive(Debug, thiserror::Error)]
pub enum KeyDerivationError {
    #[error("missing derivation prefix")]
    MissingDerivationPrefix,
    
    #[error("missing derivation suffix")]
    MissingDerivationSuffix,
    
    #[error("missing sender identity key")]
    MissingSenderIdentityKey,
    
    #[error("invalid base64 encoding: {0}")]
    InvalidBase64(String),
    
    #[error("invalid hex encoding: {0}")]
    InvalidHex(String),
    
    #[error("invoice number error: {0}")]
    InvalidInvoiceNumber(String),
    
    #[error("BRC-42 derivation error: {0}")]
    Brc42Error(String),
}

/// Derive private key from TableOutput record
///
/// **wallet-toolbox Reference**: signAction uses this to derive signing keys
///
/// The TableOutput record contains:
/// - `derivation_prefix`: Base64-encoded protocol information
/// - `derivation_suffix`: Base64-encoded key ID
/// - `sender_identity_key`: Hex-encoded sender's public key
///
/// ## Process
/// 1. Decode derivation_prefix and derivation_suffix from base64
/// 2. Construct invoice number from prefix + suffix
/// 3. Decode sender's public key from hex
/// 4. Use BRC-42 to derive child private key
///
/// ## Arguments
/// - `output`: TableOutput record containing derivation info
/// - `ctx`: Key derivation context with master private key
///
/// ## Returns
/// 32-byte child private key for signing
pub fn derive_key_from_output(
    output: &TableOutput,
    ctx: &KeyDerivationContext,
) -> Result<Vec<u8>, KeyDerivationError> {
    use base64::{Engine as _, engine::general_purpose};
    
    // Get derivation components
    let derivation_prefix = output.derivation_prefix.as_ref()
        .ok_or(KeyDerivationError::MissingDerivationPrefix)?;
    
    let derivation_suffix = output.derivation_suffix.as_ref()
        .ok_or(KeyDerivationError::MissingDerivationSuffix)?;
    
    let sender_identity_key = output.sender_identity_key.as_ref()
        .ok_or(KeyDerivationError::MissingSenderIdentityKey)?;
    
    // Decode prefix and suffix from base64
    let prefix_bytes = general_purpose::STANDARD.decode(derivation_prefix)
        .map_err(|e| KeyDerivationError::InvalidBase64(format!("prefix: {}", e)))?;
    
    let suffix_bytes = general_purpose::STANDARD.decode(derivation_suffix)
        .map_err(|e| KeyDerivationError::InvalidBase64(format!("suffix: {}", e)))?;
    
    // Convert to strings (UTF-8)
    let prefix_str = String::from_utf8(prefix_bytes)
        .map_err(|e| KeyDerivationError::InvalidBase64(format!("prefix not UTF-8: {}", e)))?;
    
    let suffix_str = String::from_utf8(suffix_bytes)
        .map_err(|e| KeyDerivationError::InvalidBase64(format!("suffix not UTF-8: {}", e)))?;
    
    // Construct invoice number: prefix + suffix
    // In wallet-toolbox, the prefix typically contains the protocol info
    // and the suffix contains the key ID
    let invoice_number = if prefix_str.is_empty() {
        // Just use suffix if prefix is empty
        suffix_str
    } else if suffix_str.is_empty() {
        // Just use prefix if suffix is empty
        prefix_str
    } else {
        // Combine prefix and suffix
        format!("{}{}", prefix_str, suffix_str)
    };
    
    // Decode sender's public key from hex
    let sender_pubkey = hex::decode(sender_identity_key)
        .map_err(|e| KeyDerivationError::InvalidHex(format!("sender key: {}", e)))?;
    
    // Derive child private key using BRC-42
    let child_private_key = brc42::derive_child_private_key(
        &ctx.master_private_key,
        &sender_pubkey,
        &invoice_number,
    ).map_err(|e| KeyDerivationError::Brc42Error(e.to_string()))?;
    
    Ok(child_private_key)
}

/// Derive private key from explicit invoice number
///
/// **wallet-toolbox Reference**: Alternative derivation when invoice number is known
///
/// ## Arguments
/// - `master_private_key`: Recipient's 32-byte master private key
/// - `sender_public_key`: Sender's 33-byte public key
/// - `invoice_number`: Complete invoice number string
///
/// ## Returns
/// 32-byte child private key
pub fn derive_key_from_invoice(
    master_private_key: &[u8],
    sender_public_key: &[u8],
    invoice_number: &str,
) -> Result<Vec<u8>, KeyDerivationError> {
    brc42::derive_child_private_key(
        master_private_key,
        sender_public_key,
        invoice_number,
    ).map_err(|e| KeyDerivationError::Brc42Error(e.to_string()))
}

/// Derive public key for counterparty (sender's perspective)
///
/// **wallet-toolbox Reference**: Used when creating outputs for recipients
///
/// ## Arguments
/// - `sender_private_key`: Sender's 32-byte master private key
/// - `recipient_public_key`: Recipient's 33-byte master public key
/// - `invoice_number`: Invoice number string
///
/// ## Returns
/// 33-byte compressed child public key
pub fn derive_public_key_for_recipient(
    sender_private_key: &[u8],
    recipient_public_key: &[u8],
    invoice_number: &str,
) -> Result<Vec<u8>, KeyDerivationError> {
    brc42::derive_child_public_key(
        sender_private_key,
        recipient_public_key,
        invoice_number,
    ).map_err(|e| KeyDerivationError::Brc42Error(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{Engine as _, engine::general_purpose};
    
    #[test]
    fn test_derive_key_from_output() {
        // Create a test output with derivation info
        use wallet_storage::StorageProvidedBy;
        
        let mut output = TableOutput::new(
            1, // output_id
            1, // user_id
            1, // transaction_id
            true, // spendable
            false, // change
            "test output",
            0, // vout
            50000, // satoshis
            StorageProvidedBy::Storage,
            "test",
            "P2PKH",
        );
        
        // Use BRC-42 test vector data
        let sender_pubkey_hex = "033f9160df035156f1c48e75eae99914fa1a1546bec19781e8eddb900200bff9d1";
        let recipient_privkey_hex = "6a1751169c111b4667a6539ee1be6b7cd9f6e9c8fe011a5f2fe31e03a15e0ede";
        let invoice_number = "f3WCaUmnN9U=";
        let expected_privkey_hex = "761656715bbfa172f8f9f58f5af95d9d0dfd69014cfdcacc9a245a10ff8893ef";
        
        // Set derivation fields (prefix empty, suffix contains invoice)
        output.derivation_prefix = Some(general_purpose::STANDARD.encode(""));
        output.derivation_suffix = Some(general_purpose::STANDARD.encode(invoice_number));
        output.sender_identity_key = Some(sender_pubkey_hex.to_string());
        
        // Create context
        let ctx = KeyDerivationContext {
            master_private_key: hex::decode(recipient_privkey_hex).unwrap(),
        };
        
        // Derive key
        let derived = derive_key_from_output(&output, &ctx).unwrap();
        let expected = hex::decode(expected_privkey_hex).unwrap();
        
        assert_eq!(derived, expected);
    }
    
    #[test]
    fn test_derive_key_from_invoice() {
        // Test direct invoice number derivation
        let recipient_privkey = hex::decode("6a1751169c111b4667a6539ee1be6b7cd9f6e9c8fe011a5f2fe31e03a15e0ede").unwrap();
        let sender_pubkey = hex::decode("033f9160df035156f1c48e75eae99914fa1a1546bec19781e8eddb900200bff9d1").unwrap();
        let invoice_number = "f3WCaUmnN9U=";
        let expected = hex::decode("761656715bbfa172f8f9f58f5af95d9d0dfd69014cfdcacc9a245a10ff8893ef").unwrap();
        
        let derived = derive_key_from_invoice(&recipient_privkey, &sender_pubkey, invoice_number).unwrap();
        
        assert_eq!(derived, expected);
    }
    
    #[test]
    fn test_derive_public_key_for_recipient() {
        // Test public key derivation (sender's perspective)
        let sender_privkey = hex::decode("583755110a8c059de5cd81b8a04e1be884c46083ade3f779c1e022f6f89da94c").unwrap();
        let recipient_pubkey = hex::decode("02c0c1e1a1f7d247827d1bcf399f0ef2deef7695c322fd91a01a91378f101b6ffc").unwrap();
        let invoice_number = "IBioA4D/OaE=";
        let expected = hex::decode("03c1bf5baadee39721ae8c9882b3cf324f0bf3b9eb3fc1b8af8089ca7a7c2e669f").unwrap();
        
        let derived = derive_public_key_for_recipient(&sender_privkey, &recipient_pubkey, invoice_number).unwrap();
        
        assert_eq!(derived, expected);
    }
    
    #[test]
    fn test_missing_derivation_fields() {
        use wallet_storage::StorageProvidedBy;
        
        let output = TableOutput::new(
            1, 1, 1, true, false, "test", 0, 50000,
            StorageProvidedBy::Storage, "test", "P2PKH",
        );
        
        let ctx = KeyDerivationContext {
            master_private_key: vec![1u8; 32],
        };
        
        // Should error with missing derivation prefix
        let result = derive_key_from_output(&output, &ctx);
        assert!(result.is_err());
    }
}
