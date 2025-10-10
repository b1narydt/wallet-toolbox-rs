//! List action types - ListActions, ListOutputs, ListCertificates
//!
//! Translates TypeScript list/query interfaces from @wallet-toolbox
//! Reference: src/sdk/validationHelpers.ts lines 926-975

use serde::{Deserialize, Serialize};

/// Label query mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LabelQueryMode {
    Any,
    All,
}

/// Tag query mode  
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TagQueryMode {
    Any,
    All,
}

/// List actions arguments
/// Matches TypeScript `ValidListActionsArgs`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidListActionsArgs {
    /// Labels to filter by
    pub labels: Vec<String>, // LabelStringUnder300Bytes
    
    /// How to match labels (any or all)
    #[serde(rename = "labelQueryMode")]
    pub label_query_mode: LabelQueryMode,
    
    /// Include labels in results (default false)
    #[serde(rename = "includeLabels")]
    pub include_labels: bool,
    
    /// Include input details (default false)
    #[serde(rename = "includeInputs")]
    pub include_inputs: bool,
    
    /// Include input source locking scripts (default false)
    #[serde(rename = "includeInputSourceLockingScripts")]
    pub include_input_source_locking_scripts: bool,
    
    /// Include input unlocking scripts (default false)
    #[serde(rename = "includeInputUnlockingScripts")]
    pub include_input_unlocking_scripts: bool,
    
    /// Include output details (default false)
    #[serde(rename = "includeOutputs")]
    pub include_outputs: bool,
    
    /// Include output locking scripts (default false)
    #[serde(rename = "includeOutputLockingScripts")]
    pub include_output_locking_scripts: bool,
    
    /// Maximum results (default 10, max 10000)
    pub limit: u32,
    
    /// Results offset (default 0)
    pub offset: u32,
    
    /// Seek permission (default true)
    #[serde(rename = "seekPermission")]
    pub seek_permission: bool,
}

impl Default for ValidListActionsArgs {
    fn default() -> Self {
        Self {
            labels: Vec::new(),
            label_query_mode: LabelQueryMode::Any,
            include_labels: false,
            include_inputs: false,
            include_input_source_locking_scripts: false,
            include_input_unlocking_scripts: false,
            include_outputs: false,
            include_output_locking_scripts: false,
            limit: 10,
            offset: 0,
            seek_permission: true,
        }
    }
}

/// List outputs arguments
/// Matches TypeScript `ValidListOutputsArgs`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidListOutputsArgs {
    /// Basket name (1-300 bytes)
    pub basket: String, // BasketStringUnder300Bytes
    
    /// Tags to filter by
    pub tags: Vec<String>, // OutputTagStringUnder300Bytes
    
    /// How to match tags (any or all)
    #[serde(rename = "tagQueryMode")]
    pub tag_query_mode: TagQueryMode,
    
    /// Include entire transactions (default false)
    #[serde(rename = "includeEntireTransactions")]
    pub include_entire_transactions: bool,
    
    /// Include custom instructions (default false)
    #[serde(rename = "includeCustomInstructions")]
    pub include_custom_instructions: bool,
    
    /// Include tags in results (default false)
    #[serde(rename = "includeTags")]
    pub include_tags: bool,
    
    /// Include labels in results (default false)
    #[serde(rename = "includeLabels")]
    pub include_labels: bool,
    
    /// Include locking scripts (default true)
    #[serde(rename = "includeLockingScripts")]
    pub include_locking_scripts: bool,
    
    /// Maximum results (default 10, max 10000)
    pub limit: u32,
    
    /// Results offset (default 0)
    pub offset: u32,
    
    /// Seek permission (default true)
    #[serde(rename = "seekPermission")]
    pub seek_permission: bool,
}

impl Default for ValidListOutputsArgs {
    fn default() -> Self {
        Self {
            basket: String::new(),
            tags: Vec::new(),
            tag_query_mode: TagQueryMode::Any,
            include_entire_transactions: false,
            include_custom_instructions: false,
            include_tags: false,
            include_labels: false,
            include_locking_scripts: true,
            limit: 10,
            offset: 0,
            seek_permission: true,
        }
    }
}

/// Partial certificate for filtering
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PartialCertificateFilter {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<String>, // Base64String
    
    #[serde(rename = "serialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>, // Base64String
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certifier: Option<String>, // PubKeyHex
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>, // PubKeyHex
}

/// List certificates arguments
/// Matches TypeScript `ValidListCertificatesArgs`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidListCertificatesArgs {
    /// Certifier public keys to filter by
    pub certifiers: Vec<String>, // PubKeyHex[]
    
    /// Certificate types to filter by
    pub types: Vec<String>, // Base64String[]
    
    /// Maximum results (default 10, max 10000)
    pub limit: u32,
    
    /// Results offset (default 0)
    pub offset: u32,
    
    /// Optional partial filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<PartialCertificateFilter>,
    
    /// Include fields in results (default false)
    #[serde(rename = "includeFields")]
    pub include_fields: bool,
    
    /// Seek permission (default true)
    #[serde(rename = "seekPermission")]
    pub seek_permission: bool,
    
    /// Privileged access reason (if needed)
    #[serde(rename = "privilegedReason", skip_serializing_if = "Option::is_none")]
    pub privileged_reason: Option<String>, // DescriptionString5to50Bytes
}

impl Default for ValidListCertificatesArgs {
    fn default() -> Self {
        Self {
            certifiers: Vec::new(),
            types: Vec::new(),
            limit: 10,
            offset: 0,
            partial: None,
            include_fields: false,
            seek_permission: true,
            privileged_reason: None,
        }
    }
}

// Note: RelinquishOutputArgs is defined in wallet_interface.rs
// to avoid duplication with the main SDK type

/// Relinquish certificate arguments
/// Matches SDK `RelinquishCertificateArgs`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelinquishCertificateArgs {
    #[serde(rename = "type")]
    pub certificate_type: String,
    
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    
    pub certifier: String,
}

/// Wallet output result
/// Matches SDK `WalletOutput`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletOutput {
    /// Output point (txid.vout format)
    pub outpoint: String,
    
    /// Satoshi value
    pub satoshis: i64,
    
    /// Whether output is spendable
    pub spendable: bool,
    
    /// Optional custom instructions
    #[serde(rename = "customInstructions", skip_serializing_if = "Option::is_none")]
    pub custom_instructions: Option<String>,
    
    /// Optional locking script (hex)
    #[serde(rename = "lockingScript", skip_serializing_if = "Option::is_none")]
    pub locking_script: Option<String>,
    
    /// Optional tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    
    /// Optional labels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
}

/// Wallet action result
/// Matches SDK `WalletAction`  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAction {
    /// Transaction ID
    pub txid: Option<String>,
    
    /// Satoshi amount
    pub satoshis: Option<i64>,
    
    /// Transaction status
    pub status: String,
    
    /// Whether outgoing
    #[serde(rename = "isOutgoing")]
    pub is_outgoing: bool,
    
    /// Description
    pub description: String,
    
    /// Optional labels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    
    /// Transaction version
    pub version: i32,
    
    /// Lock time
    #[serde(rename = "lockTime")]
    pub lock_time: u32,
    
    /// Optional inputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<serde_json::Value>>,
    
    /// Optional outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<serde_json::Value>>,
}

/// Find outputs args (for internal queries)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FindOutputsArgs {
    /// Basket filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basket: Option<String>,
    
    /// Tags filter
    #[serde(default)]
    pub tags: Vec<String>,
    
    /// Tag query mode
    #[serde(rename = "tagQueryMode", skip_serializing_if = "Option::is_none")]
    pub tag_query_mode: Option<TagQueryMode>,
    
    /// Limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    
    /// Offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    
    /// Spendable filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spendable: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_actions_default() {
        let args = ValidListActionsArgs::default();
        assert_eq!(args.limit, 10);
        assert_eq!(args.offset, 0);
        assert!(args.seek_permission);
        assert!(!args.include_labels);
        assert_eq!(args.label_query_mode, LabelQueryMode::Any);
    }

    #[test]
    fn test_list_outputs_default() {
        let args = ValidListOutputsArgs::default();
        assert_eq!(args.limit, 10);
        assert!(args.include_locking_scripts);
        assert!(!args.include_tags);
        assert_eq!(args.tag_query_mode, TagQueryMode::Any);
    }

    #[test]
    fn test_list_certificates_default() {
        let args = ValidListCertificatesArgs::default();
        assert_eq!(args.limit, 10);
        assert!(!args.include_fields);
        assert!(args.seek_permission);
    }

    #[test]
    fn test_query_mode_serde() {
        let mode = LabelQueryMode::All;
        let json = serde_json::to_string(&mode).unwrap();
        assert_eq!(json, "\"all\"");
        
        let mode: TagQueryMode = serde_json::from_str("\"any\"").unwrap();
        assert_eq!(mode, TagQueryMode::Any);
    }

    #[test]
    fn test_relinquish_output() {
        let args = RelinquishOutputArgs {
            basket: "savings".to_string(),
            output: 0,
        };
        let json = serde_json::to_string(&args).unwrap();
        assert!(json.contains("savings"));
    }
}
