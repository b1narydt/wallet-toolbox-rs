//! Sign Action Implementation
//!
//! **Reference**: TypeScript `src/storage/methods/signAction.ts`
//!
//! Signs a transaction created by createAction, generating unlocking scripts
//! for each input and preparing the transaction for broadcast.
//!
//! ## Overview
//!
//! The signAction method takes a transaction reference (from createAction) and:
//! 1. Retrieves the transaction from storage
//! 2. Validates transaction status and inputs
//! 3. Generates unlocking scripts for each input
//! 4. Signs with appropriate keys (derived via BRC-42/43)
//! 5. Updates transaction status
//! 6. Optionally broadcasts or marks for broadcast
//!
//! ## Process Flow (TypeScript Reference)
//!
//! 1. **Validate Arguments** (TS lines 30-40)
//!     - Validate reference format
//!     - Validate spends structure
//!     - Check options
//!
//! 2. **Retrieve Transaction** (TS lines 42-55)
//!     - Find by reference
//!     - Verify status = 'unsigned'
//!     - Load inputs and outputs
//!
//! 3. **Build Transaction** (TS lines 57-120)
//!     - Parse version, lockTime
//!     - Add all inputs with unlocking scripts
//!     - Add all outputs with locking scripts
//!     - Calculate txid
//!
//! 4. **Sign Inputs** (TS lines 122-180)
//!     - For each input:
//!       - Derive signing key (BRC-42/43)
//!       - Generate signature
//!       - Build unlocking script
//!       - Add to transaction
//!
//! 5. **Update Storage** (TS lines 182-200)
//!     - Update transaction status to 'signed' or 'sending'
//!     - Store raw transaction
//!     - Store txid
//!     - Update spent outputs
//!
//! 6. **Prepare Result** (TS lines 202-220)
//!     - Build result with txid
//!     - Include rawTx if requested
//!     - Add sendWith info
//!     - Return processing result
//!
//! **Returns**: `StorageProcessActionResults` with txid, optional raw tx, sendWith results

use crate::sdk::action_process::{
    ValidSignActionArgs, SignActionSpend,
    StorageProcessActionResults, SendWithResult,
};
use wallet_storage::{
    StorageError, WalletStorageProvider, AuthId,
    TableTransaction, TableOutput, TransactionStatus,
};

/// Storage-level sign action result (internal)
/// Matches TypeScript signAction return structure
#[derive(Debug, Clone)]
pub struct StorageSignActionResult {
    /// Transaction ID
    pub txid: String,
    
    /// Raw transaction bytes (if requested)
    pub raw_tx: Option<Vec<u8>>,
    
    /// SendWith transaction results
    pub send_with_results: Vec<SendWithResult>,
    
    /// Processing log
    pub log: Option<String>,
}

/// Main signAction implementation
///
/// Reference: TypeScript src/storage/methods/signAction.ts
///
/// Signs a transaction by:
/// 1. Retrieving unsigned transaction from storage
/// 2. Generating unlocking scripts for inputs
/// 3. Signing with derived keys
/// 4. Updating transaction status
/// 5. Optionally broadcasting
pub async fn sign_action(
    storage: &mut dyn WalletStorageProvider,
    auth: &AuthId,
    vargs: ValidSignActionArgs,
) -> Result<StorageProcessActionResults, StorageError> {
    let user_id = auth.user_id.ok_or_else(|| {
        StorageError::Unauthorized("user_id required".to_string())
    })?;
    
    // STEP 1: Validate and retrieve transaction
    // TS lines 42-55: Find transaction by reference
    let transaction = find_transaction_by_reference(
        storage,
        user_id,
        &vargs.reference,
    ).await?;
    
    // STEP 2: Validate transaction status
    // TS lines 56-60: Must be 'unsigned'
    validate_transaction_status(&transaction)?;
    
    // STEP 3: Load transaction inputs and outputs
    // TS lines 62-75: Get all inputs/outputs for this transaction
    let inputs = load_transaction_inputs(storage, user_id, transaction.transaction_id).await?;
    let outputs = load_transaction_outputs(storage, user_id, transaction.transaction_id).await?;
    
    // STEP 4: Build and sign transaction
    // TS lines 77-180: Generate unlocking scripts and sign
    let signed_tx = build_and_sign_transaction(
        storage,
        user_id,
        &transaction,
        &inputs,
        &outputs,
        &vargs.spends,
    ).await?;
    
    // STEP 5: Update transaction in storage
    // TS lines 182-200: Mark as signed, store raw tx and txid
    update_signed_transaction(
        storage,
        transaction.transaction_id,
        &signed_tx.txid,
        &signed_tx.raw_tx,
        vargs.is_no_send,
    ).await?;
    
    // STEP 6: Handle broadcast if needed
    // TS lines 202-220: Process sendWith, mark for broadcast
    let send_with_results = if !vargs.is_no_send {
        handle_broadcast(storage, &signed_tx, &vargs).await?
    } else {
        Vec::new()
    };
    
    // STEP 7: Build result
    Ok(StorageProcessActionResults {
        send_with_results: if send_with_results.is_empty() {
            None
        } else {
            Some(send_with_results)
        },
        not_delayed_results: None, // Set by processAction later
        log: signed_tx.log,
    })
}

// ============================================================================
// STEP IMPLEMENTATIONS
// ============================================================================

/// STEP 1: Find transaction by reference
/// Reference: TypeScript signAction.ts lines 42-55
///
/// Looks up a transaction by its reference ID.
/// Must return exactly one transaction, otherwise error.
async fn find_transaction_by_reference(
    storage: &dyn WalletStorageProvider,
    user_id: i64,
    reference: &str,
) -> Result<TableTransaction, StorageError> {
    // TS line 42: const tx = await storage.findTransactions({ reference, userId })
    let transactions = storage.find_transactions(
        user_id,
        Some(reference),
        None, // No status filter
    ).await?;
    
    // TS line 48-51: Must find exactly one transaction
    if transactions.is_empty() {
        return Err(StorageError::NotFound(
            format!("Transaction not found with reference: {}", reference)
        ));
    }
    
    if transactions.len() > 1 {
        return Err(StorageError::InvalidArg(
            format!("Multiple transactions found with reference: {}", reference)
        ));
    }
    
    Ok(transactions.into_iter().next().unwrap())
}

/// STEP 2: Validate transaction status
/// Reference: TypeScript signAction.ts lines 56-60
fn validate_transaction_status(
    transaction: &TableTransaction,
) -> Result<(), StorageError> {
    match transaction.status {
        TransactionStatus::Unsigned => Ok(()),
        _ => Err(StorageError::InvalidArg(
            format!("Transaction must be unsigned, got: {:?}", transaction.status)
        )),
    }
}

/// STEP 3: Load transaction inputs
/// Reference: TypeScript signAction.ts lines 62-70
///
/// Loads all inputs for a transaction by finding outputs that are spent by it.
/// Inputs are outputs from other transactions that this transaction spends.
async fn load_transaction_inputs(
    storage: &dyn WalletStorageProvider,
    user_id: i64,
    transaction_id: i64,
) -> Result<Vec<TableOutput>, StorageError> {
    // TS line 64: const inputs = await storage.findOutputs({ spentBy: transactionId })
    // We need to find outputs where spent_by = transaction_id
    storage.find_outputs_by_transaction(
        user_id,
        transaction_id,
        true, // is_input = true (spent_by)
    ).await
}

/// STEP 4: Load transaction outputs
/// Reference: TypeScript signAction.ts lines 72-75
///
/// Loads all outputs created by this transaction.
async fn load_transaction_outputs(
    storage: &dyn WalletStorageProvider,
    user_id: i64,
    transaction_id: i64,
) -> Result<Vec<TableOutput>, StorageError> {
    // TS line 73: const outputs = await storage.findOutputs({ transactionId })
    storage.find_outputs_by_transaction(
        user_id,
        transaction_id,
        false, // is_input = false (transaction_id)
    ).await
}

/// Signed transaction result
struct SignedTransaction {
    txid: String,
    raw_tx: Vec<u8>,
    log: Option<String>,
}

/// STEP 5: Build and sign transaction
/// Reference: TypeScript signAction.ts lines 77-180
///
/// This is the core signing logic that:
/// - Builds BSV transaction structure
/// - Generates unlocking scripts
/// - Signs each input with derived keys
/// - Calculates txid
async fn build_and_sign_transaction(
    storage: &dyn WalletStorageProvider,
    user_id: i64,
    transaction: &TableTransaction,
    inputs: &[TableOutput],
    outputs: &[TableOutput],
    spends: &std::collections::HashMap<u32, SignActionSpend>,
) -> Result<SignedTransaction, StorageError> {
    use crate::transaction::{Transaction, TxInput, TxOutput, OutPoint, SigHash, SigHashType, Script};
    use crate::crypto::{sign_ecdsa, derive_public_key};
    use crate::keys::derivation::{derive_key_from_output, KeyDerivationContext};
    
    // STEP 5.1: Build transaction structure
    // TS lines 79-95: Create transaction with version and lockTime
    let mut tx = Transaction::new();
    tx.version = transaction.version.unwrap_or(1);
    tx.lock_time = transaction.lock_time.unwrap_or(0);
    
    // STEP 5.2: Add inputs (with empty scripts initially)
    // TS lines 97-110: Add all inputs from storage
    for (vin, input_data) in inputs.iter().enumerate() {
        let txid = input_data.txid.as_ref()
            .ok_or_else(|| StorageError::InvalidArg("Input missing txid".to_string()))?;
        let vout = input_data.vout as u32;
        
        let outpoint = OutPoint::new(txid, vout);
        let mut input = TxInput::new(outpoint);
        
        // Set sequence from spends if provided
        if let Some(spend) = spends.get(&(vin as u32)) {
            input.set_sequence(spend.sequence_number);
        }
        
        tx.add_input(input);
    }
    
    // STEP 5.3: Add outputs
    // TS lines 112-120: Add all outputs from storage
    for output_data in outputs {
        let locking_script = output_data.locking_script.as_ref()
            .ok_or_else(|| StorageError::InvalidArg("Output missing locking script".to_string()))?;
        
        tx.add_output(TxOutput::new(output_data.satoshis, locking_script.clone()));
    }
    
    // STEP 5.4: Sign each input
    // TS lines 122-180: For each input, calculate sighash, sign, and build unlocking script
    for (vin, input_data) in inputs.iter().enumerate() {
        // Get locking script from previous output (subscript for sighash)
        let prev_script = input_data.locking_script.as_ref()
            .ok_or_else(|| StorageError::InvalidArg(format!("Input {} missing locking script", vin)))?;
        
        // Calculate sighash for this input
        let sighash = SigHash::calculate(
            &tx,
            vin,
            prev_script,
            SigHashType::All,
            input_data.satoshis,
        ).map_err(|e| StorageError::InvalidArg(format!("Sighash calculation failed: {}", e)))?;
        
        // Check if custom unlocking script provided
        if let Some(spend) = spends.get(&(vin as u32)) {
            if !spend.unlocking_script.is_empty() {
                // Use provided unlocking script
                let script_bytes = hex::decode(&spend.unlocking_script)
                    .map_err(|e| StorageError::InvalidArg(format!("Invalid unlocking script hex: {}", e)))?;
                tx.inputs[vin].set_script(script_bytes);
                continue;
            }
        }
        
        // STEP 5.4.1: Derive private key from derivation_prefix/suffix (BRC-42/43)
        // TS lines 130-145: Key derivation happens here
        
        // Get master private key from storage (wallet's root key)
        // This would typically come from the authenticated user's wallet
        let master_private_key = get_master_private_key(storage, user_id).await?;
        
        let ctx = KeyDerivationContext {
            master_private_key,
        };
        
        // Derive the private key for this input using BRC-42/43
        let private_key = derive_key_from_output(input_data, &ctx)
            .map_err(|e| StorageError::InvalidArg(format!("Key derivation failed: {}", e)))?;
        
        // STEP 5.4.2: Sign the sighash
        let signature = sign_ecdsa(&sighash, &private_key, SigHashType::All.as_u8())
            .map_err(|e| StorageError::InvalidArg(format!("Signing failed: {}", e)))?;
        
        // STEP 5.4.3: Derive public key
        let public_key = derive_public_key(&private_key)
            .map_err(|e| StorageError::InvalidArg(format!("Public key derivation failed: {}", e)))?;
        
        // STEP 5.4.4: Build P2PKH unlocking script: <signature> <publicKey>
        let unlocking_script = Script::p2pkh_unlocking_script(&signature, &public_key);
        
        // STEP 5.4.5: Set the unlocking script on the input
        tx.inputs[vin].set_script(unlocking_script.to_bytes().to_vec());
    }
    
    // STEP 5.5: Calculate final txid and serialize
    // TS lines 185-190: Get txid and raw transaction
    let txid = tx.txid()
        .map_err(|e| StorageError::InvalidArg(format!("Txid calculation failed: {}", e)))?;
    
    let raw_tx = tx.serialize()
        .map_err(|e| StorageError::InvalidArg(format!("Transaction serialization failed: {}", e)))?;
    
    Ok(SignedTransaction {
        txid,
        raw_tx,
        log: Some("Transaction built and signed successfully".to_string()),
    })
}

/// Get master private key for user
/// This retrieves the user's root key for BRC-42/43 derivation
async fn get_master_private_key(
    storage: &dyn WalletStorageProvider,
    user_id: i64,
) -> Result<Vec<u8>, StorageError> {
    // In a real implementation, this would retrieve the user's encrypted master key
    // For now, return a NotImplemented error with clear guidance
    // TS: This would come from the user's wallet/identity store
    
    // TEMPORARY: Return an error indicating this needs to be implemented
    // In production, this must:
    // 1. Retrieve encrypted master key from storage
    // 2. Decrypt using user's credentials
    // 3. Return 32-byte private key
    Err(StorageError::NotImplemented(
        "get_master_private_key - requires wallet key management implementation"
    ))
}

/// STEP 6: Update signed transaction in storage
/// Reference: TypeScript signAction.ts lines 182-200
async fn update_signed_transaction(
    storage: &mut dyn WalletStorageProvider,
    transaction_id: i64,
    txid: &str,
    raw_tx: &[u8],
    is_no_send: bool,
) -> Result<(), StorageError> {
    // TS: await storage.updateTransaction(transactionId, {
    //   status: isNoSend ? 'nosend' : 'signed',
    //   txid,
    //   rawTx
    // })
    
    let status = if is_no_send {
        TransactionStatus::Nosend
    } else {
        TransactionStatus::Sending
    };
    
    storage.update_transaction_status(transaction_id, status).await?;
    storage.update_transaction_txid(transaction_id, txid).await?;
    storage.update_transaction_raw_tx(transaction_id, raw_tx).await?;
    
    Ok(())
}

/// STEP 7: Handle broadcast preparation
/// Reference: TypeScript signAction.ts lines 202-220
async fn handle_broadcast(
    _storage: &dyn WalletStorageProvider,
    _signed_tx: &SignedTransaction,
    vargs: &ValidSignActionArgs,
) -> Result<Vec<SendWithResult>, StorageError> {
    // TS: Process sendWith transactions, prepare for broadcast
    // Reference: TypeScript signAction.ts lines 202-220
    
    let mut results = Vec::new();
    
    // If sendWith is provided, process each broadcast target
    for _protocol in &vargs.options.send_with {
        // Prepare send with result
        // In a full implementation, this would interact with overlay services
        // or other broadcast mechanisms
        results.push(SendWithResult {
            status: "prepared".to_string(),
            txid: _signed_tx.txid.clone(),
        });
    }
    
    Ok(results)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_transaction_status_unsigned() {
        // TS Reference: Transaction must be in 'unsigned' status
        let tx = TableTransaction::new(
            1,                             // transaction_id
            1,                             // user_id
            TransactionStatus::Unsigned,   // status
            "test_ref_123",                // reference
            true,                          // is_outgoing
            10000,                         // satoshis
            "test transaction",            // description
        );
        
        assert!(validate_transaction_status(&tx).is_ok());
    }
    
    #[test]
    fn test_validate_transaction_status_wrong() {
        // TS Reference: Signing requires unsigned transaction
        let tx = TableTransaction::new(
            1,                             // transaction_id
            1,                             // user_id
            TransactionStatus::Completed,  // status - WRONG
            "test_ref_123",                // reference
            true,                          // is_outgoing
            10000,                         // satoshis
            "test transaction",            // description
        );
        
        let result = validate_transaction_status(&tx);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unsigned"));
    }
    
    #[test]
    fn test_validate_transaction_status_sending() {
        // TS Reference: Can't sign transaction that's already sending
        let tx = TableTransaction::new(
            1,
            1,
            TransactionStatus::Sending,
            "test_ref",
            true,
            10000,
            "test",
        );
        
        assert!(validate_transaction_status(&tx).is_err());
    }
    
    #[test]
    fn test_validate_transaction_status_nosend() {
        // TS Reference: Can't sign transaction that's in nosend status
        let tx = TableTransaction::new(
            1,
            1,
            TransactionStatus::Nosend,
            "test_ref",
            true,
            10000,
            "test",
        );
        
        assert!(validate_transaction_status(&tx).is_err());
    }
}
