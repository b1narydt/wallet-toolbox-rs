//! EntitySyncState - Sync state entity wrapper
//!
//! Translates TypeScript EntitySyncState class to Rust.
//! Reference: wallet-toolbox/src/storage/schema/entities/EntitySyncState.ts

use crate::schema::tables::{TableSyncState, SyncStatus};
use super::{EntityBase, SyncMap, SyncError};
use serde::{Deserialize, Serialize};

/// SyncState entity wrapper providing merge logic and property accessors
///
/// Matches TypeScript `EntitySyncState` class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntitySyncState {
    api: TableSyncState,
    error_local: Option<SyncError>,
    error_other: Option<SyncError>,
    sync_map: SyncMap,
}

impl EntitySyncState {
    /// Create new EntitySyncState from table record
    pub fn new(api: Option<TableSyncState>) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        let default_sync_map = SyncMap::default();
        let default_sync_map_json = serde_json::to_string(&default_sync_map).unwrap_or_else(|_| "{}".to_string());
        
        let api = api.unwrap_or_else(|| TableSyncState {
            created_at: now.clone(),
            updated_at: now,
            sync_state_id: 0,
            user_id: 0,
            storage_identity_key: String::new(),
            storage_name: String::new(),
            status: SyncStatus::Unknown,
            init: false,
            ref_num: String::new(),
            sync_map: default_sync_map_json,
            when: None,
            satoshis: None,
            error_local: None,
            error_other: None,
        });

        let error_local = api.error_local.as_ref()
            .and_then(|s| serde_json::from_str(s).ok());
        let error_other = api.error_other.as_ref()
            .and_then(|s| serde_json::from_str(s).ok());
        let sync_map = serde_json::from_str(&api.sync_map).unwrap_or_default();

        Self {
            api,
            error_local,
            error_other,
            sync_map,
        }
    }

    // Property accessors matching TypeScript getters/setters

    pub fn sync_state_id(&self) -> i64 {
        self.api.sync_state_id
    }

    pub fn set_sync_state_id(&mut self, v: i64) {
        self.api.sync_state_id = v;
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

    pub fn storage_identity_key(&self) -> &str {
        &self.api.storage_identity_key
    }

    pub fn set_storage_identity_key(&mut self, v: impl Into<String>) {
        self.api.storage_identity_key = v.into();
    }

    pub fn storage_name(&self) -> &str {
        &self.api.storage_name
    }

    pub fn set_storage_name(&mut self, v: impl Into<String>) {
        self.api.storage_name = v.into();
    }

    pub fn status(&self) -> SyncStatus {
        self.api.status
    }

    pub fn set_status(&mut self, v: SyncStatus) {
        self.api.status = v;
    }

    pub fn init(&self) -> bool {
        self.api.init
    }

    pub fn set_init(&mut self, v: bool) {
        self.api.init = v;
    }

    pub fn ref_num(&self) -> &str {
        &self.api.ref_num
    }

    pub fn set_ref_num(&mut self, v: impl Into<String>) {
        self.api.ref_num = v.into();
    }

    pub fn when(&self) -> Option<&str> {
        self.api.when.as_deref()
    }

    pub fn set_when(&mut self, v: Option<String>) {
        self.api.when = v;
    }

    pub fn satoshis(&self) -> Option<i64> {
        self.api.satoshis
    }

    pub fn set_satoshis(&mut self, v: Option<i64>) {
        self.api.satoshis = v;
    }

    /// Get reference to error_local
    pub fn error_local(&self) -> Option<&SyncError> {
        self.error_local.as_ref()
    }

    /// Get mutable reference to error_local
    pub fn error_local_mut(&mut self) -> &mut Option<SyncError> {
        &mut self.error_local
    }

    /// Get reference to error_other
    pub fn error_other(&self) -> Option<&SyncError> {
        self.error_other.as_ref()
    }

    /// Get mutable reference to error_other
    pub fn error_other_mut(&mut self) -> &mut Option<SyncError> {
        &mut self.error_other
    }

    /// Get reference to sync_map
    pub fn sync_map(&self) -> &SyncMap {
        &self.sync_map
    }

    /// Get mutable reference to sync_map
    pub fn sync_map_mut(&mut self) -> &mut SyncMap {
        &mut self.sync_map
    }

    /// Get mutable reference to underlying API
    pub fn get_api_mut(&mut self) -> &mut TableSyncState {
        &mut self.api
    }

    /// Consume entity and return API (packs JSON fields first)
    pub fn into_api(mut self) -> TableSyncState {
        self.pack_api();
        self.api
    }

    /// Pack JSON fields into API strings
    fn pack_api(&mut self) {
        self.api.error_local = self.error_local.as_ref()
            .and_then(|e| serde_json::to_string(e).ok());
        self.api.error_other = self.error_other.as_ref()
            .and_then(|e| serde_json::to_string(e).ok());
        self.api.sync_map = serde_json::to_string(&self.sync_map)
            .unwrap_or_else(|_| "{}".to_string());
    }
}

impl EntityBase for EntitySyncState {
    type Api = TableSyncState;

    fn id(&self) -> i64 {
        self.api.sync_state_id
    }

    fn set_id(&mut self, v: i64) {
        self.api.sync_state_id = v;
    }

    fn entity_name(&self) -> &'static str {
        "syncState"
    }

    fn entity_table(&self) -> &'static str {
        "sync_states"
    }

    fn update_api(&mut self) {
        // Pack JSON fields into API strings
        self.pack_api();
    }

    fn get_api(&self) -> &Self::Api {
        &self.api
    }

    fn equals(&self, _other: &Self::Api, _sync_map: Option<&SyncMap>) -> bool {
        // TypeScript implementation returns false - sync states are not compared for equality
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_sync_state_new_default() {
        let entity = EntitySyncState::new(None);
        assert_eq!(entity.sync_state_id(), 0);
        assert_eq!(entity.user_id(), 0);
        assert_eq!(entity.storage_identity_key(), "");
        assert_eq!(entity.storage_name(), "");
        assert_eq!(entity.status(), SyncStatus::Unknown);
        assert_eq!(entity.init(), false);
        assert_eq!(entity.ref_num(), "");
        assert_eq!(entity.when(), None);
        assert_eq!(entity.satoshis(), None);
        assert!(entity.error_local().is_none());
        assert!(entity.error_other().is_none());
    }

    #[test]
    fn test_entity_sync_state_new_with_api() {
        let state = TableSyncState {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            sync_state_id: 1,
            user_id: 100,
            storage_identity_key: "key123".to_string(),
            storage_name: "My Storage".to_string(),
            status: SyncStatus::Success,
            init: true,
            ref_num: "ref001".to_string(),
            sync_map: "{}".to_string(),
            when: Some("2024-01-01T00:00:00Z".to_string()),
            satoshis: Some(50000),
            error_local: None,
            error_other: None,
        };

        let entity = EntitySyncState::new(Some(state));
        assert_eq!(entity.sync_state_id(), 1);
        assert_eq!(entity.user_id(), 100);
        assert_eq!(entity.storage_identity_key(), "key123");
        assert_eq!(entity.storage_name(), "My Storage");
        assert_eq!(entity.status(), SyncStatus::Success);
        assert_eq!(entity.init(), true);
        assert_eq!(entity.when(), Some("2024-01-01T00:00:00Z"));
        assert_eq!(entity.satoshis(), Some(50000));
    }

    #[test]
    fn test_entity_sync_state_property_accessors() {
        let mut entity = EntitySyncState::new(None);

        entity.set_sync_state_id(42);
        assert_eq!(entity.sync_state_id(), 42);

        entity.set_user_id(100);
        assert_eq!(entity.user_id(), 100);

        entity.set_storage_identity_key("storage_key");
        assert_eq!(entity.storage_identity_key(), "storage_key");

        entity.set_storage_name("Storage Name");
        assert_eq!(entity.storage_name(), "Storage Name");

        entity.set_status(SyncStatus::Identified);
        assert_eq!(entity.status(), SyncStatus::Identified);

        entity.set_init(true);
        assert_eq!(entity.init(), true);

        entity.set_ref_num("ref123");
        assert_eq!(entity.ref_num(), "ref123");

        entity.set_when(Some("2024-01-01T00:00:00Z".to_string()));
        assert_eq!(entity.when(), Some("2024-01-01T00:00:00Z"));

        entity.set_satoshis(Some(10000));
        assert_eq!(entity.satoshis(), Some(10000));
    }

    #[test]
    fn test_entity_sync_state_error_handling() {
        let mut entity = EntitySyncState::new(None);

        // Set error_local
        *entity.error_local_mut() = Some(SyncError {
            code: "ERR_TEST".to_string(),
            description: "Test error".to_string(),
            stack: None,
        });
        
        assert!(entity.error_local().is_some());
        assert_eq!(entity.error_local().unwrap().code, "ERR_TEST");

        // Pack into API
        entity.update_api();
        assert!(entity.get_api().error_local.is_some());
    }

    #[test]
    fn test_entity_sync_state_equals_always_false() {
        let state = TableSyncState {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            sync_state_id: 1,
            user_id: 100,
            storage_identity_key: "key123".to_string(),
            storage_name: "My Storage".to_string(),
            status: SyncStatus::Success,
            init: true,
            ref_num: "ref001".to_string(),
            sync_map: "{}".to_string(),
            when: None,
            satoshis: None,
            error_local: None,
            error_other: None,
        };

        let entity = EntitySyncState::new(Some(state.clone()));
        // Per TypeScript, equals always returns false for sync states
        assert!(!entity.equals(&state, None));
    }

    #[test]
    fn test_entity_sync_state_entity_name() {
        let entity = EntitySyncState::new(None);
        assert_eq!(entity.entity_name(), "syncState");
    }

    #[test]
    fn test_entity_sync_state_entity_table() {
        let entity = EntitySyncState::new(None);
        assert_eq!(entity.entity_table(), "sync_states");
    }

    #[test]
    fn test_entity_sync_state_id_methods() {
        let mut entity = EntitySyncState::new(None);
        
        assert_eq!(entity.id(), 0);
        entity.set_id(999);
        assert_eq!(entity.id(), 999);
        assert_eq!(entity.sync_state_id(), 999);
    }
}
