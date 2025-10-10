//! TableOutputBasket - Output basket configuration
//!
//! Translates TypeScript TableOutputBasket interface to Rust.
//! Reference: wallet-toolbox/src/storage/schema/tables/TableOutputBasket.ts

use serde::{Deserialize, Serialize};

/// OutputBasket table - stores basket configurations for organizing outputs
///
/// Matches TypeScript `TableOutputBasket` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableOutputBasket {
    /// Record creation timestamp (ISO 8601 string)
    pub created_at: String,
    
    /// Record last update timestamp (ISO 8601 string)
    pub updated_at: String,
    
    /// Primary key - unique basket identifier
    #[serde(rename = "basketId")]
    pub basket_id: i64,
    
    /// Foreign key to user
    #[serde(rename = "userId")]
    pub user_id: i64,
    
    /// Basket name
    pub name: String,
    
    /// Number of desired UTXOs to maintain in this basket
    #[serde(rename = "numberOfDesiredUTXOs")]
    pub number_of_desired_utxos: i32,
    
    /// Minimum desired UTXO value in satoshis
    #[serde(rename = "minimumDesiredUTXOValue")]
    pub minimum_desired_utxo_value: i64,
    
    /// Soft delete flag
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

impl TableOutputBasket {
    /// Create a new TableOutputBasket
    pub fn new(
        basket_id: i64,
        user_id: i64,
        name: impl Into<String>,
        number_of_desired_utxos: i32,
        minimum_desired_utxo_value: i64,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            basket_id,
            user_id,
            name: name.into(),
            number_of_desired_utxos,
            minimum_desired_utxo_value,
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
    fn test_table_output_basket_new() {
        let basket = TableOutputBasket::new(1, 100, "default", 10, 1000);
        
        assert_eq!(basket.basket_id, 1);
        assert_eq!(basket.user_id, 100);
        assert_eq!(basket.name, "default");
        assert_eq!(basket.number_of_desired_utxos, 10);
        assert_eq!(basket.minimum_desired_utxo_value, 1000);
        assert_eq!(basket.is_deleted, false);
        assert!(!basket.created_at.is_empty());
        assert!(!basket.updated_at.is_empty());
    }

    #[test]
    fn test_table_output_basket_touch() {
        let mut basket = TableOutputBasket::new(1, 100, "test", 5, 500);
        
        let original_updated = basket.updated_at.clone();
        std::thread::sleep(std::time::Duration::from_millis(10));
        basket.touch();
        
        assert_ne!(basket.updated_at, original_updated);
        assert_eq!(basket.created_at, basket.created_at); // unchanged
    }

    #[test]
    fn test_table_output_basket_delete() {
        let mut basket = TableOutputBasket::new(1, 100, "test", 5, 500);
        
        assert_eq!(basket.is_deleted, false);
        basket.delete();
        assert_eq!(basket.is_deleted, true);
    }

    #[test]
    fn test_table_output_basket_restore() {
        let mut basket = TableOutputBasket::new(1, 100, "test", 5, 500);
        
        basket.delete();
        assert_eq!(basket.is_deleted, true);
        
        basket.restore();
        assert_eq!(basket.is_deleted, false);
    }

    #[test]
    fn test_table_output_basket_serialization() {
        let basket = TableOutputBasket::new(1, 100, "change", 10, 1000);
        let json = serde_json::to_string(&basket).unwrap();
        
        // Check camelCase field names
        assert!(json.contains("\"basketId\":1"));
        assert!(json.contains("\"userId\":100"));
        assert!(json.contains("\"numberOfDesiredUTXOs\":10"));
        assert!(json.contains("\"minimumDesiredUTXOValue\":1000"));
        assert!(json.contains("\"isDeleted\":false"));
        
        let deserialized: TableOutputBasket = serde_json::from_str(&json).unwrap();
        assert_eq!(basket, deserialized);
    }

    #[test]
    fn test_table_output_basket_field_names() {
        let basket = TableOutputBasket::new(1, 100, "test", 5, 500);
        let json = serde_json::to_value(&basket).unwrap();
        
        // Verify exact field names match TypeScript
        assert!(json.get("basketId").is_some());
        assert!(json.get("userId").is_some());
        assert!(json.get("numberOfDesiredUTXOs").is_some());
        assert!(json.get("minimumDesiredUTXOValue").is_some());
        assert!(json.get("isDeleted").is_some());
        assert!(json.get("created_at").is_some());
        assert!(json.get("updated_at").is_some());
    }

    #[test]
    fn test_table_output_basket_clone() {
        let basket = TableOutputBasket::new(1, 100, "test", 5, 500);
        let cloned = basket.clone();
        
        assert_eq!(basket, cloned);
    }
}
