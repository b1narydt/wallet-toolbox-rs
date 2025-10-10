//! EntityUser - User entity wrapper
//!
//! Translates TypeScript EntityUser class to Rust.
//! Reference: wallet-toolbox/src/storage/schema/entities/EntityUser.ts

use crate::schema::tables::TableUser;
use super::{EntityBase, SyncMap};

/// User entity wrapper providing merge logic and property accessors
///
/// Matches TypeScript `EntityUser` class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityUser {
    api: TableUser,
}

impl EntityUser {
    /// Create new EntityUser from table record
    pub fn new(api: Option<TableUser>) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            api: api.unwrap_or_else(|| TableUser {
                created_at: now.clone(),
                updated_at: now,
                user_id: 0,
                identity_key: String::new(),
                active_storage: String::new(),
            }),
        }
    }

    /// Create with all fields specified
    pub fn with_fields(
        user_id: i64,
        identity_key: impl Into<String>,
        active_storage: impl Into<String>,
        created_at: impl Into<String>,
        updated_at: impl Into<String>,
    ) -> Self {
        Self {
            api: TableUser {
                created_at: created_at.into(),
                updated_at: updated_at.into(),
                user_id,
                identity_key: identity_key.into(),
                active_storage: active_storage.into(),
            },
        }
    }

    // Property accessors matching TypeScript getters/setters

    pub fn user_id(&self) -> i64 {
        self.api.user_id
    }

    pub fn set_user_id(&mut self, v: i64) {
        self.api.user_id = v;
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

    pub fn identity_key(&self) -> &str {
        &self.api.identity_key
    }

    pub fn set_identity_key(&mut self, v: impl Into<String>) {
        self.api.identity_key = v.into();
    }

    pub fn active_storage(&self) -> &str {
        &self.api.active_storage
    }

    pub fn set_active_storage(&mut self, v: impl Into<String>) {
        self.api.active_storage = v.into();
    }

    /// Get mutable reference to underlying API
    pub fn get_api_mut(&mut self) -> &mut TableUser {
        &mut self.api
    }

    /// Consume entity and return API
    pub fn into_api(self) -> TableUser {
        self.api
    }
}

impl EntityBase for EntityUser {
    type Api = TableUser;

    fn id(&self) -> i64 {
        self.api.user_id
    }

    fn set_id(&mut self, v: i64) {
        self.api.user_id = v;
    }

    fn entity_name(&self) -> &'static str {
        "user"
    }

    fn entity_table(&self) -> &'static str {
        "users"
    }

    fn update_api(&mut self) {
        // Nothing needed yet - matches TypeScript implementation
    }

    fn get_api(&self) -> &Self::Api {
        &self.api
    }

    fn equals(&self, other: &Self::Api, _sync_map: Option<&SyncMap>) -> bool {
        // Matches TypeScript equals logic
        let eo = &self.api;
        if eo.identity_key != other.identity_key || eo.active_storage != other.active_storage {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_user_new_with_api() {
        let user = TableUser {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            user_id: 1,
            identity_key: "key123".to_string(),
            active_storage: "storage456".to_string(),
        };
        
        let entity = EntityUser::new(Some(user.clone()));
        assert_eq!(entity.user_id(), 1);
        assert_eq!(entity.identity_key(), "key123");
        assert_eq!(entity.active_storage(), "storage456");
    }

    #[test]
    fn test_entity_user_new_default() {
        let entity = EntityUser::new(None);
        assert_eq!(entity.user_id(), 0);
        assert_eq!(entity.identity_key(), "");
        assert_eq!(entity.active_storage(), "");
    }

    #[test]
    fn test_entity_user_with_fields() {
        let entity = EntityUser::with_fields(
            1,
            "key123",
            "storage456",
            "2024-01-01T00:00:00Z",
            "2024-01-01T00:00:00Z",
        );
        
        assert_eq!(entity.user_id(), 1);
        assert_eq!(entity.identity_key(), "key123");
        assert_eq!(entity.active_storage(), "storage456");
    }

    #[test]
    fn test_entity_user_property_accessors() {
        let mut entity = EntityUser::new(None);
        
        entity.set_user_id(42);
        assert_eq!(entity.user_id(), 42);
        
        entity.set_identity_key("new_key");
        assert_eq!(entity.identity_key(), "new_key");
        
        entity.set_active_storage("new_storage");
        assert_eq!(entity.active_storage(), "new_storage");
        
        entity.set_updated_at("2024-12-31T23:59:59Z");
        assert_eq!(entity.updated_at(), "2024-12-31T23:59:59Z");
    }

    #[test]
    fn test_entity_user_id_methods() {
        let mut entity = EntityUser::new(None);
        
        assert_eq!(entity.id(), 0);
        entity.set_id(100);
        assert_eq!(entity.id(), 100);
        assert_eq!(entity.user_id(), 100);
    }

    #[test]
    fn test_entity_user_entity_name() {
        let entity = EntityUser::new(None);
        assert_eq!(entity.entity_name(), "user");
    }

    #[test]
    fn test_entity_user_entity_table() {
        let entity = EntityUser::new(None);
        assert_eq!(entity.entity_table(), "users");
    }

    #[test]
    fn test_entity_user_equals_same() {
        let entity1 = EntityUser::with_fields(
            1, "key", "storage",
            "2024-01-01T00:00:00Z", "2024-01-01T00:00:00Z",
        );
        
        let api2 = TableUser {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            user_id: 1,
            identity_key: "key".to_string(),
            active_storage: "storage".to_string(),
        };
        
        assert!(entity1.equals(&api2, None));
    }

    #[test]
    fn test_entity_user_equals_different_identity_key() {
        let entity1 = EntityUser::with_fields(
            1, "key1", "storage",
            "2024-01-01T00:00:00Z", "2024-01-01T00:00:00Z",
        );
        
        let api2 = TableUser {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            user_id: 1,
            identity_key: "key2".to_string(),
            active_storage: "storage".to_string(),
        };
        
        assert!(!entity1.equals(&api2, None));
    }

    #[test]
    fn test_entity_user_equals_different_active_storage() {
        let entity1 = EntityUser::with_fields(
            1, "key", "storage1",
            "2024-01-01T00:00:00Z", "2024-01-01T00:00:00Z",
        );
        
        let api2 = TableUser {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            user_id: 1,
            identity_key: "key".to_string(),
            active_storage: "storage2".to_string(),
        };
        
        assert!(!entity1.equals(&api2, None));
    }

    #[test]
    fn test_entity_user_to_api() {
        let mut entity = EntityUser::with_fields(
            1, "key", "storage",
            "2024-01-01T00:00:00Z", "2024-01-01T00:00:00Z",
        );
        
        let api = entity.to_api();
        assert_eq!(api.user_id, 1);
        assert_eq!(api.identity_key, "key");
        assert_eq!(api.active_storage, "storage");
    }

    #[test]
    fn test_entity_user_into_api() {
        let entity = EntityUser::with_fields(
            1, "key", "storage",
            "2024-01-01T00:00:00Z", "2024-01-01T00:00:00Z",
        );
        
        let api = entity.into_api();
        assert_eq!(api.user_id, 1);
        assert_eq!(api.identity_key, "key");
        assert_eq!(api.active_storage, "storage");
    }

    #[test]
    fn test_entity_user_clone() {
        let entity1 = EntityUser::with_fields(
            1, "key", "storage",
            "2024-01-01T00:00:00Z", "2024-01-01T00:00:00Z",
        );
        
        let entity2 = entity1.clone();
        assert_eq!(entity1, entity2);
        assert_eq!(entity2.user_id(), 1);
    }
}
