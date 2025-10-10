//! TableTxLabel - Transaction label definitions
//!
//! Translates TypeScript TableTxLabel interface to Rust.
//! Reference: wallet-toolbox/src/storage/schema/tables/TableTxLabel.ts

use serde::{Deserialize, Serialize};

/// TxLabel table - stores transaction label definitions
///
/// Matches TypeScript `TableTxLabel` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableTxLabel {
    pub created_at: String,
    pub updated_at: String,
    
    #[serde(rename = "txLabelId")]
    pub tx_label_id: i64,
    
    #[serde(rename = "userId")]
    pub user_id: i64,
    
    pub label: String,
    
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

impl TableTxLabel {
    pub fn new(tx_label_id: i64, user_id: i64, label: impl Into<String>) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            tx_label_id,
            user_id,
            label: label.into(),
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

    pub fn restore(&mut self) {
        self.is_deleted = false;
        self.touch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_tx_label_new() {
        let label = TableTxLabel::new(1, 100, "invoice");
        assert_eq!(label.tx_label_id, 1);
        assert_eq!(label.user_id, 100);
        assert_eq!(label.label, "invoice");
        assert_eq!(label.is_deleted, false);
    }

    #[test]
    fn test_table_tx_label_delete_restore() {
        let mut label = TableTxLabel::new(1, 100, "test");
        label.delete();
        assert_eq!(label.is_deleted, true);
        label.restore();
        assert_eq!(label.is_deleted, false);
    }

    #[test]
    fn test_table_tx_label_serialization() {
        let label = TableTxLabel::new(1, 100, "payment");
        let json = serde_json::to_string(&label).unwrap();
        assert!(json.contains("\"txLabelId\":1"));
        assert!(json.contains("\"userId\":100"));
        let deserialized: TableTxLabel = serde_json::from_str(&json).unwrap();
        assert_eq!(label, deserialized);
    }
}
