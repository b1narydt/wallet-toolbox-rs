//! EntityTxLabelMap - Many-to-many mapping entity for transactions and labels
//!
//! Translates TypeScript EntityTxLabelMap class to Rust.
//! Reference: wallet-toolbox/src/storage/schema/entities/EntityTxLabelMap.ts

use crate::schema::tables::TableTxLabelMap;
use super::{EntityBase, SyncMap};

/// TxLabelMap entity wrapper providing merge logic and property accessors
///
/// This entity uses a composite key (transactionId, txLabelId) and has no primary id.
/// Matches TypeScript `EntityTxLabelMap` class
#[derive(Debug, Clone)]
pub struct EntityTxLabelMap {
    api: TableTxLabelMap,
}

impl EntityTxLabelMap {
    /// Create new EntityTxLabelMap from table data
    pub fn new(api: TableTxLabelMap) -> Self {
        Self { api }
    }

    /// Create new EntityTxLabelMap with default values
    pub fn new_default() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            api: TableTxLabelMap {
                created_at: now.clone(),
                updated_at: now,
                transaction_id: 0,
                tx_label_id: 0,
                is_deleted: false,
            },
        }
    }

    // Property accessors

    pub fn tx_label_id(&self) -> i64 {
        self.api.tx_label_id
    }

    pub fn set_tx_label_id(&mut self, v: i64) {
        self.api.tx_label_id = v;
    }

    pub fn transaction_id(&self) -> i64 {
        self.api.transaction_id
    }

    pub fn set_transaction_id(&mut self, v: i64) {
        self.api.transaction_id = v;
    }

    pub fn created_at(&self) -> &str {
        &self.api.created_at
    }

    pub fn set_created_at(&mut self, v: String) {
        self.api.created_at = v;
    }

    pub fn updated_at(&self) -> &str {
        &self.api.updated_at
    }

    pub fn set_updated_at(&mut self, v: String) {
        self.api.updated_at = v;
    }

    pub fn is_deleted(&self) -> bool {
        self.api.is_deleted
    }

    pub fn set_is_deleted(&mut self, v: bool) {
        self.api.is_deleted = v;
    }

    pub fn touch(&mut self) {
        self.api.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

impl EntityBase for EntityTxLabelMap {
    type Api = TableTxLabelMap;

    fn id(&self) -> i64 {
        // Match TypeScript: "entity has no 'id' value"
        panic!("entity has no 'id' value");
    }

    fn set_id(&mut self, _id: i64) {
        // Match TypeScript: entity has no id setter
        panic!("entity has no 'id' value");
    }

    fn entity_name(&self) -> &'static str {
        "txLabelMap"
    }

    fn entity_table(&self) -> &'static str {
        "tx_labels_map"
    }

    fn update_api(&mut self) {
        // No special encoding needed
    }

    fn get_api(&self) -> &Self::Api {
        &self.api
    }

    fn equals(&self, other: &Self::Api, sync_map: Option<&SyncMap>) -> bool {
        let eo = &self.api;

        // Match TypeScript equals logic with sync map support
        if let Some(map) = sync_map {
            // With sync map: compare mapped IDs
            let mapped_tx_id = map.transaction.id_map.get(&other.transaction_id).copied().unwrap_or(other.transaction_id);
            let mapped_label_id = map.tx_label.id_map.get(&other.tx_label_id).copied().unwrap_or(other.tx_label_id);
            
            if eo.transaction_id != mapped_tx_id || eo.tx_label_id != mapped_label_id {
                return false;
            }
        } else {
            // Without sync map: direct comparison
            if eo.transaction_id != other.transaction_id || eo.tx_label_id != other.tx_label_id {
                return false;
            }
        }

        // Compare is_deleted
        if eo.is_deleted != other.is_deleted {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_tx_label_map_new_default() {
        let entity = EntityTxLabelMap::new_default();
        assert_eq!(entity.transaction_id(), 0);
        assert_eq!(entity.tx_label_id(), 0);
        assert!(!entity.is_deleted());
    }

    #[test]
    fn test_entity_tx_label_map_property_accessors() {
        let mut entity = EntityTxLabelMap::new_default();
        
        entity.set_transaction_id(100);
        entity.set_tx_label_id(200);
        entity.set_is_deleted(true);
        
        assert_eq!(entity.transaction_id(), 100);
        assert_eq!(entity.tx_label_id(), 200);
        assert!(entity.is_deleted());
    }

    #[test]
    #[should_panic(expected = "entity has no 'id' value")]
    fn test_entity_tx_label_map_id_panics() {
        let entity = EntityTxLabelMap::new_default();
        let _ = entity.id(); // Should panic
    }

    #[test]
    #[should_panic(expected = "entity has no 'id' value")]
    fn test_entity_tx_label_map_set_id_panics() {
        let mut entity = EntityTxLabelMap::new_default();
        entity.set_id(1); // Should panic
    }

    #[test]
    fn test_entity_tx_label_map_entity_name() {
        let entity = EntityTxLabelMap::new_default();
        assert_eq!(entity.entity_name(), "txLabelMap");
    }

    #[test]
    fn test_entity_tx_label_map_entity_table() {
        let entity = EntityTxLabelMap::new_default();
        assert_eq!(entity.entity_table(), "tx_labels_map");
    }

    #[test]
    fn test_entity_tx_label_map_equals_without_sync_map() {
        let entity = EntityTxLabelMap::new_default();
        let mut other = TableTxLabelMap {
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            transaction_id: 0,
            tx_label_id: 0,
            is_deleted: false,
        };
        
        assert!(entity.equals(&other, None));
        
        other.transaction_id = 1;
        assert!(!entity.equals(&other, None));
        
        other.transaction_id = 0;
        other.is_deleted = true;
        assert!(!entity.equals(&other, None));
    }

    #[test]
    fn test_entity_tx_label_map_equals_with_sync_map() {
        let mut entity = EntityTxLabelMap::new_default();
        entity.set_transaction_id(100);
        entity.set_tx_label_id(200);
        
        let mut sync_map = SyncMap::new();
        sync_map.transaction.id_map.insert(10, 100);
        sync_map.tx_label.id_map.insert(20, 200);
        
        let other = TableTxLabelMap {
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            transaction_id: 10,
            tx_label_id: 20,
            is_deleted: false,
        };
        
        // Should match because 10 maps to 100 and 20 maps to 200
        assert!(entity.equals(&other, Some(&sync_map)));
    }

    #[test]
    fn test_entity_tx_label_map_touch() {
        let mut entity = EntityTxLabelMap::new_default();
        let old_time = entity.updated_at().to_string();
        
        std::thread::sleep(std::time::Duration::from_millis(10));
        entity.touch();
        
        assert_ne!(entity.updated_at(), old_time);
    }

    #[test]
    fn test_entity_tx_label_map_to_api() {
        let mut entity = EntityTxLabelMap::new_default();
        entity.set_transaction_id(42);
        
        let api = entity.to_api();
        assert_eq!(api.transaction_id, 42);
    }
}
