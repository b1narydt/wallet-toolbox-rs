//! EntityProvenTx - Proven transaction entity wrapper
//!
//! Translates TypeScript EntityProvenTx class to Rust.
//! Reference: wallet-toolbox/src/storage/schema/entities/EntityProvenTx.ts

use crate::schema::tables::TableProvenTx;
use super::{EntityBase, SyncMap};

/// ProvenTx entity wrapper providing merge logic and property accessors
///
/// Matches TypeScript `EntityProvenTx` class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityProvenTx {
    api: TableProvenTx,
}

impl EntityProvenTx {
    /// Create new EntityProvenTx from table record
    pub fn new(api: Option<TableProvenTx>) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            api: api.unwrap_or_else(|| TableProvenTx {
                created_at: now.clone(),
                updated_at: now,
                proven_tx_id: 0,
                txid: String::new(),
                height: 0,
                index: 0,
                merkle_path: Vec::new(),
                raw_tx: Vec::new(),
                block_hash: String::new(),
                merkle_root: String::new(),
            }),
        }
    }

    // Property accessors matching TypeScript getters/setters

    pub fn proven_tx_id(&self) -> i64 {
        self.api.proven_tx_id
    }

    pub fn set_proven_tx_id(&mut self, v: i64) {
        self.api.proven_tx_id = v;
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

    pub fn height(&self) -> i64 {
        self.api.height
    }

    pub fn set_height(&mut self, v: i64) {
        self.api.height = v;
    }

    pub fn index(&self) -> i64 {
        self.api.index
    }

    pub fn set_index(&mut self, v: i64) {
        self.api.index = v;
    }

    pub fn merkle_path(&self) -> &[u8] {
        &self.api.merkle_path
    }

    pub fn set_merkle_path(&mut self, v: Vec<u8>) {
        self.api.merkle_path = v;
    }

    pub fn raw_tx(&self) -> &[u8] {
        &self.api.raw_tx
    }

    pub fn set_raw_tx(&mut self, v: Vec<u8>) {
        self.api.raw_tx = v;
    }

    pub fn block_hash(&self) -> &str {
        &self.api.block_hash
    }

    pub fn set_block_hash(&mut self, v: impl Into<String>) {
        self.api.block_hash = v.into();
    }

    pub fn merkle_root(&self) -> &str {
        &self.api.merkle_root
    }

    pub fn set_merkle_root(&mut self, v: impl Into<String>) {
        self.api.merkle_root = v.into();
    }

    /// Get mutable reference to underlying API
    pub fn get_api_mut(&mut self) -> &mut TableProvenTx {
        &mut self.api
    }

    /// Consume entity and return API
    pub fn into_api(self) -> TableProvenTx {
        self.api
    }

    /// Helper to compare byte arrays
    fn arrays_equal(a: &[u8], b: &[u8]) -> bool {
        a == b
    }
}

impl EntityBase for EntityProvenTx {
    type Api = TableProvenTx;

    fn id(&self) -> i64 {
        self.api.proven_tx_id
    }

    fn set_id(&mut self, v: i64) {
        self.api.proven_tx_id = v;
    }

    fn entity_name(&self) -> &'static str {
        "provenTx"
    }

    fn entity_table(&self) -> &'static str {
        "proven_txs"
    }

    fn update_api(&mut self) {
        // Nothing needed yet - matches TypeScript implementation
    }

    fn get_api(&self) -> &Self::Api {
        &self.api
    }

    fn equals(&self, other: &Self::Api, sync_map: Option<&SyncMap>) -> bool {
        // Match TypeScript equals logic exactly
        // Note: equality does not depend on timestamps per TypeScript comments
        
        // Compare provenTxId with optional sync map
        if let Some(map) = sync_map {
            let other_proven_tx_id = map.proven_tx.id_map.get(&other.proven_tx_id).copied().unwrap_or(other.proven_tx_id);
            if self.proven_tx_id() != other_proven_tx_id {
                return false;
            }
        } else {
            if self.proven_tx_id() != other.proven_tx_id {
                return false;
            }
        }

        // Compare all other fields (except timestamps)
        if self.txid() != other.txid
            || self.height() != other.height
            || self.index() != other.index
            || !Self::arrays_equal(self.merkle_path(), &other.merkle_path)
            || !Self::arrays_equal(self.raw_tx(), &other.raw_tx)
            || self.block_hash() != other.block_hash
            || self.merkle_root() != other.merkle_root
        {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_proven_tx_new_default() {
        let entity = EntityProvenTx::new(None);
        assert_eq!(entity.proven_tx_id(), 0);
        assert_eq!(entity.txid(), "");
        assert_eq!(entity.height(), 0);
        assert_eq!(entity.index(), 0);
        assert_eq!(entity.merkle_path(), &[] as &[u8]);
        assert_eq!(entity.raw_tx(), &[] as &[u8]);
        assert_eq!(entity.block_hash(), "");
        assert_eq!(entity.merkle_root(), "");
    }

    #[test]
    fn test_entity_proven_tx_new_with_api() {
        let proven_tx = TableProvenTx {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            proven_tx_id: 1,
            txid: "abc123".to_string(),
            height: 700000,
            index: 5,
            merkle_path: vec![1, 2, 3],
            raw_tx: vec![4, 5, 6],
            block_hash: "block123".to_string(),
            merkle_root: "root123".to_string(),
        };

        let entity = EntityProvenTx::new(Some(proven_tx));
        assert_eq!(entity.proven_tx_id(), 1);
        assert_eq!(entity.txid(), "abc123");
        assert_eq!(entity.height(), 700000);
        assert_eq!(entity.index(), 5);
        assert_eq!(entity.merkle_path(), &[1, 2, 3]);
        assert_eq!(entity.raw_tx(), &[4, 5, 6]);
        assert_eq!(entity.block_hash(), "block123");
        assert_eq!(entity.merkle_root(), "root123");
    }

    #[test]
    fn test_entity_proven_tx_property_accessors() {
        let mut entity = EntityProvenTx::new(None);

        entity.set_proven_tx_id(42);
        assert_eq!(entity.proven_tx_id(), 42);

        entity.set_txid("txid123");
        assert_eq!(entity.txid(), "txid123");

        entity.set_height(800000);
        assert_eq!(entity.height(), 800000);

        entity.set_index(10);
        assert_eq!(entity.index(), 10);

        entity.set_merkle_path(vec![1, 2, 3, 4]);
        assert_eq!(entity.merkle_path(), &[1, 2, 3, 4]);

        entity.set_raw_tx(vec![5, 6, 7, 8]);
        assert_eq!(entity.raw_tx(), &[5, 6, 7, 8]);

        entity.set_block_hash("hash123");
        assert_eq!(entity.block_hash(), "hash123");

        entity.set_merkle_root("root456");
        assert_eq!(entity.merkle_root(), "root456");

        entity.set_created_at("2024-01-01T00:00:00Z");
        assert_eq!(entity.created_at(), "2024-01-01T00:00:00Z");

        entity.set_updated_at("2024-01-02T00:00:00Z");
        assert_eq!(entity.updated_at(), "2024-01-02T00:00:00Z");
    }

    #[test]
    fn test_entity_proven_tx_equals_same() {
        let proven_tx = TableProvenTx {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            proven_tx_id: 1,
            txid: "abc123".to_string(),
            height: 700000,
            index: 5,
            merkle_path: vec![1, 2, 3],
            raw_tx: vec![4, 5, 6],
            block_hash: "block123".to_string(),
            merkle_root: "root123".to_string(),
        };

        let entity = EntityProvenTx::new(Some(proven_tx.clone()));
        assert!(entity.equals(&proven_tx, None));
    }

    #[test]
    fn test_entity_proven_tx_equals_different_txid() {
        let proven_tx1 = TableProvenTx {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            proven_tx_id: 1,
            txid: "abc123".to_string(),
            height: 700000,
            index: 5,
            merkle_path: vec![1, 2, 3],
            raw_tx: vec![4, 5, 6],
            block_hash: "block123".to_string(),
            merkle_root: "root123".to_string(),
        };

        let mut proven_tx2 = proven_tx1.clone();
        proven_tx2.txid = "different".to_string();

        let entity = EntityProvenTx::new(Some(proven_tx1));
        assert!(!entity.equals(&proven_tx2, None));
    }

    #[test]
    fn test_entity_proven_tx_equals_different_height() {
        let proven_tx1 = TableProvenTx {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            proven_tx_id: 1,
            txid: "abc123".to_string(),
            height: 700000,
            index: 5,
            merkle_path: vec![1, 2, 3],
            raw_tx: vec![4, 5, 6],
            block_hash: "block123".to_string(),
            merkle_root: "root123".to_string(),
        };

        let mut proven_tx2 = proven_tx1.clone();
        proven_tx2.height = 800000;

        let entity = EntityProvenTx::new(Some(proven_tx1));
        assert!(!entity.equals(&proven_tx2, None));
    }

    #[test]
    fn test_entity_proven_tx_equals_different_merkle_path() {
        let proven_tx1 = TableProvenTx {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            proven_tx_id: 1,
            txid: "abc123".to_string(),
            height: 700000,
            index: 5,
            merkle_path: vec![1, 2, 3],
            raw_tx: vec![4, 5, 6],
            block_hash: "block123".to_string(),
            merkle_root: "root123".to_string(),
        };

        let mut proven_tx2 = proven_tx1.clone();
        proven_tx2.merkle_path = vec![9, 9, 9];

        let entity = EntityProvenTx::new(Some(proven_tx1));
        assert!(!entity.equals(&proven_tx2, None));
    }

    #[test]
    fn test_entity_proven_tx_equals_different_raw_tx() {
        let proven_tx1 = TableProvenTx {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            proven_tx_id: 1,
            txid: "abc123".to_string(),
            height: 700000,
            index: 5,
            merkle_path: vec![1, 2, 3],
            raw_tx: vec![4, 5, 6],
            block_hash: "block123".to_string(),
            merkle_root: "root123".to_string(),
        };

        let mut proven_tx2 = proven_tx1.clone();
        proven_tx2.raw_tx = vec![7, 8, 9];

        let entity = EntityProvenTx::new(Some(proven_tx1));
        assert!(!entity.equals(&proven_tx2, None));
    }

    #[test]
    fn test_entity_proven_tx_equals_ignores_timestamps() {
        // Per TypeScript comment: equality does not depend on timestamps
        let proven_tx1 = TableProvenTx {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            proven_tx_id: 1,
            txid: "abc123".to_string(),
            height: 700000,
            index: 5,
            merkle_path: vec![1, 2, 3],
            raw_tx: vec![4, 5, 6],
            block_hash: "block123".to_string(),
            merkle_root: "root123".to_string(),
        };

        let mut proven_tx2 = proven_tx1.clone();
        proven_tx2.created_at = "2024-12-31T23:59:59Z".to_string();
        proven_tx2.updated_at = "2024-12-31T23:59:59Z".to_string();

        let entity = EntityProvenTx::new(Some(proven_tx1));
        // Should still be equal since timestamps are ignored
        assert!(entity.equals(&proven_tx2, None));
    }

    #[test]
    fn test_entity_proven_tx_entity_name() {
        let entity = EntityProvenTx::new(None);
        assert_eq!(entity.entity_name(), "provenTx");
    }

    #[test]
    fn test_entity_proven_tx_entity_table() {
        let entity = EntityProvenTx::new(None);
        assert_eq!(entity.entity_table(), "proven_txs");
    }

    #[test]
    fn test_entity_proven_tx_id_methods() {
        let mut entity = EntityProvenTx::new(None);
        
        assert_eq!(entity.id(), 0);
        entity.set_id(999);
        assert_eq!(entity.id(), 999);
        assert_eq!(entity.proven_tx_id(), 999);
    }

    #[test]
    fn test_entity_proven_tx_clone() {
        let proven_tx = TableProvenTx {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            proven_tx_id: 1,
            txid: "abc123".to_string(),
            height: 700000,
            index: 5,
            merkle_path: vec![1, 2, 3],
            raw_tx: vec![4, 5, 6],
            block_hash: "block123".to_string(),
            merkle_root: "root123".to_string(),
        };

        let entity1 = EntityProvenTx::new(Some(proven_tx));
        let entity2 = entity1.clone();
        
        assert_eq!(entity1, entity2);
        assert_eq!(entity2.proven_tx_id(), 1);
        assert_eq!(entity2.txid(), "abc123");
    }

    #[test]
    fn test_entity_proven_tx_into_api() {
        let proven_tx = TableProvenTx {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            proven_tx_id: 1,
            txid: "abc123".to_string(),
            height: 700000,
            index: 5,
            merkle_path: vec![1, 2, 3],
            raw_tx: vec![4, 5, 6],
            block_hash: "block123".to_string(),
            merkle_root: "root123".to_string(),
        };

        let entity = EntityProvenTx::new(Some(proven_tx.clone()));
        let api = entity.into_api();
        
        assert_eq!(api.proven_tx_id, 1);
        assert_eq!(api.txid, "abc123");
        assert_eq!(api.height, 700000);
    }
}
