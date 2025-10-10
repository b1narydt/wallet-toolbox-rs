//! TableUser - User identity and active storage
//!
//! Translates TypeScript TableUser interface to Rust.
//! Reference: wallet-toolbox/src/storage/schema/tables/TableUser.ts

use serde::{Deserialize, Serialize};

/// User table - stores user identity and active storage reference
///
/// Matches TypeScript `TableUser` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableUser {
    /// Record creation timestamp (ISO 8601 string)
    pub created_at: String,
    
    /// Record last update timestamp (ISO 8601 string)
    pub updated_at: String,
    
    /// Primary key - unique user identifier
    #[serde(rename = "userId")]
    pub user_id: i64,
    
    /// PubKeyHex uniquely identifying user.
    /// Typically 66 hex digits.
    #[serde(rename = "identityKey")]
    pub identity_key: String,
    
    /// The storageIdentityKey value of the active wallet storage.
    #[serde(rename = "activeStorage")]
    pub active_storage: String,
}

impl TableUser {
    /// Create a new TableUser
    pub fn new(
        user_id: i64,
        identity_key: impl Into<String>,
        active_storage: impl Into<String>,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            user_id,
            identity_key: identity_key.into(),
            active_storage: active_storage.into(),
        }
    }

    /// Create with explicit timestamps
    pub fn with_timestamps(
        user_id: i64,
        identity_key: impl Into<String>,
        active_storage: impl Into<String>,
        created_at: impl Into<String>,
        updated_at: impl Into<String>,
    ) -> Self {
        Self {
            created_at: created_at.into(),
            updated_at: updated_at.into(),
            user_id,
            identity_key: identity_key.into(),
            active_storage: active_storage.into(),
        }
    }

    /// Update the timestamp
    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_user_new() {
        let user = TableUser::new(1, "identity123", "storage456");
        
        assert_eq!(user.user_id, 1);
        assert_eq!(user.identity_key, "identity123");
        assert_eq!(user.active_storage, "storage456");
        assert!(!user.created_at.is_empty());
        assert!(!user.updated_at.is_empty());
        assert_eq!(user.created_at, user.updated_at);
    }

    #[test]
    fn test_table_user_with_timestamps() {
        let user = TableUser::with_timestamps(
            1,
            "identity123",
            "storage456",
            "2024-01-01T00:00:00Z",
            "2024-01-02T00:00:00Z",
        );
        
        assert_eq!(user.created_at, "2024-01-01T00:00:00Z");
        assert_eq!(user.updated_at, "2024-01-02T00:00:00Z");
    }

    #[test]
    fn test_table_user_touch() {
        let mut user = TableUser::with_timestamps(
            1,
            "identity123",
            "storage456",
            "2024-01-01T00:00:00Z",
            "2024-01-01T00:00:00Z",
        );
        
        let original_updated = user.updated_at.clone();
        std::thread::sleep(std::time::Duration::from_millis(10));
        user.touch();
        
        assert_ne!(user.updated_at, original_updated);
        // created_at should not change
        assert_eq!(user.created_at, "2024-01-01T00:00:00Z");
    }

    #[test]
    fn test_table_user_serialization() {
        let user = TableUser::new(1, "identity123", "storage456");
        let json = serde_json::to_string(&user).unwrap();
        
        // Should use camelCase for JSON compatibility with TypeScript
        assert!(json.contains("\"userId\":1"));
        assert!(json.contains("\"identityKey\":\"identity123\""));
        assert!(json.contains("\"activeStorage\":\"storage456\""));
        
        let deserialized: TableUser = serde_json::from_str(&json).unwrap();
        assert_eq!(user, deserialized);
    }

    #[test]
    fn test_table_user_field_names() {
        let user = TableUser::new(1, "identity", "storage");
        let json = serde_json::to_value(&user).unwrap();
        
        // Verify exact field names match TypeScript
        assert!(json.get("userId").is_some());
        assert!(json.get("identityKey").is_some());
        assert!(json.get("activeStorage").is_some());
        assert!(json.get("created_at").is_some());
        assert!(json.get("updated_at").is_some());
        
        // Should not have snake_case versions
        assert!(json.get("user_id").is_none());
        assert!(json.get("identity_key").is_none());
        assert!(json.get("active_storage").is_none());
    }

    #[test]
    fn test_table_user_equality() {
        let user1 = TableUser::with_timestamps(
            1,
            "identity",
            "storage",
            "2024-01-01T00:00:00Z",
            "2024-01-01T00:00:00Z",
        );
        let user2 = TableUser::with_timestamps(
            1,
            "identity",
            "storage",
            "2024-01-01T00:00:00Z",
            "2024-01-01T00:00:00Z",
        );
        let user3 = TableUser::with_timestamps(
            2,
            "identity",
            "storage",
            "2024-01-01T00:00:00Z",
            "2024-01-01T00:00:00Z",
        );
        
        assert_eq!(user1, user2);
        assert_ne!(user1, user3);
    }

    #[test]
    fn test_table_user_clone() {
        let user = TableUser::new(1, "identity", "storage");
        let cloned = user.clone();
        
        assert_eq!(user, cloned);
    }
}
