//! Acquire Direct Certificate
//!
//! **Reference**: TypeScript `src/signer/methods/acquireDirectCertificate.ts`
//!
//! Acquires a certificate directly and stores it

use crate::sdk::errors::{WalletError, WalletResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Validated acquire certificate arguments
///
/// Reference: TS ValidAcquireDirectCertificateArgs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidAcquireDirectCertificateArgs {
    /// Certificate type
    #[serde(rename = "type")]
    pub cert_type: String,
    
    /// Subject identity key
    pub subject: String,
    
    /// Serial number
    pub serial_number: String,
    
    /// Certifier identity key
    pub certifier: String,
    
    /// Revocation outpoint
    pub revocation_outpoint: String,
    
    /// Signature
    pub signature: String,
    
    /// Certificate fields
    pub fields: HashMap<String, String>,
    
    /// Keyring for subject
    pub keyring_for_subject: HashMap<String, String>,
    
    /// Keyring revealer
    pub keyring_revealer: String,
}

/// Acquire certificate result
///
/// Reference: TS AcquireCertificateResult from @bsv/sdk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquireCertificateResult {
    /// Certificate type
    #[serde(rename = "type")]
    pub cert_type: String,
    
    /// Subject identity key
    pub subject: String,
    
    /// Serial number
    pub serial_number: String,
    
    /// Certifier identity key  
    pub certifier: String,
    
    /// Revocation outpoint
    pub revocation_outpoint: String,
    
    /// Signature
    pub signature: String,
    
    /// Certificate fields
    pub fields: HashMap<String, String>,
}

/// Certificate field for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateField {
    /// Certificate ID (set by storage)
    pub certificate_id: u64,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Update timestamp
    pub updated_at: DateTime<Utc>,
    
    /// User ID
    pub user_id: u64,
    
    /// Field name
    pub field_name: String,
    
    /// Field value
    pub field_value: String,
    
    /// Master key
    pub master_key: String,
}

/// New certificate for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCertificate {
    /// Certificate ID (will be replaced by storage insert)
    pub certificate_id: u64,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Update timestamp
    pub updated_at: DateTime<Utc>,
    
    /// User ID
    pub user_id: u64,
    
    /// Certificate type
    #[serde(rename = "type")]
    pub cert_type: String,
    
    /// Subject
    pub subject: String,
    
    /// Verifier
    pub verifier: String,
    
    /// Serial number
    pub serial_number: String,
    
    /// Certifier
    pub certifier: String,
    
    /// Revocation outpoint
    pub revocation_outpoint: String,
    
    /// Signature
    pub signature: String,
    
    /// Fields
    pub fields: Vec<CertificateField>,
    
    /// Is deleted
    pub is_deleted: bool,
}

/// Acquire a direct certificate
///
/// Reference: TS acquireDirectCertificate (acquireDirectCertificate.ts lines 7-53)
///
/// # Arguments
/// * `user_id` - User ID for authentication
/// * `vargs` - Validated certificate arguments
/// * `storage` - Storage interface for saving certificate
///
/// # Returns
/// Certificate acquisition result
pub async fn acquire_direct_certificate(
    user_id: u64,
    vargs: ValidAcquireDirectCertificateArgs,
) -> WalletResult<AcquireCertificateResult> {
    // Get current timestamp (TS line 12)
    let now = Utc::now();
    
    // Determine verifier (TS line 20)
    let verifier = if vargs.keyring_revealer == "certifier" {
        vargs.certifier.clone()
    } else {
        vargs.keyring_revealer.clone()
    };
    
    // Create new certificate (TS lines 13-27)
    let mut new_cert = NewCertificate {
        certificate_id: 0, // Will be replaced by storage insert
        created_at: now,
        updated_at: now,
        user_id,
        cert_type: vargs.cert_type.clone(),
        subject: vargs.subject.clone(),
        verifier,
        serial_number: vargs.serial_number.clone(),
        certifier: vargs.certifier.clone(),
        revocation_outpoint: vargs.revocation_outpoint.clone(),
        signature: vargs.signature.clone(),
        fields: vec![],
        is_deleted: false,
    };
    
    // Add fields (TS lines 28-38)
    for (name, value) in vargs.fields.iter() {
        let master_key = vargs.keyring_for_subject.get(name)
            .cloned()
            .unwrap_or_default();
        
        new_cert.fields.push(CertificateField {
            certificate_id: 0, // Will be replaced by storage insert
            created_at: now,
            updated_at: now,
            user_id,
            field_name: name.clone(),
            field_value: value.clone(),
            master_key,
        });
    }
    
    // Insert certificate into storage (TS line 40)
    // TODO: Integrate with actual storage
    // let count = wallet.storage.insertCertificate(newCert).await?;
    
    // Build result (TS lines 42-50)
    let result = AcquireCertificateResult {
        cert_type: vargs.cert_type,
        subject: vargs.subject,
        serial_number: vargs.serial_number,
        certifier: vargs.certifier,
        revocation_outpoint: vargs.revocation_outpoint,
        signature: vargs.signature,
        fields: vargs.fields,
    };
    
    Ok(result)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_acquire_direct_certificate() {
        let mut fields = HashMap::new();
        fields.insert("name".to_string(), "John Doe".to_string());
        
        let mut keyring = HashMap::new();
        keyring.insert("name".to_string(), "master_key_123".to_string());
        
        let vargs = ValidAcquireDirectCertificateArgs {
            cert_type: "identity".to_string(),
            subject: "subject_key".to_string(),
            serial_number: "12345".to_string(),
            certifier: "certifier_key".to_string(),
            revocation_outpoint: "txid.0".to_string(),
            signature: "sig_data".to_string(),
            fields,
            keyring_for_subject: keyring,
            keyring_revealer: "certifier".to_string(),
        };
        
        let result = acquire_direct_certificate(1, vargs).await;
        assert!(result.is_ok());
        
        let cert = result.unwrap();
        assert_eq!(cert.cert_type, "identity");
        assert_eq!(cert.serial_number, "12345");
        assert_eq!(cert.fields.len(), 1);
    }
    
    #[test]
    fn test_certificate_field_creation() {
        let field = CertificateField {
            certificate_id: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            user_id: 1,
            field_name: "email".to_string(),
            field_value: "test@example.com".to_string(),
            master_key: "key123".to_string(),
        };
        
        assert_eq!(field.field_name, "email");
        assert_eq!(field.field_value, "test@example.com");
    }
}
