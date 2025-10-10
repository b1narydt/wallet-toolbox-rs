//! Transaction OutPoint
//!
//! Represents a reference to a specific output in a previous transaction.
//! Consists of a transaction ID (txid) and output index (vout).
//!
//! **Reference**: TypeScript bsv-sdk OutPoint

use serde::{Deserialize, Serialize};

/// Transaction output point
/// 
/// References a specific output in a previous transaction.
/// Used as input to spend that output.
///
/// Matches TypeScript `OutPoint` structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutPoint {
    /// Transaction ID (32 bytes, hex encoded)
    pub txid: String,
    
    /// Output index (vout)
    pub vout: u32,
}

impl OutPoint {
    /// Create a new OutPoint
    pub fn new(txid: impl Into<String>, vout: u32) -> Self {
        Self {
            txid: txid.into(),
            vout,
        }
    }
    
    /// Parse txid as bytes (reversed for Bitcoin wire format)
    /// 
    /// Bitcoin wire format uses little-endian byte order for txid
    pub fn txid_bytes(&self) -> Result<Vec<u8>, hex::FromHexError> {
        let bytes = hex::decode(&self.txid)?;
        // Reverse for wire format (Bitcoin uses little-endian)
        Ok(bytes.into_iter().rev().collect())
    }
    
    /// Serialize OutPoint for transaction signing
    /// 
    /// Format: txid (32 bytes, little-endian) + vout (4 bytes, little-endian)
    pub fn serialize(&self) -> Result<Vec<u8>, hex::FromHexError> {
        let mut buffer = Vec::with_capacity(36);
        
        // Add txid (32 bytes, reversed)
        buffer.extend_from_slice(&self.txid_bytes()?);
        
        // Add vout (4 bytes, little-endian)
        buffer.extend_from_slice(&self.vout.to_le_bytes());
        
        Ok(buffer)
    }
}

impl std::fmt::Display for OutPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.txid, self.vout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_outpoint_creation() {
        // TS Reference: new OutPoint(txid, vout)
        let outpoint = OutPoint::new(
            "0000000000000000000000000000000000000000000000000000000000000001",
            0,
        );
        
        assert_eq!(outpoint.vout, 0);
        assert_eq!(outpoint.txid.len(), 64); // 32 bytes hex = 64 chars
    }
    
    #[test]
    fn test_outpoint_display() {
        // TS Reference: OutPoint.toString() returns "txid:vout"
        let outpoint = OutPoint::new("abc123", 5);
        assert_eq!(outpoint.to_string(), "abc123:5");
    }
    
    #[test]
    fn test_outpoint_serialize() {
        // TS Reference: OutPoint serialization for transaction
        let outpoint = OutPoint::new(
            "0100000000000000000000000000000000000000000000000000000000000000",
            0,
        );
        
        let serialized = outpoint.serialize().unwrap();
        
        // Should be 36 bytes (32 for txid + 4 for vout)
        assert_eq!(serialized.len(), 36);
        
        // Last 4 bytes should be vout (0) in little-endian
        assert_eq!(&serialized[32..], &[0, 0, 0, 0]);
    }
}
