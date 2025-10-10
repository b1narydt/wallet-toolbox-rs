//! EntityCertificate - Certificate entity wrapper
//!
//! Translates TypeScript EntityCertificate class to Rust.
//! Reference: wallet-toolbox/src/storage/schema/entities/EntityCertificate.ts

use crate::schema::tables::TableCertificate;
use super::{EntityBase, SyncMap};

/// Certificate entity wrapper providing merge logic and property accessors
///
/// Matches TypeScript `EntityCertificate` class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityCertificate {
    api: TableCertificate,
}

impl EntityCertificate {
    /// Create new EntityCertificate from table record
    pub fn new(api: Option<TableCertificate>) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            api: api.unwrap_or_else(|| TableCertificate {
                created_at: now.clone(),
                updated_at: now,
                certificate_id: 0,
                user_id: 0,
                certificate_type: String::new(),
                serial_number: String::new(),
                certifier: String::new(),
                subject: String::new(),
                verifier: None,
                revocation_outpoint: String::new(),
                signature: String::new(),
                is_deleted: false,
            }),
        }
    }

    // Property accessors matching TypeScript getters/setters

    pub fn certificate_id(&self) -> i64 {
        self.api.certificate_id
    }

    pub fn set_certificate_id(&mut self, v: i64) {
        self.api.certificate_id = v;
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

    pub fn certificate_type(&self) -> &str {
        &self.api.certificate_type
    }

    pub fn set_certificate_type(&mut self, v: impl Into<String>) {
        self.api.certificate_type = v.into();
    }

    pub fn subject(&self) -> &str {
        &self.api.subject
    }

    pub fn set_subject(&mut self, v: impl Into<String>) {
        self.api.subject = v.into();
    }

    pub fn verifier(&self) -> Option<&str> {
        self.api.verifier.as_deref()
    }

    pub fn set_verifier(&mut self, v: Option<String>) {
        self.api.verifier = v;
    }

    pub fn serial_number(&self) -> &str {
        &self.api.serial_number
    }

    pub fn set_serial_number(&mut self, v: impl Into<String>) {
        self.api.serial_number = v.into();
    }

    pub fn certifier(&self) -> &str {
        &self.api.certifier
    }

    pub fn set_certifier(&mut self, v: impl Into<String>) {
        self.api.certifier = v.into();
    }

    pub fn revocation_outpoint(&self) -> &str {
        &self.api.revocation_outpoint
    }

    pub fn set_revocation_outpoint(&mut self, v: impl Into<String>) {
        self.api.revocation_outpoint = v.into();
    }

    pub fn signature(&self) -> &str {
        &self.api.signature
    }

    pub fn set_signature(&mut self, v: impl Into<String>) {
        self.api.signature = v.into();
    }

    pub fn is_deleted(&self) -> bool {
        self.api.is_deleted
    }

    pub fn set_is_deleted(&mut self, v: bool) {
        self.api.is_deleted = v;
    }

    /// Get mutable reference to underlying API
    pub fn get_api_mut(&mut self) -> &mut TableCertificate {
        &mut self.api
    }

    /// Consume entity and return API
    pub fn into_api(self) -> TableCertificate {
        self.api
    }
}

impl EntityBase for EntityCertificate {
    type Api = TableCertificate;

    fn id(&self) -> i64 {
        self.api.certificate_id
    }

    fn set_id(&mut self, v: i64) {
        self.api.certificate_id = v;
    }

    fn entity_name(&self) -> &'static str {
        "certificate"
    }

    fn entity_table(&self) -> &'static str {
        "certificates"
    }

    fn update_api(&mut self) {
        // Nothing needed yet - matches TypeScript implementation
    }

    fn get_api(&self) -> &Self::Api {
        &self.api
    }

    fn equals(&self, other: &Self::Api, _sync_map: Option<&SyncMap>) -> bool {
        // Match TypeScript equals logic exactly
        // Note: syncMap not used for certificates, userId and certificateId not compared
        
        if self.certificate_type() != other.certificate_type
            || self.subject() != other.subject
            || self.serial_number() != other.serial_number
            || self.revocation_outpoint() != other.revocation_outpoint
            || self.signature() != other.signature
            || self.verifier() != other.verifier.as_deref()
            || self.is_deleted() != other.is_deleted
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
    fn test_entity_certificate_new_default() {
        let entity = EntityCertificate::new(None);
        assert_eq!(entity.certificate_id(), 0);
        assert_eq!(entity.user_id(), 0);
        assert_eq!(entity.certificate_type(), "");
        assert_eq!(entity.serial_number(), "");
        assert_eq!(entity.certifier(), "");
        assert_eq!(entity.subject(), "");
        assert_eq!(entity.verifier(), None);
        assert_eq!(entity.revocation_outpoint(), "");
        assert_eq!(entity.signature(), "");
        assert_eq!(entity.is_deleted(), false);
    }

    #[test]
    fn test_entity_certificate_new_with_api() {
        let cert = TableCertificate {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            certificate_id: 1,
            user_id: 100,
            certificate_type: "identity".to_string(),
            serial_number: "SN123".to_string(),
            certifier: "certifier_pubkey".to_string(),
            subject: "subject_pubkey".to_string(),
            verifier: Some("verifier_pubkey".to_string()),
            revocation_outpoint: "txid:0".to_string(),
            signature: "signature_hex".to_string(),
            is_deleted: false,
        };

        let entity = EntityCertificate::new(Some(cert));
        assert_eq!(entity.certificate_id(), 1);
        assert_eq!(entity.user_id(), 100);
        assert_eq!(entity.certificate_type(), "identity");
        assert_eq!(entity.serial_number(), "SN123");
        assert_eq!(entity.certifier(), "certifier_pubkey");
        assert_eq!(entity.subject(), "subject_pubkey");
        assert_eq!(entity.verifier(), Some("verifier_pubkey"));
        assert_eq!(entity.revocation_outpoint(), "txid:0");
        assert_eq!(entity.signature(), "signature_hex");
        assert_eq!(entity.is_deleted(), false);
    }

    #[test]
    fn test_entity_certificate_property_accessors() {
        let mut entity = EntityCertificate::new(None);

        entity.set_certificate_id(42);
        assert_eq!(entity.certificate_id(), 42);

        entity.set_user_id(100);
        assert_eq!(entity.user_id(), 100);

        entity.set_certificate_type("identity");
        assert_eq!(entity.certificate_type(), "identity");

        entity.set_serial_number("SN456");
        assert_eq!(entity.serial_number(), "SN456");

        entity.set_certifier("cert_key");
        assert_eq!(entity.certifier(), "cert_key");

        entity.set_subject("subj_key");
        assert_eq!(entity.subject(), "subj_key");

        entity.set_verifier(Some("verif_key".to_string()));
        assert_eq!(entity.verifier(), Some("verif_key"));

        entity.set_revocation_outpoint("txid:5");
        assert_eq!(entity.revocation_outpoint(), "txid:5");

        entity.set_signature("sig_hex");
        assert_eq!(entity.signature(), "sig_hex");

        entity.set_is_deleted(true);
        assert_eq!(entity.is_deleted(), true);
    }

    #[test]
    fn test_entity_certificate_equals_same() {
        let cert = TableCertificate {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            certificate_id: 1,
            user_id: 100,
            certificate_type: "identity".to_string(),
            serial_number: "SN123".to_string(),
            certifier: "certifier_pubkey".to_string(),
            subject: "subject_pubkey".to_string(),
            verifier: Some("verifier_pubkey".to_string()),
            revocation_outpoint: "txid:0".to_string(),
            signature: "signature_hex".to_string(),
            is_deleted: false,
        };

        let entity = EntityCertificate::new(Some(cert.clone()));
        assert!(entity.equals(&cert, None));
    }

    #[test]
    fn test_entity_certificate_equals_different_type() {
        let cert1 = TableCertificate {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            certificate_id: 1,
            user_id: 100,
            certificate_type: "identity".to_string(),
            serial_number: "SN123".to_string(),
            certifier: "certifier_pubkey".to_string(),
            subject: "subject_pubkey".to_string(),
            verifier: None,
            revocation_outpoint: "txid:0".to_string(),
            signature: "signature_hex".to_string(),
            is_deleted: false,
        };

        let mut cert2 = cert1.clone();
        cert2.certificate_type = "different".to_string();

        let entity = EntityCertificate::new(Some(cert1));
        assert!(!entity.equals(&cert2, None));
    }

    #[test]
    fn test_entity_certificate_equals_different_is_deleted() {
        let cert1 = TableCertificate {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            certificate_id: 1,
            user_id: 100,
            certificate_type: "identity".to_string(),
            serial_number: "SN123".to_string(),
            certifier: "certifier_pubkey".to_string(),
            subject: "subject_pubkey".to_string(),
            verifier: None,
            revocation_outpoint: "txid:0".to_string(),
            signature: "signature_hex".to_string(),
            is_deleted: false,
        };

        let mut cert2 = cert1.clone();
        cert2.is_deleted = true;

        let entity = EntityCertificate::new(Some(cert1));
        assert!(!entity.equals(&cert2, None));
    }

    #[test]
    fn test_entity_certificate_equals_ignores_id() {
        // Certificate equality doesn't compare certificateId or userId
        let cert1 = TableCertificate {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            certificate_id: 1,
            user_id: 100,
            certificate_type: "identity".to_string(),
            serial_number: "SN123".to_string(),
            certifier: "certifier_pubkey".to_string(),
            subject: "subject_pubkey".to_string(),
            verifier: None,
            revocation_outpoint: "txid:0".to_string(),
            signature: "signature_hex".to_string(),
            is_deleted: false,
        };

        let mut cert2 = cert1.clone();
        cert2.certificate_id = 999;
        cert2.user_id = 888;

        let entity = EntityCertificate::new(Some(cert1));
        // Should still be equal since IDs are not compared
        assert!(entity.equals(&cert2, None));
    }

    #[test]
    fn test_entity_certificate_entity_name() {
        let entity = EntityCertificate::new(None);
        assert_eq!(entity.entity_name(), "certificate");
    }

    #[test]
    fn test_entity_certificate_entity_table() {
        let entity = EntityCertificate::new(None);
        assert_eq!(entity.entity_table(), "certificates");
    }

    #[test]
    fn test_entity_certificate_id_methods() {
        let mut entity = EntityCertificate::new(None);
        
        assert_eq!(entity.id(), 0);
        entity.set_id(999);
        assert_eq!(entity.id(), 999);
        assert_eq!(entity.certificate_id(), 999);
    }

    #[test]
    fn test_entity_certificate_clone() {
        let cert = TableCertificate {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            certificate_id: 1,
            user_id: 100,
            certificate_type: "identity".to_string(),
            serial_number: "SN123".to_string(),
            certifier: "certifier_pubkey".to_string(),
            subject: "subject_pubkey".to_string(),
            verifier: None,
            revocation_outpoint: "txid:0".to_string(),
            signature: "signature_hex".to_string(),
            is_deleted: false,
        };

        let entity1 = EntityCertificate::new(Some(cert));
        let entity2 = entity1.clone();
        
        assert_eq!(entity1, entity2);
        assert_eq!(entity2.certificate_id(), 1);
        assert_eq!(entity2.serial_number(), "SN123");
    }
}
