//! UTXO service types
//!
//! **Reference**: TypeScript `src/services/providers/WhatsOnChain.ts`

use serde::{Deserialize, Serialize};

/// WhatsOnChain UTXO status response
///
/// Reference: TS WhatsOnChainUtxoStatus (WhatsOnChain.ts)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsOnChainUtxoStatus {
    /// Script hash
    pub script: String,
    
    /// UTXO results
    pub result: Vec<WhatsOnChainUtxo>,
}

/// WhatsOnChain UTXO entry
///
/// Reference: TS response data structure (WhatsOnChain.ts lines 399-406)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsOnChainUtxo {
    /// Transaction hash
    pub tx_hash: String,
    
    /// Output index (position)
    pub tx_pos: u32,
    
    /// Satoshi value
    pub value: u64,
    
    /// Block height
    pub height: u32,
}

/// WhatsOnChain transaction status response
///
/// Reference: TS WhatsOnChainTxsStatusData (WhatsOnChain.ts lines 36-49)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsOnChainTxStatus {
    /// Transaction ID
    pub txid: String,
    
    /// Block hash (if mined)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockhash: Option<String>,
    
    /// Block height (if mined)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockheight: Option<u32>,
    
    /// Block time (if mined)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocktime: Option<u64>,
    
    /// Confirmations (if mined)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmations: Option<u32>,
    
    /// Error message (if unknown)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// WhatsOnChain script hash history response
///
/// Reference: TS WhatsOnChainScriptHashHistoryData (WhatsOnChain.ts lines 444-469)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsOnChainScriptHashHistory {
    /// History results
    pub result: Vec<WhatsOnChainHistoryEntry>,
    
    /// Error message (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// WhatsOnChain history entry
///
/// Reference: TS response mapping (WhatsOnChain.ts line 469)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsOnChainHistoryEntry {
    /// Transaction hash
    pub tx_hash: String,
    
    /// Block height
    pub height: u32,
}

/// UTXO detail
///
/// Reference: TS r.details structure (WhatsOnChain.ts lines 400-405)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtxoDetail {
    /// Transaction ID
    pub txid: String,
    
    /// Output index
    pub index: u32,
    
    /// Satoshi value
    pub satoshis: u64,
    
    /// Block height
    pub height: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_utxo_detail() {
        let detail = UtxoDetail {
            txid: "abc123".to_string(),
            index: 0,
            satoshis: 10000,
            height: 100,
        };
        
        assert_eq!(detail.satoshis, 10000);
        assert_eq!(detail.index, 0);
    }
    
    #[test]
    fn test_whatsonchain_tx_status_serde() {
        let status = WhatsOnChainTxStatus {
            txid: "abc123".to_string(),
            blockhash: Some("block123".to_string()),
            blockheight: Some(100),
            blocktime: Some(1234567890),
            confirmations: Some(6),
            error: None,
        };
        
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("abc123"));
    }
}
