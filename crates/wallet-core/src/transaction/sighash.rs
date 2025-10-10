//! Signature Hash (Sighash) Calculation
//!
//! Implements Bitcoin signature hash calculation for transaction signing.
//!
//! **Reference**: TypeScript bsv-sdk sighash calculation

use super::{Transaction, TxInput, TransactionError, TransactionResult};
use sha2::{Sha256, Digest};

/// Sighash type flags
///
/// Determines which parts of the transaction are signed.
///
/// **Reference**: TypeScript `SigHash` enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SigHashType {
    /// Sign all inputs and outputs (most common)
    All = 0x01,
    
    /// Sign all inputs, no outputs
    None = 0x02,
    
    /// Sign all inputs, only one output
    Single = 0x03,
    
    /// Can be combined with above using OR
    AnyoneCanPay = 0x80,
}

impl SigHashType {
    /// Get sighash type byte value
    pub fn as_u8(self) -> u8 {
        self as u8
    }
    
    /// Get sighash type as u32 for serialization
    pub fn as_u32(self) -> u32 {
        self as u32
    }
}

/// Sighash calculator
///
/// Calculates the hash that will be signed for a specific input.
///
/// **Reference**: TypeScript `tx.sighash(inputIndex, subscript, sighashType)`
pub struct SigHash;

impl SigHash {
    /// Calculate sighash for an input
    ///
    /// This is the hash that gets signed by the private key.
    ///
    /// **Reference**: TypeScript `tx.sighash(vin, subscript, type)`
    ///
    /// ## Arguments
    /// - `tx`: The transaction being signed
    /// - `input_index`: Index of the input being signed
    /// - `prev_script`: The locking script from the output being spent (subscript)
    /// - `sighash_type`: Type of signature (usually SigHashType::All)
    /// - `prev_value`: Value of the output being spent (for BIP143)
    ///
    /// ## Returns
    /// 32-byte hash to be signed
    pub fn calculate(
        tx: &Transaction,
        input_index: usize,
        prev_script: &[u8],
        sighash_type: SigHashType,
        _prev_value: i64, // For BIP143 (not used in legacy)
    ) -> TransactionResult<Vec<u8>> {
        if input_index >= tx.inputs.len() {
            return Err(TransactionError::InvalidFormat(
                format!("Input index {} out of range", input_index)
            ));
        }
        
        // For SIGHASH_ALL (most common case):
        // 1. Copy transaction
        // 2. Replace all input scripts with empty scripts
        // 3. Set the current input's script to the previous output's script
        // 4. Serialize with sighash type appended
        // 5. Double SHA-256
        
        let mut sighash_tx = tx.clone();
        
        // Clear all input scripts
        for input in &mut sighash_tx.inputs {
            input.script_sig = Vec::new();
        }
        
        // Set current input's script to prev_script (subscript)
        sighash_tx.inputs[input_index].script_sig = prev_script.to_vec();
        
        // Serialize transaction
        let mut serialized = sighash_tx.serialize()?;
        
        // Append sighash type (4 bytes, little-endian)
        serialized.extend_from_slice(&sighash_type.as_u32().to_le_bytes());
        
        // Double SHA-256
        let hash1 = Sha256::digest(&serialized);
        let hash2 = Sha256::digest(&hash1);
        
        Ok(hash2.to_vec())
    }
    
    /// Calculate sighash and return as hex string
    pub fn calculate_hex(
        tx: &Transaction,
        input_index: usize,
        prev_script: &[u8],
        sighash_type: SigHashType,
        prev_value: i64,
    ) -> TransactionResult<String> {
        let hash = Self::calculate(tx, input_index, prev_script, sighash_type, prev_value)?;
        Ok(hex::encode(hash))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::{OutPoint, TxOutput};
    
    #[test]
    fn test_sighash_type_values() {
        // TS Reference: SIGHASH type constants
        assert_eq!(SigHashType::All.as_u8(), 0x01);
        assert_eq!(SigHashType::None.as_u8(), 0x02);
        assert_eq!(SigHashType::Single.as_u8(), 0x03);
        assert_eq!(SigHashType::AnyoneCanPay.as_u8(), 0x80);
    }
    
    #[test]
    fn test_sighash_calculate_basic() {
        // TS Reference: Basic sighash calculation
        let mut tx = Transaction::new();
        
        // Add input
        let outpoint = OutPoint::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            0,
        );
        let input = TxInput::new(outpoint);
        tx.add_input(input);
        
        // Add output
        let output = TxOutput::new(50000, vec![0x76, 0xa9]);
        tx.add_output(output);
        
        // Calculate sighash
        let prev_script = vec![0x76, 0xa9, 0x14]; // Simple script
        let sighash = SigHash::calculate(
            &tx,
            0,
            &prev_script,
            SigHashType::All,
            50000,
        ).unwrap();
        
        // Should be 32 bytes
        assert_eq!(sighash.len(), 32);
    }
    
    #[test]
    fn test_sighash_invalid_input_index() {
        // TS Reference: Error handling for invalid input index
        let tx = Transaction::new();
        let prev_script = vec![0x76, 0xa9];
        
        let result = SigHash::calculate(
            &tx,
            0, // No inputs exist
            &prev_script,
            SigHashType::All,
            0,
        );
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_sighash_deterministic() {
        // TS Reference: Sighash should be deterministic
        let mut tx = Transaction::new();
        let outpoint = OutPoint::new("abc123", 0);
        let input = TxInput::new(outpoint);
        tx.add_input(input);
        
        let output = TxOutput::new(50000, vec![0x76, 0xa9]);
        tx.add_output(output);
        
        let prev_script = vec![0x76, 0xa9, 0x14];
        
        let hash1 = SigHash::calculate(&tx, 0, &prev_script, SigHashType::All, 50000).unwrap();
        let hash2 = SigHash::calculate(&tx, 0, &prev_script, SigHashType::All, 50000).unwrap();
        
        // Should be identical
        assert_eq!(hash1, hash2);
    }
}
