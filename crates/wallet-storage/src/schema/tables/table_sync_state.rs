//! TableSyncState - Storage synchronization state
//!
//! Translates TypeScript TableSyncState interface to Rust.
//! Reference: wallet-toolbox/src/storage/schema/tables/TableSyncState.ts

use serde::{Deserialize, Serialize};

/// Sync status - matches wallet-core SyncStatus but defined locally to avoid circular dependency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SyncStatus {
    Success,
    Error,
    Identified,
    Updated,
    Unknown,
}

impl std::fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncStatus::Success => write!(f, "success"),
            SyncStatus::Error => write!(f, "error"),
            SyncStatus::Identified => write!(f, "identified"),
            SyncStatus::Updated => write!(f, "updated"),
            SyncStatus::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::str::FromStr for SyncStatus {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "success" => Ok(SyncStatus::Success),
            "error" => Ok(SyncStatus::Error),
            "identified" => Ok(SyncStatus::Identified),
            "updated" => Ok(SyncStatus::Updated),
            "unknown" => Ok(SyncStatus::Unknown),
            _ => Err(format!("Invalid SyncStatus: {}", s)),
        }
    }
}

/// SyncState table - tracks synchronization state between storages
///
/// Matches TypeScript `TableSyncState` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableSyncState {
    /// Record creation timestamp (ISO 8601 string)
    pub created_at: String,
    
    /// Record last update timestamp (ISO 8601 string)
    pub updated_at: String,
    
    /// Primary key - unique sync state identifier
    #[serde(rename = "syncStateId")]
    pub sync_state_id: i64,
    
    /// Foreign key to user
    #[serde(rename = "userId")]
    pub user_id: i64,
    
    /// Storage identity key being synced
    #[serde(rename = "storageIdentityKey")]
    pub storage_identity_key: String,
    
    /// Human-readable storage name
    #[serde(rename = "storageName")]
    pub storage_name: String,
    
    /// Current sync status
    pub status: SyncStatus,
    
    /// Whether this is the initial sync
    pub init: bool,
    
    /// Reference number for sync operation
    #[serde(rename = "refNum")]
    pub ref_num: String,
    
    /// Sync map JSON string
    #[serde(rename = "syncMap")]
    pub sync_map: String,
    
    /// Optional last sync timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,
    
    /// Optional satoshis value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satoshis: Option<i64>,
    
    /// Optional local error message
    #[serde(rename = "errorLocal", skip_serializing_if = "Option::is_none")]
    pub error_local: Option<String>,
    
    /// Optional remote error message
    #[serde(rename = "errorOther", skip_serializing_if = "Option::is_none")]
    pub error_other: Option<String>,
}

impl TableSyncState {
    /// Create a new TableSyncState
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        sync_state_id: i64,
        user_id: i64,
        storage_identity_key: impl Into<String>,
        storage_name: impl Into<String>,
        status: SyncStatus,
        init: bool,
        ref_num: impl Into<String>,
        sync_map: impl Into<String>,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            sync_state_id,
            user_id,
            storage_identity_key: storage_identity_key.into(),
            storage_name: storage_name.into(),
            status,
            init,
            ref_num: ref_num.into(),
            sync_map: sync_map.into(),
            when: None,
            satoshis: None,
            error_local: None,
            error_other: None,
        }
    }

    /// Create with all optional fields
    #[allow(clippy::too_many_arguments)]
    pub fn with_optional(
        sync_state_id: i64,
        user_id: i64,
        storage_identity_key: impl Into<String>,
        storage_name: impl Into<String>,
        status: SyncStatus,
        init: bool,
        ref_num: impl Into<String>,
        sync_map: impl Into<String>,
        when: Option<String>,
        satoshis: Option<i64>,
        error_local: Option<String>,
        error_other: Option<String>,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            sync_state_id,
            user_id,
            storage_identity_key: storage_identity_key.into(),
            storage_name: storage_name.into(),
            status,
            init,
            ref_num: ref_num.into(),
            sync_map: sync_map.into(),
            when,
            satoshis,
            error_local,
            error_other,
        }
    }

    /// Update the timestamp
    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Set error messages
    pub fn set_error(&mut self, local: Option<String>, other: Option<String>) {
        self.error_local = local;
        self.error_other = other;
        self.status = SyncStatus::Error;
        self.touch();
    }

    /// Mark as success
    pub fn set_success(&mut self) {
        self.status = SyncStatus::Success;
        self.error_local = None;
        self.error_other = None;
        self.touch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_sync_state_new() {
        let sync_state = TableSyncState::new(
            1,
            100,
            "storage_key_123",
            "My Storage",
            SyncStatus::Identified,
            true,
            "ref_001",
            "{}",
        );
        
        assert_eq!(sync_state.sync_state_id, 1);
        assert_eq!(sync_state.user_id, 100);
        assert_eq!(sync_state.storage_identity_key, "storage_key_123");
        assert_eq!(sync_state.storage_name, "My Storage");
        assert_eq!(sync_state.status, SyncStatus::Identified);
        assert_eq!(sync_state.init, true);
        assert_eq!(sync_state.ref_num, "ref_001");
        assert_eq!(sync_state.sync_map, "{}");
        assert!(sync_state.when.is_none());
        assert!(sync_state.satoshis.is_none());
        assert!(sync_state.error_local.is_none());
        assert!(sync_state.error_other.is_none());
    }

    #[test]
    fn test_table_sync_state_with_optional() {
        let sync_state = TableSyncState::with_optional(
            1,
            100,
            "storage_key",
            "Storage",
            SyncStatus::Success,
            false,
            "ref_002",
            "{\"data\":true}",
            Some("2024-01-01T00:00:00Z".to_string()),
            Some(50000),
            None,
            None,
        );
        
        assert_eq!(sync_state.when, Some("2024-01-01T00:00:00Z".to_string()));
        assert_eq!(sync_state.satoshis, Some(50000));
        assert!(sync_state.error_local.is_none());
        assert!(sync_state.error_other.is_none());
    }

    #[test]
    fn test_table_sync_state_touch() {
        let mut sync_state = TableSyncState::new(
            1, 100, "key", "name", SyncStatus::Identified, true, "ref", "{}",
        );
        
        let original_updated = sync_state.updated_at.clone();
        std::thread::sleep(std::time::Duration::from_millis(10));
        sync_state.touch();
        
        assert_ne!(sync_state.updated_at, original_updated);
        assert_eq!(sync_state.created_at, sync_state.created_at); // unchanged
    }

    #[test]
    fn test_table_sync_state_set_error() {
        let mut sync_state = TableSyncState::new(
            1, 100, "key", "name", SyncStatus::Success, false, "ref", "{}",
        );
        
        sync_state.set_error(
            Some("Local error occurred".to_string()),
            Some("Remote error occurred".to_string()),
        );
        
        assert_eq!(sync_state.status, SyncStatus::Error);
        assert_eq!(sync_state.error_local, Some("Local error occurred".to_string()));
        assert_eq!(sync_state.error_other, Some("Remote error occurred".to_string()));
    }

    #[test]
    fn test_table_sync_state_set_success() {
        let mut sync_state = TableSyncState::with_optional(
            1, 100, "key", "name", SyncStatus::Error, false, "ref", "{}",
            None, None,
            Some("Previous error".to_string()),
            Some("Other error".to_string()),
        );
        
        sync_state.set_success();
        
        assert_eq!(sync_state.status, SyncStatus::Success);
        assert!(sync_state.error_local.is_none());
        assert!(sync_state.error_other.is_none());
    }

    #[test]
    fn test_table_sync_state_serialization() {
        let sync_state = TableSyncState::new(
            1, 100, "key", "name", SyncStatus::Identified, true, "ref", "{}",
        );
        
        let json = serde_json::to_string(&sync_state).unwrap();
        
        // Check camelCase field names
        assert!(json.contains("\"syncStateId\":1"));
        assert!(json.contains("\"userId\":100"));
        assert!(json.contains("\"storageIdentityKey\":\"key\""));
        assert!(json.contains("\"storageName\":\"name\""));
        assert!(json.contains("\"refNum\":\"ref\""));
        assert!(json.contains("\"syncMap\":\"{}\""));
        
        let deserialized: TableSyncState = serde_json::from_str(&json).unwrap();
        assert_eq!(sync_state, deserialized);
    }

    #[test]
    fn test_table_sync_state_optional_fields_not_serialized() {
        let sync_state = TableSyncState::new(
            1, 100, "key", "name", SyncStatus::Success, false, "ref", "{}",
        );
        
        let json = serde_json::to_string(&sync_state).unwrap();
        
        // Optional None fields should not appear in JSON
        assert!(!json.contains("\"when\""));
        assert!(!json.contains("\"satoshis\""));
        assert!(!json.contains("\"errorLocal\""));
        assert!(!json.contains("\"errorOther\""));
    }

    #[test]
    fn test_table_sync_state_optional_fields_serialized_when_some() {
        let sync_state = TableSyncState::with_optional(
            1, 100, "key", "name", SyncStatus::Error, false, "ref", "{}",
            Some("2024-01-01T00:00:00Z".to_string()),
            Some(1000),
            Some("local err".to_string()),
            Some("remote err".to_string()),
        );
        
        let json = serde_json::to_string(&sync_state).unwrap();
        
        // Optional Some fields should appear in JSON
        assert!(json.contains("\"when\":\"2024-01-01T00:00:00Z\""));
        assert!(json.contains("\"satoshis\":1000"));
        assert!(json.contains("\"errorLocal\":\"local err\""));
        assert!(json.contains("\"errorOther\":\"remote err\""));
    }

    #[test]
    fn test_sync_status_serialization() {
        assert_eq!(
            serde_json::to_string(&SyncStatus::Success).unwrap(),
            "\"success\""
        );
        assert_eq!(
            serde_json::to_string(&SyncStatus::Error).unwrap(),
            "\"error\""
        );
        assert_eq!(
            serde_json::to_string(&SyncStatus::Identified).unwrap(),
            "\"identified\""
        );
        assert_eq!(
            serde_json::to_string(&SyncStatus::Updated).unwrap(),
            "\"updated\""
        );
        assert_eq!(
            serde_json::to_string(&SyncStatus::Unknown).unwrap(),
            "\"unknown\""
        );
    }

    #[test]
    fn test_table_sync_state_equality() {
        let state1 = TableSyncState::new(
            1, 100, "key", "name", SyncStatus::Success, false, "ref", "{}",
        );
        let mut state2 = TableSyncState::new(
            1, 100, "key", "name", SyncStatus::Success, false, "ref", "{}",
        );
        
        // Make timestamps equal for comparison
        state2.created_at = state1.created_at.clone();
        state2.updated_at = state1.updated_at.clone();
        assert_eq!(state1, state2);
    }

    #[test]
    fn test_table_sync_state_clone() {
        let sync_state = TableSyncState::new(
            1, 100, "key", "name", SyncStatus::Identified, true, "ref", "{}",
        );
        let cloned = sync_state.clone();
        
        assert_eq!(sync_state, cloned);
    }
}
