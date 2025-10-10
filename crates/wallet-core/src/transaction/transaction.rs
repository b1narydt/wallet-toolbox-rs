//! Bitcoin Transaction
//!
//! Core transaction structure with serialization and txid calculation.
//!
//! **Reference**: TypeScript bsv-sdk Transaction class

use super::{TxInput, TxOutput, OutPoint, Script, TransactionError, TransactionResult};
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

/// Bitcoin transaction
///
/// Contains all data needed for a valid Bitcoin transaction:
/// - Version
/// - Inputs (spending previous outputs)
/// - Outputs (creating new outputs)
/// - Lock time (for timelocks)
///
/// **Reference**: TypeScript `Transaction` class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction version (typically 1 or 2)
    pub version: u32,
    
    /// Transaction inputs
    pub inputs: Vec<TxInput>,
    
    /// Transaction outputs
    pub outputs: Vec<TxOutput>,
    
    /// Lock time (0 = no timelock)
    #[serde(rename = "lockTime")]
    pub lock_time: u32,
}

impl Transaction {
    /// Create a new empty transaction
    ///
    /// **Reference**: TypeScript `new Transaction()`
    pub fn new() -> Self {
        Self {
            version: 1,
            inputs: Vec::new(),
            outputs: Vec::new(),
            lock_time: 0,
        }
    }
    
    /// Create a transaction with specific parameters
    ///
    /// **Reference**: TypeScript `new Transaction(version, inputs, outputs, lockTime)`
    pub fn with_params(version: u32, inputs: Vec<TxInput>, outputs: Vec<TxOutput>, lock_time: u32) -> Self {
        Self {
            version,
            inputs,
            outputs,
            lock_time,
        }
    }
    
    /// Add an input to the transaction
    ///
    /// **Reference**: TypeScript `tx.addInput(input)`
    pub fn add_input(&mut self, input: TxInput) {
        self.inputs.push(input);
    }
    
    /// Add an output to the transaction
    ///
    /// **Reference**: TypeScript `tx.addOutput(output)`
    pub fn add_output(&mut self, output: TxOutput) {
        self.outputs.push(output);
    }
    
    /// Serialize transaction to bytes
    ///
    /// Format:
    /// - Version (4 bytes, little-endian)
    /// - Input count (varint)
    /// - Inputs
    /// - Output count (varint)
    /// - Outputs
    /// - Lock time (4 bytes, little-endian)
    ///
    /// **Reference**: TypeScript `tx.serialize()`
    pub fn serialize(&self) -> TransactionResult<Vec<u8>> {
        let mut buffer = Vec::new();
        
        // Version (4 bytes, little-endian)
        buffer.extend_from_slice(&self.version.to_le_bytes());
        
        // Input count (varint)
        buffer.extend_from_slice(&encode_varint(self.inputs.len() as u64));
        
        // Inputs
        for input in &self.inputs {
            buffer.extend_from_slice(&input.serialize()
                .map_err(|e| TransactionError::Serialization(e.to_string()))?);
        }
        
        // Output count (varint)
        buffer.extend_from_slice(&encode_varint(self.outputs.len() as u64));
        
        // Outputs
        for output in &self.outputs {
            buffer.extend_from_slice(&output.serialize());
        }
        
        // Lock time (4 bytes, little-endian)
        buffer.extend_from_slice(&self.lock_time.to_le_bytes());
        
        Ok(buffer)
    }
    
    /// Calculate transaction ID (txid)
    ///
    /// Txid is double SHA-256 of serialized transaction, reversed.
    ///
    /// **Reference**: TypeScript `tx.id('hex')` or `tx.hash`
    pub fn txid(&self) -> TransactionResult<String> {
        let serialized = self.serialize()?;
        
        // Double SHA-256
        let hash1 = Sha256::digest(&serialized);
        let hash2 = Sha256::digest(&hash1);
        
        // Reverse bytes for display (Bitcoin convention)
        let reversed: Vec<u8> = hash2.into_iter().rev().collect();
        
        Ok(hex::encode(reversed))
    }
    
    /// Calculate transaction hash (same as txid but not reversed)
    ///
    /// Used internally for sighash calculation
    pub fn hash(&self) -> TransactionResult<Vec<u8>> {
        let serialized = self.serialize()?;
        
        // Double SHA-256
        let hash1 = Sha256::digest(&serialized);
        let hash2 = Sha256::digest(&hash1);
        
        Ok(hash2.to_vec())
    }
    
    /// Get transaction size in bytes
    ///
    /// **Reference**: TypeScript `tx.size()`
    pub fn size(&self) -> TransactionResult<usize> {
        Ok(self.serialize()?.len())
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}

/// Encode variable-length integer (varint)
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
    fn test_transaction_new() {
        // TS Reference: new Transaction()
        let tx = Transaction::new();
        
        assert_eq!(tx.version, 1);
        assert_eq!(tx.lock_time, 0);
        assert!(tx.inputs.is_empty());
        assert!(tx.outputs.is_empty());
    }
    
    #[test]
    fn test_transaction_add_input() {
        // TS Reference: tx.addInput()
        let mut tx = Transaction::new();
        let outpoint = OutPoint::new("abc123", 0);
        let input = TxInput::new(outpoint);
        
        tx.add_input(input);
        
        assert_eq!(tx.inputs.len(), 1);
    }
    
    #[test]
    fn test_transaction_add_output() {
        // TS Reference: tx.addOutput()
        let mut tx = Transaction::new();
        let output = TxOutput::new(50000, vec![0x76, 0xa9]);
        
        tx.add_output(output);
        
        assert_eq!(tx.outputs.len(), 1);
    }
    
    #[test]
    fn test_transaction_serialize_empty() {
        // TS Reference: Empty transaction serialization
        let tx = Transaction::new();
        let serialized = tx.serialize().unwrap();
        
        // Version (4) + input count (1) + output count (1) + locktime (4) = 10 bytes
        assert_eq!(serialized.len(), 10);
        
        // Version = 1
        assert_eq!(u32::from_le_bytes(serialized[0..4].try_into().unwrap()), 1);
        
        // Input count = 0
        assert_eq!(serialized[4], 0);
        
        // Output count = 0
        assert_eq!(serialized[5], 0);
        
        // Lock time = 0
        assert_eq!(u32::from_le_bytes(serialized[6..10].try_into().unwrap()), 0);
    }
    
    #[test]
    fn test_transaction_txid() {
        // TS Reference: tx.id('hex') - txid calculation
        let tx = Transaction::new();
        let txid = tx.txid().unwrap();
        
        // Txid should be 64 hex characters (32 bytes)
        assert_eq!(txid.len(), 64);
        
        // Should be valid hex
        assert!(hex::decode(&txid).is_ok());
    }
    
    #[test]
    fn test_transaction_size() {
        // TS Reference: tx.size()
        let mut tx = Transaction::new();
        
        // Empty transaction
        assert_eq!(tx.size().unwrap(), 10);
        
        // Add output
        let output = TxOutput::new(50000, vec![0x76, 0xa9]);
        tx.add_output(output);
        
        // Size should increase
        assert!(tx.size().unwrap() > 10);
    }
}
