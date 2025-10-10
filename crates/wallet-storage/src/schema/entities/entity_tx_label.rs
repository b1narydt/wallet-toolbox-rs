//! EntityTxLabel - Transaction label entity wrapper
//!
//! Translates TypeScript EntityTxLabel class to Rust.
//! Reference: wallet-toolbox/src/storage/schema/entities/EntityTxLabel.ts

use crate::schema::tables::TableTxLabel;
use super::{EntityBase, SyncMap};

/// TxLabel entity wrapper providing merge logic and property accessors
///
/// Matches TypeScript `EntityTxLabel` class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityTxLabel {
    api: TableTxLabel,
}

impl EntityTxLabel {
    /// Create new EntityTxLabel from table record
    pub fn new(api: Option<TableTxLabel>) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            api: api.unwrap_or_else(|| TableTxLabel {
                created_at: now.clone(),
                updated_at: now,
                tx_label_id: 0,
                user_id: 0,
                label: String::new(),
                is_deleted: false,
            }),
        }
    }

    // Property accessors matching TypeScript getters/setters

    pub fn tx_label_id(&self) -> i64 {
        self.api.tx_label_id
    }

    pub fn set_tx_label_id(&mut self, v: i64) {
        self.api.tx_label_id = v;
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

    pub fn label(&self) -> &str {
        &self.api.label
    }

    pub fn set_label(&mut self, v: impl Into<String>) {
        self.api.label = v.into();
    }

    pub fn user_id(&self) -> i64 {
        self.api.user_id
    }

    pub fn set_user_id(&mut self, v: i64) {
        self.api.user_id = v;
    }

    pub fn is_deleted(&self) -> bool {
        self.api.is_deleted
    }

    pub fn set_is_deleted(&mut self, v: bool) {
        self.api.is_deleted = v;
    }

    /// Get mutable reference to underlying API
    pub fn get_api_mut(&mut self) -> &mut TableTxLabel {
        &mut self.api
    }

    /// Consume entity and return API
    pub fn into_api(self) -> TableTxLabel {
        self.api
    }
}

impl EntityBase for EntityTxLabel {
    type Api = TableTxLabel;

    fn id(&self) -> i64 {
        self.api.tx_label_id
    }

    fn set_id(&mut self, v: i64) {
        self.api.tx_label_id = v;
    }

    fn entity_name(&self) -> &'static str {
        "txLabel"
    }

    fn entity_table(&self) -> &'static str {
        "tx_labels"
    }

    fn update_api(&mut self) {
        // Nothing needed yet - matches TypeScript implementation
    }

    fn get_api(&self) -> &Self::Api {
        &self.api
    }

    fn equals(&self, other: &Self::Api, sync_map: Option<&SyncMap>) -> bool {
        // Match TypeScript equals logic exactly
        
        // Compare label and isDeleted
        if self.label() != other.label || self.is_deleted() != other.is_deleted {
            return false;
        }

        // Without sync_map, also compare userId
        if sync_map.is_none() {
            if self.user_id() != other.user_id {
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
    fn test_entity_tx_label_new_default() {
        let entity = EntityTxLabel::new(None);
        assert_eq!(entity.tx_label_id(), 0);
        assert_eq!(entity.user_id(), 0);
        assert_eq!(entity.label(), "");
        assert_eq!(entity.is_deleted(), false);
    }

    #[test]
    fn test_entity_tx_label_new_with_api() {
        let label = TableTxLabel {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            tx_label_id: 1,
            user_id: 100,
            label: "invoice".to_string(),
            is_deleted: false,
        };

        let entity = EntityTxLabel::new(Some(label));
        assert_eq!(entity.tx_label_id(), 1);
        assert_eq!(entity.user_id(), 100);
        assert_eq!(entity.label(), "invoice");
        assert_eq!(entity.is_deleted(), false);
    }

    #[test]
    fn test_entity_tx_label_property_accessors() {
        let mut entity = EntityTxLabel::new(None);

        entity.set_tx_label_id(42);
        assert_eq!(entity.tx_label_id(), 42);

        entity.set_user_id(100);
        assert_eq!(entity.user_id(), 100);

        entity.set_label("payment");
        assert_eq!(entity.label(), "payment");

        entity.set_is_deleted(true);
        assert_eq!(entity.is_deleted(), true);
    }

    #[test]
    fn test_entity_tx_label_equals_same() {
        let label = TableTxLabel {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            tx_label_id: 1,
            user_id: 100,
            label: "invoice".to_string(),
            is_deleted: false,
        };

        let entity = EntityTxLabel::new(Some(label.clone()));
        assert!(entity.equals(&label, None));
    }

    #[test]
    fn test_entity_tx_label_equals_different_label() {
        let label1 = TableTxLabel {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            tx_label_id: 1,
            user_id: 100,
            label: "invoice".to_string(),
            is_deleted: false,
        };

        let mut label2 = label1.clone();
        label2.label = "payment".to_string();

        let entity = EntityTxLabel::new(Some(label1));
        assert!(!entity.equals(&label2, None));
    }

    #[test]
    fn test_entity_tx_label_entity_name() {
        let entity = EntityTxLabel::new(None);
        assert_eq!(entity.entity_name(), "txLabel");
    }

    #[test]
    fn test_entity_tx_label_entity_table() {
        let entity = EntityTxLabel::new(None);
        assert_eq!(entity.entity_table(), "tx_labels");
    }

    #[test]
    fn test_entity_tx_label_id_methods() {
        let mut entity = EntityTxLabel::new(None);
        
        assert_eq!(entity.id(), 0);
        entity.set_id(999);
        assert_eq!(entity.id(), 999);
        assert_eq!(entity.tx_label_id(), 999);
    }
}
