//! Permission Request Methods
//!
//! **Reference**: TypeScript `src/WalletPermissionsManager.ts` lines 526-1180
//!
//! Handles permission grant/deny flows and ensures permission checks

use super::types::*;
use super::callbacks::*;
use super::constants::*;
use crate::sdk::errors::{WalletError, WalletResult};
use std::collections::HashMap;
use tokio::sync::oneshot;

/// Grant permission parameters
///
/// Reference: TS grantPermission params (WalletPermissionsManager.ts lines 535-540)
#[derive(Debug, Clone)]
pub struct GrantPermissionParams {
    /// Request ID to identify which request is granted
    pub request_id: String,
    
    /// Optional expiry time (UNIX epoch seconds)
    pub expiry: Option<i64>,
    
    /// If true, permission is ephemeral (one-time, no on-chain token)
    pub ephemeral: Option<bool>,
    
    /// For spending authorizations, the authorized amount
    pub amount: Option<i64>,
}

/// Grant grouped permission parameters
///
/// Reference: TS grantGroupedPermission params (WalletPermissionsManager.ts lines 609-613)
#[derive(Debug, Clone)]
pub struct GrantGroupedPermissionParams {
    /// Request ID to identify which request is granted
    pub request_id: String,
    
    /// Subset of originally requested permissions that user has granted
    pub granted: GroupedPermissions,
    
    /// Optional expiry time (UNIX epoch seconds)
    pub expiry: Option<i64>,
}

/// Ensure protocol permission parameters
///
/// Reference: TS ensureProtocolPermission params (WalletPermissionsManager.ts lines 750-766)
#[derive(Debug, Clone)]
pub struct EnsureProtocolPermissionParams {
    /// The originator domain or FQDN
    pub originator: String,
    
    /// Whether this is a privileged operation
    pub privileged: bool,
    
    /// Protocol ID [securityLevel, protocolName]
    pub protocol_id: Vec<String>,
    
    /// Counterparty (public key, "self", or "anyone")
    pub counterparty: String,
    
    /// Human-readable reason for the permission
    pub reason: Option<String>,
    
    /// Whether to seek permission if not found (default: true)
    pub seek_permission: bool,
    
    /// Type of usage for config checking
    pub usage_type: ProtocolUsageType,
}

/// Protocol usage type enumeration
///
/// Reference: TS usageType in ensureProtocolPermission (line 765)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolUsageType {
    /// Signing operations
    Signing,
    /// Encryption operations
    Encrypting,
    /// HMAC operations
    Hmac,
    /// Public key revelation
    PublicKey,
    /// Identity key revelation
    IdentityKey,
    /// Linkage revelation
    LinkageRevelation,
    /// Generic protocol usage
    Generic,
}

/// Ensure basket access parameters
///
/// Reference: TS ensureBasketAccess params (WalletPermissionsManager.ts lines 864-876)
#[derive(Debug, Clone)]
pub struct EnsureBasketAccessParams {
    /// The originator domain or FQDN
    pub originator: String,
    
    /// Basket name
    pub basket: String,
    
    /// Human-readable reason
    pub reason: Option<String>,
    
    /// Whether to seek permission if not found
    pub seek_permission: bool,
    
    /// Type of basket usage
    pub usage_type: BasketUsageType,
}

/// Basket usage type enumeration
///
/// Reference: TS usageType in ensureBasketAccess (line 875)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasketUsageType {
    /// Inserting into basket
    Insertion,
    /// Removing from basket
    Removal,
    /// Listing basket contents
    Listing,
}

/// Ensure certificate access parameters
///
/// Reference: TS ensureCertificateAccess params (WalletPermissionsManager.ts lines 926-944)
#[derive(Debug, Clone)]
pub struct EnsureCertificateAccessParams {
    /// The originator domain or FQDN
    pub originator: String,
    
    /// Whether this is a privileged operation
    pub privileged: bool,
    
    /// Verifier public key
    pub verifier: String,
    
    /// Certificate type
    pub cert_type: String,
    
    /// Certificate fields
    pub fields: Vec<String>,
    
    /// Human-readable reason
    pub reason: Option<String>,
    
    /// Whether to seek permission if not found
    pub seek_permission: bool,
    
    /// Type of certificate usage (currently only 'disclosure')
    pub usage_type: CertificateUsageType,
}

/// Certificate usage type enumeration
///
/// Reference: TS usageType in ensureCertificateAccess (line 943)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateUsageType {
    /// Disclosing certificate fields
    Disclosure,
}

/// Ensure spending authorization parameters
///
/// Reference: TS ensureSpendingAuthorization params (WalletPermissionsManager.ts lines 1007-1023)
#[derive(Debug, Clone)]
pub struct EnsureSpendingAuthorizationParams {
    /// The originator domain or FQDN
    pub originator: String,
    
    /// Amount in satoshis
    pub satoshis: i64,
    
    /// Optional line item breakdown
    pub line_items: Option<Vec<SpendingLineItem>>,
    
    /// Human-readable reason
    pub reason: Option<String>,
    
    /// Whether to seek permission if not found
    pub seek_permission: bool,
}

impl Default for EnsureProtocolPermissionParams {
    fn default() -> Self {
        Self {
            originator: String::new(),
            privileged: false,
            protocol_id: Vec::new(),
            counterparty: "self".to_string(),
            reason: None,
            seek_permission: true,
            usage_type: ProtocolUsageType::Generic,
        }
    }
}

impl Default for EnsureBasketAccessParams {
    fn default() -> Self {
        Self {
            originator: String::new(),
            basket: String::new(),
            reason: None,
            seek_permission: true,
            usage_type: BasketUsageType::Listing,
        }
    }
}

impl Default for EnsureCertificateAccessParams {
    fn default() -> Self {
        Self {
            originator: String::new(),
            privileged: false,
            verifier: String::new(),
            cert_type: String::new(),
            fields: Vec::new(),
            reason: None,
            seek_permission: true,
            usage_type: CertificateUsageType::Disclosure,
        }
    }
}

impl Default for EnsureSpendingAuthorizationParams {
    fn default() -> Self {
        Self {
            originator: String::new(),
            satoshis: 0,
            line_items: None,
            reason: None,
            seek_permission: true,
        }
    }
}

/// Calculate default expiry (30 days from now)
///
/// Reference: TS default expiry calculation (lines 560, 568, 577, 646)
pub fn calculate_default_expiry() -> i64 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    // TS: Math.floor(Date.now() / 1000) + 3600 * 24 * 30
    now + 3600 * 24 * 30 // 30 days
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_grant_permission_params() {
        let params = GrantPermissionParams {
            request_id: "test-123".to_string(),
            expiry: Some(calculate_default_expiry()),
            ephemeral: Some(false),
            amount: Some(1000),
        };
        
        assert_eq!(params.request_id, "test-123");
        assert!(params.expiry.is_some());
        assert_eq!(params.ephemeral, Some(false));
        assert_eq!(params.amount, Some(1000));
    }
    
    #[test]
    fn test_ensure_protocol_permission_params_defaults() {
        let params = EnsureProtocolPermissionParams::default();
        
        assert_eq!(params.counterparty, "self");
        assert!(params.seek_permission);
        assert!(!params.privileged);
    }
    
    #[test]
    fn test_ensure_basket_access_params_defaults() {
        let params = EnsureBasketAccessParams::default();
        
        assert!(params.seek_permission);
        assert!(matches!(params.usage_type, BasketUsageType::Listing));
    }
    
    #[test]
    fn test_calculate_default_expiry() {
        let expiry = calculate_default_expiry();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        // Should be approximately 30 days from now
        let diff = expiry - now;
        assert!(diff >= 29 * 24 * 3600 && diff <= 31 * 24 * 3600);
    }
    
    #[test]
    fn test_protocol_usage_types() {
        assert_eq!(
            std::mem::discriminant(&ProtocolUsageType::Signing),
            std::mem::discriminant(&ProtocolUsageType::Signing)
        );
        assert_ne!(
            std::mem::discriminant(&ProtocolUsageType::Signing),
            std::mem::discriminant(&ProtocolUsageType::Encrypting)
        );
    }
}
