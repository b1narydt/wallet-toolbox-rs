//! Transaction Output
//!
//! Represents an output in a Bitcoin transaction, containing value and locking script.
//!
//! **Reference**: TypeScript bsv-sdk TxOut / TransactionOutput

use serde::{Deserialize, Serialize};

/// Transaction output
///
/// Contains:
/// - Value in satoshis
/// - Locking script (scriptPubKey) defining spending conditions
///
/// Matches TypeScript `TxOut` structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxOutput {
    /// Value in satoshis
    pub value: i64,
    
    /// Locking script (scriptPubKey)
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: Vec<u8>,
}

impl TxOutput {
    /// Create a new TxOutput
    ///
    /// **Reference**: TypeScript `new TxOut(value, script)`
    pub fn new(value: i64, script_pubkey: Vec<u8>) -> Self {
        Self {
            value,
            script_pubkey,
        }
    }
    
    /// Create TxOutput from hex-encoded script
    pub fn from_hex_script(value: i64, script_hex: &str) -> Result<Self, hex::FromHexError> {
        let script_pubkey = hex::decode(script_hex)?;
        Ok(Self::new(value, script_pubkey))
    }
    
    /// Serialize output for transaction
    ///
    /// Format:
    /// - Value (8 bytes, little-endian)
    /// - Script length (varint)
    /// - Script bytes
    ///
    /// **Reference**: TypeScript transaction serialization
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        
        // Add value (8 bytes, little-endian)
        buffer.extend_from_slice(&self.value.to_le_bytes());
        
        // Add script length (varint)
        buffer.extend_from_slice(&encode_varint(self.script_pubkey.len() as u64));
        
        // Add script bytes
        buffer.extend_from_slice(&self.script_pubkey);
        
        buffer
    }
}

/// Encode variable-length integer (varint)
/// Same implementation as in tx_input.rs
fn encode_varint(n: u64) -> Vec<u8> {
    if n < 0xFD {
        vec![n as u8]
    } else if n <= 0xFFFF {
        let mut buf = vec![0xFD];
        buf.extend_from_slice(&(n as u16).to_le_bytes());
        buf
    } else if n <= 0xFFFFFFFF {
        let mut buf = vec![0xFE];
        buf.extend_from_slice(&(n as u32).to_le_bytes());
        buf
    } else {
        let mut buf = vec![0xFF];
        buf.extend_from_slice(&n.to_le_bytes());
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tx_output_creation() {
        // TS Reference: new TxOut(value, script)
        let script = vec![0x76, 0xa9]; // OP_DUP OP_HASH160
        let output = TxOutput::new(50000, script.clone());
        
        assert_eq!(output.value, 50000);
        assert_eq!(output.script_pubkey, script);
    }
    
    #[test]
    fn test_tx_output_from_hex() {
        // TS Reference: Creating output with hex script
        let output = TxOutput::from_hex_script(
            100000,
            "76a914" // OP_DUP OP_HASH160
        ).unwrap();
        
        assert_eq!(output.value, 100000);
        assert_eq!(output.script_pubkey, vec![0x76, 0xa9, 0x14]);
    }
    
    #[test]
    fn test_tx_output_serialize() {
        // TS Reference: Output serialization for transaction
        let script = vec![0x76, 0xa9];
        let output = TxOutput::new(50000, script);
        
        let serialized = output.serialize();
        
        // First 8 bytes should be value (50000) in little-endian
        let value_bytes = &serialized[0..8];
        assert_eq!(i64::from_le_bytes(value_bytes.try_into().unwrap()), 50000);
        
        // Next byte should be script length (2)
        assert_eq!(serialized[8], 2);
        
        // Remaining bytes should be script
        assert_eq!(&serialized[9..], &[0x76, 0xa9]);
    }
}
