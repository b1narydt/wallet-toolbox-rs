//! Bitcoin Script Operations
//!
//! Minimal script building functionality for P2PKH transactions.
//!
//! **Reference**: TypeScript bsv-sdk Script class

use super::TransactionError;

/// Bitcoin script builder
///
/// Provides basic script construction for P2PKH (Pay-to-Public-Key-Hash) transactions,
/// which are the most common type.
///
/// **Reference**: TypeScript `Script` class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Script {
    pub(crate) bytes: Vec<u8>,
}

impl Script {
    /// Create empty script
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }
    
    /// Create script from bytes
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
    
    /// Create script from hex string
    ///
    /// **Reference**: TypeScript `Script.fromHex(hex)`
    pub fn from_hex(hex: &str) -> Result<Self, hex::FromHexError> {
        Ok(Self {
            bytes: hex::decode(hex)?,
        })
    }
    
    /// Get script bytes
    pub fn to_bytes(&self) -> &[u8] {
        &self.bytes
    }
    
    /// Get script as hex string
    pub fn to_hex(&self) -> String {
        hex::encode(&self.bytes)
    }
    
    /// Build P2PKH locking script
    ///
    /// Format: OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG
    ///
    /// **Reference**: TypeScript `Script.buildPublicKeyHashOut(pubKeyHash)`
    pub fn p2pkh_locking_script(pub_key_hash: &[u8]) -> Result<Self, TransactionError> {
        if pub_key_hash.len() != 20 {
            return Err(TransactionError::InvalidScript(
                format!("Public key hash must be 20 bytes, got {}", pub_key_hash.len())
            ));
        }
        
        let mut bytes = Vec::with_capacity(25);
        bytes.push(0x76); // OP_DUP
        bytes.push(0xa9); // OP_HASH160
        bytes.push(0x14); // Push 20 bytes
        bytes.extend_from_slice(pub_key_hash);
        bytes.push(0x88); // OP_EQUALVERIFY
        bytes.push(0xac); // OP_CHECKSIG
        
        Ok(Self { bytes })
    }
    
    /// Build P2PKH unlocking script
    ///
    /// Format: <signature> <publicKey>
    ///
    /// **Reference**: TypeScript unlocking script construction
    pub fn p2pkh_unlocking_script(signature: &[u8], public_key: &[u8]) -> Self {
        let mut bytes = Vec::new();
        
        // Push signature
        bytes.push(signature.len() as u8);
        bytes.extend_from_slice(signature);
        
        // Push public key
        bytes.push(public_key.len() as u8);
        bytes.extend_from_slice(public_key);
        
        Self { bytes }
    }
    
    /// Get script length
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
    
    /// Check if script is empty
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

impl Default for Script {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_script_from_hex() {
        // TS Reference: Script.fromHex()
        let script = Script::from_hex("76a914").unwrap();
        assert_eq!(script.bytes, vec![0x76, 0xa9, 0x14]);
    }
    
    #[test]
    fn test_script_to_hex() {
        // TS Reference: script.toHex()
        let script = Script::from_bytes(vec![0x76, 0xa9, 0x14]);
        assert_eq!(script.to_hex(), "76a914");
    }
    
    #[test]
    fn test_p2pkh_locking_script() {
        // TS Reference: P2PKH locking script (scriptPubKey)
        // Format: OP_DUP OP_HASH160 <20-byte hash> OP_EQUALVERIFY OP_CHECKSIG
        
        let pub_key_hash = vec![0u8; 20]; // 20-byte hash
        let script = Script::p2pkh_locking_script(&pub_key_hash).unwrap();
        
        assert_eq!(script.len(), 25); // 5 opcodes + 20 bytes
        assert_eq!(script.bytes[0], 0x76); // OP_DUP
        assert_eq!(script.bytes[1], 0xa9); // OP_HASH160
        assert_eq!(script.bytes[2], 0x14); // Push 20 bytes
        assert_eq!(script.bytes[23], 0x88); // OP_EQUALVERIFY
        assert_eq!(script.bytes[24], 0xac); // OP_CHECKSIG
    }
    
    #[test]
    fn test_p2pkh_unlocking_script() {
        // TS Reference: P2PKH unlocking script (scriptSig)
        // Format: <signature> <publicKey>
        
        let signature = vec![0u8; 71]; // Typical DER signature length
        let public_key = vec![1u8; 33]; // Compressed public key
        
        let script = Script::p2pkh_unlocking_script(&signature, &public_key);
        
        // Should have: sig_len + sig + pubkey_len + pubkey
        assert_eq!(script.len(), 1 + 71 + 1 + 33);
        assert_eq!(script.bytes[0], 71); // Signature length
        assert_eq!(script.bytes[72], 33); // Public key length
    }
    
    #[test]
    fn test_p2pkh_invalid_hash_length() {
        // TS Reference: Validation of public key hash length
        let invalid_hash = vec![0u8; 19]; // Wrong length
        let result = Script::p2pkh_locking_script(&invalid_hash);
        
        assert!(result.is_err());
    }
}
