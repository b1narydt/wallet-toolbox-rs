//! TableOutputTag - Output tag definitions
//!
//! Translates TypeScript TableOutputTag interface to Rust.
//! Reference: wallet-toolbox/src/storage/schema/tables/TableOutputTag.ts

use serde::{Deserialize, Serialize};

/// OutputTag table - stores tag definitions for categorizing outputs
///
/// Matches TypeScript `TableOutputTag` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableOutputTag {
    /// Record creation timestamp (ISO 8601 string)
    pub created_at: String,
    
    /// Record last update timestamp (ISO 8601 string)
    pub updated_at: String,
    
    /// Primary key - unique output tag identifier
    #[serde(rename = "outputTagId")]
    pub output_tag_id: i64,
    
    /// Foreign key to user
    #[serde(rename = "userId")]
    pub user_id: i64,
    
    /// Tag name/value
    pub tag: String,
    
    /// Soft delete flag
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

impl TableOutputTag {
    /// Create a new TableOutputTag
    pub fn new(
        output_tag_id: i64,
        user_id: i64,
        tag: impl Into<String>,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            output_tag_id,
            user_id,
            tag: tag.into(),
            is_deleted: false,
        }
    }

    /// Update the timestamp
    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Mark as deleted (soft delete)
    pub fn delete(&mut self) {
        self.is_deleted = true;
        self.touch();
    }

    /// Restore from deleted state
    pub fn restore(&mut self) {
        self.is_deleted = false;
        self.touch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_output_tag_new() {
        let tag = TableOutputTag::new(1, 100, "important");
        
        assert_eq!(tag.output_tag_id, 1);
        assert_eq!(tag.user_id, 100);
        assert_eq!(tag.tag, "important");
        assert_eq!(tag.is_deleted, false);
        assert!(!tag.created_at.is_empty());
        assert!(!tag.updated_at.is_empty());
    }

    #[test]
    fn test_table_output_tag_touch() {
        let mut tag = TableOutputTag::new(1, 100, "test");
        
        let original_updated = tag.updated_at.clone();
        std::thread::sleep(std::time::Duration::from_millis(10));
        tag.touch();
        
        assert_ne!(tag.updated_at, original_updated);
        assert_eq!(tag.created_at, tag.created_at); // unchanged
    }

    #[test]
    fn test_table_output_tag_delete() {
        let mut tag = TableOutputTag::new(1, 100, "test");
        
        assert_eq!(tag.is_deleted, false);
        tag.delete();
        assert_eq!(tag.is_deleted, true);
    }

    #[test]
    fn test_table_output_tag_restore() {
        let mut tag = TableOutputTag::new(1, 100, "test");
        
        tag.delete();
        assert_eq!(tag.is_deleted, true);
        
        tag.restore();
        assert_eq!(tag.is_deleted, false);
    }

    #[test]
    fn test_table_output_tag_serialization() {
        let tag = TableOutputTag::new(1, 100, "archived");
        let json = serde_json::to_string(&tag).unwrap();
        
        // Check camelCase field names
        assert!(json.contains("\"outputTagId\":1"));
        assert!(json.contains("\"userId\":100"));
        assert!(json.contains("\"tag\":\"archived\""));
        assert!(json.contains("\"isDeleted\":false"));
        
        let deserialized: TableOutputTag = serde_json::from_str(&json).unwrap();
        assert_eq!(tag, deserialized);
    }

    #[test]
    fn test_table_output_tag_field_names() {
        let tag = TableOutputTag::new(1, 100, "test");
        let json = serde_json::to_value(&tag).unwrap();
        
        // Verify exact field names match TypeScript
        assert!(json.get("outputTagId").is_some());
        assert!(json.get("userId").is_some());
        assert!(json.get("tag").is_some());
        assert!(json.get("isDeleted").is_some());
        assert!(json.get("created_at").is_some());
        assert!(json.get("updated_at").is_some());
    }

    #[test]
    fn test_table_output_tag_clone() {
        let tag = TableOutputTag::new(1, 100, "test");
        let cloned = tag.clone();
        
        assert_eq!(tag, cloned);
    }
}
