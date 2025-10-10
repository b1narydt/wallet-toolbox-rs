//! EntityOutputBasket - Output basket entity wrapper
//!
//! Translates TypeScript EntityOutputBasket class to Rust.
//! Reference: wallet-toolbox/src/storage/schema/entities/EntityOutputBasket.ts

use crate::schema::tables::TableOutputBasket;
use super::{EntityBase, SyncMap};

/// OutputBasket entity wrapper providing merge logic and property accessors
///
/// Matches TypeScript `EntityOutputBasket` class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityOutputBasket {
    api: TableOutputBasket,
}

impl EntityOutputBasket {
    /// Create new EntityOutputBasket from table record
    pub fn new(api: Option<TableOutputBasket>) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            api: api.unwrap_or_else(|| TableOutputBasket {
                created_at: now.clone(),
                updated_at: now,
                basket_id: 0,
                user_id: 0,
                name: String::new(),
                number_of_desired_utxos: 0,
                minimum_desired_utxo_value: 0,
                is_deleted: false,
            }),
        }
    }

    // Property accessors matching TypeScript getters/setters

    pub fn basket_id(&self) -> i64 {
        self.api.basket_id
    }

    pub fn set_basket_id(&mut self, v: i64) {
        self.api.basket_id = v;
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

    pub fn user_id(&self) -> i64 {
        self.api.user_id
    }

    pub fn set_user_id(&mut self, v: i64) {
        self.api.user_id = v;
    }

    pub fn name(&self) -> &str {
        &self.api.name
    }

    pub fn set_name(&mut self, v: impl Into<String>) {
        self.api.name = v.into();
    }

    pub fn number_of_desired_utxos(&self) -> i32 {
        self.api.number_of_desired_utxos
    }

    pub fn set_number_of_desired_utxos(&mut self, v: i32) {
        self.api.number_of_desired_utxos = v;
    }

    pub fn minimum_desired_utxo_value(&self) -> i64 {
        self.api.minimum_desired_utxo_value
    }

    pub fn set_minimum_desired_utxo_value(&mut self, v: i64) {
        self.api.minimum_desired_utxo_value = v;
    }

    pub fn is_deleted(&self) -> bool {
        self.api.is_deleted
    }

    pub fn set_is_deleted(&mut self, v: bool) {
        self.api.is_deleted = v;
    }

    /// Get mutable reference to underlying API
    pub fn get_api_mut(&mut self) -> &mut TableOutputBasket {
        &mut self.api
    }

    /// Consume entity and return API
    pub fn into_api(self) -> TableOutputBasket {
        self.api
    }
}

impl EntityBase for EntityOutputBasket {
    type Api = TableOutputBasket;

    fn id(&self) -> i64 {
        self.api.basket_id
    }

    fn set_id(&mut self, v: i64) {
        self.api.basket_id = v;
    }

    fn entity_name(&self) -> &'static str {
        "outputBasket"
    }

    fn entity_table(&self) -> &'static str {
        "output_baskets"
    }

    fn update_api(&mut self) {
        // Nothing needed yet - matches TypeScript implementation
    }

    fn get_api(&self) -> &Self::Api {
        &self.api
    }

    fn equals(&self, other: &Self::Api, sync_map: Option<&SyncMap>) -> bool {
        // Match TypeScript equals logic exactly
        
        // Compare basic fields
        if self.name() != other.name
            || self.number_of_desired_utxos() != other.number_of_desired_utxos
            || self.minimum_desired_utxo_value() != other.minimum_desired_utxo_value
        {
            return false;
        }

        if let Some(map) = sync_map {
            // With sync map - only compare mapped basketId
            let other_basket_id = map.output_basket.id_map.get(&other.basket_id).copied()
                .unwrap_or(other.basket_id);
            if self.basket_id() != other_basket_id {
                return false;
            }
        } else {
            // Without sync map - compare both basketId and userId
            if self.basket_id() != other.basket_id || self.user_id() != other.user_id {
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
    fn test_entity_output_basket_new_default() {
        let entity = EntityOutputBasket::new(None);
        assert_eq!(entity.basket_id(), 0);
        assert_eq!(entity.user_id(), 0);
        assert_eq!(entity.name(), "");
        assert_eq!(entity.number_of_desired_utxos(), 0);
        assert_eq!(entity.minimum_desired_utxo_value(), 0);
        assert_eq!(entity.is_deleted(), false);
    }

    #[test]
    fn test_entity_output_basket_new_with_api() {
        let basket = TableOutputBasket {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            basket_id: 1,
            user_id: 100,
            name: "default".to_string(),
            number_of_desired_utxos: 10,
            minimum_desired_utxo_value: 1000,
            is_deleted: false,
        };

        let entity = EntityOutputBasket::new(Some(basket));
        assert_eq!(entity.basket_id(), 1);
        assert_eq!(entity.user_id(), 100);
        assert_eq!(entity.name(), "default");
        assert_eq!(entity.number_of_desired_utxos(), 10);
        assert_eq!(entity.minimum_desired_utxo_value(), 1000);
        assert_eq!(entity.is_deleted(), false);
    }

    #[test]
    fn test_entity_output_basket_property_accessors() {
        let mut entity = EntityOutputBasket::new(None);

        entity.set_basket_id(42);
        assert_eq!(entity.basket_id(), 42);

        entity.set_user_id(100);
        assert_eq!(entity.user_id(), 100);

        entity.set_name("savings");
        assert_eq!(entity.name(), "savings");

        entity.set_number_of_desired_utxos(20);
        assert_eq!(entity.number_of_desired_utxos(), 20);

        entity.set_minimum_desired_utxo_value(5000);
        assert_eq!(entity.minimum_desired_utxo_value(), 5000);

        entity.set_is_deleted(true);
        assert_eq!(entity.is_deleted(), true);
    }

    #[test]
    fn test_entity_output_basket_equals_same() {
        let basket = TableOutputBasket {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            basket_id: 1,
            user_id: 100,
            name: "default".to_string(),
            number_of_desired_utxos: 10,
            minimum_desired_utxo_value: 1000,
            is_deleted: false,
        };

        let entity = EntityOutputBasket::new(Some(basket.clone()));
        assert!(entity.equals(&basket, None));
    }

    #[test]
    fn test_entity_output_basket_equals_different_name() {
        let basket1 = TableOutputBasket {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            basket_id: 1,
            user_id: 100,
            name: "default".to_string(),
            number_of_desired_utxos: 10,
            minimum_desired_utxo_value: 1000,
            is_deleted: false,
        };

        let mut basket2 = basket1.clone();
        basket2.name = "savings".to_string();

        let entity = EntityOutputBasket::new(Some(basket1));
        assert!(!entity.equals(&basket2, None));
    }

    #[test]
    fn test_entity_output_basket_equals_different_utxo_count() {
        let basket1 = TableOutputBasket {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            basket_id: 1,
            user_id: 100,
            name: "default".to_string(),
            number_of_desired_utxos: 10,
            minimum_desired_utxo_value: 1000,
            is_deleted: false,
        };

        let mut basket2 = basket1.clone();
        basket2.number_of_desired_utxos = 20;

        let entity = EntityOutputBasket::new(Some(basket1));
        assert!(!entity.equals(&basket2, None));
    }

    #[test]
    fn test_entity_output_basket_equals_without_sync_map() {
        let basket1 = TableOutputBasket {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            basket_id: 1,
            user_id: 100,
            name: "default".to_string(),
            number_of_desired_utxos: 10,
            minimum_desired_utxo_value: 1000,
            is_deleted: false,
        };

        let mut basket2 = basket1.clone();
        basket2.user_id = 200; // Different userId

        let entity = EntityOutputBasket::new(Some(basket1));
        // Without sync_map, userId is compared
        assert!(!entity.equals(&basket2, None));
    }

    #[test]
    fn test_entity_output_basket_entity_name() {
        let entity = EntityOutputBasket::new(None);
        assert_eq!(entity.entity_name(), "outputBasket");
    }

    #[test]
    fn test_entity_output_basket_entity_table() {
        let entity = EntityOutputBasket::new(None);
        assert_eq!(entity.entity_table(), "output_baskets");
    }

    #[test]
    fn test_entity_output_basket_id_methods() {
        let mut entity = EntityOutputBasket::new(None);
        
        assert_eq!(entity.id(), 0);
        entity.set_id(999);
        assert_eq!(entity.id(), 999);
        assert_eq!(entity.basket_id(), 999);
    }

    #[test]
    fn test_entity_output_basket_clone() {
        let basket = TableOutputBasket {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            basket_id: 1,
            user_id: 100,
            name: "default".to_string(),
            number_of_desired_utxos: 10,
            minimum_desired_utxo_value: 1000,
            is_deleted: false,
        };

        let entity1 = EntityOutputBasket::new(Some(basket));
        let entity2 = entity1.clone();
        
        assert_eq!(entity1, entity2);
        assert_eq!(entity2.basket_id(), 1);
        assert_eq!(entity2.name(), "default");
    }
}
