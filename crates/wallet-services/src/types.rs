//! Service result types
//!
//! **Reference**: TypeScript `src/sdk/WalletServices.interfaces.ts`
//!
//! Defines all result types returned by WalletServices methods

use serde::{Deserialize, Serialize};

/// Chain identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Chain {
    Main,
    Test,
}

impl Default for Chain {
    fn default() -> Self {
        Chain::Main
    }
}

/// GetRawTx result
/// Reference: TypeScript GetRawTxResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRawTxResult {
    /// Transaction ID
    pub txid: String,
    
    /// Raw transaction bytes (if found)
    #[serde(rename = "rawTx", skip_serializing_if = "Option::is_none")]
    pub raw_tx: Option<Vec<u8>>,
    
    /// Service name that provided the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// Error if request failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ServiceError>,
}

/// GetMerklePath result
/// Reference: TypeScript GetMerklePathResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMerklePathResult {
    /// Transaction ID
    pub txid: String,
    
    /// Merkle proof (if found)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<MerklePath>,
    
    /// Service name that provided the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// Error if request failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ServiceError>,
}

/// Merkle path proof structure
/// Reference: TypeScript MerklePath from @bsv/sdk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerklePath {
    /// Block height
    #[serde(rename = "blockHeight")]
    pub block_height: u32,
    
    /// Merkle tree path
    pub path: Vec<Vec<PathElement>>,
}

/// Path element in merkle proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathElement {
    /// Hash value
    pub hash: Option<String>,
    
    /// Transaction ID
    pub txid: Option<bool>,
    
    /// Duplicate flag
    pub duplicate: Option<bool>,
}

/// PostBeef result
/// Reference: TypeScript PostBeefResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostBeefResult {
    /// Transaction ID
    pub txid: String,
    
    /// Status of submission
    pub status: String,
    
    /// Service name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// Error if submission failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ServiceError>,
}

/// GetUtxoStatus result
/// Reference: TypeScript GetUtxoStatusResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUtxoStatusResult {
    /// Whether output is unspent
    #[serde(rename = "isUtxo")]
    pub is_utxo: bool,
    
    /// Service name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// Error if request failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ServiceError>,
}

/// Output format for UTXO status queries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GetUtxoStatusOutputFormat {
    /// Little-endian SHA256 hash of output script
    HashLE,
    /// Big-endian SHA256 hash of output script
    HashBE,
    /// Entire output script
    Script,
}

/// GetStatusForTxids result
/// Reference: TypeScript GetStatusForTxidsResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatusForTxidsResult {
    /// Status for each transaction
    pub statuses: Vec<TxStatus>,
    
    /// Service name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Transaction status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxStatus {
    /// Transaction ID
    pub txid: String,
    
    /// Status: known, mined, or unknown
    pub status: TxStatusType,
    
    /// Depth from chain tip (if mined)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
}

/// Transaction status type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TxStatusType {
    /// Transaction is known to network
    Known,
    /// Transaction is mined in a block
    Mined,
    /// Transaction is unknown
    Unknown,
}

/// GetScriptHashHistory result
/// Reference: TypeScript GetScriptHashHistoryResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScriptHashHistoryResult {
    /// Script hash
    #[serde(rename = "scriptHash")]
    pub script_hash: String,
    
    /// History entries
    pub history: Vec<HistoryEntry>,
    
    /// Service name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// History entry for a script hash
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// Transaction ID
    pub txid: String,
    
    /// Block height (if confirmed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
}

/// PostRawTx result
/// Reference: TypeScript PostRawTxResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRawTxResult {
    /// Transaction ID
    pub txid: String,
    
    /// Whether transaction was accepted
    pub success: bool,
    
    /// Service name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// Error if submission failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ServiceError>,
}

/// GetBlockHeader result
/// Reference: TypeScript GetBlockHeaderResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlockHeaderResult {
    /// Block header bytes
    pub header: Vec<u8>,
    
    /// Block height
    pub height: u32,
    
    /// Service name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Service error details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceError {
    /// Service name that generated the error
    pub service: String,
    
    /// Error message
    pub message: String,
    
    /// HTTP status code (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<u16>,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chain_serde() {
        let chain = Chain::Main;
        let json = serde_json::to_string(&chain).unwrap();
        assert_eq!(json, "\"main\"");
        
        let chain: Chain = serde_json::from_str("\"test\"").unwrap();
        assert_eq!(chain, Chain::Test);
    }
    
    #[test]
    fn test_tx_status_type() {
        let status = TxStatusType::Mined;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"mined\"");
    }
    
    #[test]
    fn test_get_raw_tx_result() {
        let result = GetRawTxResult {
            txid: "abc123".to_string(),
            raw_tx: Some(vec![0x01, 0x02]),
            name: Some("test-service".to_string()),
            error: None,
        };
        
        assert_eq!(result.txid, "abc123");
        assert!(result.raw_tx.is_some());
    }
    
    #[test]
    fn test_service_error() {
        let error = ServiceError {
            service: "test-service".to_string(),
            message: "Connection timeout".to_string(),
            status_code: Some(504),
        };
        
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("test-service"));
        assert!(json.contains("Connection timeout"));
    }
    
    #[test]
    fn test_utxo_status_result() {
        let result = GetUtxoStatusResult {
            is_utxo: true,
            name: Some("whatsonchain".to_string()),
            error: None,
        };
        
        assert!(result.is_utxo);
    }
}
