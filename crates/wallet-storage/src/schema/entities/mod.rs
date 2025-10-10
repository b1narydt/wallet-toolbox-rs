//! Entity wrappers for storage tables
//!
//! Entities provide:
//! - Property accessors
//! - Merge logic for synchronization
//! - Equality checks
//! - API conversion
//!
//! Translates TypeScript entity classes to Rust.
//! Reference: wallet-toolbox/src/storage/schema/entities/

use crate::schema::tables::*;
use crate::StorageError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod entity_user;
pub mod entity_transaction;
pub mod entity_output;
pub mod entity_proven_tx;
pub mod entity_proven_tx_req;
pub mod entity_certificate;
pub mod entity_certificate_field;
pub mod entity_output_basket;
pub mod entity_output_tag;
pub mod entity_output_tag_map;
pub mod entity_tx_label;
pub mod entity_tx_label_map;
pub mod entity_commission;
pub mod entity_sync_state;
pub mod merge_entity;

pub use entity_user::EntityUser;
pub use entity_transaction::EntityTransaction;
pub use entity_output::EntityOutput;
pub use entity_proven_tx::EntityProvenTx;
pub use entity_proven_tx_req::EntityProvenTxReq;
pub use entity_certificate::EntityCertificate;
pub use entity_certificate_field::EntityCertificateField;
pub use entity_output_basket::EntityOutputBasket;
pub use entity_output_tag::EntityOutputTag;
pub use entity_output_tag_map::EntityOutputTagMap;
pub use entity_tx_label::EntityTxLabel;
pub use entity_tx_label_map::EntityTxLabelMap;
pub use entity_commission::EntityCommission;
pub use entity_sync_state::EntitySyncState;
pub use merge_entity::{MergeEntity, max_date};

/// Entity synchronization map for tracking foreign-to-local ID mappings
///
/// Matches TypeScript `EntitySyncMap` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitySyncMap {
    #[serde(rename = "entityName")]
    pub entity_name: String,
    
    /// Maps foreign IDs to local IDs
    #[serde(rename = "idMap")]
    pub id_map: HashMap<i64, i64>,
    
    /// Maximum updated_at value seen for this entity
    #[serde(rename = "maxUpdated_at", skip_serializing_if = "Option::is_none")]
    pub max_updated_at: Option<String>,
    
    /// Cumulative count of items received
    pub count: usize,
}

impl EntitySyncMap {
    pub fn new(entity_name: impl Into<String>) -> Self {
        Self {
            entity_name: entity_name.into(),
            id_map: HashMap::new(),
            max_updated_at: None,
            count: 0,
        }
    }
}

/// Complete synchronization map for all entity types
///
/// Matches TypeScript `SyncMap` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncMap {
    #[serde(rename = "provenTx")]
    pub proven_tx: EntitySyncMap,
    
    #[serde(rename = "outputBasket")]
    pub output_basket: EntitySyncMap,
    
    pub transaction: EntitySyncMap,
    
    #[serde(rename = "provenTxReq")]
    pub proven_tx_req: EntitySyncMap,
    
    #[serde(rename = "txLabel")]
    pub tx_label: EntitySyncMap,
    
    #[serde(rename = "txLabelMap")]
    pub tx_label_map: EntitySyncMap,
    
    pub output: EntitySyncMap,
    
    #[serde(rename = "outputTag")]
    pub output_tag: EntitySyncMap,
    
    #[serde(rename = "outputTagMap")]
    pub output_tag_map: EntitySyncMap,
    
    pub certificate: EntitySyncMap,
    
    #[serde(rename = "certificateField")]
    pub certificate_field: EntitySyncMap,
    
    pub commission: EntitySyncMap,
}

impl SyncMap {
    /// Create a new SyncMap with all entities initialized
    ///
    /// Matches TypeScript `createSyncMap` function
    pub fn new() -> Self {
        Self {
            proven_tx: EntitySyncMap::new("provenTx"),
            output_basket: EntitySyncMap::new("outputBasket"),
            transaction: EntitySyncMap::new("transaction"),
            proven_tx_req: EntitySyncMap::new("provenTxReq"),
            tx_label: EntitySyncMap::new("txLabel"),
            tx_label_map: EntitySyncMap::new("txLabelMap"),
            output: EntitySyncMap::new("output"),
            output_tag: EntitySyncMap::new("outputTag"),
            output_tag_map: EntitySyncMap::new("outputTagMap"),
            certificate: EntitySyncMap::new("certificate"),
            certificate_field: EntitySyncMap::new("certificateField"),
            commission: EntitySyncMap::new("commission"),
        }
    }
}

impl Default for SyncMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Sync error information
///
/// Matches TypeScript `SyncError` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncError {
    pub code: String,
    pub description: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
}

/// Base entity trait - defines common entity operations
///
/// Matches TypeScript `EntityBase<T>` abstract class
pub trait EntityBase {
    /// Type of the underlying table record
    type Api;
    
    /// Get the entity's database ID
    fn id(&self) -> i64;
    
    /// Set the entity's database ID
    fn set_id(&mut self, id: i64);
    
    /// Get the entity name
    fn entity_name(&self) -> &'static str;
    
    /// Get the entity table name
    fn entity_table(&self) -> &'static str;
    
    /// Update the underlying API object from entity state
    fn update_api(&mut self);
    
    /// Get the underlying API object (forcing update)
    fn to_api(&mut self) -> &Self::Api {
        self.update_api();
        self.get_api()
    }
    
    /// Get reference to underlying API object
    fn get_api(&self) -> &Self::Api;
    
    /// Test for equality (or convergent equality if sync_map provided)
    fn equals(&self, other: &Self::Api, sync_map: Option<&SyncMap>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_sync_map_new() {
        let map = EntitySyncMap::new("test");
        assert_eq!(map.entity_name, "test");
        assert_eq!(map.count, 0);
        assert!(map.id_map.is_empty());
        assert!(map.max_updated_at.is_none());
    }

    #[test]
    fn test_sync_map_new() {
        let sync_map = SyncMap::new();
        assert_eq!(sync_map.proven_tx.entity_name, "provenTx");
        assert_eq!(sync_map.output_basket.entity_name, "outputBasket");
        assert_eq!(sync_map.transaction.entity_name, "transaction");
        assert_eq!(sync_map.proven_tx_req.entity_name, "provenTxReq");
        assert_eq!(sync_map.tx_label.entity_name, "txLabel");
        assert_eq!(sync_map.output.entity_name, "output");
        assert_eq!(sync_map.certificate.entity_name, "certificate");
    }

    #[test]
    fn test_sync_map_default() {
        let sync_map = SyncMap::default();
        assert_eq!(sync_map.proven_tx.count, 0);
        assert!(sync_map.output_basket.id_map.is_empty());
    }

    #[test]
    fn test_entity_sync_map_serialization() {
        let mut map = EntitySyncMap::new("test");
        map.id_map.insert(1, 100);
        map.count = 5;
        
        let json = serde_json::to_string(&map).unwrap();
        assert!(json.contains("\"entityName\":\"test\""));
        assert!(json.contains("\"count\":5"));
        
        let deserialized: EntitySyncMap = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.entity_name, "test");
        assert_eq!(deserialized.count, 5);
        assert_eq!(deserialized.id_map.get(&1), Some(&100));
    }

    #[test]
    fn test_sync_map_serialization() {
        let sync_map = SyncMap::new();
        let json = serde_json::to_string(&sync_map).unwrap();
        
        // Verify camelCase field names
        assert!(json.contains("\"provenTx\""));
        assert!(json.contains("\"outputBasket\""));
        assert!(json.contains("\"provenTxReq\""));
        
        let deserialized: SyncMap = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.proven_tx.entity_name, sync_map.proven_tx.entity_name);
    }

    #[test]
    fn test_sync_error() {
        let error = SyncError {
            code: "ERR_001".to_string(),
            description: "Test error".to_string(),
            stack: Some("line 1\nline 2".to_string()),
        };
        
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("\"code\":\"ERR_001\""));
        assert!(json.contains("\"description\":\"Test error\""));
        
        let deserialized: SyncError = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.code, "ERR_001");
    }
}
