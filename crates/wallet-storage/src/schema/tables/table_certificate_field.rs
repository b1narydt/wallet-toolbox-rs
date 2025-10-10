//! TableCertificateField - Certificate field records
//!
//! Translates TypeScript TableCertificateField interface to Rust.
//! Reference: wallet-toolbox/src/storage/schema/tables/TableCertificateField.ts

use serde::{Deserialize, Serialize};

/// CertificateField table - stores certificate field data
///
/// Matches TypeScript `TableCertificateField` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableCertificateField {
    pub created_at: String,
    pub updated_at: String,
    
    #[serde(rename = "userId")]
    pub user_id: i64,
    
    #[serde(rename = "certificateId")]
    pub certificate_id: i64,
    
    #[serde(rename = "fieldName")]
    pub field_name: String,
    
    #[serde(rename = "fieldValue")]
    pub field_value: String,
    
    /// Base64 string
    #[serde(rename = "masterKey")]
    pub master_key: String,
}

impl TableCertificateField {
    pub fn new(
        user_id: i64,
        certificate_id: i64,
        field_name: impl Into<String>,
        field_value: impl Into<String>,
        master_key: impl Into<String>,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            user_id,
            certificate_id,
            field_name: field_name.into(),
            field_value: field_value.into(),
            master_key: master_key.into(),
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_certificate_field_new() {
        let field = TableCertificateField::new(
            100, 1, "email", "user@example.com", "master_key_base64"
        );
        assert_eq!(field.user_id, 100);
        assert_eq!(field.certificate_id, 1);
        assert_eq!(field.field_name, "email");
        assert_eq!(field.field_value, "user@example.com");
        assert_eq!(field.master_key, "master_key_base64");
    }

    #[test]
    fn test_table_certificate_field_serialization() {
        let field = TableCertificateField::new(
            100, 1, "name", "value", "key"
        );
        let json = serde_json::to_string(&field).unwrap();
        assert!(json.contains("\"userId\":100"));
        assert!(json.contains("\"certificateId\":1"));
        assert!(json.contains("\"fieldName\":\"name\""));
        assert!(json.contains("\"fieldValue\":\"value\""));
        let deserialized: TableCertificateField = serde_json::from_str(&json).unwrap();
        assert_eq!(field, deserialized);
    }
}
