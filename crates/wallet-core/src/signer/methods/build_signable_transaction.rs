//! Build Signable Transaction
//!
//! **Reference**: TypeScript `src/signer/methods/buildSignableTransaction.ts`
//!
//! Builds a transaction ready for signing from a createAction result

use crate::sdk::{StorageCreateActionResult, ValidCreateActionArgs};
use crate::sdk::errors::{WalletError, WalletResult};
use crate::transaction::Transaction;
use crate::keys::KeyPair;
use crate::utility::ScriptTemplateSABPPP;
use serde::{Deserialize, Serialize};

/// Pending storage input needing signature
///
/// Reference: TS PendingStorageInput (Wallet.ts lines 1060-1067)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingStorageInput {
    /// Input index in transaction
    pub vin: u32,
    
    /// BRC-42 derivation prefix
    pub derivation_prefix: String,
    
    /// BRC-43 derivation suffix
    pub derivation_suffix: String,
    
    /// Unlocker public key (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlocker_pub_key: Option<String>,
    
    /// Source output satoshis
    pub source_satoshis: u64,
    
    /// Source locking script
    pub locking_script: String,
}

/// Result of building a signable transaction
///
/// Reference: TS buildSignableTransaction return type (lines 18-23)
#[derive(Debug, Clone)]
pub struct BuildSignableTransactionResult {
    /// The transaction ready for signing
    pub tx: Transaction,
    
    /// Amount (change inputs - change outputs)
    pub amount: i64,
    
    /// Pending storage inputs needing signatures
    pub pdi: Vec<PendingStorageInput>,
    
    /// Log message
    pub log: String,
}

/// Build a signable transaction from createAction result
///
/// Reference: TS buildSignableTransaction (buildSignableTransaction.ts lines 14-163)
///
/// # Arguments
/// * `dctr` - Storage create action result from createAction
/// * `args` - Validated create action arguments
/// * `change_keys` - Client change key pair
/// * `input_beef` - Optional input BEEF data
///
/// # Returns
/// Transaction ready for signing with pending inputs
pub fn build_signable_transaction(
    dctr: &StorageCreateActionResult,
    args: &ValidCreateActionArgs,
    change_keys: &KeyPair,
    input_beef: Option<&[u8]>,
) -> WalletResult<BuildSignableTransactionResult> {
    // Create base transaction (TS line 30)
    let mut tx = Transaction::with_params(
        args.version,
        vec![],
        vec![],
        args.lock_time,
    );
    
    let storage_outputs = &dctr.outputs;
    let storage_inputs = &dctr.inputs;
    
    // Build vout to index mapping (TS lines 37-42)
    // The order of outputs in storageOutputs is always:
    // CreateActionArgs.outputs in the original order
    // Commission output
    // Change outputs
    // The Vout values will be randomized if args.options.randomizeOutputs is true (default true)
    let mut vout_to_index = vec![0usize; storage_outputs.len()];
    for vout in 0..storage_outputs.len() {
        let index = storage_outputs.iter()
            .position(|o| o.vout as usize == vout)
            .ok_or_else(|| WalletError::invalid_parameter(
                "output.vout",
                &format!("sequential. {} is missing", vout)
            ))?;
        vout_to_index[vout] = index;
    }
    
    //////////////
    // Add OUTPUTS (TS lines 47-65)
    /////////////
    for vout in 0..storage_outputs.len() {
        let i = vout_to_index[vout];
        let out = &storage_outputs[i];
        
        // Validate vout matches (TS lines 50-51)
        if vout != out.vout as usize {
            return Err(WalletError::invalid_parameter(
                "output.vout",
                &format!("equal to array index. {} !== {}", out.vout, vout)
            ));
        }
        
        // Determine if this is a change output (TS line 53)
        let is_change = matches!(out.provided_by, crate::sdk::action::StorageProvidedBy::Storage)
            && out.purpose.as_deref() == Some("change");
        
        // Get locking script (TS lines 55-57)
        let locking_script = if is_change {
            // TODO: Call makeChangeLock when BRC-29 is available (TS line 56)
            if let Some(ds) = &out.derivation_suffix {
                format!("changeLock_{}", ds)
            } else {
                return Err(WalletError::invalid_parameter(
                    "derivationSuffix",
                    "required for change outputs"
                ));
            }
        } else {
            out.locking_script.clone()
        };
        
        // Add output to transaction (TS lines 59-64)
        let tx_output = crate::transaction::TxOutput {
            value: out.satoshis,
            script_pubkey: hex::decode(&locking_script).unwrap_or_default(),
        };
        tx.add_output(tx_output);
    }
    
    // Add dummy output if no outputs (TS lines 67-75)
    if storage_outputs.is_empty() {
        // Add a dummy output to avoid transaction rejection by processors for having no outputs.
        // OP_FALSE OP_RETURN followed by data (TS line 71)
        let dummy_script = vec![0x00, 0x6a, 0x01, 0x2a]; // OP_FALSE OP_RETURN <1 byte: 0x2a>
        let tx_output = crate::transaction::TxOutput {
            value: 0,
            script_pubkey: dummy_script,
        };
        tx.add_output(tx_output);
    }
    
    //////////////
    // Merge and sort INPUTS info by vin order (TS lines 77-93)
    /////////////
    struct InputPair<'a> {
        args_input: Option<&'a crate::sdk::action::ValidCreateActionInput>,
        storage_input: &'a crate::sdk::StorageCreateTransactionInput,
    }
    
    let mut inputs: Vec<InputPair> = Vec::new();
    for storage_input in storage_inputs {
        let vin = storage_input.vin;
        let args_input = if (vin as usize) < args.inputs.len() {
            Some(&args.inputs[vin as usize])
        } else {
            None
        };
        
        inputs.push(InputPair {
            args_input,
            storage_input,
        });
    }
    
    // Sort by vin (TS lines 91-93)
    inputs.sort_by(|a, b| {
        a.storage_input.vin.cmp(&b.storage_input.vin)
    });
    
    let mut pending_storage_inputs: Vec<PendingStorageInput> = Vec::new();
    
    //////////////
    // Add INPUTS (TS lines 97-148)
    /////////////
    let mut total_change_inputs: i64 = 0;
    
    for input_pair in inputs {
        if let Some(args_input) = input_pair.args_input {
            // Type 1: User supplied input, with or without an explicit unlockingScript (TS lines 103-118)
            // If without, signAction must be used to provide the actual unlockScript.
            let has_unlock = args_input.unlocking_script.is_some();
            let unlock = if has_unlock {
                args_input.unlocking_script.clone().unwrap()
            } else {
                String::new() // Empty script
            };
            
            // Get source transaction from input BEEF if this is signAction (TS line 108)
            let source_transaction: Option<Transaction> = if args.is_sign_action && input_beef.is_some() {
                // TODO: Parse BEEF and find transaction
                // For now, None
                None
            } else {
                None
            };
            
            // Add input to transaction (TS lines 109-118)
            let tx_input = crate::transaction::TxInput {
                prev_out: crate::transaction::OutPoint {
                    txid: args_input.outpoint.txid.clone(),
                    vout: args_input.outpoint.vout,
                },
                script_sig: hex::decode(&unlock).unwrap_or_default(),
                sequence: args_input.sequence_number,
            };
            tx.add_input(tx_input);
            
        } else {
            // Type 2: SABPPP protocol inputs signed using ScriptTemplateBRC29 (TS lines 119-147)
            let storage_input = input_pair.storage_input;
            
            // Validate type is P2PKH (TS lines 121-125)
            if storage_input.input_type != "P2PKH" {
                return Err(WalletError::invalid_parameter(
                    "type",
                    &format!(
                        "vin {}, \"{}\" is not a supported unlocking script type.",
                        storage_input.vin,
                        storage_input.input_type
                    )
                ));
            }
            
            // Create pending storage input (TS lines 127-134)
            let derivation_prefix = storage_input.derivation_prefix.clone()
                .ok_or_else(|| WalletError::invalid_parameter(
                    "derivationPrefix",
                    "must be provided for storage inputs"
                ))?;
                
            let derivation_suffix = storage_input.derivation_suffix.clone()
                .ok_or_else(|| WalletError::invalid_parameter(
                    "derivationSuffix",
                    "must be provided for storage inputs"
                ))?;
            
            pending_storage_inputs.push(PendingStorageInput {
                vin: tx.inputs.len() as u32,
                derivation_prefix,
                derivation_suffix,
                unlocker_pub_key: storage_input.sender_identity_key.clone(),
                source_satoshis: storage_input.source_satoshis as u64,
                locking_script: storage_input.source_locking_script.clone(),
            });
            
            // Get source transaction if available (TS lines 139-141)
            let source_transaction: Option<Transaction> = storage_input.source_transaction.as_ref()
                .and_then(|bytes| {
                    // TODO: Parse transaction from bytes
                    None
                });
            
            // Add input to transaction with empty unlocking script (TS lines 136-145)
            let tx_input = crate::transaction::TxInput {
                prev_out: crate::transaction::OutPoint {
                    txid: storage_input.source_txid.clone(),
                    vout: storage_input.source_vout,
                },
                script_sig: Vec::new(), // Empty unlocking script
                sequence: 0xffffffff,
            };
            tx.add_input(tx_input);
            
            // Add to total change inputs (TS line 146)
            total_change_inputs += storage_input.source_satoshis;
        }
    }
    
    // Calculate amount (TS lines 150-155)
    // The amount is the total of non-foreign inputs minus change outputs
    // Note that the amount can be negative when we are redeeming more inputs than we are spending
    let total_change_outputs: i64 = storage_outputs.iter()
        .filter(|x| x.purpose.as_deref() == Some("change"))
        .map(|x| x.satoshis as i64)
        .sum();
    
    let amount = total_change_inputs - total_change_outputs;
    
    // Return result (TS lines 157-162)
    Ok(BuildSignableTransactionResult {
        tx,
        amount,
        pdi: pending_storage_inputs,
        log: String::new(),
    })
}

/// Derive a change output locking script
///
/// Reference: TS makeChangeLock (buildSignableTransaction.ts lines 165-184)
///
/// # Arguments
/// * `out` - Output requiring change lock
/// * `dctr` - Create action result
/// * `args` - Create action arguments
/// * `change_keys` - Change key pair
fn make_change_lock(
    out: &crate::sdk::StorageCreateTransactionOutput,
    dctr: &StorageCreateActionResult,
    _args: &ValidCreateActionArgs,
    change_keys: &KeyPair,
) -> WalletResult<String> {
    // Get derivation parameters (TS lines 175-176)
    let derivation_prefix = &dctr.derivation_prefix;
    let derivation_suffix = out.derivation_suffix.as_ref()
        .ok_or_else(|| WalletError::invalid_parameter(
            "derivationSuffix",
            "required for change outputs"
        ))?;
    
    // Create SABPPP template (TS lines 177-181)
    let sabppp = ScriptTemplateSABPPP::new(
        derivation_prefix.clone(),
        derivation_suffix.clone(),
    );
    
    // Generate locking script (TS line 182)
    let locking_script = sabppp.lock(&change_keys.private_key, &change_keys.public_key);
    
    // Convert to hex string
    Ok(hex::encode(locking_script))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pending_storage_input_serde() {
        let psi = PendingStorageInput {
            vin: 0,
            derivation_prefix: "prefix".to_string(),
            derivation_suffix: "suffix".to_string(),
            unlocker_pub_key: Some("pubkey".to_string()),
            source_satoshis: 10000,
            locking_script: "76a914...88ac".to_string(),
        };
        
        let json = serde_json::to_string(&psi).unwrap();
        let deserialized: PendingStorageInput = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.vin, 0);
        assert_eq!(deserialized.source_satoshis, 10000);
    }
    
    #[test]
    fn test_build_result_creation() {
        let result = BuildSignableTransactionResult {
            tx: Transaction::new(1, vec![], vec![], 0),
            amount: 1000,
            pdi: vec![],
            log: String::new(),
        };
        
        assert_eq!(result.amount, 1000);
        assert!(result.pdi.is_empty());
    }
}
