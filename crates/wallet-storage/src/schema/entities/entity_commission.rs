//! EntityCommission - Commission entity wrapper
//!
//! Translates TypeScript EntityCommission class to Rust.
//! Reference: wallet-toolbox/src/storage/schema/entities/EntityCommission.ts

use crate::schema::tables::TableCommission;
use super::{EntityBase, SyncMap};

/// Commission entity wrapper providing merge logic and property accessors
///
/// Matches TypeScript `EntityCommission` class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityCommission {
    api: TableCommission,
}

impl EntityCommission {
    /// Create new EntityCommission from table record
    pub fn new(api: Option<TableCommission>) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            api: api.unwrap_or_else(|| TableCommission {
                created_at: now.clone(),
                updated_at: now,
                commission_id: 0,
                user_id: 0,
                transaction_id: 0,
                satoshis: 0,
                key_offset: String::new(),
                is_redeemed: false,
                locking_script: Vec::new(),
            }),
        }
    }

    // Property accessors matching TypeScript getters/setters

    pub fn commission_id(&self) -> i64 {
        self.api.commission_id
    }

    pub fn set_commission_id(&mut self, v: i64) {
        self.api.commission_id = v;
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

    pub fn transaction_id(&self) -> i64 {
        self.api.transaction_id
    }

    pub fn set_transaction_id(&mut self, v: i64) {
        self.api.transaction_id = v;
    }

    pub fn user_id(&self) -> i64 {
        self.api.user_id
    }

    pub fn set_user_id(&mut self, v: i64) {
        self.api.user_id = v;
    }

    pub fn is_redeemed(&self) -> bool {
        self.api.is_redeemed
    }

    pub fn set_is_redeemed(&mut self, v: bool) {
        self.api.is_redeemed = v;
    }

    pub fn key_offset(&self) -> &str {
        &self.api.key_offset
    }

    pub fn set_key_offset(&mut self, v: impl Into<String>) {
        self.api.key_offset = v.into();
    }

    pub fn locking_script(&self) -> &[u8] {
        &self.api.locking_script
    }

    pub fn set_locking_script(&mut self, v: Vec<u8>) {
        self.api.locking_script = v;
    }

    pub fn satoshis(&self) -> i64 {
        self.api.satoshis
    }

    pub fn set_satoshis(&mut self, v: i64) {
        self.api.satoshis = v;
    }

    /// Get mutable reference to underlying API
    pub fn get_api_mut(&mut self) -> &mut TableCommission {
        &mut self.api
    }

    /// Consume entity and return API
    pub fn into_api(self) -> TableCommission {
        self.api
    }

    /// Helper to compare byte arrays
    fn arrays_equal(a: &[u8], b: &[u8]) -> bool {
        a == b
    }
}

impl EntityBase for EntityCommission {
    type Api = TableCommission;

    fn id(&self) -> i64 {
        self.api.commission_id
    }

    fn set_id(&mut self, v: i64) {
        self.api.commission_id = v;
    }

    fn entity_name(&self) -> &'static str {
        "commission"
    }

    fn entity_table(&self) -> &'static str {
        "commissions"
    }

    fn update_api(&mut self) {
        // Nothing needed yet - matches TypeScript implementation
    }

    fn get_api(&self) -> &Self::Api {
        &self.api
    }

    fn equals(&self, other: &Self::Api, sync_map: Option<&SyncMap>) -> bool {
        // Match TypeScript equals logic exactly
        
        // Compare transactionId with optional sync map
        let other_transaction_id = if let Some(map) = sync_map {
            map.transaction.id_map.get(&other.transaction_id).copied()
                .unwrap_or(other.transaction_id)
        } else {
            other.transaction_id
        };
        
        if self.is_redeemed() != other.is_redeemed
            || self.transaction_id() != other_transaction_id
            || self.key_offset() != other.key_offset
            || !Self::arrays_equal(self.locking_script(), &other.locking_script)
            || self.satoshis() != other.satoshis
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
    fn test_entity_commission_new_default() {
        let entity = EntityCommission::new(None);
        assert_eq!(entity.commission_id(), 0);
        assert_eq!(entity.user_id(), 0);
        assert_eq!(entity.transaction_id(), 0);
        assert_eq!(entity.satoshis(), 0);
        assert_eq!(entity.key_offset(), "");
        assert_eq!(entity.is_redeemed(), false);
        assert_eq!(entity.locking_script(), &[] as &[u8]);
    }

    #[test]
    fn test_entity_commission_new_with_api() {
        let commission = TableCommission {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            commission_id: 1,
            user_id: 100,
            transaction_id: 200,
            satoshis: 5000,
            key_offset: "offset123".to_string(),
            is_redeemed: false,
            locking_script: vec![1, 2, 3],
        };

        let entity = EntityCommission::new(Some(commission));
        assert_eq!(entity.commission_id(), 1);
        assert_eq!(entity.user_id(), 100);
        assert_eq!(entity.transaction_id(), 200);
        assert_eq!(entity.satoshis(), 5000);
        assert_eq!(entity.key_offset(), "offset123");
        assert_eq!(entity.locking_script(), &[1, 2, 3]);
    }

    #[test]
    fn test_entity_commission_property_accessors() {
        let mut entity = EntityCommission::new(None);

        entity.set_commission_id(42);
        assert_eq!(entity.commission_id(), 42);

        entity.set_user_id(100);
        assert_eq!(entity.user_id(), 100);

        entity.set_transaction_id(200);
        assert_eq!(entity.transaction_id(), 200);

        entity.set_satoshis(10000);
        assert_eq!(entity.satoshis(), 10000);

        entity.set_key_offset("key123");
        assert_eq!(entity.key_offset(), "key123");

        entity.set_is_redeemed(true);
        assert_eq!(entity.is_redeemed(), true);

        entity.set_locking_script(vec![4, 5, 6]);
        assert_eq!(entity.locking_script(), &[4, 5, 6]);
    }

    #[test]
    fn test_entity_commission_equals_same() {
        let commission = TableCommission {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            commission_id: 1,
            user_id: 100,
            transaction_id: 200,
            satoshis: 5000,
            key_offset: "offset123".to_string(),
            is_redeemed: false,
            locking_script: vec![1, 2, 3],
        };

        let entity = EntityCommission::new(Some(commission.clone()));
        assert!(entity.equals(&commission, None));
    }

    #[test]
    fn test_entity_commission_equals_different_satoshis() {
        let commission1 = TableCommission {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            commission_id: 1,
            user_id: 100,
            transaction_id: 200,
            satoshis: 5000,
            key_offset: "offset123".to_string(),
            is_redeemed: false,
            locking_script: vec![1, 2, 3],
        };

        let mut commission2 = commission1.clone();
        commission2.satoshis = 10000;

        let entity = EntityCommission::new(Some(commission1));
        assert!(!entity.equals(&commission2, None));
    }

    #[test]
    fn test_entity_commission_equals_different_locking_script() {
        let commission1 = TableCommission {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            commission_id: 1,
            user_id: 100,
            transaction_id: 200,
            satoshis: 5000,
            key_offset: "offset123".to_string(),
            is_redeemed: false,
            locking_script: vec![1, 2, 3],
        };

        let mut commission2 = commission1.clone();
        commission2.locking_script = vec![4, 5, 6];

        let entity = EntityCommission::new(Some(commission1));
        assert!(!entity.equals(&commission2, None));
    }

    #[test]
    fn test_entity_commission_entity_name() {
        let entity = EntityCommission::new(None);
        assert_eq!(entity.entity_name(), "commission");
    }

    #[test]
    fn test_entity_commission_entity_table() {
        let entity = EntityCommission::new(None);
        assert_eq!(entity.entity_table(), "commissions");
    }

    #[test]
    fn test_entity_commission_id_methods() {
        let mut entity = EntityCommission::new(None);
        
        assert_eq!(entity.id(), 0);
        entity.set_id(999);
        assert_eq!(entity.id(), 999);
        assert_eq!(entity.commission_id(), 999);
    }
}
