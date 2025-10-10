//! Action interfaces - Core wallet transaction operations
//!
//! Translates TypeScript action interfaces from @wallet-toolbox
//! Reference: src/sdk/WalletStorage.interfaces.ts, src/sdk/validationHelpers.ts

use serde::{Deserialize, Serialize};

/// Outpoint reference (txid + vout)
/// Matches SDK OutPoint
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutPoint {
    pub txid: String, // TXIDHexString
    pub vout: u32,    // PositiveIntegerOrZero
}

/// Input for creating an action
/// Matches TypeScript `ValidCreateActionInput`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidCreateActionInput {
    /// Output being spent
    pub outpoint: OutPoint,
    
    /// Description of this input (5-2000 bytes)
    #[serde(rename = "inputDescription")]
    pub input_description: String,
    
    /// Sequence number for this input
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: u32,
    
    /// Unlocking script template or pre-built script
    #[serde(rename = "unlockingScript", skip_serializing_if = "Option::is_none")]
    pub unlocking_script: Option<String>,
    
    /// Unlocking script length estimate for fee calculation
    #[serde(rename = "unlockingScriptLength", skip_serializing_if = "Option::is_none")]
    pub unlocking_script_length: Option<u32>,
    
    /// Source output satoshis
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satoshis: Option<i64>,
    
    /// Source locking script
    #[serde(rename = "lockingScript", skip_serializing_if = "Option::is_none")]
    pub locking_script: Option<String>,
}

/// Output for creating an action
/// Matches TypeScript `ValidCreateActionOutput`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidCreateActionOutput {
    /// Locking script for this output
    #[serde(rename = "lockingScript")]
    pub locking_script: String, // HexString
    
    /// Amount in satoshis
    pub satoshis: i64, // SatoshiValue
    
    /// Description of this output (5-2000 bytes)
    #[serde(rename = "outputDescription")]
    pub output_description: String,
    
    /// Optional custom instructions JSON string
    #[serde(rename = "customInstructions", skip_serializing_if = "Option::is_none")]
    pub custom_instructions: Option<String>,
    
    /// Optional basket assignment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basket: Option<String>,
    
    /// Optional tags for this output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Process action options
/// Matches TypeScript `ValidProcessActionOptions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidProcessActionOptions {
    /// Accept delayed broadcast (default true)
    #[serde(rename = "acceptDelayedBroadcast")]
    pub accept_delayed_broadcast: bool,
    
    /// Return only txid without full result (default false)
    #[serde(rename = "returnTXIDOnly")]
    pub return_txid_only: bool,
    
    /// Don't broadcast, just sign (default false)
    #[serde(rename = "noSend")]
    pub no_send: bool,
    
    /// TXIDs to broadcast together with this transaction
    #[serde(rename = "sendWith")]
    pub send_with: Vec<String>,
}

impl Default for ValidProcessActionOptions {
    fn default() -> Self {
        Self {
            accept_delayed_broadcast: true,
            return_txid_only: false,
            no_send: false,
            send_with: Vec::new(),
        }
    }
}

/// Create action options
/// Matches TypeScript `ValidCreateActionOptions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidCreateActionOptions {
    #[serde(flatten)]
    pub process_options: ValidProcessActionOptions,
    
    /// Sign and process immediately (vs just create)
    #[serde(rename = "signAndProcess")]
    pub sign_and_process: bool,
    
    /// Trust level for self-signed transactions
    #[serde(rename = "trustSelf", skip_serializing_if = "Option::is_none")]
    pub trust_self: Option<String>, // TrustSelf enum
    
    /// Known txids to validate against
    #[serde(rename = "knownTxids", default)]
    pub known_txids: Vec<String>,
    
    /// Version number for transaction
    pub version: u32,
    
    /// Lock time for transaction
    #[serde(rename = "lockTime")]
    pub lock_time: u32,
    
    /// Randomize output order for privacy
    #[serde(rename = "randomizeOutputs")]
    pub randomize_outputs: bool,
    
    /// No send change outpoints
    #[serde(rename = "noSendChange", skip_serializing_if = "Option::is_none")]
    pub no_send_change: Option<Vec<OutPoint>>,
    
    /// Return only TXID
    #[serde(rename = "returnTXIDOnly", default)]
    pub return_txid_only: bool,
}

impl Default for ValidCreateActionOptions {
    fn default() -> Self {
        Self {
            process_options: ValidProcessActionOptions::default(),
            sign_and_process: true,
            trust_self: None,
            known_txids: Vec::new(),
            version: 1,
            lock_time: 0,
            randomize_outputs: true,
            no_send_change: None,
            return_txid_only: false,
        }
    }
}

/// Create action arguments
/// Matches TypeScript `ValidCreateActionArgs`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidCreateActionArgs {
    /// Description of this action (5-2000 bytes)
    pub description: String,
    
    /// Optional input BEEF for external inputs
    #[serde(rename = "inputBEEF", skip_serializing_if = "Option::is_none")]
    pub input_beef: Option<Vec<u8>>,
    
    /// Transaction inputs
    pub inputs: Vec<ValidCreateActionInput>,
    
    /// Transaction outputs
    pub outputs: Vec<ValidCreateActionOutput>,
    
    /// Optional labels for this action
    #[serde(default)]
    pub labels: Vec<String>,
    
    /// Creation options
    pub options: ValidCreateActionOptions,
    
    /// Is this a new transaction (vs existing)
    #[serde(rename = "isNewTx")]
    pub is_new_tx: bool,
    
    /// Is this a delayed broadcast
    #[serde(rename = "isDelayed")]
    pub is_delayed: bool,
    
    /// Is this a no-send transaction
    #[serde(rename = "isNoSend")]
    pub is_no_send: bool,
    
    /// Is this a sign action
    #[serde(rename = "isSignAction")]
    pub is_sign_action: bool,
    
    /// Transaction version
    pub version: u32,
    
    /// Transaction lock time
    #[serde(rename = "lockTime")]
    pub lock_time: u32,
    
    /// Random values for deterministic testing
    #[serde(rename = "randomVals", skip_serializing_if = "Option::is_none")]
    pub random_vals: Option<Vec<f64>>,
    
    /// Include all source transactions
    #[serde(rename = "includeAllSourceTransactions", default)]
    pub include_all_source_transactions: bool,
}

/// Storage-level create action result
/// Matches TypeScript `StorageCreateActionResult`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCreateActionResult {
    /// Optional input BEEF
    #[serde(rename = "inputBeef", skip_serializing_if = "Option::is_none")]
    pub input_beef: Option<Vec<u8>>,
    
    /// Processed inputs with details
    pub inputs: Vec<StorageCreateTransactionInput>,
    
    /// Processed outputs with details
    pub outputs: Vec<StorageCreateTransactionOutput>,
    
    /// Vouts that should not send change
    #[serde(rename = "noSendChangeOutputVouts", skip_serializing_if = "Option::is_none")]
    pub no_send_change_output_vouts: Option<Vec<u32>>,
    
    /// Derivation prefix for keys
    #[serde(rename = "derivationPrefix")]
    pub derivation_prefix: String,
    
    /// Transaction version
    pub version: u32,
    
    /// Transaction lock time
    #[serde(rename = "lockTime")]
    pub lock_time: u32,
    
    /// Action reference ID
    pub reference: String,
}

/// Storage provider type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StorageProvidedBy {
    You,
    Storage,
    YouAndStorage,
}

/// Transaction input details
/// Matches TypeScript `StorageCreateTransactionSdkInput`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCreateTransactionInput {
    /// Input index
    pub vin: u32,
    
    /// Source transaction ID
    #[serde(rename = "sourceTxid")]
    pub source_txid: String,
    
    /// Source output index
    #[serde(rename = "sourceVout")]
    pub source_vout: u32,
    
    /// Source output satoshis
    #[serde(rename = "sourceSatoshis")]
    pub source_satoshis: i64,
    
    /// Source locking script
    #[serde(rename = "sourceLockingScript")]
    pub source_locking_script: String,
    
    /// Optional source transaction bytes
    #[serde(rename = "sourceTransaction", skip_serializing_if = "Option::is_none")]
    pub source_transaction: Option<Vec<u8>>,
    
    /// Unlocking script length
    #[serde(rename = "unlockingScriptLength")]
    pub unlocking_script_length: u32,
    
    /// Who provided this input
    #[serde(rename = "providedBy")]
    pub provided_by: StorageProvidedBy,
    
    /// Input type
    #[serde(rename = "type")]
    pub input_type: String,
    
    /// Spending description
    #[serde(rename = "spendingDescription", skip_serializing_if = "Option::is_none")]
    pub spending_description: Option<String>,
    
    /// Derivation prefix
    #[serde(rename = "derivationPrefix", skip_serializing_if = "Option::is_none")]
    pub derivation_prefix: Option<String>,
    
    /// Derivation suffix
    #[serde(rename = "derivationSuffix", skip_serializing_if = "Option::is_none")]
    pub derivation_suffix: Option<String>,
    
    /// Sender identity key
    #[serde(rename = "senderIdentityKey", skip_serializing_if = "Option::is_none")]
    pub sender_identity_key: Option<String>,
}

/// Transaction output details
/// Matches TypeScript `StorageCreateTransactionSdkOutput`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCreateTransactionOutput {
    /// Output index
    pub vout: u32,
    
    /// Who provided this output
    #[serde(rename = "providedBy")]
    pub provided_by: StorageProvidedBy,
    
    /// Optional purpose
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    
    /// Derivation suffix
    #[serde(rename = "derivationSuffix", skip_serializing_if = "Option::is_none")]
    pub derivation_suffix: Option<String>,
    
    /// Inherits from ValidCreateActionOutput
    #[serde(rename = "lockingScript")]
    pub locking_script: String,
    
    pub satoshis: i64,
    
    #[serde(rename = "outputDescription")]
    pub output_description: String,
    
    #[serde(rename = "customInstructions", skip_serializing_if = "Option::is_none")]
    pub custom_instructions: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basket: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outpoint() {
        let op = OutPoint {
            txid: "abc123".to_string(),
            vout: 0,
        };
        assert_eq!(op.vout, 0);
    }

    #[test]
    fn test_storage_provided_by_serde() {
        let json = serde_json::to_string(&StorageProvidedBy::You).unwrap();
        assert_eq!(json, "\"you\"");
        
        let val: StorageProvidedBy = serde_json::from_str("\"you-and-storage\"").unwrap();
        assert_eq!(val, StorageProvidedBy::YouAndStorage);
    }

    #[test]
    fn test_process_options_default() {
        let opts = ValidProcessActionOptions::default();
        assert!(opts.accept_delayed_broadcast);
        assert!(!opts.return_txid_only);
        assert!(!opts.no_send);
        assert_eq!(opts.send_with.len(), 0);
    }
}
