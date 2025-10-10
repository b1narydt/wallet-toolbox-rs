//! TableOutputTagMap - Output to tag mapping
//!
//! Translates TypeScript TableOutputTagMap interface to Rust.
//! Reference: wallet-toolbox/src/storage/schema/tables/TableOutputTagMap.ts

use serde::{Deserialize, Serialize};

/// OutputTagMap table - maps outputs to tags (many-to-many)
///
/// Matches TypeScript `TableOutputTagMap` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableOutputTagMap {
    pub created_at: String,
    pub updated_at: String,
    
    #[serde(rename = "outputTagId")]
    pub output_tag_id: i64,
    
    #[serde(rename = "outputId")]
    pub output_id: i64,
    
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

impl TableOutputTagMap {
    pub fn new(output_tag_id: i64, output_id: i64) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            output_tag_id,
            output_id,
            is_deleted: false,
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    pub fn delete(&mut self) {
        self.is_deleted = true;
        self.touch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_output_tag_map_new() {
        let map = TableOutputTagMap::new(1, 300);
        assert_eq!(map.output_tag_id, 1);
        assert_eq!(map.output_id, 300);
        assert_eq!(map.is_deleted, false);
    }

    #[test]
    fn test_table_output_tag_map_serialization() {
        let map = TableOutputTagMap::new(10, 1000);
        let json = serde_json::to_string(&map).unwrap();
        assert!(json.contains("\"outputTagId\":10"));
        assert!(json.contains("\"outputId\":1000"));
        let deserialized: TableOutputTagMap = serde_json::from_str(&json).unwrap();
        assert_eq!(map, deserialized);
    }
}
