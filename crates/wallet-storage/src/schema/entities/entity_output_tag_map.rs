//! EntityOutputTagMap - Many-to-many mapping entity for outputs and tags
//!
//! Translates TypeScript EntityOutputTagMap class to Rust.
//! Reference: wallet-toolbox/src/storage/schema/entities/EntityOutputTagMap.ts

use crate::schema::tables::TableOutputTagMap;
use super::{EntityBase, SyncMap};

/// OutputTagMap entity wrapper providing merge logic and property accessors
///
/// This entity uses a composite key (outputId, outputTagId) and has no primary id.
/// Matches TypeScript `EntityOutputTagMap` class
#[derive(Debug, Clone)]
pub struct EntityOutputTagMap {
    api: TableOutputTagMap,
}

impl EntityOutputTagMap {
    /// Create new EntityOutputTagMap from table data
    pub fn new(api: TableOutputTagMap) -> Self {
        Self { api }
    }

    /// Create new EntityOutputTagMap with default values
    pub fn new_default() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            api: TableOutputTagMap {
                created_at: now.clone(),
                updated_at: now,
                output_id: 0,
                output_tag_id: 0,
                is_deleted: false,
            },
        }
    }

    // Property accessors

    pub fn output_tag_id(&self) -> i64 {
        self.api.output_tag_id
    }

    pub fn set_output_tag_id(&mut self, v: i64) {
        self.api.output_tag_id = v;
    }

    pub fn output_id(&self) -> i64 {
        self.api.output_id
    }

    pub fn set_output_id(&mut self, v: i64) {
        self.api.output_id = v;
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

impl EntityBase for EntityOutputTagMap {
    type Api = TableOutputTagMap;

    fn id(&self) -> i64 {
        // Match TypeScript: "entity has no 'id' value"
        panic!("entity has no 'id' value");
    }

    fn set_id(&mut self, _id: i64) {
        // Match TypeScript: entity has no id setter
        panic!("entity has no 'id' value");
    }

    fn entity_name(&self) -> &'static str {
        "outputTagMap"
    }

    fn entity_table(&self) -> &'static str {
        "output_tags_map"
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
            let mapped_output_id = map.output.id_map.get(&other.output_id).copied().unwrap_or(other.output_id);
            let mapped_tag_id = map.output_tag.id_map.get(&other.output_tag_id).copied().unwrap_or(other.output_tag_id);
            
            if eo.output_id != mapped_output_id || eo.output_tag_id != mapped_tag_id {
                return false;
            }
        } else {
            // Without sync map: direct comparison
            if eo.output_id != other.output_id || eo.output_tag_id != other.output_tag_id {
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
    fn test_entity_output_tag_map_new_default() {
        let entity = EntityOutputTagMap::new_default();
        assert_eq!(entity.output_id(), 0);
        assert_eq!(entity.output_tag_id(), 0);
        assert!(!entity.is_deleted());
    }

    #[test]
    fn test_entity_output_tag_map_property_accessors() {
        let mut entity = EntityOutputTagMap::new_default();
        
        entity.set_output_id(100);
        entity.set_output_tag_id(200);
        entity.set_is_deleted(true);
        
        assert_eq!(entity.output_id(), 100);
        assert_eq!(entity.output_tag_id(), 200);
        assert!(entity.is_deleted());
    }

    #[test]
    #[should_panic(expected = "entity has no 'id' value")]
    fn test_entity_output_tag_map_id_panics() {
        let entity = EntityOutputTagMap::new_default();
        let _ = entity.id(); // Should panic
    }

    #[test]
    #[should_panic(expected = "entity has no 'id' value")]
    fn test_entity_output_tag_map_set_id_panics() {
        let mut entity = EntityOutputTagMap::new_default();
        entity.set_id(1); // Should panic
    }

    #[test]
    fn test_entity_output_tag_map_entity_name() {
        let entity = EntityOutputTagMap::new_default();
        assert_eq!(entity.entity_name(), "outputTagMap");
    }

    #[test]
    fn test_entity_output_tag_map_entity_table() {
        let entity = EntityOutputTagMap::new_default();
        assert_eq!(entity.entity_table(), "output_tags_map");
    }

    #[test]
    fn test_entity_output_tag_map_equals_without_sync_map() {
        let entity = EntityOutputTagMap::new_default();
        let mut other = TableOutputTagMap {
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            output_id: 0,
            output_tag_id: 0,
            is_deleted: false,
        };
        
        assert!(entity.equals(&other, None));
        
        other.output_id = 1;
        assert!(!entity.equals(&other, None));
        
        other.output_id = 0;
        other.is_deleted = true;
        assert!(!entity.equals(&other, None));
    }

    #[test]
    fn test_entity_output_tag_map_equals_with_sync_map() {
        let mut entity = EntityOutputTagMap::new_default();
        entity.set_output_id(100);
        entity.set_output_tag_id(200);
        
        let mut sync_map = SyncMap::new();
        sync_map.output.id_map.insert(10, 100);
        sync_map.output_tag.id_map.insert(20, 200);
        
        let other = TableOutputTagMap {
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            output_id: 10,
            output_tag_id: 20,
            is_deleted: false,
        };
        
        // Should match because 10 maps to 100 and 20 maps to 200
        assert!(entity.equals(&other, Some(&sync_map)));
    }

    #[test]
    fn test_entity_output_tag_map_touch() {
        let mut entity = EntityOutputTagMap::new_default();
        let old_time = entity.updated_at().to_string();
        
        std::thread::sleep(std::time::Duration::from_millis(10));
        entity.touch();
        
        assert_ne!(entity.updated_at(), old_time);
    }

    #[test]
    fn test_entity_output_tag_map_to_api() {
        let mut entity = EntityOutputTagMap::new_default();
        entity.set_output_id(42);
        
        let api = entity.to_api();
        assert_eq!(api.output_id, 42);
    }
}
