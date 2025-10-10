//! EntityOutputTag - Output tag entity wrapper
//!
//! Translates TypeScript EntityOutputTag class to Rust.
//! Reference: wallet-toolbox/src/storage/schema/entities/EntityOutputTag.ts

use crate::schema::tables::TableOutputTag;
use super::{EntityBase, SyncMap};

/// OutputTag entity wrapper providing merge logic and property accessors
///
/// Matches TypeScript `EntityOutputTag` class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityOutputTag {
    api: TableOutputTag,
}

impl EntityOutputTag {
    /// Create new EntityOutputTag from table record
    pub fn new(api: Option<TableOutputTag>) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            api: api.unwrap_or_else(|| TableOutputTag {
                created_at: now.clone(),
                updated_at: now,
                output_tag_id: 0,
                user_id: 0,
                tag: String::new(),
                is_deleted: false,
            }),
        }
    }

    // Property accessors matching TypeScript getters/setters

    pub fn output_tag_id(&self) -> i64 {
        self.api.output_tag_id
    }

    pub fn set_output_tag_id(&mut self, v: i64) {
        self.api.output_tag_id = v;
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

    pub fn tag(&self) -> &str {
        &self.api.tag
    }

    pub fn set_tag(&mut self, v: impl Into<String>) {
        self.api.tag = v.into();
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
    pub fn get_api_mut(&mut self) -> &mut TableOutputTag {
        &mut self.api
    }

    /// Consume entity and return API
    pub fn into_api(self) -> TableOutputTag {
        self.api
    }
}

impl EntityBase for EntityOutputTag {
    type Api = TableOutputTag;

    fn id(&self) -> i64 {
        self.api.output_tag_id
    }

    fn set_id(&mut self, v: i64) {
        self.api.output_tag_id = v;
    }

    fn entity_name(&self) -> &'static str {
        "outputTag"
    }

    fn entity_table(&self) -> &'static str {
        "output_tags"
    }

    fn update_api(&mut self) {
        // Nothing needed yet - matches TypeScript implementation
    }

    fn get_api(&self) -> &Self::Api {
        &self.api
    }

    fn equals(&self, other: &Self::Api, sync_map: Option<&SyncMap>) -> bool {
        // Match TypeScript equals logic exactly
        
        // Compare tag and isDeleted
        if self.tag() != other.tag || self.is_deleted() != other.is_deleted {
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
    fn test_entity_output_tag_new_default() {
        let entity = EntityOutputTag::new(None);
        assert_eq!(entity.output_tag_id(), 0);
        assert_eq!(entity.user_id(), 0);
        assert_eq!(entity.tag(), "");
        assert_eq!(entity.is_deleted(), false);
    }

    #[test]
    fn test_entity_output_tag_new_with_api() {
        let tag = TableOutputTag {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            output_tag_id: 1,
            user_id: 100,
            tag: "important".to_string(),
            is_deleted: false,
        };

        let entity = EntityOutputTag::new(Some(tag));
        assert_eq!(entity.output_tag_id(), 1);
        assert_eq!(entity.user_id(), 100);
        assert_eq!(entity.tag(), "important");
        assert_eq!(entity.is_deleted(), false);
    }

    #[test]
    fn test_entity_output_tag_property_accessors() {
        let mut entity = EntityOutputTag::new(None);

        entity.set_output_tag_id(42);
        assert_eq!(entity.output_tag_id(), 42);

        entity.set_user_id(100);
        assert_eq!(entity.user_id(), 100);

        entity.set_tag("archived");
        assert_eq!(entity.tag(), "archived");

        entity.set_is_deleted(true);
        assert_eq!(entity.is_deleted(), true);

        entity.set_created_at("2024-01-01T00:00:00Z");
        assert_eq!(entity.created_at(), "2024-01-01T00:00:00Z");

        entity.set_updated_at("2024-01-02T00:00:00Z");
        assert_eq!(entity.updated_at(), "2024-01-02T00:00:00Z");
    }

    #[test]
    fn test_entity_output_tag_equals_same() {
        let tag = TableOutputTag {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            output_tag_id: 1,
            user_id: 100,
            tag: "important".to_string(),
            is_deleted: false,
        };

        let entity = EntityOutputTag::new(Some(tag.clone()));
        assert!(entity.equals(&tag, None));
    }

    #[test]
    fn test_entity_output_tag_equals_different_tag() {
        let tag1 = TableOutputTag {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            output_tag_id: 1,
            user_id: 100,
            tag: "important".to_string(),
            is_deleted: false,
        };

        let mut tag2 = tag1.clone();
        tag2.tag = "archived".to_string();

        let entity = EntityOutputTag::new(Some(tag1));
        assert!(!entity.equals(&tag2, None));
    }

    #[test]
    fn test_entity_output_tag_equals_different_is_deleted() {
        let tag1 = TableOutputTag {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            output_tag_id: 1,
            user_id: 100,
            tag: "important".to_string(),
            is_deleted: false,
        };

        let mut tag2 = tag1.clone();
        tag2.is_deleted = true;

        let entity = EntityOutputTag::new(Some(tag1));
        assert!(!entity.equals(&tag2, None));
    }

    #[test]
    fn test_entity_output_tag_equals_without_sync_map_checks_user_id() {
        let tag1 = TableOutputTag {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            output_tag_id: 1,
            user_id: 100,
            tag: "important".to_string(),
            is_deleted: false,
        };

        let mut tag2 = tag1.clone();
        tag2.user_id = 200; // Different userId

        let entity = EntityOutputTag::new(Some(tag1));
        // Without sync_map, userId is compared
        assert!(!entity.equals(&tag2, None));
    }

    #[test]
    fn test_entity_output_tag_entity_name() {
        let entity = EntityOutputTag::new(None);
        assert_eq!(entity.entity_name(), "outputTag");
    }

    #[test]
    fn test_entity_output_tag_entity_table() {
        let entity = EntityOutputTag::new(None);
        assert_eq!(entity.entity_table(), "output_tags");
    }

    #[test]
    fn test_entity_output_tag_id_methods() {
        let mut entity = EntityOutputTag::new(None);
        
        assert_eq!(entity.id(), 0);
        entity.set_id(999);
        assert_eq!(entity.id(), 999);
        assert_eq!(entity.output_tag_id(), 999);
    }

    #[test]
    fn test_entity_output_tag_clone() {
        let tag = TableOutputTag {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            output_tag_id: 1,
            user_id: 100,
            tag: "important".to_string(),
            is_deleted: false,
        };

        let entity1 = EntityOutputTag::new(Some(tag));
        let entity2 = entity1.clone();
        
        assert_eq!(entity1, entity2);
        assert_eq!(entity2.output_tag_id(), 1);
        assert_eq!(entity2.tag(), "important");
    }
}
