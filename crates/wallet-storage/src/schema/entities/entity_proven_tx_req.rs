//! EntityProvenTxReq - Proven transaction request entity wrapper
//!
//! Translates TypeScript EntityProvenTxReq class to Rust.
//! Reference: wallet-toolbox/src/storage/schema/entities/EntityProvenTxReq.ts

use crate::schema::tables::{TableProvenTxReq, ProvenTxReqStatus};
use super::{EntityBase, SyncMap};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// History note structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReqHistoryNote {
    pub when: Option<String>,
    pub what: String,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// ProvenTxReq history structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProvenTxReqHistory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<ReqHistoryNote>>,
}

/// ProvenTxReq notify structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProvenTxReqNotify {
    #[serde(rename = "transactionIds", skip_serializing_if = "Option::is_none")]
    pub transaction_ids: Option<Vec<i64>>,
}

/// ProvenTxReq entity wrapper providing merge logic and property accessors
///
/// Matches TypeScript `EntityProvenTxReq` class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityProvenTxReq {
    api: TableProvenTxReq,
    history: ProvenTxReqHistory,
    notify: ProvenTxReqNotify,
}

impl EntityProvenTxReq {
    /// Create new EntityProvenTxReq from table record
    pub fn new(api: Option<TableProvenTxReq>) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        let api = api.unwrap_or_else(|| TableProvenTxReq {
            created_at: now.clone(),
            updated_at: now,
            proven_tx_req_id: 0,
            proven_tx_id: None,
            status: ProvenTxReqStatus::Unknown,
            attempts: 0,
            notified: false,
            txid: String::new(),
            batch: None,
            history: "{}".to_string(),
            notify: "{}".to_string(),
            raw_tx: Vec::new(),
            input_beef: None,
        });
        
        let mut entity = Self {
            api,
            history: ProvenTxReqHistory::default(),
            notify: ProvenTxReqNotify::default(),
        };
        entity.unpack_api();
        entity
    }

    /// Pack history into API JSON string
    pub fn pack_api_history(&mut self) {
        self.api.history = serde_json::to_string(&self.history).unwrap_or_else(|_| "{}".to_string());
    }

    /// Pack notify into API JSON string
    pub fn pack_api_notify(&mut self) {
        self.api.notify = serde_json::to_string(&self.notify).unwrap_or_else(|_| "{}".to_string());
    }

    /// Unpack history from API JSON string
    pub fn unpack_api_history(&mut self) {
        self.history = serde_json::from_str(&self.api.history).unwrap_or_default();
    }

    /// Unpack notify from API JSON string
    pub fn unpack_api_notify(&mut self) {
        self.notify = serde_json::from_str(&self.api.notify).unwrap_or_default();
        
        // Cleanup null values and duplicates
        if let Some(transaction_ids) = &mut self.notify.transaction_ids {
            transaction_ids.sort_unstable();
            transaction_ids.dedup();
        }
    }

    /// Unpack both history and notify from API
    pub fn unpack_api(&mut self) {
        self.unpack_api_history();
        self.unpack_api_notify();
    }

    // Property accessors matching TypeScript getters/setters

    pub fn proven_tx_req_id(&self) -> i64 {
        self.api.proven_tx_req_id
    }

    pub fn set_proven_tx_req_id(&mut self, v: i64) {
        self.api.proven_tx_req_id = v;
    }

    pub fn created_at(&self) -> &str {
        &self.api.created_at
    }

    pub fn set_created_at(&mut self, v: impl Into<String>) {
        self.api.created_at = v.into();
    }

    pub fn updated_at(&self) -> &str {
        &self.api.updated_at
    }

    pub fn set_updated_at(&mut self, v: impl Into<String>) {
        self.api.updated_at = v.into();
    }

    pub fn txid(&self) -> &str {
        &self.api.txid
    }

    pub fn set_txid(&mut self, v: impl Into<String>) {
        self.api.txid = v.into();
    }

    pub fn status(&self) -> ProvenTxReqStatus {
        self.api.status
    }

    pub fn set_status(&mut self, v: ProvenTxReqStatus) {
        self.api.status = v;
    }

    pub fn attempts(&self) -> i32 {
        self.api.attempts
    }

    pub fn set_attempts(&mut self, v: i32) {
        self.api.attempts = v;
    }

    pub fn notified(&self) -> bool {
        self.api.notified
    }

    pub fn set_notified(&mut self, v: bool) {
        self.api.notified = v;
    }

    pub fn proven_tx_id(&self) -> Option<i64> {
        self.api.proven_tx_id
    }

    pub fn set_proven_tx_id(&mut self, v: Option<i64>) {
        self.api.proven_tx_id = v;
    }

    pub fn batch(&self) -> Option<&str> {
        self.api.batch.as_deref()
    }

    pub fn set_batch(&mut self, v: Option<String>) {
        self.api.batch = v;
    }

    pub fn raw_tx(&self) -> &[u8] {
        &self.api.raw_tx
    }

    pub fn set_raw_tx(&mut self, v: Vec<u8>) {
        self.api.raw_tx = v;
    }

    pub fn input_beef(&self) -> Option<&Vec<u8>> {
        self.api.input_beef.as_ref()
    }

    pub fn set_input_beef(&mut self, v: Option<Vec<u8>>) {
        self.api.input_beef = v;
    }

    /// Get reference to history
    pub fn history(&self) -> &ProvenTxReqHistory {
        &self.history
    }

    /// Get mutable reference to history
    pub fn history_mut(&mut self) -> &mut ProvenTxReqHistory {
        &mut self.history
    }

    /// Get reference to notify
    pub fn notify(&self) -> &ProvenTxReqNotify {
        &self.notify
    }

    /// Get mutable reference to notify
    pub fn notify_mut(&mut self) -> &mut ProvenTxReqNotify {
        &mut self.notify
    }

    /// Get mutable reference to underlying API
    pub fn get_api_mut(&mut self) -> &mut TableProvenTxReq {
        &mut self.api
    }

    /// Consume entity and return API (packs history/notify first)
    pub fn into_api(mut self) -> TableProvenTxReq {
        self.pack_api_history();
        self.pack_api_notify();
        self.api
    }

    /// Helper to compare optional byte arrays
    fn optional_arrays_equal(a: Option<&Vec<u8>>, b: Option<&Vec<u8>>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }

    /// Helper to compare byte arrays
    fn arrays_equal(a: &[u8], b: &[u8]) -> bool {
        a == b
    }
}

impl EntityBase for EntityProvenTxReq {
    type Api = TableProvenTxReq;

    fn id(&self) -> i64 {
        self.api.proven_tx_req_id
    }

    fn set_id(&mut self, v: i64) {
        self.api.proven_tx_req_id = v;
    }

    fn entity_name(&self) -> &'static str {
        "provenTxReq"
    }

    fn entity_table(&self) -> &'static str {
        "proven_tx_reqs"
    }

    fn update_api(&mut self) {
        // Pack JSON fields into API strings
        self.pack_api_history();
        self.pack_api_notify();
    }

    fn get_api(&self) -> &Self::Api {
        &self.api
    }

    fn equals(&self, other: &Self::Api, sync_map: Option<&SyncMap>) -> bool {
        // Match TypeScript 'convergent' equality logic
        
        // Compare basic fields
        if self.txid() != other.txid
            || !Self::arrays_equal(self.raw_tx(), &other.raw_tx)
            || self.batch() != other.batch.as_deref()
        {
            return false;
        }

        // Compare inputBEEF
        if !Self::optional_arrays_equal(self.input_beef(), other.input_beef.as_ref()) {
            return false;
        }

        if let Some(map) = sync_map {
            // Convergent equality with sync map
            // provenTxReqId mapping
            let other_req_id = map.proven_tx_req.id_map.get(&other.proven_tx_req_id).copied()
                .unwrap_or(other.proven_tx_req_id);
            if self.proven_tx_req_id() != other_req_id {
                return false;
            }

            // provenTxId mapping
            match (self.proven_tx_id(), other.proven_tx_id) {
                (None, Some(_)) => return false,
                (Some(_), None) => return false,
                (Some(local_id), Some(other_id)) => {
                    let mapped_other_id = map.proven_tx.id_map.get(&other_id).copied()
                        .unwrap_or(other_id);
                    if local_id != mapped_other_id {
                        return false;
                    }
                }
                (None, None) => {}
            }
            
            // Note: attempts and history don't matter for convergent equality
        } else {
            // Non-convergent equality (all fields must match)
            if self.proven_tx_req_id() != other.proven_tx_req_id
                || self.proven_tx_id() != other.proven_tx_id
                || self.attempts() != other.attempts
                || self.api.history != other.history
                || self.api.notify != other.notify
            {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_proven_tx_req_new_default() {
        let entity = EntityProvenTxReq::new(None);
        assert_eq!(entity.proven_tx_req_id(), 0);
        assert_eq!(entity.txid(), "");
        assert_eq!(entity.status(), ProvenTxReqStatus::Unknown);
        assert_eq!(entity.attempts(), 0);
        assert_eq!(entity.notified(), false);
        assert_eq!(entity.proven_tx_id(), None);
        assert_eq!(entity.batch(), None);
    }

    #[test]
    fn test_entity_proven_tx_req_new_with_api() {
        let req = TableProvenTxReq {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            proven_tx_req_id: 1,
            proven_tx_id: Some(10),
            status: ProvenTxReqStatus::Completed,
            attempts: 5,
            notified: true,
            txid: "abc123".to_string(),
            batch: Some("batch1".to_string()),
            history: "{}".to_string(),
            notify: "{}".to_string(),
            raw_tx: vec![1, 2, 3],
            input_beef: Some(vec![4, 5, 6]),
        };

        let entity = EntityProvenTxReq::new(Some(req));
        assert_eq!(entity.proven_tx_req_id(), 1);
        assert_eq!(entity.proven_tx_id(), Some(10));
        assert_eq!(entity.status(), ProvenTxReqStatus::Completed);
        assert_eq!(entity.attempts(), 5);
        assert_eq!(entity.notified(), true);
        assert_eq!(entity.txid(), "abc123");
        assert_eq!(entity.batch(), Some("batch1"));
        assert_eq!(entity.raw_tx(), &[1, 2, 3]);
        assert_eq!(entity.input_beef(), Some(&vec![4, 5, 6]));
    }

    #[test]
    fn test_entity_proven_tx_req_property_accessors() {
        let mut entity = EntityProvenTxReq::new(None);

        entity.set_proven_tx_req_id(42);
        assert_eq!(entity.proven_tx_req_id(), 42);

        entity.set_txid("txid123");
        assert_eq!(entity.txid(), "txid123");

        entity.set_status(ProvenTxReqStatus::Unmined);
        assert_eq!(entity.status(), ProvenTxReqStatus::Unmined);

        entity.set_attempts(10);
        assert_eq!(entity.attempts(), 10);

        entity.set_notified(true);
        assert_eq!(entity.notified(), true);

        entity.set_proven_tx_id(Some(99));
        assert_eq!(entity.proven_tx_id(), Some(99));

        entity.set_batch(Some("batch2".to_string()));
        assert_eq!(entity.batch(), Some("batch2"));

        entity.set_raw_tx(vec![7, 8, 9]);
        assert_eq!(entity.raw_tx(), &[7, 8, 9]);

        entity.set_input_beef(Some(vec![10, 11]));
        assert_eq!(entity.input_beef(), Some(&vec![10, 11]));
    }

    #[test]
    fn test_entity_proven_tx_req_pack_unpack_history() {
        let mut entity = EntityProvenTxReq::new(None);
        
        // Add a note to history
        entity.history_mut().notes = Some(vec![
            ReqHistoryNote {
                when: Some("2024-01-01T00:00:00Z".to_string()),
                what: "test".to_string(),
                extra: HashMap::new(),
            }
        ]);

        // Pack it
        entity.pack_api_history();
        let json = entity.get_api().history.clone();
        assert!(json.contains("test"));

        // Unpack it
        entity.history_mut().notes = None;
        entity.unpack_api_history();
        assert!(entity.history().notes.is_some());
        assert_eq!(entity.history().notes.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_entity_proven_tx_req_pack_unpack_notify() {
        let mut entity = EntityProvenTxReq::new(None);
        
        entity.notify_mut().transaction_ids = Some(vec![1, 2, 3]);

        entity.pack_api_notify();
        let json = entity.get_api().notify.clone();
        assert!(json.contains("transactionIds"));

        entity.notify_mut().transaction_ids = None;
        entity.unpack_api_notify();
        assert_eq!(entity.notify().transaction_ids, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_entity_proven_tx_req_equals_same() {
        let req = TableProvenTxReq {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            proven_tx_req_id: 1,
            proven_tx_id: None,
            status: ProvenTxReqStatus::Unmined,
            attempts: 5,
            notified: false,
            txid: "abc123".to_string(),
            batch: None,
            history: "{}".to_string(),
            notify: "{}".to_string(),
            raw_tx: vec![1, 2, 3],
            input_beef: None,
        };

        let entity = EntityProvenTxReq::new(Some(req.clone()));
        assert!(entity.equals(&req, None));
    }

    #[test]
    fn test_entity_proven_tx_req_equals_different_txid() {
        let req1 = TableProvenTxReq {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            proven_tx_req_id: 1,
            proven_tx_id: None,
            status: ProvenTxReqStatus::Unmined,
            attempts: 5,
            notified: false,
            txid: "abc123".to_string(),
            batch: None,
            history: "{}".to_string(),
            notify: "{}".to_string(),
            raw_tx: vec![1, 2, 3],
            input_beef: None,
        };

        let mut req2 = req1.clone();
        req2.txid = "different".to_string();

        let entity = EntityProvenTxReq::new(Some(req1));
        assert!(!entity.equals(&req2, None));
    }

    #[test]
    fn test_entity_proven_tx_req_equals_different_raw_tx() {
        let req1 = TableProvenTxReq {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            proven_tx_req_id: 1,
            proven_tx_id: None,
            status: ProvenTxReqStatus::Unmined,
            attempts: 5,
            notified: false,
            txid: "abc123".to_string(),
            batch: None,
            history: "{}".to_string(),
            notify: "{}".to_string(),
            raw_tx: vec![1, 2, 3],
            input_beef: None,
        };

        let mut req2 = req1.clone();
        req2.raw_tx = vec![4, 5, 6];

        let entity = EntityProvenTxReq::new(Some(req1));
        assert!(!entity.equals(&req2, None));
    }

    #[test]
    fn test_entity_proven_tx_req_entity_name() {
        let entity = EntityProvenTxReq::new(None);
        assert_eq!(entity.entity_name(), "provenTxReq");
    }

    #[test]
    fn test_entity_proven_tx_req_entity_table() {
        let entity = EntityProvenTxReq::new(None);
        assert_eq!(entity.entity_table(), "proven_tx_reqs");
    }

    #[test]
    fn test_entity_proven_tx_req_id_methods() {
        let mut entity = EntityProvenTxReq::new(None);
        
        assert_eq!(entity.id(), 0);
        entity.set_id(999);
        assert_eq!(entity.id(), 999);
        assert_eq!(entity.proven_tx_req_id(), 999);
    }

    #[test]
    fn test_entity_proven_tx_req_update_api() {
        let mut entity = EntityProvenTxReq::new(None);
        
        entity.history_mut().notes = Some(vec![
            ReqHistoryNote {
                when: Some("2024-01-01T00:00:00Z".to_string()),
                what: "test".to_string(),
                extra: HashMap::new(),
            }
        ]);

        entity.update_api();
        
        // History should be packed into JSON string
        assert!(entity.get_api().history.contains("test"));
    }
}
