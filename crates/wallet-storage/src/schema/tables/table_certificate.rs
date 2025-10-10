//! TableCertificate - Certificate records
//!
//! Translates TypeScript TableCertificate interface to Rust.
//! Reference: wallet-toolbox/src/storage/schema/tables/TableCertificate.ts

use serde::{Deserialize, Serialize};

/// Certificate table - stores identity certificates
///
/// Matches TypeScript `TableCertificate` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableCertificate {
    pub created_at: String,
    pub updated_at: String,
    
    #[serde(rename = "certificateId")]
    pub certificate_id: i64,
    
    #[serde(rename = "userId")]
    pub user_id: i64,
    
    /// Base64 string
    #[serde(rename = "type")]
    pub certificate_type: String,
    
    /// Base64 string
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    
    /// Public key hex
    pub certifier: String,
    
    /// Public key hex
    pub subject: String,
    
    /// Optional public key hex
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifier: Option<String>,
    
    /// Outpoint string
    #[serde(rename = "revocationOutpoint")]
    pub revocation_outpoint: String,
    
    /// Hex string
    pub signature: String,
    
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

impl TableCertificate {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        certificate_id: i64,
        user_id: i64,
        certificate_type: impl Into<String>,
        serial_number: impl Into<String>,
        certifier: impl Into<String>,
        subject: impl Into<String>,
        revocation_outpoint: impl Into<String>,
        signature: impl Into<String>,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            certificate_id,
            user_id,
            certificate_type: certificate_type.into(),
            serial_number: serial_number.into(),
            certifier: certifier.into(),
            subject: subject.into(),
            verifier: None,
            revocation_outpoint: revocation_outpoint.into(),
            signature: signature.into(),
            is_deleted: false,
        }
    }

    pub fn with_verifier(mut self, verifier: impl Into<String>) -> Self {
        self.verifier = Some(verifier.into());
        self
    }

    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    pub fn delete(&mut self) {
        self.is_deleted = true;
        self.touch();
    }

    pub fn restore(&mut self) {
        self.is_deleted = false;
        self.touch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_certificate_new() {
        let cert = TableCertificate::new(
            1, 100, "identity", "SN123",
            "certifier_pubkey", "subject_pubkey",
            "txid:0", "signature_hex"
        );
        assert_eq!(cert.certificate_id, 1);
        assert_eq!(cert.user_id, 100);
        assert_eq!(cert.certificate_type, "identity");
        assert_eq!(cert.is_deleted, false);
    }

    #[test]
    fn test_table_certificate_with_verifier() {
        let cert = TableCertificate::new(
            1, 100, "type", "serial", "cert", "subj", "out", "sig"
        ).with_verifier("verifier_pubkey");
        assert_eq!(cert.verifier, Some("verifier_pubkey".to_string()));
    }

    #[test]
    fn test_table_certificate_serialization() {
        let cert = TableCertificate::new(
            1, 100, "type", "serial", "cert", "subj", "out", "sig"
        );
        let json = serde_json::to_string(&cert).unwrap();
        assert!(json.contains("\"certificateId\":1"));
        assert!(json.contains("\"serialNumber\":\"serial\""));
        let deserialized: TableCertificate = serde_json::from_str(&json).unwrap();
        assert_eq!(cert, deserialized);
    }
}
