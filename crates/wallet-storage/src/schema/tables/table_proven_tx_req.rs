//! TableProvenTxReq - Proven transaction request records
//!
//! Translates TypeScript TableProvenTxReq interface to Rust.
//! Reference: wallet-toolbox/src/storage/schema/tables/TableProvenTxReq.ts

use serde::{Deserialize, Serialize};

/// ProvenTxReqStatus - matches wallet-core but defined locally
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProvenTxReqStatus {
    Sending,
    Unsent,
    Nosend,
    Unknown,
    Nonfinal,
    Unprocessed,
    Unmined,
    Callback,
    Unconfirmed,
    Completed,
    Invalid,
    #[serde(rename = "doubleSpend")]
    DoubleSpend,
    Unfail,
}

impl std::fmt::Display for ProvenTxReqStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProvenTxReqStatus::Sending => write!(f, "sending"),
            ProvenTxReqStatus::Unsent => write!(f, "unsent"),
            ProvenTxReqStatus::Nosend => write!(f, "nosend"),
            ProvenTxReqStatus::Unknown => write!(f, "unknown"),
            ProvenTxReqStatus::Nonfinal => write!(f, "nonfinal"),
            ProvenTxReqStatus::Unprocessed => write!(f, "unprocessed"),
            ProvenTxReqStatus::Unmined => write!(f, "unmined"),
            ProvenTxReqStatus::Callback => write!(f, "callback"),
            ProvenTxReqStatus::Unconfirmed => write!(f, "unconfirmed"),
            ProvenTxReqStatus::Completed => write!(f, "completed"),
            ProvenTxReqStatus::Invalid => write!(f, "invalid"),
            ProvenTxReqStatus::DoubleSpend => write!(f, "doubleSpend"),
            ProvenTxReqStatus::Unfail => write!(f, "unfail"),
        }
    }
}

impl std::str::FromStr for ProvenTxReqStatus {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sending" => Ok(ProvenTxReqStatus::Sending),
            "unsent" => Ok(ProvenTxReqStatus::Unsent),
            "nosend" => Ok(ProvenTxReqStatus::Nosend),
            "unknown" => Ok(ProvenTxReqStatus::Unknown),
            "nonfinal" => Ok(ProvenTxReqStatus::Nonfinal),
            "unprocessed" => Ok(ProvenTxReqStatus::Unprocessed),
            "unmined" => Ok(ProvenTxReqStatus::Unmined),
            "callback" => Ok(ProvenTxReqStatus::Callback),
            "unconfirmed" => Ok(ProvenTxReqStatus::Unconfirmed),
            "completed" => Ok(ProvenTxReqStatus::Completed),
            "invalid" => Ok(ProvenTxReqStatus::Invalid),
            "doublespend" => Ok(ProvenTxReqStatus::DoubleSpend),
            "unfail" => Ok(ProvenTxReqStatus::Unfail),
            _ => Err(format!("Invalid ProvenTxReqStatus: {}", s)),
        }
    }
}

/// ProvenTxReq table - stores transaction proof requests
///
/// Matches TypeScript `TableProvenTxReq` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableProvenTxReq {
    pub created_at: String,
    pub updated_at: String,
    
    #[serde(rename = "provenTxReqId")]
    pub proven_tx_req_id: i64,
    
    #[serde(rename = "provenTxId", skip_serializing_if = "Option::is_none")]
    pub proven_tx_id: Option<i64>,
    
    pub status: ProvenTxReqStatus,
    
    /// Count of how many times a service has been asked about this txid
    pub attempts: i32,
    
    /// Set to true when terminal status set and notification occurred
    pub notified: bool,
    
    pub txid: String,
    
    /// Unique string identifying a batch of transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch: Option<String>,
    
    /// JSON string of processing history (ProvenTxReqHistoryApi)
    pub history: String,
    
    /// JSON string of notification data (ProvenTxReqNotifyApi)
    pub notify: String,
    
    #[serde(rename = "rawTx")]
    pub raw_tx: Vec<u8>,
    
    #[serde(rename = "inputBEEF", skip_serializing_if = "Option::is_none")]
    pub input_beef: Option<Vec<u8>>,
}

impl TableProvenTxReq {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        proven_tx_req_id: i64,
        status: ProvenTxReqStatus,
        txid: impl Into<String>,
        history: impl Into<String>,
        notify: impl Into<String>,
        raw_tx: Vec<u8>,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            proven_tx_req_id,
            proven_tx_id: None,
            status,
            attempts: 0,
            notified: false,
            txid: txid.into(),
            batch: None,
            history: history.into(),
            notify: notify.into(),
            raw_tx,
            input_beef: None,
        }
    }

    pub fn with_proven_tx_id(mut self, proven_tx_id: i64) -> Self {
        self.proven_tx_id = Some(proven_tx_id);
        self
    }

    pub fn with_batch(mut self, batch: impl Into<String>) -> Self {
        self.batch = Some(batch.into());
        self
    }

    pub fn with_input_beef(mut self, input_beef: Vec<u8>) -> Self {
        self.input_beef = Some(input_beef);
        self
    }

    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    pub fn increment_attempts(&mut self) {
        self.attempts += 1;
        self.touch();
    }

    pub fn set_status(&mut self, status: ProvenTxReqStatus) {
        self.status = status;
        self.touch();
    }

    pub fn mark_notified(&mut self) {
        self.notified = true;
        self.touch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_proven_tx_req_new() {
        let req = TableProvenTxReq::new(
            1, ProvenTxReqStatus::Unprocessed, "txid123",
            "{}", "{}", vec![1, 2, 3]
        );
        assert_eq!(req.proven_tx_req_id, 1);
        assert_eq!(req.status, ProvenTxReqStatus::Unprocessed);
        assert_eq!(req.attempts, 0);
        assert_eq!(req.notified, false);
    }

    #[test]
    fn test_table_proven_tx_req_increment_attempts() {
        let mut req = TableProvenTxReq::new(
            1, ProvenTxReqStatus::Unmined, "txid", "{}", "{}", vec![]
        );
        assert_eq!(req.attempts, 0);
        req.increment_attempts();
        assert_eq!(req.attempts, 1);
        req.increment_attempts();
        assert_eq!(req.attempts, 2);
    }

    #[test]
    fn test_table_proven_tx_req_serialization() {
        let req = TableProvenTxReq::new(
            1, ProvenTxReqStatus::Completed, "abc", "{}", "{}", vec![]
        );
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"provenTxReqId\":1"));
        assert!(json.contains("\"status\":\"completed\""));
        let deserialized: TableProvenTxReq = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }
}
