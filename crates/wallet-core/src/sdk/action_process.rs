//! Action processing types - Sign, Internalize, Abort, Process
//!
//! Translates TypeScript action processing interfaces from @wallet-toolbox
//! Reference: src/sdk/validationHelpers.ts, src/sdk/WalletStorage.interfaces.ts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sign action spend information
/// Matches TypeScript `SignActionSpend`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignActionSpend {
    /// Unlocking script for this input
    #[serde(rename = "unlockingScript")]
    pub unlocking_script: String,
    
    /// Sequence number
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: u32,
}

/// Sign action options
/// Matches TypeScript `ValidSignActionOptions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidSignActionOptions {
    /// Accept delayed broadcast (default true)
    #[serde(rename = "acceptDelayedBroadcast")]
    pub accept_delayed_broadcast: bool,
    
    /// Return only txid (default false)
    #[serde(rename = "returnTXIDOnly")]
    pub return_txid_only: bool,
    
    /// Don't broadcast (default false)
    #[serde(rename = "noSend")]
    pub no_send: bool,
    
    /// TXIDs to broadcast with
    #[serde(rename = "sendWith")]
    pub send_with: Vec<String>,
}

impl Default for ValidSignActionOptions {
    fn default() -> Self {
        Self {
            accept_delayed_broadcast: true,
            return_txid_only: false,
            no_send: false,
            send_with: Vec::new(),
        }
    }
}

/// Sign action arguments
/// Matches TypeScript `ValidSignActionArgs`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidSignActionArgs {
    /// Map of input index to spend info
    pub spends: HashMap<u32, SignActionSpend>,
    
    /// Action reference (Base64)
    pub reference: String,
    
    /// Signing options
    pub options: ValidSignActionOptions,
    
    /// Is sending with other transactions
    #[serde(rename = "isSendWith")]
    pub is_send_with: bool,
    
    /// Is delayed broadcast
    #[serde(rename = "isDelayed")]
    pub is_delayed: bool,
    
    /// Is no-send mode
    #[serde(rename = "isNoSend")]
    pub is_no_send: bool,
    
    /// Is new transaction
    #[serde(rename = "isNewTx")]
    pub is_new_tx: bool,
    
    /// Is remix change
    #[serde(rename = "isRemixChange")]
    pub is_remix_change: bool,
}

/// Wallet payment remittance info
/// Matches TypeScript `ValidWalletPayment`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidWalletPayment {
    /// Derivation prefix (Base64)
    #[serde(rename = "derivationPrefix")]
    pub derivation_prefix: String,
    
    /// Derivation suffix (Base64)
    #[serde(rename = "derivationSuffix")]
    pub derivation_suffix: String,
    
    /// Sender identity key (PubKey hex)
    #[serde(rename = "senderIdentityKey")]
    pub sender_identity_key: String,
}

/// Basket insertion remittance info
/// Matches TypeScript `ValidBasketInsertion`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidBasketInsertion {
    /// Basket name
    pub basket: String,
    
    /// Custom instructions
    #[serde(rename = "customInstructions", skip_serializing_if = "Option::is_none")]
    pub custom_instructions: Option<String>,
    
    /// Tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Output protocol type for internalization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InternalizeProtocol {
    #[serde(rename = "wallet payment")]
    WalletPayment,
    #[serde(rename = "basket insertion")]
    BasketInsertion,
}

/// Internalize output specification
/// Matches TypeScript `ValidInternalizeOutput`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidInternalizeOutput {
    /// Output index in transaction
    #[serde(rename = "outputIndex")]
    pub output_index: u32,
    
    /// Protocol type
    pub protocol: InternalizeProtocol,
    
    /// Payment remittance (if protocol = wallet payment)
    #[serde(rename = "paymentRemittance", skip_serializing_if = "Option::is_none")]
    pub payment_remittance: Option<ValidWalletPayment>,
    
    /// Insertion remittance (if protocol = basket insertion)
    #[serde(rename = "insertionRemittance", skip_serializing_if = "Option::is_none")]
    pub insertion_remittance: Option<ValidBasketInsertion>,
}

/// Internalize action arguments
/// Matches TypeScript `ValidInternalizeActionArgs`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidInternalizeActionArgs {
    /// Transaction as AtomicBEEF
    pub tx: Vec<u8>, // AtomicBEEF is BEEF bytes
    
    /// Outputs to internalize
    pub outputs: Vec<ValidInternalizeOutput>,
    
    /// Description (5-2000 bytes)
    pub description: String,
    
    /// Labels for this action
    pub labels: Vec<String>,
    
    /// Seek permission before internalizing
    #[serde(rename = "seekPermission")]
    pub seek_permission: bool,
}

/// Abort action arguments
/// Matches TypeScript `ValidAbortActionArgs`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidAbortActionArgs {
    /// Action reference (Base64)
    pub reference: String,
}

/// Valid process action arguments (validated)
/// Matches TypeScript `ValidProcessActionArgs`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidProcessActionArgs {
    /// Options for processing
    pub options: ValidSignActionOptions,
}

/// Storage process action arguments
/// Matches TypeScript `StorageProcessActionArgs`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageProcessActionArgs {
    /// Is new transaction
    #[serde(rename = "isNewTx")]
    pub is_new_tx: bool,
    
    /// Is sending with others
    #[serde(rename = "isSendWith")]
    pub is_send_with: bool,
    
    /// Is no-send mode
    #[serde(rename = "isNoSend")]
    pub is_no_send: bool,
    
    /// Is delayed broadcast
    #[serde(rename = "isDelayed")]
    pub is_delayed: bool,
    
    /// Optional action reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    
    /// Optional txid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    
    /// Optional raw transaction
    #[serde(rename = "rawTx", skip_serializing_if = "Option::is_none")]
    pub raw_tx: Option<Vec<u8>>,
    
    /// Transactions to send with
    #[serde(rename = "sendWith")]
    pub send_with: Vec<String>,
    
    /// Optional processing log
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
}

/// Review action result status
/// Matches TypeScript `ReviewActionResultStatus`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReviewActionResultStatus {
    Success,
    DoubleSpend,
    ServiceError,
    InvalidTx,
}

/// Review action result
/// Matches TypeScript `ReviewActionResult`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewActionResult {
    /// Transaction ID
    pub txid: String,
    
    /// Result status
    pub status: ReviewActionResultStatus,
    
    /// Optional error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    
    /// Optional competing BEEF (for double spend)
    #[serde(rename = "competingBeef", skip_serializing_if = "Option::is_none")]
    pub competing_beef: Option<Vec<u8>>,
}

/// Send with result
/// Matches SDK `SendWithResult`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendWithResult {
    /// Transaction ID
    pub txid: String,
    
    /// Result status
    pub status: String,
}

/// Storage process action results
/// Matches TypeScript `StorageProcessActionResults`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageProcessActionResults {
    /// Results from sendWith transactions
    #[serde(rename = "sendWithResults", skip_serializing_if = "Option::is_none")]
    pub send_with_results: Option<Vec<SendWithResult>>,
    
    /// Results from non-delayed transactions
    #[serde(rename = "notDelayedResults", skip_serializing_if = "Option::is_none")]
    pub not_delayed_results: Option<Vec<ReviewActionResult>>,
    
    /// Optional processing log
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
}

/// Storage internalize action result
/// Matches TypeScript `StorageInternalizeActionResult`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInternalizeActionResult {
    /// Is merging with existing transaction
    #[serde(rename = "isMerge")]
    pub is_merge: bool,
    
    /// Transaction ID
    pub txid: String,
    
    /// Net satoshis change for user
    pub satoshis: i64,
    
    /// Send with results (if not merge)
    #[serde(rename = "sendWithResults", skip_serializing_if = "Option::is_none")]
    pub send_with_results: Option<Vec<SendWithResult>>,
    
    /// Not delayed results (if not merge)
    #[serde(rename = "notDelayedResults", skip_serializing_if = "Option::is_none")]
    pub not_delayed_results: Option<Vec<ReviewActionResult>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_action_options_default() {
        let opts = ValidSignActionOptions::default();
        assert!(opts.accept_delayed_broadcast);
        assert!(!opts.return_txid_only);
        assert!(!opts.no_send);
    }

    #[test]
    fn test_internalize_protocol_serde() {
        let json = serde_json::to_string(&InternalizeProtocol::WalletPayment).unwrap();
        assert_eq!(json, "\"wallet payment\"");
        
        let proto: InternalizeProtocol = serde_json::from_str("\"basket insertion\"").unwrap();
        assert_eq!(proto, InternalizeProtocol::BasketInsertion);
    }

    #[test]
    fn test_review_status_serde() {
        let status = ReviewActionResultStatus::Success;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"success\"");
        
        let status: ReviewActionResultStatus = serde_json::from_str("\"doubleSpend\"").unwrap();
        assert_eq!(status, ReviewActionResultStatus::DoubleSpend);
    }

    #[test]
    fn test_abort_action_args() {
        let args = ValidAbortActionArgs {
            reference: "abc123base64".to_string(),
        };
        assert_eq!(args.reference, "abc123base64");
    }

    #[test]
    fn test_wallet_payment() {
        let payment = ValidWalletPayment {
            derivation_prefix: "prefix_b64".to_string(),
            derivation_suffix: "suffix_b64".to_string(),
            sender_identity_key: "03abc123".to_string(),
        };
        let json = serde_json::to_string(&payment).unwrap();
        assert!(json.contains("derivationPrefix"));
    }
}
