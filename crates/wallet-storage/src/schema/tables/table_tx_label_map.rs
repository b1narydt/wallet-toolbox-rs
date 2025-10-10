//! TableTxLabelMap - Transaction to label mapping
//!
//! Translates TypeScript TableTxLabelMap interface to Rust.
//! Reference: wallet-toolbox/src/storage/schema/tables/TableTxLabelMap.ts

use serde::{Deserialize, Serialize};

/// TxLabelMap table - maps transactions to labels (many-to-many)
///
/// Matches TypeScript `TableTxLabelMap` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableTxLabelMap {
    pub created_at: String,
    pub updated_at: String,
    
    #[serde(rename = "txLabelId")]
    pub tx_label_id: i64,
    
    #[serde(rename = "transactionId")]
    pub transaction_id: i64,
    
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

impl TableTxLabelMap {
    pub fn new(tx_label_id: i64, transaction_id: i64) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            tx_label_id,
            transaction_id,
            is_deleted: false,
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    pub fn delete(&mut self) {
        self.is_deleted = true;
        self.touch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_tx_label_map_new() {
        let map = TableTxLabelMap::new(1, 200);
        assert_eq!(map.tx_label_id, 1);
        assert_eq!(map.transaction_id, 200);
        assert_eq!(map.is_deleted, false);
    }

    #[test]
    fn test_table_tx_label_map_serialization() {
        let map = TableTxLabelMap::new(5, 500);
        let json = serde_json::to_string(&map).unwrap();
        assert!(json.contains("\"txLabelId\":5"));
        assert!(json.contains("\"transactionId\":500"));
        let deserialized: TableTxLabelMap = serde_json::from_str(&json).unwrap();
        assert_eq!(map, deserialized);
    }
}
