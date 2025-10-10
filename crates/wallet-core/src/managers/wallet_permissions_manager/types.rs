//! Permission Manager Types
//!
//! **Reference**: TypeScript `src/WalletPermissionsManager.ts` lines 47-269
//!
//! Defines all data structures for the BRC-73 permission system, including:
//! - DPACP (Domain Protocol Access Control Protocol)
//! - DBAP (Domain Basket Access Protocol)
//! - DCAP (Domain Certificate Access Protocol)
//! - DSAP (Domain Spending Authorization Protocol)

use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Security level for protocol permissions
///
/// Reference: TS SecurityLevels from @bsv/sdk
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Level 0 - Public operations
    #[serde(rename = "0")]
    Public = 0,
    /// Level 1 - Shared operations
    #[serde(rename = "1")]
    Shared = 1,
    /// Level 2 - Private operations
    #[serde(rename = "2")]
    Private = 2,
}

/// Permission type enumeration
///
/// Reference: TS PermissionRequest.type (WalletPermissionsManager.ts line 104)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PermissionType {
    /// Protocol usage permission (DPACP)
    Protocol,
    /// Basket access permission (DBAP)
    Basket,
    /// Certificate access permission (DCAP)
    Certificate,
    /// Spending authorization permission (DSAP)
    Spending,
}

/// Spending line item
///
/// Reference: TS lineItems in spending (WalletPermissionsManager.ts lines 122-126)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpendingLineItem {
    /// Type of line item
    #[serde(rename = "type")]
    pub item_type: String, // 'input' | 'output' | 'fee'
    
    /// Human-readable description
    pub description: String,
    
    /// Amount in satoshis
    pub satoshis: i64,
}

/// Spending details for authorization
///
/// Reference: TS spending in PermissionRequest (WalletPermissionsManager.ts lines 119-127)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpendingDetails {
    /// Total satoshis to authorize
    pub satoshis: i64,
    
    /// Optional breakdown of spending
    #[serde(rename = "lineItems", skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<SpendingLineItem>>,
}

/// Certificate details for access
///
/// Reference: TS certificate in PermissionRequest (WalletPermissionsManager.ts lines 112-117)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificateDetails {
    /// Verifier public key
    pub verifier: String,
    
    /// Certificate type identifier
    #[serde(rename = "certType")]
    pub cert_type: String,
    
    /// Certificate fields being accessed
    pub fields: Vec<String>,
}

/// Spending authorization for grouped permissions
///
/// Reference: TS spendingAuthorization in GroupedPermissions (WalletPermissionsManager.ts lines 53-56)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpendingAuthorization {
    /// Maximum amount to authorize
    pub amount: i64,
    
    /// Description of the authorization
    pub description: String,
}

/// Protocol permission for grouped permissions
///
/// Reference: TS protocolPermissions in GroupedPermissions (WalletPermissionsManager.ts lines 57-61)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolPermission {
    /// Protocol ID (BRC-43 format)
    #[serde(rename = "protocolID")]
    pub protocol_id: Vec<String>, // [securityLevel, protocolName]
    
    /// Optional counterparty (public key, "self", or "anyone")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<String>,
    
    /// Human-readable description
    pub description: String,
}

/// Basket access for grouped permissions
///
/// Reference: TS basketAccess in GroupedPermissions (WalletPermissionsManager.ts lines 62-65)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BasketAccess {
    /// Basket name
    pub basket: String,
    
    /// Human-readable description
    pub description: String,
}

/// Certificate access for grouped permissions
///
/// Reference: TS certificateAccess in GroupedPermissions (WalletPermissionsManager.ts lines 66-71)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificateAccess {
    /// Certificate type identifier
    #[serde(rename = "type")]
    pub cert_type: String,
    
    /// Certificate fields
    pub fields: Vec<String>,
    
    /// Verifier public key
    #[serde(rename = "verifierPublicKey")]
    pub verifier_public_key: String,
    
    /// Human-readable description
    pub description: String,
}

/// Describes a group of permissions that can be requested together
///
/// Reference: TS GroupedPermissions (WalletPermissionsManager.ts lines 51-72)
///
/// This structure is based on BRC-73.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupedPermissions {
    /// Optional overall description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Spending authorization details
    #[serde(rename = "spendingAuthorization", skip_serializing_if = "Option::is_none")]
    pub spending_authorization: Option<SpendingAuthorization>,
    
    /// Protocol permissions
    #[serde(rename = "protocolPermissions", skip_serializing_if = "Option::is_none")]
    pub protocol_permissions: Option<Vec<ProtocolPermission>>,
    
    /// Basket access permissions
    #[serde(rename = "basketAccess", skip_serializing_if = "Option::is_none")]
    pub basket_access: Option<Vec<BasketAccess>>,
    
    /// Certificate access permissions
    #[serde(rename = "certificateAccess", skip_serializing_if = "Option::is_none")]
    pub certificate_access: Option<Vec<CertificateAccess>>,
}

/// The object passed to the UI when a grouped permission is requested
///
/// Reference: TS GroupedPermissionRequest (WalletPermissionsManager.ts lines 77-81)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupedPermissionRequest {
    /// The originator domain or FQDN
    pub originator: String,
    
    /// Unique request identifier
    #[serde(rename = "requestID")]
    pub request_id: String,
    
    /// The grouped permissions being requested
    pub permissions: GroupedPermissions,
}

/// Describes a single requested permission that the user must either grant or deny
///
/// Reference: TS PermissionRequest (WalletPermissionsManager.ts lines 103-132)
///
/// Four categories of permission are supported, each with a unique protocol:
///  1) protocol - "DPACP" (Domain Protocol Access Control Protocol)
///  2) basket   - "DBAP"  (Domain Basket Access Protocol)
///  3) certificate - "DCAP" (Domain Certificate Access Protocol)
///  4) spending - "DSAP"  (Domain Spending Authorization Protocol)
///
/// This model underpins "requests" made to the user for permission, which the user can
/// either grant or deny. The manager can then create on-chain tokens (PushDrop outputs)
/// if permission is granted. Denying requests cause the underlying operation to throw,
/// and no token is created. An "ephemeral" grant is also possible, denoting a one-time
/// authorization without an associated persistent on-chain token.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionRequest {
    /// Permission type
    #[serde(rename = "type")]
    pub permission_type: PermissionType,
    
    /// The domain or FQDN of the requesting application
    pub originator: String,
    
    /// For "protocol" or "certificate" usage, indicating privileged key usage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    
    /// For type='protocol': BRC-43 style (securityLevel, protocolName)
    #[serde(rename = "protocolID", skip_serializing_if = "Option::is_none")]
    pub protocol_id: Option<Vec<String>>,
    
    /// For type='protocol': e.g. target public key or "self"/"anyone"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<String>,
    
    /// For type='basket': the basket name being requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basket: Option<String>,
    
    /// For type='certificate': details about the cert usage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<CertificateDetails>,
    
    /// For type='spending': details about the requested spend
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending: Option<SpendingDetails>,
    
    /// Human-readable explanation for requesting permission
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    
    /// Whether this request is for renewing an expired token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal: Option<bool>,
    
    /// If renewing an expired permission, reference to the old token
    #[serde(rename = "previousToken", skip_serializing_if = "Option::is_none")]
    pub previous_token: Option<Box<PermissionToken>>,
}

/// Data structure representing an on-chain permission token
///
/// Reference: TS PermissionToken (WalletPermissionsManager.ts lines 150-198)
///
/// It is typically stored as a single unspent PushDrop output in a special "internal" admin basket
/// belonging to the user, held in their underlying wallet.
///
/// It can represent any of the four permission categories by having the relevant fields:
///  - DPACP: originator, privileged, protocol, securityLevel, counterparty
///  - DBAP:  originator, basketName
///  - DCAP:  originator, privileged, verifier, certType, certFields
///  - DSAP:  originator, authorizedAmount
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionToken {
    /// The transaction ID where this token resides
    pub txid: String,
    
    /// The current transaction encapsulating the token
    pub tx: Vec<u8>,
    
    /// The output index within that transaction
    #[serde(rename = "outputIndex")]
    pub output_index: u32,
    
    /// The exact script hex for the locking script
    #[serde(rename = "outputScript")]
    pub output_script: String,
    
    /// The amount of satoshis assigned to the permission output (often 1)
    pub satoshis: i64,
    
    /// The originator domain or FQDN that is allowed to use this permission
    pub originator: String,
    
    /// The expiration time for this token in UNIX epoch seconds
    /// (0 or omitted for spending authorizations, which are indefinite)
    pub expiry: i64,
    
    /// Whether this token grants privileged usage (for protocol or certificate)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    
    /// The protocol name, if this is a DPACP token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    
    /// The security level (0,1,2) for DPACP
    #[serde(rename = "securityLevel", skip_serializing_if = "Option::is_none")]
    pub security_level: Option<SecurityLevel>,
    
    /// The counterparty, for DPACP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<String>,
    
    /// The name of a basket, if this is a DBAP token
    #[serde(rename = "basketName", skip_serializing_if = "Option::is_none")]
    pub basket_name: Option<String>,
    
    /// The certificate type, if this is a DCAP token
    #[serde(rename = "certType", skip_serializing_if = "Option::is_none")]
    pub cert_type: Option<String>,
    
    /// The certificate fields that this token covers, if DCAP token
    #[serde(rename = "certFields", skip_serializing_if = "Option::is_none")]
    pub cert_fields: Option<Vec<String>>,
    
    /// The "verifier" public key string, if DCAP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifier: Option<String>,
    
    /// For DSAP, the maximum authorized spending for the month
    #[serde(rename = "authorizedAmount", skip_serializing_if = "Option::is_none")]
    pub authorized_amount: Option<i64>,
}

/// Permission request with request ID
///
/// Reference: TS PermissionRequest & { requestID: string } (WalletPermissionsManager.ts line 137)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionRequestWithId {
    #[serde(flatten)]
    pub request: PermissionRequest,
    
    /// Unique request identifier
    #[serde(rename = "requestID")]
    pub request_id: String,
}

/// Signature for functions that handle a permission request event
///
/// Reference: TS PermissionEventHandler (WalletPermissionsManager.ts line 137)
pub type PermissionEventHandler = Arc<dyn Fn(PermissionRequestWithId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync>;

/// Signature for functions that handle a grouped permission request event
///
/// Reference: TS GroupedPermissionEventHandler (WalletPermissionsManager.ts line 86)
pub type GroupedPermissionEventHandler = Arc<dyn Fn(GroupedPermissionRequest) -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync>;

/// The set of callbacks that external code can bind to
///
/// Reference: TS WalletPermissionsManagerCallbacks (WalletPermissionsManager.ts lines 216-222)
///
/// These callbacks are used to display UI prompts or logs when a permission is requested.
#[derive(Default)]
pub struct WalletPermissionsManagerCallbacks {
    /// Callbacks for protocol permission requests
    #[allow(clippy::type_complexity)]
    pub on_protocol_permission_requested: Vec<PermissionEventHandler>,
    
    /// Callbacks for basket access requests
    #[allow(clippy::type_complexity)]
    pub on_basket_access_requested: Vec<PermissionEventHandler>,
    
    /// Callbacks for certificate access requests
    #[allow(clippy::type_complexity)]
    pub on_certificate_access_requested: Vec<PermissionEventHandler>,
    
    /// Callbacks for spending authorization requests
    #[allow(clippy::type_complexity)]
    pub on_spending_authorization_requested: Vec<PermissionEventHandler>,
    
    /// Callbacks for grouped permission requests
    #[allow(clippy::type_complexity)]
    pub on_grouped_permission_requested: Vec<GroupedPermissionEventHandler>,
}

/// Configuration object for the WalletPermissionsManager
///
/// Reference: TS PermissionsManagerConfig (WalletPermissionsManager.ts lines 230-269)
///
/// If a given option is `false`, the manager will skip or alter certain permission checks.
/// By default, all of these are `true` unless specified otherwise. This is the most secure configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionsManagerConfig {
    /// For `createSignature` and `verifySignature`, require a "protocol usage" permission check?
    ///
    /// Reference: TS seekProtocolPermissionsForSigning (line 235)
    #[serde(rename = "seekProtocolPermissionsForSigning", default = "default_true")]
    pub seek_protocol_permissions_for_signing: bool,
    
    /// For methods that perform encryption (encrypt/decrypt), require a "protocol usage" permission check?
    ///
    /// Reference: TS seekProtocolPermissionsForEncrypting (line 241)
    #[serde(rename = "seekProtocolPermissionsForEncrypting", default = "default_true")]
    pub seek_protocol_permissions_for_encrypting: bool,
    
    /// For methods that perform HMAC creation or verification (createHmac, verifyHmac), require a "protocol usage" permission check?
    ///
    /// Reference: TS seekProtocolPermissionsForHMAC (line 247)
    #[serde(rename = "seekProtocolPermissionsForHMAC", default = "default_true")]
    pub seek_protocol_permissions_for_hmac: bool,
    
    /// For key derivation operations, require "protocol usage" permission?
    ///
    /// Reference: TS seekProtocolPermissionsForKeyDerivation (line 253)
    #[serde(rename = "seekProtocolPermissionsForKeyDerivation", default = "default_true")]
    pub seek_protocol_permissions_for_key_derivation: bool,
    
    /// For certificate operations (acquire, prove, relinquish), require "certificate access" permission?
    ///
    /// Reference: TS seekCertificatePermissionsForCertificateOps (line 259)
    #[serde(rename = "seekCertificatePermissionsForCertificateOps", default = "default_true")]
    pub seek_certificate_permissions_for_certificate_ops: bool,
    
    /// For basket operations (listOutputs with basket filter), require "basket access" permission?
    ///
    /// Reference: TS seekBasketPermissionsForBasketOps (line 265)
    #[serde(rename = "seekBasketPermissionsForBasketOps", default = "default_true")]
    pub seek_basket_permissions_for_basket_ops: bool,
    
    /// If false, permissions are checked without regard for whether we are in privileged mode
    ///
    /// Reference: TS differentiatePrivilegedOperations (lines 337-342)
    ///
    /// Privileged status is ignored with respect to whether permissions are granted.
    /// Internally, they are always sought and checked with privileged=false, regardless of the actual value.
    #[serde(rename = "differentiatePrivilegedOperations", default = "default_true")]
    pub differentiate_privileged_operations: bool,
    
    /// When inserting into baskets, ask for basket permission?
    ///
    /// Reference: TS seekBasketInsertionPermissions (lines 273-276)
    #[serde(rename = "seekBasketInsertionPermissions", default = "default_true")]
    pub seek_basket_insertion_permissions: bool,
    
    /// When removing from baskets (relinquishOutput), ask for basket permission?
    ///
    /// Reference: TS seekBasketRemovalPermissions (lines 278-281)
    #[serde(rename = "seekBasketRemovalPermissions", default = "default_true")]
    pub seek_basket_removal_permissions: bool,
    
    /// When listing basket outputs (listOutputs), ask for basket permission?
    ///
    /// Reference: TS seekBasketListingPermissions (lines 283-286)
    #[serde(rename = "seekBasketListingPermissions", default = "default_true")]
    pub seek_basket_listing_permissions: bool,
    
    /// When revealing certificate fields, require certificate access permission?
    ///
    /// Reference: TS seekCertificateDisclosurePermissions (lines 299-302)
    #[serde(rename = "seekCertificateDisclosurePermissions", default = "default_true")]
    pub seek_certificate_disclosure_permissions: bool,
    
    /// When spending wallet funds (netSpent > 0), seek spending authorization?
    ///
    /// Reference: TS seekSpendingPermissions (lines 326-329)
    #[serde(rename = "seekSpendingPermissions", default = "default_true")]
    pub seek_spending_permissions: bool,
}

impl Default for PermissionsManagerConfig {
    fn default() -> Self {
        Self {
            seek_protocol_permissions_for_signing: true,
            seek_protocol_permissions_for_encrypting: true,
            seek_protocol_permissions_for_hmac: true,
            seek_protocol_permissions_for_key_derivation: true,
            seek_certificate_permissions_for_certificate_ops: true,
            seek_basket_permissions_for_basket_ops: true,
            differentiate_privileged_operations: true,
            seek_basket_insertion_permissions: true,
            seek_basket_removal_permissions: true,
            seek_basket_listing_permissions: true,
            seek_certificate_disclosure_permissions: true,
            seek_spending_permissions: true,
        }
    }
}

/// Helper function for serde default
fn default_true() -> bool {
    true
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_security_level_serde() {
        let level = SecurityLevel::Private;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"2\"");
        
        let deserialized: SecurityLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, SecurityLevel::Private);
    }
    
    #[test]
    fn test_permission_type_serde() {
        let ptype = PermissionType::Protocol;
        let json = serde_json::to_string(&ptype).unwrap();
        assert_eq!(json, "\"protocol\"");
        
        let deserialized: PermissionType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, PermissionType::Protocol);
    }
    
    #[test]
    fn test_config_defaults() {
        let config = PermissionsManagerConfig::default();
        assert!(config.seek_protocol_permissions_for_signing);
        assert!(config.seek_protocol_permissions_for_encrypting);
        assert!(config.seek_protocol_permissions_for_hmac);
        assert!(config.seek_protocol_permissions_for_key_derivation);
        assert!(config.seek_certificate_permissions_for_certificate_ops);
        assert!(config.seek_basket_permissions_for_basket_ops);
    }
    
    #[test]
    fn test_grouped_permissions_serde() {
        let permissions = GroupedPermissions {
            description: Some("Test permissions".to_string()),
            spending_authorization: Some(SpendingAuthorization {
                amount: 1000,
                description: "Test spending".to_string(),
            }),
            protocol_permissions: None,
            basket_access: None,
            certificate_access: None,
        };
        
        let json = serde_json::to_string(&permissions).unwrap();
        let deserialized: GroupedPermissions = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, permissions);
    }
}
