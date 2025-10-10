//! MergeEntity - Generic helper for entity synchronization
//!
//! Translates TypeScript MergeEntity<API, DE> class to Rust.
//! Reference: wallet-toolbox/src/storage/schema/entities/MergeEntity.ts

use super::{EntitySyncMap, SyncMap};
use crate::StorageError;
use std::collections::HashMap;

/// Generic merge coordinator for entity synchronization
///
/// Matches TypeScript `MergeEntity<API, DE>` class
/// 
/// Type parameters:
/// - `API`: One of the storage table interfaces (e.g., TableUser, TableTransaction)
/// - `DE`: The corresponding entity class (e.g., EntityUser, EntityTransaction)
pub struct MergeEntity<API, DE> {
    /// Array of external state records to merge
    pub state_array: Option<Vec<API>>,
    
    /// ID map for primary ID of API and DE objects
    pub esm: EntitySyncMap,
    
    /// Reference to the ID map (shortcut to esm.id_map)
    pub id_map: HashMap<i64, i64>,
    
    /// Find function to locate or create entity
    find_fn: Box<dyn Fn(&API, &SyncMap) -> Result<(bool, DE, i64), StorageError>>,
}

impl<API, DE> MergeEntity<API, DE> {
    /// Create a new MergeEntity coordinator
    ///
    /// # Arguments
    /// * `state_array` - Optional array of external state records
    /// * `esm` - Entity sync map for tracking ID mappings
    /// * `find_fn` - Function to find or create entity from external state
    pub fn new(
        state_array: Option<Vec<API>>,
        esm: EntitySyncMap,
        find_fn: Box<dyn Fn(&API, &SyncMap) -> Result<(bool, DE, i64), StorageError>>,
    ) -> Self {
        let id_map = esm.id_map.clone();
        Self {
            state_array,
            esm,
            id_map,
            find_fn,
        }
    }

    /// Update sync map with ID mapping
    ///
    /// Ensures idempotency - won't override existing mappings unless they match.
    /// Matches TypeScript `updateSyncMap` method.
    ///
    /// # Arguments
    /// * `map` - ID mapping to update
    /// * `in_id` - Foreign/external ID
    /// * `out_id` - Local ID
    ///
    /// # Errors
    /// Returns error if trying to override existing mapping with different value
    pub fn update_sync_map(
        map: &mut HashMap<i64, i64>,
        in_id: i64,
        out_id: i64,
    ) -> Result<(), StorageError> {
        if in_id <= 0 {
            return Err(StorageError::InvalidArg(
                format!("Invalid in_id: {}", in_id)
            ));
        }
        if out_id <= 0 {
            return Err(StorageError::InvalidArg(
                format!("Invalid out_id: {}", out_id)
            ));
        }

        match map.get(&in_id) {
            None => {
                map.insert(in_id, out_id);
                Ok(())
            }
            Some(&existing) if existing == out_id => Ok(()), // Already correctly mapped
            Some(&existing) => Err(StorageError::Database(
                format!(
                    "updateSyncMap map[{}] can't override {} with {}",
                    in_id, existing, out_id
                )
            )),
        }
    }

    /// Get the ID map reference
    pub fn id_map(&self) -> &HashMap<i64, i64> {
        &self.id_map
    }

    /// Get mutable reference to entity sync map
    pub fn esm_mut(&mut self) -> &mut EntitySyncMap {
        &mut self.esm
    }

    /// Get reference to entity sync map
    pub fn esm(&self) -> &EntitySyncMap {
        &self.esm
    }
}

/// Helper function to find maximum date
///
/// Matches TypeScript `maxDate` utility
pub fn max_date(a: Option<String>, b: String) -> Option<String> {
    match a {
        None => Some(b),
        Some(a_str) => {
            // Simple string comparison works for RFC3339 timestamps
            if b > a_str {
                Some(b)
            } else {
                Some(a_str)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_sync_map_new_mapping() {
        let mut map = HashMap::new();
        let result = MergeEntity::<(), ()>::update_sync_map(&mut map, 1, 100);
        assert!(result.is_ok());
        assert_eq!(map.get(&1), Some(&100));
    }

    #[test]
    fn test_update_sync_map_same_value() {
        let mut map = HashMap::new();
        map.insert(1, 100);
        
        let result = MergeEntity::<(), ()>::update_sync_map(&mut map, 1, 100);
        assert!(result.is_ok());
        assert_eq!(map.get(&1), Some(&100));
    }

    #[test]
    fn test_update_sync_map_different_value_error() {
        let mut map = HashMap::new();
        map.insert(1, 100);
        
        let result = MergeEntity::<(), ()>::update_sync_map(&mut map, 1, 200);
        assert!(result.is_err());
        match result {
            Err(StorageError::Database(msg)) => {
                assert!(msg.contains("can't override 100 with 200"));
            }
            _ => panic!("Expected Database error"),
        }
    }

    #[test]
    fn test_update_sync_map_invalid_in_id() {
        let mut map = HashMap::new();
        let result = MergeEntity::<(), ()>::update_sync_map(&mut map, 0, 100);
        assert!(result.is_err());
        match result {
            Err(StorageError::InvalidArg(msg)) => {
                assert!(msg.contains("Invalid in_id"));
            }
            _ => panic!("Expected InvalidArg error"),
        }
    }

    #[test]
    fn test_update_sync_map_invalid_out_id() {
        let mut map = HashMap::new();
        let result = MergeEntity::<(), ()>::update_sync_map(&mut map, 1, -1);
        assert!(result.is_err());
        match result {
            Err(StorageError::InvalidArg(msg)) => {
                assert!(msg.contains("Invalid out_id"));
            }
            _ => panic!("Expected InvalidArg error"),
        }
    }

    #[test]
    fn test_max_date_none() {
        let result = max_date(None, "2024-01-01T00:00:00Z".to_string());
        assert_eq!(result, Some("2024-01-01T00:00:00Z".to_string()));
    }

    #[test]
    fn test_max_date_newer() {
        let result = max_date(
            Some("2024-01-01T00:00:00Z".to_string()),
            "2024-12-31T23:59:59Z".to_string(),
        );
        assert_eq!(result, Some("2024-12-31T23:59:59Z".to_string()));
    }

    #[test]
    fn test_max_date_older() {
        let result = max_date(
            Some("2024-12-31T23:59:59Z".to_string()),
            "2024-01-01T00:00:00Z".to_string(),
        );
        assert_eq!(result, Some("2024-12-31T23:59:59Z".to_string()));
    }

    #[test]
    fn test_max_date_equal() {
        let date = "2024-06-15T12:00:00Z".to_string();
        let result = max_date(Some(date.clone()), date.clone());
        assert_eq!(result, Some(date));
    }
}
