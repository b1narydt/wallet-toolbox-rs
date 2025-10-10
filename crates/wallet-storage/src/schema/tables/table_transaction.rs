//! TableTransaction - Transaction records
//!
//! Translates TypeScript TableTransaction interface to Rust.
//! Reference: wallet-toolbox/src/storage/schema/tables/TableTransaction.ts

use serde::{Deserialize, Serialize};

/// Transaction status - matches wallet-core TransactionStatus but defined locally
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionStatus {
    Completed,
    Failed,
    Unprocessed,
    Sending,
    Unproven,
    Unsigned,
    Nosend,
    Nonfinal,
    Unfail,
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionStatus::Completed => write!(f, "completed"),
            TransactionStatus::Failed => write!(f, "failed"),
            TransactionStatus::Unprocessed => write!(f, "unprocessed"),
            TransactionStatus::Sending => write!(f, "sending"),
            TransactionStatus::Unproven => write!(f, "unproven"),
            TransactionStatus::Unsigned => write!(f, "unsigned"),
            TransactionStatus::Nosend => write!(f, "nosend"),
            TransactionStatus::Nonfinal => write!(f, "nonfinal"),
            TransactionStatus::Unfail => write!(f, "unfail"),
        }
    }
}

impl std::str::FromStr for TransactionStatus {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "completed" => Ok(TransactionStatus::Completed),
            "failed" => Ok(TransactionStatus::Failed),
            "unprocessed" => Ok(TransactionStatus::Unprocessed),
            "sending" => Ok(TransactionStatus::Sending),
            "unproven" => Ok(TransactionStatus::Unproven),
            "unsigned" => Ok(TransactionStatus::Unsigned),
            "nosend" => Ok(TransactionStatus::Nosend),
            "nonfinal" => Ok(TransactionStatus::Nonfinal),
            "unfail" => Ok(TransactionStatus::Unfail),
            _ => Err(format!("Invalid transaction status: {}", s)),
        }
    }
}

/// Transaction table - stores transaction records
///
/// Matches TypeScript `TableTransaction` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableTransaction {
    /// Record creation timestamp (ISO 8601 string)
    pub created_at: String,
    
    /// Record last update timestamp (ISO 8601 string)
    pub updated_at: String,
    
    /// Primary key - unique transaction identifier
    #[serde(rename = "transactionId")]
    pub transaction_id: i64,
    
    /// Foreign key to user
    #[serde(rename = "userId")]
    pub user_id: i64,
    
    /// Optional foreign key to proven transaction
    #[serde(rename = "provenTxId", skip_serializing_if = "Option::is_none")]
    pub proven_tx_id: Option<i64>,
    
    /// Transaction status
    pub status: TransactionStatus,
    
    /// Base64 encoded reference (max length 64)
    pub reference: String,
    
    /// True if transaction originated in this wallet, change returns to it.
    /// False for a transaction created externally and handed in to this wallet.
    #[serde(rename = "isOutgoing")]
    pub is_outgoing: bool,
    
    /// Total satoshis in transaction
    pub satoshis: i64,
    
    /// Transaction description
    pub description: String,
    
    /// Optional version - if present, must match value in associated rawTransaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
    
    /// Optional lock time. Default is zero.
    /// When the transaction can be processed into a block:
    /// >= 500,000,000 values are interpreted as minimum required unix time stamps in seconds
    /// < 500,000,000 values are interpreted as minimum required block height
    #[serde(rename = "lockTime", skip_serializing_if = "Option::is_none")]
    pub lock_time: Option<u32>,
    
    /// Optional transaction ID (txid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    
    /// Optional input BEEF data (byte array)
    #[serde(rename = "inputBEEF", skip_serializing_if = "Option::is_none")]
    pub input_beef: Option<Vec<u8>>,
    
    /// Optional raw transaction data (byte array)
    #[serde(rename = "rawTx", skip_serializing_if = "Option::is_none")]
    pub raw_tx: Option<Vec<u8>>,
}

impl TableTransaction {
    /// Create a new TableTransaction with required fields
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        transaction_id: i64,
        user_id: i64,
        status: TransactionStatus,
        reference: impl Into<String>,
        is_outgoing: bool,
        satoshis: i64,
        description: impl Into<String>,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            transaction_id,
            user_id,
            proven_tx_id: None,
            status,
            reference: reference.into(),
            is_outgoing,
            satoshis,
            description: description.into(),
            version: None,
            lock_time: None,
            txid: None,
            input_beef: None,
            raw_tx: None,
        }
    }

    /// Builder-style method to set proven_tx_id
    pub fn with_proven_tx_id(mut self, proven_tx_id: i64) -> Self {
        self.proven_tx_id = Some(proven_tx_id);
        self
    }

    /// Builder-style method to set version
    pub fn with_version(mut self, version: u32) -> Self {
        self.version = Some(version);
        self
    }

    /// Builder-style method to set lock_time
    pub fn with_lock_time(mut self, lock_time: u32) -> Self {
        self.lock_time = Some(lock_time);
        self
    }

    /// Builder-style method to set txid
    pub fn with_txid(mut self, txid: impl Into<String>) -> Self {
        self.txid = Some(txid.into());
        self
    }

    /// Builder-style method to set input_beef
    pub fn with_input_beef(mut self, input_beef: Vec<u8>) -> Self {
        self.input_beef = Some(input_beef);
        self
    }

    /// Builder-style method to set raw_tx
    pub fn with_raw_tx(mut self, raw_tx: Vec<u8>) -> Self {
        self.raw_tx = Some(raw_tx);
        self
    }

    /// Update the timestamp
    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Update status and timestamp
    pub fn set_status(&mut self, status: TransactionStatus) {
        self.status = status;
        self.touch();
    }

    /// Get columns without rawTx (matches TypeScript transactionColumnsWithoutRawTx)
    pub fn columns_without_raw_tx() -> &'static [&'static str] {
        &[
            "created_at",
            "updated_at",
            "transactionId",
            "userId",
            "provenTxId",
            "status",
            "reference",
            "isOutgoing",
            "satoshis",
            "version",
            "lockTime",
            "description",
            "txid",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_transaction_new() {
        let tx = TableTransaction::new(
            1,
            100,
            TransactionStatus::Unprocessed,
            "ref123",
            true,
            5000,
            "Test transaction",
        );
        
        assert_eq!(tx.transaction_id, 1);
        assert_eq!(tx.user_id, 100);
        assert_eq!(tx.status, TransactionStatus::Unprocessed);
        assert_eq!(tx.reference, "ref123");
        assert_eq!(tx.is_outgoing, true);
        assert_eq!(tx.satoshis, 5000);
        assert_eq!(tx.description, "Test transaction");
        assert!(tx.proven_tx_id.is_none());
        assert!(tx.version.is_none());
        assert!(tx.lock_time.is_none());
        assert!(tx.txid.is_none());
        assert!(tx.input_beef.is_none());
        assert!(tx.raw_tx.is_none());
    }

    #[test]
    fn test_table_transaction_builder() {
        let tx = TableTransaction::new(
            1, 100, TransactionStatus::Completed, "ref", false, 1000, "desc",
        )
        .with_proven_tx_id(50)
        .with_version(1)
        .with_lock_time(500000000)
        .with_txid("abc123def456")
        .with_input_beef(vec![1, 2, 3])
        .with_raw_tx(vec![4, 5, 6]);
        
        assert_eq!(tx.proven_tx_id, Some(50));
        assert_eq!(tx.version, Some(1));
        assert_eq!(tx.lock_time, Some(500000000));
        assert_eq!(tx.txid, Some("abc123def456".to_string()));
        assert_eq!(tx.input_beef, Some(vec![1, 2, 3]));
        assert_eq!(tx.raw_tx, Some(vec![4, 5, 6]));
    }

    #[test]
    fn test_table_transaction_touch() {
        let mut tx = TableTransaction::new(
            1, 100, TransactionStatus::Unprocessed, "ref", true, 1000, "desc",
        );
        
        let original_updated = tx.updated_at.clone();
        std::thread::sleep(std::time::Duration::from_millis(10));
        tx.touch();
        
        assert_ne!(tx.updated_at, original_updated);
    }

    #[test]
    fn test_table_transaction_set_status() {
        let mut tx = TableTransaction::new(
            1, 100, TransactionStatus::Unprocessed, "ref", true, 1000, "desc",
        );
        
        assert_eq!(tx.status, TransactionStatus::Unprocessed);
        tx.set_status(TransactionStatus::Completed);
        assert_eq!(tx.status, TransactionStatus::Completed);
    }

    #[test]
    fn test_table_transaction_serialization() {
        let tx = TableTransaction::new(
            1, 100, TransactionStatus::Sending, "ref", true, 5000, "Test",
        );
        
        let json = serde_json::to_string(&tx).unwrap();
        
        // Check camelCase field names
        assert!(json.contains("\"transactionId\":1"));
        assert!(json.contains("\"userId\":100"));
        assert!(json.contains("\"isOutgoing\":true"));
        assert!(json.contains("\"status\":\"sending\""));
        
        let deserialized: TableTransaction = serde_json::from_str(&json).unwrap();
        assert_eq!(tx, deserialized);
    }

    #[test]
    fn test_table_transaction_optional_fields_not_serialized() {
        let tx = TableTransaction::new(
            1, 100, TransactionStatus::Unprocessed, "ref", false, 1000, "desc",
        );
        
        let json = serde_json::to_string(&tx).unwrap();
        
        // Optional None fields should not appear
        assert!(!json.contains("\"provenTxId\""));
        assert!(!json.contains("\"version\""));
        assert!(!json.contains("\"lockTime\""));
        assert!(!json.contains("\"txid\""));
        assert!(!json.contains("\"inputBEEF\""));
        assert!(!json.contains("\"rawTx\""));
    }

    #[test]
    fn test_table_transaction_optional_fields_serialized_when_some() {
        let tx = TableTransaction::new(
            1, 100, TransactionStatus::Completed, "ref", true, 1000, "desc",
        )
        .with_proven_tx_id(50)
        .with_version(1)
        .with_lock_time(123456)
        .with_txid("txid123");
        
        let json = serde_json::to_string(&tx).unwrap();
        
        assert!(json.contains("\"provenTxId\":50"));
        assert!(json.contains("\"version\":1"));
        assert!(json.contains("\"lockTime\":123456"));
        assert!(json.contains("\"txid\":\"txid123\""));
    }

    #[test]
    fn test_transaction_status_serialization() {
        assert_eq!(
            serde_json::to_string(&TransactionStatus::Completed).unwrap(),
            "\"completed\""
        );
        assert_eq!(
            serde_json::to_string(&TransactionStatus::Failed).unwrap(),
            "\"failed\""
        );
        assert_eq!(
            serde_json::to_string(&TransactionStatus::Unprocessed).unwrap(),
            "\"unprocessed\""
        );
    }

    #[test]
    fn test_columns_without_raw_tx() {
        let columns = TableTransaction::columns_without_raw_tx();
        
        assert_eq!(columns.len(), 13);
        assert!(columns.contains(&"transactionId"));
        assert!(columns.contains(&"userId"));
        assert!(columns.contains(&"status"));
        assert!(!columns.contains(&"rawTx"));
        assert!(!columns.contains(&"inputBEEF"));
    }

    #[test]
    fn test_table_transaction_lock_time_interpretation() {
        // Lock time < 500,000,000 = block height
        let tx_height = TableTransaction::new(
            1, 100, TransactionStatus::Unprocessed, "ref", true, 1000, "desc",
        )
        .with_lock_time(100000);
        
        assert!(tx_height.lock_time.unwrap() < 500_000_000);
        
        // Lock time >= 500,000,000 = unix timestamp
        let tx_time = TableTransaction::new(
            1, 100, TransactionStatus::Unprocessed, "ref", true, 1000, "desc",
        )
        .with_lock_time(1609459200); // 2021-01-01
        
        assert!(tx_time.lock_time.unwrap() >= 500_000_000);
    }

    #[test]
    fn test_table_transaction_clone() {
        let tx = TableTransaction::new(
            1, 100, TransactionStatus::Completed, "ref", true, 5000, "desc",
        );
        let cloned = tx.clone();
        
        assert_eq!(tx, cloned);
    }
}
