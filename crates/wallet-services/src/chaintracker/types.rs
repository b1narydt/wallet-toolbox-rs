//! ChainTracker types
//!
//! **Reference**: TypeScript `src/services/chaintracker/chaintracks/Api/`

use serde::{Deserialize, Serialize};

/// Block header structure
/// Reference: TypeScript BlockHeader
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Block height
    pub height: u32,
    
    /// Block hash (hex)
    pub hash: String,
    
    /// Previous block hash (hex)
    #[serde(rename = "previousHash")]
    pub previous_hash: String,
    
    /// Merkle root (hex)
    #[serde(rename = "merkleRoot")]
    pub merkle_root: String,
    
    /// Block timestamp
    pub time: u32,
    
    /// Difficulty bits
    pub bits: u32,
    
    /// Nonce
    pub nonce: u32,
    
    /// Version
    pub version: u32,
}

/// Chaintracks service info
/// Reference: TypeScript ChaintracksInfoApi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaintracksInfo {
    /// Service version
    pub version: String,
    
    /// Current chain height
    pub height: u32,
    
    /// Chain identifier
    pub chain: String,
}

/// Fetch status wrapper
/// Reference: TypeScript FetchStatus<T>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchStatus<T> {
    /// Status: success or error
    pub status: String,
    
    /// Error code (if error)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    
    /// Error description (if error)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Result value (if success)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,
}

impl<T> FetchStatus<T> {
    /// Check if status is success
    pub fn is_success(&self) -> bool {
        self.status == "success"
    }
    
    /// Extract value or return error
    pub fn into_result(self) -> Result<T, String> {
        if self.is_success() {
            self.value.ok_or_else(|| "Success status but no value".to_string())
        } else {
            Err(self.description.unwrap_or_else(|| "Unknown error".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fetch_status_success() {
        let status = FetchStatus {
            status: "success".to_string(),
            code: None,
            description: None,
            value: Some(42),
        };
        
        assert!(status.is_success());
        assert_eq!(status.into_result().unwrap(), 42);
    }
    
    #[test]
    fn test_fetch_status_error() {
        let status: FetchStatus<i32> = FetchStatus {
            status: "error".to_string(),
            code: Some("ERR_001".to_string()),
            description: Some("Test error".to_string()),
            value: None,
        };
        
        assert!(!status.is_success());
        assert!(status.into_result().is_err());
    }
    
    #[test]
    fn test_block_header_serde() {
        let header = BlockHeader {
            height: 100,
            hash: "abc123".to_string(),
            previous_hash: "def456".to_string(),
            merkle_root: "root123".to_string(),
            time: 1234567890,
            bits: 0x1d00ffff,
            nonce: 12345,
            version: 1,
        };
        
        let json = serde_json::to_string(&header).unwrap();
        assert!(json.contains("\"height\":100"));
        assert!(json.contains("merkleRoot"));
    }
}
