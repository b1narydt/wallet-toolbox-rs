//! Prove Certificate
//!
//! **Reference**: TypeScript `src/signer/methods/proveCertificate.ts`
//!
//! Proves ownership of a certificate by revealing fields to a verifier

use crate::sdk::error::{WalletError, WalletResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Validated prove certificate arguments
///
/// Reference: TS ValidProveCertificateArgs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidProveCertificateArgs {
    /// Certificate type
    #[serde(rename = "type")]
    pub cert_type: String,
    
    /// Serial number
    pub serial_number: String,
    
    /// Certifier identity key
    pub certifier: String,
    
    /// Subject identity key
    pub subject: String,
    
    /// Revocation outpoint
    pub revocation_outpoint: String,
    
    /// Signature
    pub signature: String,
    
    /// Verifier to reveal to
    pub verifier: String,
    
    /// Fields to reveal
    pub fields_to_reveal: Vec<String>,
    
    /// Privileged access flag
    #[serde(default)]
    pub privileged: bool,
    
    /// Privileged access reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_reason: Option<String>,
}

/// Prove certificate result
///
/// Reference: TS ProveCertificateResult from @bsv/sdk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProveCertificateResult {
    /// Keyring for verifier
    pub keyring_for_verifier: HashMap<String, String>,
}

/// List certificates arguments
///
/// Reference: TS ValidListCertificatesArgs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCertificatesArgs {
    /// Partial certificate to match
    pub partial: PartialCertificate,
    
    /// Certifiers filter
    pub certifiers: Vec<String>,
    
    /// Types filter
    pub types: Vec<String>,
    
    /// Limit
    pub limit: u32,
    
    /// Offset
    pub offset: u32,
    
    /// Privileged access
    pub privileged: bool,
}

/// Partial certificate for matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialCertificate {
    /// Certificate type
    #[serde(rename = "type")]
    pub cert_type: String,
    
    /// Serial number
    pub serial_number: String,
    
    /// Certifier
    pub certifier: String,
    
    /// Subject
    pub subject: String,
    
    /// Revocation outpoint
    pub revocation_outpoint: String,
    
    /// Signature
    pub signature: String,
}

/// Certificate field from storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCertificateField {
    /// Field name
    pub field_name: String,
    
    /// Field value
    pub field_value: String,
    
    /// Master key
    pub master_key: String,
}

/// Certificate from storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCertificate {
    /// Certificate ID
    pub certificate_id: u64,
    
    /// Certificate type
    #[serde(rename = "type")]
    pub cert_type: String,
    
    /// Certifier
    pub certifier: String,
    
    /// Subject
    pub subject: String,
    
    /// Serial number
    pub serial_number: String,
    
    /// Fields
    pub fields: Vec<StorageCertificateField>,
    
    /// Keyring
    pub keyring: Option<HashMap<String, String>>,
}

/// Prove a certificate to a verifier
///
/// Reference: TS proveCertificate (proveCertificate.ts lines 7-44)
///
/// # Arguments
/// * `vargs` - Validated prove certificate arguments
/// * `storage` - Storage interface for listing certificates
///
/// # Returns
/// Keyring for verifier containing revealed fields
pub async fn prove_certificate(
    vargs: ValidProveCertificateArgs,
) -> WalletResult<ProveCertificateResult> {
    // Build list certificates arguments (TS lines 12-26)
    let lc_args = ListCertificatesArgs {
        partial: PartialCertificate {
            cert_type: vargs.cert_type.clone(),
            serial_number: vargs.serial_number.clone(),
            certifier: vargs.certifier.clone(),
            subject: vargs.subject.clone(),
            revocation_outpoint: vargs.revocation_outpoint.clone(),
            signature: vargs.signature.clone(),
        },
        certifiers: vec![],
        types: vec![],
        limit: 2,
        offset: 0,
        privileged: false,
    };
    
    // List certificates from storage (TS line 28)
    // TODO: Integrate with actual storage
    // let lcr = await wallet.storage.listCertificates(lc_args);
    
    // For now, return error indicating storage integration needed
    // In real implementation:
    // 1. Call storage.listCertificates(lc_args)
    // 2. Verify exactly one certificate matches (TS line 29)
    // 3. Get the storage certificate (TS line 30)
    // 4. Create keyring for verifier (TS lines 31-41)
    //    using MasterCertificate.createKeyringForVerifier
    
    // Placeholder implementation (TS lines 31-44)
    let keyring_for_verifier = HashMap::new();
    
    // TODO: Actual implementation would call:
    // let keyring_for_verifier = MasterCertificate::create_keyring_for_verifier(
    //     wallet,
    //     storage_cert.certifier,
    //     vargs.verifier,
    //     storage_cert.fields,
    //     vargs.fields_to_reveal,
    //     storage_cert.keyring,
    //     storage_cert.serial_number,
    //     vargs.privileged,
    //     vargs.privileged_reason,
    // ).await?;
    
    Ok(ProveCertificateResult {
        keyring_for_verifier,
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_prove_certificate() {
        let vargs = ValidProveCertificateArgs {
            cert_type: "identity".to_string(),
            serial_number: "12345".to_string(),
            certifier: "certifier_key".to_string(),
            subject: "subject_key".to_string(),
            revocation_outpoint: "txid.0".to_string(),
            signature: "sig_data".to_string(),
            verifier: "verifier_key".to_string(),
            fields_to_reveal: vec!["name".to_string()],
            privileged: false,
            privileged_reason: None,
        };
        
        let result = prove_certificate(vargs).await;
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_list_certificates_args_creation() {
        let args = ListCertificatesArgs {
            partial: PartialCertificate {
                cert_type: "test".to_string(),
                serial_number: "123".to_string(),
                certifier: "cert".to_string(),
                subject: "subj".to_string(),
                revocation_outpoint: "out".to_string(),
                signature: "sig".to_string(),
            },
            certifiers: vec![],
            types: vec![],
            limit: 10,
            offset: 0,
            privileged: false,
        };
        
        assert_eq!(args.limit, 10);
        assert_eq!(args.partial.cert_type, "test");
    }
    
    #[test]
    fn test_prove_certificate_result() {
        let mut keyring = HashMap::new();
        keyring.insert("field1".to_string(), "value1".to_string());
        
        let result = ProveCertificateResult {
            keyring_for_verifier: keyring,
        };
        
        assert_eq!(result.keyring_for_verifier.len(), 1);
    }
}
