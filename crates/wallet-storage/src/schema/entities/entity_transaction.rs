//! EntityTransaction - Transaction entity wrapper
//!
//! Translates TypeScript EntityTransaction class to Rust.
//! Reference: wallet-toolbox/src/storage/schema/entities/EntityTransaction.ts

use crate::schema::tables::TableTransaction;
use super::{EntityBase, SyncMap};

/// Transaction entity wrapper providing merge logic and property accessors
///
/// Matches TypeScript `EntityTransaction` class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityTransaction {
    api: TableTransaction,
}

impl EntityTransaction {
    /// Create new EntityTransaction from table record
    pub fn new(api: Option<TableTransaction>) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            api: api.unwrap_or_else(|| TableTransaction {
                created_at: now.clone(),
                updated_at: now,
                transaction_id: 0,
                user_id: 0,
                proven_tx_id: None,
                status: crate::schema::tables::TransactionStatus::Unprocessed,
                reference: String::new(),
                is_outgoing: false,
                satoshis: 0,
                description: String::new(),
                version: None,
                lock_time: None,
                txid: None,
                input_beef: None,
                raw_tx: None,
            }),
        }
    }

    // Property accessors matching TypeScript getters/setters

    pub fn transaction_id(&self) -> i64 {
        self.api.transaction_id
    }

    pub fn set_transaction_id(&mut self, v: i64) {
        self.api.transaction_id = v;
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

    pub fn version(&self) -> Option<u32> {
        self.api.version
    }

    pub fn set_version(&mut self, v: Option<u32>) {
        self.api.version = v;
    }

    pub fn lock_time(&self) -> Option<u32> {
        self.api.lock_time
    }

    pub fn set_lock_time(&mut self, v: Option<u32>) {
        self.api.lock_time = v;
    }

    pub fn is_outgoing(&self) -> bool {
        self.api.is_outgoing
    }

    pub fn set_is_outgoing(&mut self, v: bool) {
        self.api.is_outgoing = v;
    }

    pub fn status(&self) -> crate::schema::tables::TransactionStatus {
        self.api.status
    }

    pub fn set_status(&mut self, v: crate::schema::tables::TransactionStatus) {
        self.api.status = v;
    }

    pub fn user_id(&self) -> i64 {
        self.api.user_id
    }

    pub fn set_user_id(&mut self, v: i64) {
        self.api.user_id = v;
    }

    pub fn proven_tx_id(&self) -> Option<i64> {
        self.api.proven_tx_id
    }

    pub fn set_proven_tx_id(&mut self, v: Option<i64>) {
        self.api.proven_tx_id = v;
    }

    pub fn satoshis(&self) -> i64 {
        self.api.satoshis
    }

    pub fn set_satoshis(&mut self, v: i64) {
        self.api.satoshis = v;
    }

    pub fn txid(&self) -> Option<&str> {
        self.api.txid.as_deref()
    }

    pub fn set_txid(&mut self, v: Option<String>) {
        self.api.txid = v;
    }

    pub fn reference(&self) -> &str {
        &self.api.reference
    }

    pub fn set_reference(&mut self, v: impl Into<String>) {
        self.api.reference = v.into();
    }

    pub fn input_beef(&self) -> Option<&Vec<u8>> {
        self.api.input_beef.as_ref()
    }

    pub fn set_input_beef(&mut self, v: Option<Vec<u8>>) {
        self.api.input_beef = v;
    }

    pub fn description(&self) -> &str {
        &self.api.description
    }

    pub fn set_description(&mut self, v: impl Into<String>) {
        self.api.description = v.into();
    }

    pub fn raw_tx(&self) -> Option<&Vec<u8>> {
        self.api.raw_tx.as_ref()
    }

    pub fn set_raw_tx(&mut self, v: Option<Vec<u8>>) {
        self.api.raw_tx = v;
    }

    /// Get mutable reference to underlying API
    pub fn get_api_mut(&mut self) -> &mut TableTransaction {
        &mut self.api
    }

    /// Consume entity and return API
    pub fn into_api(self) -> TableTransaction {
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
}

impl EntityBase for EntityTransaction {
    type Api = TableTransaction;

    fn id(&self) -> i64 {
        self.api.transaction_id
    }

    fn set_id(&mut self, v: i64) {
        self.api.transaction_id = v;
    }

    fn entity_name(&self) -> &'static str {
        "transaction"
    }

    fn entity_table(&self) -> &'static str {
        "transactions"
    }

    fn update_api(&mut self) {
        // Nothing needed yet - matches TypeScript implementation
    }

    fn get_api(&self) -> &Self::Api {
        &self.api
    }

    fn equals(&self, other: &Self::Api, sync_map: Option<&SyncMap>) -> bool {
        let eo = &self.api;

        // Properties that are never updated - transaction_id and reference
        let other_transaction_id = if let Some(map) = sync_map {
            map.transaction.id_map.get(&other.transaction_id).copied().unwrap_or(other.transaction_id)
        } else {
            other.transaction_id
        };

        if eo.transaction_id != other_transaction_id || eo.reference != other.reference {
            return false;
        }

        // Compare all mutable properties
        if eo.version != other.version
            || eo.lock_time != other.lock_time
            || eo.is_outgoing != other.is_outgoing
            || eo.status != other.status
            || eo.satoshis != other.satoshis
            || eo.txid != other.txid
            || eo.description != other.description
            || !Self::optional_arrays_equal(eo.raw_tx.as_ref(), other.raw_tx.as_ref())
            || !Self::optional_arrays_equal(eo.input_beef.as_ref(), other.input_beef.as_ref())
        {
            return false;
        }

        // Compare provenTxId with optional sync_map mapping
        match (eo.proven_tx_id, other.proven_tx_id) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(eo_id), Some(ei_id)) => {
                if let Some(map) = sync_map {
                    let mapped_id = map.proven_tx.id_map.get(&ei_id).copied().unwrap_or(ei_id);
                    eo_id == mapped_id
                } else {
                    eo_id == ei_id
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::tables::TransactionStatus;

    #[test]
    fn test_entity_transaction_new_default() {
        let entity = EntityTransaction::new(None);
        assert_eq!(entity.transaction_id(), 0);
        assert_eq!(entity.user_id(), 0);
        assert_eq!(entity.reference(), "");
        assert_eq!(entity.is_outgoing(), false);
        assert_eq!(entity.satoshis(), 0);
        assert_eq!(entity.status(), TransactionStatus::Unprocessed);
    }

    #[test]
    fn test_entity_transaction_new_with_api() {
        let tx = TableTransaction {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            transaction_id: 1,
            user_id: 100,
            proven_tx_id: Some(50),
            status: TransactionStatus::Completed,
            reference: "ref123".to_string(),
            is_outgoing: true,
            satoshis: 5000,
            description: "Test tx".to_string(),
            version: Some(1),
            lock_time: Some(0),
            txid: Some("abc123".to_string()),
            input_beef: Some(vec![1, 2, 3]),
            raw_tx: Some(vec![4, 5, 6]),
        };

        let entity = EntityTransaction::new(Some(tx));
        assert_eq!(entity.transaction_id(), 1);
        assert_eq!(entity.user_id(), 100);
        assert_eq!(entity.proven_tx_id(), Some(50));
        assert_eq!(entity.reference(), "ref123");
        assert_eq!(entity.satoshis(), 5000);
    }

    #[test]
    fn test_entity_transaction_property_accessors() {
        let mut entity = EntityTransaction::new(None);

        entity.set_transaction_id(42);
        assert_eq!(entity.transaction_id(), 42);

        entity.set_user_id(100);
        assert_eq!(entity.user_id(), 100);

        entity.set_reference("ref456");
        assert_eq!(entity.reference(), "ref456");

        entity.set_is_outgoing(true);
        assert_eq!(entity.is_outgoing(), true);

        entity.set_satoshis(10000);
        assert_eq!(entity.satoshis(), 10000);

        entity.set_status(TransactionStatus::Completed);
        assert_eq!(entity.status(), TransactionStatus::Completed);

        entity.set_version(Some(2));
        assert_eq!(entity.version(), Some(2));

        entity.set_lock_time(Some(500000));
        assert_eq!(entity.lock_time(), Some(500000));

        entity.set_txid(Some("txid789".to_string()));
        assert_eq!(entity.txid(), Some("txid789"));

        entity.set_description("Updated description");
        assert_eq!(entity.description(), "Updated description");

        entity.set_proven_tx_id(Some(99));
        assert_eq!(entity.proven_tx_id(), Some(99));
    }

    #[test]
    fn test_entity_transaction_equals_same() {
        let tx1 = TableTransaction {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            transaction_id: 1,
            user_id: 100,
            proven_tx_id: None,
            status: TransactionStatus::Completed,
            reference: "ref123".to_string(),
            is_outgoing: true,
            satoshis: 5000,
            description: "Test".to_string(),
            version: Some(1),
            lock_time: Some(0),
            txid: Some("abc".to_string()),
            input_beef: None,
            raw_tx: None,
        };

        let entity = EntityTransaction::new(Some(tx1.clone()));
        assert!(entity.equals(&tx1, None));
    }

    #[test]
    fn test_entity_transaction_equals_different_reference() {
        let tx1 = TableTransaction {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            transaction_id: 1,
            user_id: 100,
            proven_tx_id: None,
            status: TransactionStatus::Completed,
            reference: "ref123".to_string(),
            is_outgoing: true,
            satoshis: 5000,
            description: "Test".to_string(),
            version: None,
            lock_time: None,
            txid: None,
            input_beef: None,
            raw_tx: None,
        };

        let mut tx2 = tx1.clone();
        tx2.reference = "ref456".to_string();

        let entity = EntityTransaction::new(Some(tx1));
        assert!(!entity.equals(&tx2, None));
    }

    #[test]
    fn test_entity_transaction_equals_different_status() {
        let tx1 = TableTransaction {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            transaction_id: 1,
            user_id: 100,
            proven_tx_id: None,
            status: TransactionStatus::Unprocessed,
            reference: "ref123".to_string(),
            is_outgoing: true,
            satoshis: 5000,
            description: "Test".to_string(),
            version: None,
            lock_time: None,
            txid: None,
            input_beef: None,
            raw_tx: None,
        };

        let mut tx2 = tx1.clone();
        tx2.status = TransactionStatus::Completed;

        let entity = EntityTransaction::new(Some(tx1));
        assert!(!entity.equals(&tx2, None));
    }

    #[test]
    fn test_entity_transaction_equals_with_raw_tx() {
        let tx1 = TableTransaction {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            transaction_id: 1,
            user_id: 100,
            proven_tx_id: None,
            status: TransactionStatus::Completed,
            reference: "ref123".to_string(),
            is_outgoing: true,
            satoshis: 5000,
            description: "Test".to_string(),
            version: None,
            lock_time: None,
            txid: None,
            input_beef: None,
            raw_tx: Some(vec![1, 2, 3]),
        };

        let entity = EntityTransaction::new(Some(tx1.clone()));
        assert!(entity.equals(&tx1, None));

        let mut tx2 = tx1.clone();
        tx2.raw_tx = Some(vec![4, 5, 6]);
        assert!(!entity.equals(&tx2, None));

        let mut tx3 = tx1;
        tx3.raw_tx = None;
        assert!(!entity.equals(&tx3, None));
    }

    #[test]
    fn test_entity_transaction_entity_name() {
        let entity = EntityTransaction::new(None);
        assert_eq!(entity.entity_name(), "transaction");
    }

    #[test]
    fn test_entity_transaction_entity_table() {
        let entity = EntityTransaction::new(None);
        assert_eq!(entity.entity_table(), "transactions");
    }

    #[test]
    fn test_entity_transaction_id_methods() {
        let mut entity = EntityTransaction::new(None);
        
        assert_eq!(entity.id(), 0);
        entity.set_id(999);
        assert_eq!(entity.id(), 999);
        assert_eq!(entity.transaction_id(), 999);
    }

    #[test]
    fn test_entity_transaction_to_api() {
        let mut entity = EntityTransaction::new(None);
        entity.set_transaction_id(1);
        entity.set_reference("ref");
        
        let api = entity.to_api();
        assert_eq!(api.transaction_id, 1);
        assert_eq!(api.reference, "ref");
    }

    #[test]
    fn test_entity_transaction_into_api() {
        let mut entity = EntityTransaction::new(None);
        entity.set_transaction_id(1);
        entity.set_satoshis(5000);
        
        let api = entity.into_api();
        assert_eq!(api.transaction_id, 1);
        assert_eq!(api.satoshis, 5000);
    }

    #[test]
    fn test_entity_transaction_clone() {
        let tx = TableTransaction {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            transaction_id: 1,
            user_id: 100,
            proven_tx_id: Some(50),
            status: TransactionStatus::Completed,
            reference: "ref123".to_string(),
            is_outgoing: true,
            satoshis: 5000,
            description: "Test".to_string(),
            version: Some(1),
            lock_time: None,
            txid: None,
            input_beef: None,
            raw_tx: None,
        };

        let entity1 = EntityTransaction::new(Some(tx));
        let entity2 = entity1.clone();
        
        assert_eq!(entity1, entity2);
        assert_eq!(entity2.transaction_id(), 1);
    }
}
