//! Core storage types and interfaces
//!
//! Translates TypeScript WalletStorage interface types to Rust.
//! Reference: wallet-toolbox/src/sdk/WalletStorage.interfaces.ts

use serde::{Deserialize, Serialize};
use crate::schema::tables::*;

/// Authentication identity
///
/// Matches TypeScript `AuthId` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthId {
    #[serde(rename = "identityKey")]
    pub identity_key: String,
    
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl Default for AuthId {
    fn default() -> Self {
        Self {
            identity_key: String::new(),
            user_id: None,
            is_active: None,
        }
    }
}

impl AuthId {
    pub fn new(identity_key: impl Into<String>) -> Self {
        Self {
            identity_key: identity_key.into(),
            user_id: None,
            is_active: None,
        }
    }

    pub fn with_user_id(mut self, user_id: i64) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
}

/// Pagination parameters with since timestamp
///
/// Matches TypeScript `FindSincePagedArgs` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindSincePagedArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>, // ISO 8601 timestamp
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paged: Option<Paged>,
    
    #[serde(rename = "orderDescending", skip_serializing_if = "Option::is_none")]
    pub order_descending: Option<bool>,
}

/// Find arguments with user filter
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindForUserSincePagedArgs {
    #[serde(rename = "userId")]
    pub user_id: i64,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paged: Option<Paged>,
    
    #[serde(rename = "orderDescending", skip_serializing_if = "Option::is_none")]
    pub order_descending: Option<bool>,
}

/// Find certificates arguments
/// Matches TypeScript FindCertificatesArgs from WalletStorage.interfaces.ts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindCertificatesArgs {
    #[serde(rename = "userId")]
    pub user_id: i64,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paged: Option<Paged>,
    
    #[serde(rename = "orderDescending", skip_serializing_if = "Option::is_none")]
    pub order_descending: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<PartialCertificate>,
    
    /// Filter by certifier public keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certifiers: Option<Vec<String>>,
    
    /// Filter by certificate types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    
    /// Include certificate fields in results
    #[serde(rename = "includeFields", skip_serializing_if = "Option::is_none")]
    pub include_fields: Option<bool>,
}

/// Partial certificate for filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialCertificate {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<String>,
    
    #[serde(rename = "serialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certifier: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

/// Partial output for filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialOutput {
    #[serde(rename = "basketId", skip_serializing_if = "Option::is_none")]
    pub basket_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spendable: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<bool>,
    
    #[serde(rename = "transactionId", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
}

/// Find output baskets arguments
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindOutputBasketsArgs {
    #[serde(rename = "userId")]
    pub user_id: i64,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paged: Option<Paged>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Find outputs arguments
/// Matches TypeScript FindOutputsArgs from WalletStorage.interfaces.ts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindOutputsArgs {
    #[serde(rename = "userId")]
    pub user_id: i64,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paged: Option<Paged>,
    
    #[serde(rename = "orderDescending", skip_serializing_if = "Option::is_none")]
    pub order_descending: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<PartialOutput>,
    
    /// Exclude lockingScript from results for performance
    #[serde(rename = "noScript", skip_serializing_if = "Option::is_none")]
    pub no_script: Option<bool>,
    
    /// Filter by transaction status
    #[serde(rename = "txStatus", skip_serializing_if = "Option::is_none")]
    pub tx_status: Option<Vec<TransactionStatus>>,
}

/// Find proven transaction requests arguments
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindProvenTxReqsArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ProvenTxReqStatus>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paged: Option<Paged>,
}

/// Proven or raw transaction result
/// Matches TypeScript `ProvenOrRawTx`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenOrRawTx {
    /// Proven transaction if found
    pub proven: Option<TableProvenTx>,
    
    /// Raw transaction bytes if found
    #[serde(rename = "rawTx", skip_serializing_if = "Option::is_none")]
    pub raw_tx: Option<Vec<u8>>,
    
    /// Input BEEF if found
    #[serde(rename = "inputBEEF", skip_serializing_if = "Option::is_none")]
    pub input_beef: Option<Vec<u8>>,
}

/// Output update fields
/// Used for partial updates to outputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputUpdates {
    /// Mark output as spendable/unspendable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spendable: Option<bool>,
    
    /// Transaction ID that spent this output
    #[serde(rename = "spentBy", skip_serializing_if = "Option::is_none")]
    pub spent_by: Option<i64>,
    
    /// Description of spending
    #[serde(rename = "spendingDescription", skip_serializing_if = "Option::is_none")]
    pub spending_description: Option<String>,
}

/// User insertion result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindOrInsertUserResult {
    pub user: TableUser,
    
    #[serde(rename = "isNew")]
    pub is_new: bool,
}

/// Sync state insertion result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindOrInsertSyncStateResult {
    #[serde(rename = "syncState")]
    pub sync_state: TableSyncState,
    
    #[serde(rename = "isNew")]
    pub is_new: bool,
}

/// Storage provider information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletStorageInfo {
    #[serde(rename = "isActive")]
    pub is_active: bool,
    
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    
    #[serde(rename = "isBackup")]
    pub is_backup: bool,
    
    #[serde(rename = "isConflicting")]
    pub is_conflicting: bool,
    
    #[serde(rename = "userId")]
    pub user_id: i64,
    
    #[serde(rename = "storageIdentityKey")]
    pub storage_identity_key: String,
    
    #[serde(rename = "storageName")]
    pub storage_name: String,
    
    #[serde(rename = "storageClass")]
    pub storage_class: String,
    
    #[serde(rename = "endpointURL", skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}

/// Paged type (re-exported for convenience)
pub use crate::schema::tables::TransactionStatus;
pub use crate::schema::tables::ProvenTxReqStatus;

/// Pagination (imported from tables or SDK types)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Paged {
    pub limit: u32,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

impl Paged {
    pub fn new(limit: u32) -> Self {
        Self { limit, offset: None }
    }

    pub fn with_offset(limit: u32, offset: u32) -> Self {
        Self {
            limit,
            offset: Some(offset),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_id() {
        let auth = AuthId::new("key123")
            .with_user_id(100)
            .with_is_active(true);
        
        assert_eq!(auth.identity_key, "key123");
        assert_eq!(auth.user_id, Some(100));
        assert_eq!(auth.is_active, Some(true));
    }

    #[test]
    fn test_auth_id_serialization() {
        let auth = AuthId::new("key").with_user_id(1);
        let json = serde_json::to_string(&auth).unwrap();
        
        assert!(json.contains("\"identityKey\":\"key\""));
        assert!(json.contains("\"userId\":1"));
        
        let deserialized: AuthId = serde_json::from_str(&json).unwrap();
        assert_eq!(auth, deserialized);
    }

    #[test]
    fn test_paged() {
        let paged = Paged::with_offset(20, 40);
        assert_eq!(paged.limit, 20);
        assert_eq!(paged.offset, Some(40));
    }
}
