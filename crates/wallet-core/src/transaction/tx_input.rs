//! Transaction Input
//!
//! Represents an input to a Bitcoin transaction, spending a previous output.
//!
//! **Reference**: TypeScript bsv-sdk TxIn / TransactionInput

use super::{OutPoint, Script};
use serde::{Deserialize, Serialize};

/// Transaction input
///
/// Spends a previous transaction output by providing:
/// - Reference to the output (OutPoint)
/// - Unlocking script (scriptSig) to satisfy the locking script
/// - Sequence number for timelocks
///
/// Matches TypeScript `TxIn` structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxInput {
    /// Previous output being spent
    pub prev_out: OutPoint,
    
    /// Unlocking script (scriptSig)
    #[serde(rename = "scriptSig")]
    pub script_sig: Vec<u8>,
    
    /// Sequence number (for timelocks and RBF)
    pub sequence: u32,
}

impl TxInput {
    /// Create a new TxInput
    ///
    /// **Reference**: TypeScript `new TxIn()`
    pub fn new(prev_out: OutPoint) -> Self {
        Self {
            prev_out,
            script_sig: Vec::new(),
            sequence: 0xFFFFFFFF, // Default: finalized (no timelock)
        }
    }
    
    /// Create TxInput with custom sequence
    pub fn with_sequence(prev_out: OutPoint, sequence: u32) -> Self {
        Self {
            prev_out,
            script_sig: Vec::new(),
            sequence,
        }
    }
    
    /// Set the unlocking script
    ///
    /// **Reference**: TypeScript `txIn.setScript(script)`
    pub fn set_script(&mut self, script: Vec<u8>) {
        self.script_sig = script;
    }
    
    /// Set sequence number
    pub fn set_sequence(&mut self, sequence: u32) {
        self.sequence = sequence;
    }
    
    /// Serialize input for transaction
    ///
    /// Format:
    /// - OutPoint (36 bytes)
    /// - Script length (varint)
    /// - Script bytes
    /// - Sequence (4 bytes, little-endian)
    ///
    /// **Reference**: TypeScript transaction serialization
    pub fn serialize(&self) -> Result<Vec<u8>, hex::FromHexError> {
        let mut buffer = Vec::new();
        
        // Add previous outpoint (36 bytes)
        buffer.extend_from_slice(&self.prev_out.serialize()?);
        
        // Add script length (varint)
        buffer.extend_from_slice(&encode_varint(self.script_sig.len() as u64));
        
        // Add script bytes
        buffer.extend_from_slice(&self.script_sig);
        
        // Add sequence (4 bytes, little-endian)
        buffer.extend_from_slice(&self.sequence.to_le_bytes());
        
        Ok(buffer)
    }
}

/// Encode variable-length integer (varint)
///
/// **Reference**: TypeScript `VarInt.encode()`
/// 
/// Bitcoin varint encoding:
/// - < 0xFD: 1 byte
/// - <= 0xFFFF: 0xFD + 2 bytes (little-endian)
/// - <= 0xFFFFFFFF: 0xFE + 4 bytes (little-endian)
/// - > 0xFFFFFFFF: 0xFF + 8 bytes (little-endian)
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
    fn test_tx_input_creation() {
        // TS Reference: new TxIn(outpoint)
        let outpoint = OutPoint::new("abc123", 0);
        let input = TxInput::new(outpoint.clone());
        
        assert_eq!(input.prev_out, outpoint);
        assert_eq!(input.sequence, 0xFFFFFFFF);
        assert!(input.script_sig.is_empty());
    }
    
    #[test]
    fn test_tx_input_set_script() {
        // TS Reference: txIn.setScript(script)
        let outpoint = OutPoint::new("abc123", 0);
        let mut input = TxInput::new(outpoint);
        
        let script = vec![0x76, 0xa9]; // OP_DUP OP_HASH160
        input.set_script(script.clone());
        
        assert_eq!(input.script_sig, script);
    }
    
    #[test]
    fn test_tx_input_sequence() {
        // TS Reference: txIn with sequence for timelocks
        let outpoint = OutPoint::new("abc123", 0);
        let input = TxInput::with_sequence(outpoint, 0xFFFFFFFE);
        
        assert_eq!(input.sequence, 0xFFFFFFFE);
    }
    
    #[test]
    fn test_encode_varint() {
        // TS Reference: VarInt encoding
        assert_eq!(encode_varint(0), vec![0]);
        assert_eq!(encode_varint(252), vec![252]);
        assert_eq!(encode_varint(253), vec![0xFD, 253, 0]);
        assert_eq!(encode_varint(0xFFFF), vec![0xFD, 0xFF, 0xFF]);
        assert_eq!(encode_varint(0x10000), vec![0xFE, 0, 0, 1, 0]);
    }
}
