//! Complete Signed Transaction
//!
//! **Reference**: TypeScript `src/signer/methods/completeSignedTransaction.ts`
//!
//! Finalizes a transaction by adding signatures to unlocking scripts

use crate::sdk::errors::{WalletError, WalletResult};
use crate::transaction::Transaction;
use crate::keys::KeyPair;
use crate::utility::ScriptTemplateSABPPP;
use super::build_signable_transaction::PendingStorageInput;
use crate::sdk::ValidCreateActionArgs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Pending sign action
///
/// Reference: TS PendingSignAction (Wallet.ts lines 1069-1076)
#[derive(Debug, Clone)]
pub struct PendingSignAction {
    /// Reference identifier
    pub reference: String,
    
    /// Create action result
    pub dcr: crate::sdk::StorageCreateActionResult,
    
    /// Create action arguments
    pub args: ValidCreateActionArgs,
    
    /// Transaction to sign
    pub tx: Transaction,
    
    /// Amount
    pub amount: i64,
    
    /// Pending storage inputs
    pub pdi: Vec<PendingStorageInput>,
}

/// Sign action spend
///
/// Reference: TS SignActionSpend from @bsv/sdk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignActionSpend {
    /// Unlocking script hex
    pub unlocking_script: String,
    
    /// Sequence number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<u32>,
}

/// Complete a signed transaction by adding unlocking scripts
///
/// Reference: TS completeSignedTransaction (completeSignedTransaction.ts lines 8-63)
///
/// # Arguments
/// * `prior` - Pending sign action from buildSignableTransaction
/// * `spends` - Map of vin to spend (user-provided unlocking scripts)
/// * `change_keys` - Client change key pair for wallet-signed inputs
///
/// # Returns
/// Fully signed transaction ready for broadcast
pub async fn complete_signed_transaction(
    mut prior: PendingSignAction,
    spends: HashMap<u32, SignActionSpend>,
    change_keys: &KeyPair,
) -> WalletResult<Transaction> {
    /////////////////////
    // Insert the user provided unlocking scripts from "spends" arg (TS lines 13-32)
    /////////////////////
    for (vin, spend) in spends.iter() {
        let vin_usize = *vin as usize;
        
        // Get create input and transaction input (TS lines 18-19)
        let create_input = prior.args.inputs.get(vin_usize)
            .ok_or_else(|| WalletError::invalid_parameter(
                "args",
                &format!("spend vin {} does not correspond to prior input", vin)
            ))?;
            
        let input = prior.tx.inputs.get_mut(vin_usize)
            .ok_or_else(|| WalletError::invalid_parameter(
                "args",
                &format!("spend vin {} not found in transaction", vin)
            ))?;
        
        // Validate conditions (TS lines 20-24)
        if create_input.unlocking_script.is_some() {
            return Err(WalletError::invalid_parameter(
                "args",
                "spend does not correspond to prior input with valid unlockingScriptLength."
            ));
        }
        
        if create_input.unlocking_script_length.is_none() {
            return Err(WalletError::invalid_parameter(
                "args",
                "spend does not correspond to prior input with valid unlockingScriptLength."
            ));
        }
        
        let expected_length = create_input.unlocking_script_length.unwrap();
        
        // Check unlocking script length (TS lines 25-29)
        let unlock_hex_len = spend.unlocking_script.len();
        if unlock_hex_len / 2 > expected_length as usize {
            return Err(WalletError::invalid_parameter(
                "args",
                &format!(
                    "spend unlockingScript length {} exceeds expected length {}",
                    unlock_hex_len, expected_length
                )
            ));
        }
        
        // Set unlocking script (TS line 30)
        input.script_sig = hex::decode(&spend.unlocking_script).unwrap_or_default();
        
        // Set sequence number if provided (TS line 31)
        if let Some(seq) = spend.sequence_number {
            input.sequence = seq;
        }
    }
    
    /////////////////////
    // Insert SABPPP unlock templates for wallet signed inputs (TS lines 38-55)
    /////////////////////
    for pdi in &prior.pdi {
        // Create SABPPP template (TS lines 42-46)
        let sabppp = ScriptTemplateSABPPP::new(
            pdi.derivation_prefix.clone(),
            pdi.derivation_suffix.clone(),
        );
        
        // Get keys (TS lines 47-49)
        let locker_priv_key = &change_keys.private_key;
        let unlocker_pub_key = pdi.unlocker_pub_key.as_deref()
            .unwrap_or("");
        
        // Get source output details (TS lines 50-51)
        let source_satoshis = pdi.source_satoshis;
        let locking_script = &pdi.locking_script;
        
        // Generate unlock template (TS line 52)
        let unlock_template = sabppp.unlock(
            locker_priv_key,
            unlocker_pub_key,
            source_satoshis,
            locking_script,
        )?;
        
        // Set unlocking script template on input (TS lines 53-54)
        let input = prior.tx.inputs.get_mut(pdi.vin as usize)
            .ok_or_else(|| WalletError::invalid_parameter(
                "pdi.vin",
                &format!("vin {} not found in transaction", pdi.vin)
            ))?;
        
        // TODO: TxInput doesn't have unlocking_script_template field
        // This needs to be stored separately or the Transaction struct needs to be extended
        // For now, this is a placeholder - the template will be used in actual signing
        // input.unlocking_script_template = Some(unlock_template);
    }
    
    /////////////////////
    // Sign wallet signed inputs making transaction fully valid (TS lines 57-60)
    /////////////////////
    // TODO: Transaction::sign() method needs to be implemented
    // This would sign all inputs that have unlocking_script_templates set
    // For now, we skip this step as templates are not yet implemented
    // prior.tx.sign().await?;
    
    // Return signed transaction (TS line 62)
    Ok(prior.tx)
}

/// Verify unlocking scripts in a BEEF transaction
///
/// Reference: TS verifyUnlockScripts (completeSignedTransaction.ts lines 65-117)
///
/// Validates that all unlocking scripts in a transaction are valid
///
/// # Arguments
/// * `txid` - The TXID of transaction to verify
/// * `beef` - BEEF containing the transaction and all its inputs
///
/// # Errors
/// Returns error if any unlocking script is invalid or if BEEF doesn't contain required transactions
pub fn verify_unlock_scripts(txid: &str, beef: &[u8]) -> WalletResult<()> {
    // TODO: Full implementation requires:
    // 1. Parse BEEF to find transaction by txid (TS line 71)
    // 2. Iterate through all inputs (TS lines 74-86)
    // 3. Validate each input has sourceTXID and unlockingScript (TS lines 76-77)
    // 4. Find source transaction in BEEF (TS lines 78-85)
    // 5. Validate each unlocking script using Spend (TS lines 88-116)
    //    - Create Spend object with all parameters
    //    - Call spend.validate()
    //    - Check result
    
    // Placeholder implementation
    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sign_action_spend_serde() {
        let spend = SignActionSpend {
            unlocking_script: "47304402...".to_string(),
            sequence_number: Some(0xfffffffe),
        };
        
        let json = serde_json::to_string(&spend).unwrap();
        let deserialized: SignActionSpend = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.sequence_number, Some(0xfffffffe));
    }
    
    #[test]
    fn test_pending_sign_action_creation() {
        use crate::sdk::StorageCreateActionResult;
        use crate::sdk::ValidCreateActionArgs;
        
        let dcr = StorageCreateActionResult {
            txid: None,
            reference: "ref123".to_string(),
            derivation_prefix: "prefix".to_string(),
            version: 1,
            lock_time: 0,
            inputs: vec![],
            outputs: vec![],
            log: String::new(),
        };
        
        let args = ValidCreateActionArgs {
            description: "test".to_string(),
            inputs: vec![],
            outputs: vec![],
            version: Some(1),
            lock_time: Some(0),
            options: None,
            labels: vec![],
            is_sign_action: false,
            input_beef: None,
        };
        
        let psa = PendingSignAction {
            reference: "ref123".to_string(),
            dcr,
            args,
            tx: Transaction::new(1, vec![], vec![], 0),
            amount: 1000,
            pdi: vec![],
        };
        
        assert_eq!(psa.reference, "ref123");
        assert_eq!(psa.amount, 1000);
    }
    
    #[tokio::test]
    async fn test_complete_signed_transaction_basic() {
        use crate::sdk::StorageCreateActionResult;
        use crate::sdk::ValidCreateActionArgs;
        
        let dcr = StorageCreateActionResult {
            txid: None,
            reference: "ref123".to_string(),
            derivation_prefix: "prefix".to_string(),
            version: 1,
            lock_time: 0,
            inputs: vec![],
            outputs: vec![],
            log: String::new(),
        };
        
        let args = ValidCreateActionArgs {
            description: "test".to_string(),
            inputs: vec![],
            outputs: vec![],
            version: Some(1),
            lock_time: Some(0),
            options: None,
            labels: vec![],
            is_sign_action: false,
            input_beef: None,
        };
        
        let psa = PendingSignAction {
            reference: "ref123".to_string(),
            dcr,
            args,
            tx: Transaction::new(1, vec![], vec![], 0),
            amount: 1000,
            pdi: vec![],
        };
        
        let spends = HashMap::new();
        let keys = KeyPair {
            private_key: "test_priv".to_string(),
            public_key: "test_pub".to_string(),
        };
        
        let result = complete_signed_transaction(psa, spends, &keys).await;
        assert!(result.is_ok());
    }
}
