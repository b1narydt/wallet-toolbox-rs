//! EntityCertificateField - Certificate field entity wrapper
//!
//! Translates TypeScript EntityCertificateField class to Rust.
//! Reference: wallet-toolbox/src/storage/schema/entities/EntityCertificateField.ts

use crate::schema::tables::TableCertificateField;
use super::{EntityBase, SyncMap};

/// CertificateField entity wrapper providing merge logic and property accessors
///
/// Matches TypeScript `EntityCertificateField` class
/// Note: This entity has no primary ID - uses composite key (certificateId, fieldName)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityCertificateField {
    api: TableCertificateField,
}

impl EntityCertificateField {
    /// Create new EntityCertificateField from table record
    pub fn new(api: Option<TableCertificateField>) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            api: api.unwrap_or_else(|| TableCertificateField {
                created_at: now.clone(),
                updated_at: now,
                user_id: 0,
                certificate_id: 0,
                field_name: String::new(),
                field_value: String::new(),
                master_key: String::new(),
            }),
        }
    }

    // Property accessors matching TypeScript getters/setters

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

    pub fn certificate_id(&self) -> i64 {
        self.api.certificate_id
    }

    pub fn set_certificate_id(&mut self, v: i64) {
        self.api.certificate_id = v;
    }

    pub fn field_name(&self) -> &str {
        &self.api.field_name
    }

    pub fn set_field_name(&mut self, v: impl Into<String>) {
        self.api.field_name = v.into();
    }

    pub fn field_value(&self) -> &str {
        &self.api.field_value
    }

    pub fn set_field_value(&mut self, v: impl Into<String>) {
        self.api.field_value = v.into();
    }

    pub fn master_key(&self) -> &str {
        &self.api.master_key
    }

    pub fn set_master_key(&mut self, v: impl Into<String>) {
        self.api.master_key = v.into();
    }

    /// Get mutable reference to underlying API
    pub fn get_api_mut(&mut self) -> &mut TableCertificateField {
        &mut self.api
    }

    /// Consume entity and return API
    pub fn into_api(self) -> TableCertificateField {
        self.api
    }
}

impl EntityBase for EntityCertificateField {
    type Api = TableCertificateField;

    fn id(&self) -> i64 {
        // This entity has no ID - matches TypeScript which throws error
        panic!("EntityCertificateField has no 'id' value")
    }

    fn set_id(&mut self, _v: i64) {
        // This entity has no ID - matches TypeScript which throws error
        panic!("EntityCertificateField has no 'id' value")
    }

    fn entity_name(&self) -> &'static str {
        "certificateField"
    }

    fn entity_table(&self) -> &'static str {
        "certificate_fields"
    }

    fn update_api(&mut self) {
        // Nothing needed yet - matches TypeScript implementation
    }

    fn get_api(&self) -> &Self::Api {
        &self.api
    }

    fn equals(&self, other: &Self::Api, sync_map: Option<&SyncMap>) -> bool {
        // Match TypeScript equals logic exactly
        
        // Compare certificateId with optional sync map
        let other_certificate_id = if let Some(map) = sync_map {
            map.certificate.id_map.get(&other.certificate_id).copied()
                .unwrap_or(other.certificate_id)
        } else {
            other.certificate_id
        };
        
        if self.certificate_id() != other_certificate_id
            || self.field_name() != other.field_name
            || self.field_value() != other.field_value
            || self.master_key() != other.master_key
        {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_certificate_field_new_default() {
        let entity = EntityCertificateField::new(None);
        assert_eq!(entity.user_id(), 0);
        assert_eq!(entity.certificate_id(), 0);
        assert_eq!(entity.field_name(), "");
        assert_eq!(entity.field_value(), "");
        assert_eq!(entity.master_key(), "");
    }

    #[test]
    fn test_entity_certificate_field_new_with_api() {
        let field = TableCertificateField {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            user_id: 100,
            certificate_id: 1,
            field_name: "email".to_string(),
            field_value: "user@example.com".to_string(),
            master_key: "master_key_base64".to_string(),
        };

        let entity = EntityCertificateField::new(Some(field));
        assert_eq!(entity.user_id(), 100);
        assert_eq!(entity.certificate_id(), 1);
        assert_eq!(entity.field_name(), "email");
        assert_eq!(entity.field_value(), "user@example.com");
        assert_eq!(entity.master_key(), "master_key_base64");
    }

    #[test]
    fn test_entity_certificate_field_property_accessors() {
        let mut entity = EntityCertificateField::new(None);

        entity.set_user_id(100);
        assert_eq!(entity.user_id(), 100);

        entity.set_certificate_id(1);
        assert_eq!(entity.certificate_id(), 1);

        entity.set_field_name("name");
        assert_eq!(entity.field_name(), "name");

        entity.set_field_value("value");
        assert_eq!(entity.field_value(), "value");

        entity.set_master_key("key");
        assert_eq!(entity.master_key(), "key");

        entity.set_created_at("2024-01-01T00:00:00Z");
        assert_eq!(entity.created_at(), "2024-01-01T00:00:00Z");

        entity.set_updated_at("2024-01-02T00:00:00Z");
        assert_eq!(entity.updated_at(), "2024-01-02T00:00:00Z");
    }

    #[test]
    fn test_entity_certificate_field_equals_same() {
        let field = TableCertificateField {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            user_id: 100,
            certificate_id: 1,
            field_name: "email".to_string(),
            field_value: "user@example.com".to_string(),
            master_key: "master_key_base64".to_string(),
        };

        let entity = EntityCertificateField::new(Some(field.clone()));
        assert!(entity.equals(&field, None));
    }

    #[test]
    fn test_entity_certificate_field_equals_different_field_value() {
        let field1 = TableCertificateField {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            user_id: 100,
            certificate_id: 1,
            field_name: "email".to_string(),
            field_value: "user@example.com".to_string(),
            master_key: "master_key_base64".to_string(),
        };

        let mut field2 = field1.clone();
        field2.field_value = "different@example.com".to_string();

        let entity = EntityCertificateField::new(Some(field1));
        assert!(!entity.equals(&field2, None));
    }

    #[test]
    fn test_entity_certificate_field_equals_different_field_name() {
        let field1 = TableCertificateField {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            user_id: 100,
            certificate_id: 1,
            field_name: "email".to_string(),
            field_value: "user@example.com".to_string(),
            master_key: "master_key_base64".to_string(),
        };

        let mut field2 = field1.clone();
        field2.field_name = "phone".to_string();

        let entity = EntityCertificateField::new(Some(field1));
        assert!(!entity.equals(&field2, None));
    }

    #[test]
    #[should_panic(expected = "has no 'id' value")]
    fn test_entity_certificate_field_id_panics() {
        let entity = EntityCertificateField::new(None);
        // This should panic as per TypeScript behavior
        let _ = entity.id();
    }

    #[test]
    #[should_panic(expected = "has no 'id' value")]
    fn test_entity_certificate_field_set_id_panics() {
        let mut entity = EntityCertificateField::new(None);
        // This should panic as per TypeScript behavior
        entity.set_id(1);
    }

    #[test]
    fn test_entity_certificate_field_entity_name() {
        let entity = EntityCertificateField::new(None);
        assert_eq!(entity.entity_name(), "certificateField");
    }

    #[test]
    fn test_entity_certificate_field_entity_table() {
        let entity = EntityCertificateField::new(None);
        assert_eq!(entity.entity_table(), "certificate_fields");
    }

    #[test]
    fn test_entity_certificate_field_clone() {
        let field = TableCertificateField {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            user_id: 100,
            certificate_id: 1,
            field_name: "email".to_string(),
            field_value: "user@example.com".to_string(),
            master_key: "master_key_base64".to_string(),
        };

        let entity1 = EntityCertificateField::new(Some(field));
        let entity2 = entity1.clone();
        
        assert_eq!(entity1, entity2);
        assert_eq!(entity2.field_name(), "email");
    }
}
