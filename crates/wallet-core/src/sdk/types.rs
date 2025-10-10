//! Core type definitions for wallet operations
//! 
//! Translates TypeScript SDK types to Rust.
//! Reference: wallet-toolbox/src/sdk/types.ts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Core Types
// ============================================================================

/// Identifies a unique transaction output by its `txid` and index `vout`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OutPoint {
    /// Transaction double sha256 hash as big endian hex string
    pub txid: String,
    /// Zero based output index within the transaction
    pub vout: u32,
}

impl OutPoint {
    /// Create a new OutPoint
    pub fn new(txid: impl Into<String>, vout: u32) -> Self {
        Self {
            txid: txid.into(),
            vout,
        }
    }

    /// Format as string "txid:vout"
    pub fn to_string_format(&self) -> String {
        format!("{}:{}", self.txid, self.vout)
    }

    /// Parse from string format "txid:vout"
    pub fn from_string_format(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid outpoint format: {}", s));
        }
        let vout = parts[1]
            .parse::<u32>()
            .map_err(|e| format!("Invalid vout: {}", e))?;
        Ok(Self::new(parts[0], vout))
    }
}

/// Chain identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Chain {
    Main,
    Test,
}

impl Chain {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Chain::Main => "main",
            Chain::Test => "test",
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "main" => Ok(Chain::Main),
            "test" => Ok(Chain::Test),
            _ => Err(format!("Invalid chain: {}", s)),
        }
    }
}

impl std::fmt::Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================================
// Transaction Status Types
// ============================================================================

/// Status of a proven transaction request
/// 
/// Initial status (attempts === 0):
/// - `nosend`: transaction was marked 'noSend'. It is complete and signed.
/// - `unprocessed`: indicates req is about to be posted to network
/// - `unsent`: rawTx has not yet been sent to the network for processing
/// - `sending`: At least one attempt to send rawTx to transaction processors
/// - `unknown`: rawTx status is unknown but believed previously sent
/// 
/// Attempts > 0 status, processing:
/// - `unknown`: Last status update didn't recognize txid
/// - `nonfinal`: rawTx has un-expired nLockTime
/// - `unmined`: Last attempt has txid waiting to be mined
/// - `callback`: Waiting for proof confirmation callback
/// - `unconfirmed`: Potential proof not confirmed by chaintracks
/// 
/// Terminal status:
/// - `doubleSpend`: Transaction spends same input as another
/// - `invalid`: rawTx structurally invalid or rejected
/// - `completed`: proven_txs record added, notifications complete
/// - `unfail`: assigned to force review of invalid ProvenTxReq
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProvenTxReqStatus {
    Sending,
    Unsent,
    Nosend,
    Unknown,
    Nonfinal,
    Unprocessed,
    Unmined,
    Callback,
    Unconfirmed,
    Completed,
    Invalid,
    #[serde(rename = "doubleSpend")]
    DoubleSpend,
    Unfail,
}

impl ProvenTxReqStatus {
    /// Check if this is a terminal status
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            ProvenTxReqStatus::Completed
                | ProvenTxReqStatus::Invalid
                | ProvenTxReqStatus::DoubleSpend
        )
    }

    /// Check if this is a non-terminal status
    pub fn is_non_terminal(&self) -> bool {
        !self.is_terminal()
    }

    /// Get all terminal statuses
    pub fn terminal_statuses() -> &'static [ProvenTxReqStatus] {
        &[
            ProvenTxReqStatus::Completed,
            ProvenTxReqStatus::Invalid,
            ProvenTxReqStatus::DoubleSpend,
        ]
    }

    /// Get all non-terminal statuses
    pub fn non_terminal_statuses() -> &'static [ProvenTxReqStatus] {
        &[
            ProvenTxReqStatus::Sending,
            ProvenTxReqStatus::Unsent,
            ProvenTxReqStatus::Nosend,
            ProvenTxReqStatus::Unknown,
            ProvenTxReqStatus::Nonfinal,
            ProvenTxReqStatus::Unprocessed,
            ProvenTxReqStatus::Unmined,
            ProvenTxReqStatus::Callback,
            ProvenTxReqStatus::Unconfirmed,
        ]
    }

    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            ProvenTxReqStatus::Sending => "sending",
            ProvenTxReqStatus::Unsent => "unsent",
            ProvenTxReqStatus::Nosend => "nosend",
            ProvenTxReqStatus::Unknown => "unknown",
            ProvenTxReqStatus::Nonfinal => "nonfinal",
            ProvenTxReqStatus::Unprocessed => "unprocessed",
            ProvenTxReqStatus::Unmined => "unmined",
            ProvenTxReqStatus::Callback => "callback",
            ProvenTxReqStatus::Unconfirmed => "unconfirmed",
            ProvenTxReqStatus::Completed => "completed",
            ProvenTxReqStatus::Invalid => "invalid",
            ProvenTxReqStatus::DoubleSpend => "doubleSpend",
            ProvenTxReqStatus::Unfail => "unfail",
        }
    }
}

impl std::fmt::Display for ProvenTxReqStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionStatus {
    Completed,
    Failed,
    Unprocessed,
    Sending,
    Unproven,
    Unsigned,
    Nosend,
    Nonfinal,
    Unfail,
}

impl TransactionStatus {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionStatus::Completed => "completed",
            TransactionStatus::Failed => "failed",
            TransactionStatus::Unprocessed => "unprocessed",
            TransactionStatus::Sending => "sending",
            TransactionStatus::Unproven => "unproven",
            TransactionStatus::Unsigned => "unsigned",
            TransactionStatus::Nosend => "nosend",
            TransactionStatus::Nonfinal => "nonfinal",
            TransactionStatus::Unfail => "unfail",
        }
    }
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================================
// Data Structure Types
// ============================================================================

/// Pagination parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Paged {
    pub limit: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

impl Paged {
    /// Create new pagination parameters
    pub fn new(limit: u32) -> Self {
        Self {
            limit,
            offset: None,
        }
    }

    /// Create with offset
    pub fn with_offset(limit: u32, offset: u32) -> Self {
        Self {
            limit,
            offset: Some(offset),
        }
    }
}

/// Public/private key pair
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyPair {
    #[serde(rename = "privateKey")]
    pub private_key: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

impl KeyPair {
    /// Create a new key pair
    pub fn new(private_key: impl Into<String>, public_key: impl Into<String>) -> Self {
        Self {
            private_key: private_key.into(),
            public_key: public_key.into(),
        }
    }
}

/// Storage identity information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageIdentity {
    /// The identity key (public key) assigned to this storage
    #[serde(rename = "storageIdentityKey")]
    pub storage_identity_key: String,
    /// The human readable name assigned to this storage
    #[serde(rename = "storageName")]
    pub storage_name: String,
}

impl StorageIdentity {
    /// Create a new storage identity
    pub fn new(storage_identity_key: impl Into<String>, storage_name: impl Into<String>) -> Self {
        Self {
            storage_identity_key: storage_identity_key.into(),
            storage_name: storage_name.into(),
        }
    }
}

/// Entity timestamp information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityTimeStamp {
    pub created_at: String, // ISO 8601 datetime string
    pub updated_at: String, // ISO 8601 datetime string
}

impl EntityTimeStamp {
    /// Create a new timestamp with current time
    pub fn now() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
        }
    }

    /// Create with specific times
    pub fn new(created_at: impl Into<String>, updated_at: impl Into<String>) -> Self {
        Self {
            created_at: created_at.into(),
            updated_at: updated_at.into(),
        }
    }
}

/// Wallet balance information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletBalance {
    /// Total satoshis
    pub total: u64,
    /// Individual UTXOs with satoshi amounts and outpoints
    pub utxos: Vec<UtxoBalance>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UtxoBalance {
    pub satoshis: u64,
    pub outpoint: String,
}

impl WalletBalance {
    /// Create a new wallet balance
    pub fn new(total: u64, utxos: Vec<UtxoBalance>) -> Self {
        Self { total, utxos }
    }

    /// Create an empty balance
    pub fn empty() -> Self {
        Self {
            total: 0,
            utxos: Vec::new(),
        }
    }
}

/// Request history note for tracking operations
/// 
/// Flexible structure allowing arbitrary key-value pairs alongside required `what` field
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReqHistoryNote {
    /// Optional timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,
    /// Description of what happened
    pub what: String,
    /// Additional arbitrary fields
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ReqHistoryNote {
    /// Create a new history note
    pub fn new(what: impl Into<String>) -> Self {
        Self {
            when: None,
            what: what.into(),
            extra: HashMap::new(),
        }
    }

    /// Create with timestamp
    pub fn with_timestamp(what: impl Into<String>, when: impl Into<String>) -> Self {
        Self {
            when: Some(when.into()),
            what: what.into(),
            extra: HashMap::new(),
        }
    }

    /// Add an extra field
    pub fn with_field(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.extra.insert(key.into(), value);
        self
    }
}

// ============================================================================
// Special Operation Constants
// ============================================================================

/// `listOutputs` special operation basket name value.
/// 
/// Returns wallet's current change balance in the `totalOutputs` result property.
/// The `outputs` result property will always be an empty array.
pub const SPEC_OP_WALLET_BALANCE: &str =
    "893b7646de0e1c9f741bd6e9169b76a8847ae34adef7bef1e6a285371206d2e8";

/// `listOutputs` special operation basket name value.
/// 
/// Returns currently spendable wallet change outputs that fail to validate as unspent transaction outputs.
/// 
/// Optional tag value 'release'. If present, updates invalid change outputs to not spendable.
/// Optional tag value 'all'. If present, processes all spendable true outputs, independent of baskets.
pub const SPEC_OP_INVALID_CHANGE: &str =
    "5a76fd430a311f8bc0553859061710a4475c19fed46e2ff95969aa918e612e57";

/// `listOutputs` special operation basket name value.
/// 
/// Updates the wallet's automatic change management parameters.
/// 
/// Tag at index 0 is the new desired number of spendable change outputs to maintain.
/// Tag at index 1 is the new target for minimum satoshis when creating new change outputs.
pub const SPEC_OP_SET_WALLET_CHANGE_PARAMS: &str =
    "a4979d28ced8581e9c1c92f1001cc7cb3aabf8ea32e10888ad898f0a509a3929";

/// `listActions` special operation label name value.
/// 
/// Processes only actions currently with status 'nosend'
/// 
/// Optional label value 'abort'. If present, runs abortAction on all the actions returned.
pub const SPEC_OP_NO_SEND_ACTIONS: &str =
    "ac6b20a3bb320adafecd637b25c84b792ad828d3aa510d05dc841481f664277d";

/// `listActions` special operation label name value.
/// 
/// Processes only actions currently with status 'failed'
/// 
/// Optional label value 'unfail'. If present, sets status to 'unfail', which queues them for attempted recovery by the Monitor.
pub const SPEC_OP_FAILED_ACTIONS: &str =
    "97d4eb1e49215e3374cc2c1939a7c43a55e95c7427bf2d45ed63e3b4e0c88153";

/// `createAction` special operation label name value.
/// 
/// Causes WERR_REVIEW_ACTIONS throw with dummy properties.
pub const SPEC_OP_THROW_REVIEW_ACTIONS: &str =
    "a496e747fc3ad5fabdd4ae8f91184e71f87539bd3d962aa2548942faaaf0047a";

// ============================================================================
// Helper Functions
// ============================================================================

/// Check if the basket name is a reserved `listOutputs` special operation identifier
pub fn is_list_outputs_spec_op(basket: &str) -> bool {
    matches!(
        basket,
        SPEC_OP_WALLET_BALANCE | SPEC_OP_INVALID_CHANGE | SPEC_OP_SET_WALLET_CHANGE_PARAMS
    )
}

/// Check if the label name is a reserved `listActions` special operation identifier
pub fn is_list_actions_spec_op(label: &str) -> bool {
    matches!(label, SPEC_OP_NO_SEND_ACTIONS | SPEC_OP_FAILED_ACTIONS)
}

/// Check if the label name is a reserved `createAction` special operation identifier
pub fn is_create_action_spec_op(label: &str) -> bool {
    matches!(label, SPEC_OP_THROW_REVIEW_ACTIONS)
}

// ============================================================================
// Storage-Related Types
// ============================================================================

/// Storage provider type - who provided the input/output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StorageProvidedBy {
    You,
    Storage,
    YouAndStorage,
}

impl StorageProvidedBy {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageProvidedBy::You => "you",
            StorageProvidedBy::Storage => "storage",
            StorageProvidedBy::YouAndStorage => "you-and-storage",
        }
    }
}

impl std::fmt::Display for StorageProvidedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Sync status - state of storage synchronization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SyncStatus {
    Success,
    Error,
    Identified,
    Updated,
    Unknown,
}

impl SyncStatus {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            SyncStatus::Success => "success",
            SyncStatus::Error => "error",
            SyncStatus::Identified => "identified",
            SyncStatus::Updated => "updated",
            SyncStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
