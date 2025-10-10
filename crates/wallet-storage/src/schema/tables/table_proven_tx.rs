//! TableProvenTx - Proven transaction records
//!
//! Translates TypeScript TableProvenTx interface to Rust.
//! Reference: wallet-toolbox/src/storage/schema/tables/TableProvenTx.ts

use serde::{Deserialize, Serialize};

/// ProvenTx table - stores proven transaction with merkle proof
///
/// Matches TypeScript `TableProvenTx` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableProvenTx {
    pub created_at: String,
    pub updated_at: String,
    
    #[serde(rename = "provenTxId")]
    pub proven_tx_id: i64,
    
    pub txid: String,
    
    pub height: i64,
    
    pub index: i64,
    
    #[serde(rename = "merklePath")]
    pub merkle_path: Vec<u8>,
    
    #[serde(rename = "rawTx")]
    pub raw_tx: Vec<u8>,
    
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    
    #[serde(rename = "merkleRoot")]
    pub merkle_root: String,
}

impl TableProvenTx {
    pub fn new(
        proven_tx_id: i64,
        txid: impl Into<String>,
        height: i64,
        index: i64,
        merkle_path: Vec<u8>,
        raw_tx: Vec<u8>,
        block_hash: impl Into<String>,
        merkle_root: impl Into<String>,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            proven_tx_id,
            txid: txid.into(),
            height,
            index,
            merkle_path,
            raw_tx,
            block_hash: block_hash.into(),
            merkle_root: merkle_root.into(),
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_proven_tx_new() {
        let proven = TableProvenTx::new(
            1, "txid123", 700000, 5,
            vec![1, 2, 3], vec![4, 5, 6],
            "block123", "root123"
        );
        assert_eq!(proven.proven_tx_id, 1);
        assert_eq!(proven.txid, "txid123");
        assert_eq!(proven.height, 700000);
        assert_eq!(proven.index, 5);
    }

    #[test]
    fn test_table_proven_tx_serialization() {
        let proven = TableProvenTx::new(
            1, "abc", 1000, 0,
            vec![1], vec![2], "hash", "root"
        );
        let json = serde_json::to_string(&proven).unwrap();
        assert!(json.contains("\"provenTxId\":1"));
        assert!(json.contains("\"blockHash\":\"hash\""));
        let deserialized: TableProvenTx = serde_json::from_str(&json).unwrap();
        assert_eq!(proven, deserialized);
    }
}
