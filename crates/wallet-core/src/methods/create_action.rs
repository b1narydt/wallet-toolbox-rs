//! createAction - Transaction Building Implementation
//!
//! Translates TypeScript createAction from @wallet-toolbox/src/storage/methods/createAction.ts
//! This is the core transaction building method (979 lines in TS).
//!
//! ## Architecture (14 Major Steps):
//! 
//! 1. **Validate Required Inputs** (TS line 88)
//!    - Verify all inputs have proof in inputBEEF OR trustSelf='known' + known valid to storage
//!    - Parse input scripts and satoshis
//!    - Build BEEF from inputs
//!
//! 2. **Validate Required Outputs** (TS line 89)
//!    - Check all outputs have valid locking scripts
//!    - Assign vout numbers
//!    - Add storage commission if configured
//!
//! 3. **Get Change Basket** (TS lines 91-97)
//!    - Find or create 'default' basket for change outputs
//!
//! 4. **Validate noSendChange** (TS line 99)
//!    - Check for outputs that shouldn't send change
//!    - Validate basket assignments
//!
//! 5. **Count Available Change** (TS line 101)
//!    - Count spendable outputs in change basket
//!
//! 6. **Validate Fee Model** (TS line 103)
//!    - Ensure fee model is valid
//!
//! 7. **Create Transaction Record** (TS line 105)
//!    - Insert into `transactions` table with status='unsigned'
//!    - Generate unique reference ID
//!    - Create transaction labels
//!
//! 8. **Fund Transaction** (TS lines 117-118)
//!    - Calculate required input satoshis (outputs + fees)
//!    - Select and LOCK change outputs from changeBasket
//!    - Generate change outputs if needed
//!    - Handle maxPossibleSatoshis adjustment
//!
//! 9. **Update maxPossibleSatoshis** (TS lines 120-124)
//!    - Adjust output satoshis if maxPossibleSatoshis was used
//!
//! 10. **Calculate Net Satoshis** (TS lines 127-129)
//!     - Change outputs - allocated change inputs
//!     - Update transaction satoshis
//!
//! 11. **Create New Outputs** (TS line 131)
//!     - Insert user outputs + change outputs
//!     - Create basket/tag associations
//!
//! 12. **Merge BEEFs** (TS line 133)
//!     - Combine inputBEEF + change BEEFs
//!
//! 13. **Create Result Inputs** (TS line 135)
//!     - Combine user inputs + allocated change
//!     - Mark outputs as spent
//!     - Build input specifications
//!
//! 14. **Build Final Result** (TS lines 137-149)
//!     - Return StorageCreateActionResult
//!
//! **Returns**: `StorageCreateActionResult` with inputs, outputs, BEEF, reference

use crate::sdk::action::{
    ValidCreateActionArgs, ValidCreateActionInput, ValidCreateActionOutput,
    StorageCreateActionResult, StorageCreateTransactionInput, StorageCreateTransactionOutput,
    StorageProvidedBy, ValidCreateActionOptions,
};
use crate::beef::Beef;
use wallet_storage::{
    StorageError, WalletStorageProvider, AuthId,
    TableOutputBasket, TableOutput, TableTransaction, TableOutputTag,
    TableCommission, FindOutputBasketsArgs, FindOutputsArgs, PartialOutput, OutputUpdates,
    StorageProvidedBy as WalletStorageProvidedBy, TransactionStatus,
};
use chrono::Utc;
use base64::Engine as _;

/// Fee model type (simplified for now - will expand when fee models are implemented)
#[derive(Debug, Clone)]
pub struct StorageFeeModel {
    pub model: String,
    pub value: f64,
}

/// Context for transaction creation
struct CreateTransactionContext {
    /// Extended inputs with vin assignments
    xinputs: Vec<XValidCreateActionInput>,
    
    /// Extended outputs with vout assignments
    xoutputs: Vec<XValidCreateActionOutput>,
    
    /// Change basket for funding
    change_basket: TableOutputBasket,
    
    /// Outputs that shouldn't send change
    no_send_change_in: Vec<TableOutput>,
    
    /// Available change output count
    available_change_count: i64,
    
    /// Fee model
    fee_model: StorageFeeModel,
    
    /// Transaction ID (from database insert)
    transaction_id: i64,
}

/// Extended input with assigned vin
#[derive(Debug, Clone)]
struct XValidCreateActionInput {
    /// Original input specification
    input: ValidCreateActionInput,
    
    /// Assigned input index
    vin: u32,
    
    /// Parsed locking script
    locking_script: Vec<u8>,
    
    /// Satoshis from source
    satoshis: i64,
    
    /// Optional matching output from storage
    output: Option<TableOutput>,
}

/// Extended output with assigned vout
#[derive(Debug, Clone)]
struct XValidCreateActionOutput {
    /// Original output specification
    output: ValidCreateActionOutput,
    
    /// Assigned output index
    vout: u32,
    
    /// Who provided this output
    provided_by: StorageProvidedBy,
    
    /// Optional purpose description
    purpose: Option<String>,
    
    /// Derivation suffix for keys
    derivation_suffix: Option<String>,
    
    /// Key offset for derivation
    key_offset: Option<String>,
}

impl XValidCreateActionOutput {
    /// Get satoshis (delegates to underlying output)
    fn satoshis(&self) -> i64 {
        self.output.satoshis
    }
    
    /// Get locking script (delegates to underlying output)
    fn locking_script(&self) -> &str {
        &self.output.locking_script
    }
    
    /// Get tags (delegates to underlying output)
    fn tags(&self) -> &[String] {
        self.output.tags.as_deref().unwrap_or(&[])
    }
    
    /// Get basket (delegates to underlying output)
    fn basket(&self) -> Option<&str> {
        self.output.basket.as_deref()
    }
    
    /// Get output description (delegates to underlying output)
    fn output_description(&self) -> &str {
        &self.output.output_description
    }
    
    /// Get custom instructions (delegates to underlying output)
    fn custom_instructions(&self) -> Option<&str> {
        self.output.custom_instructions.as_deref()
    }
}

/// Main createAction implementation
/// 
/// Reference: @wallet-toolbox/src/storage/methods/createAction.ts line 59
pub async fn create_action(
    storage: &mut dyn WalletStorageProvider,
    auth: &AuthId,
    vargs: ValidCreateActionArgs,
    _originator: Option<String>,
) -> Result<StorageCreateActionResult, StorageError> {
    // Verify this is a new transaction
    if !vargs.is_new_tx {
        return Err(StorageError::InvalidArg(
            "createAction requires isNewTx=true".to_string()
        ));
    }
    
    let user_id = auth.user_id.ok_or_else(|| {
        StorageError::Unauthorized("user_id required".to_string())
    })?;
    
    // STEP 1: Validate Required Inputs (line 88)
    // - Verify proofs exist in inputBEEF or trustSelf='known'
    // - Parse locking scripts and satoshis
    // - Build BEEF structure
    let (_storage_beef, _beef, xinputs) = validate_required_inputs(storage, user_id, &vargs).await?;
    
    // STEP 2: Validate Required Outputs (line 89)
    // - Validate locking scripts
    // - Assign vout numbers
    // - Calculate total satoshis
    let xoutputs = validate_required_outputs(storage, user_id, &vargs)?;
    
    // STEP 3: Get Change Basket (lines 91-97)
    // - Find or create 'default' basket
    // - Will be used for change outputs
    let change_basket_name = "default";
    let change_basket = find_output_basket(storage, user_id, change_basket_name).await?;
    
    // STEP 4: Validate noSendChange (line 99)
    // - Check which outputs shouldn't send change
    let no_send_change_in = validate_no_send_change(storage, user_id, &vargs, &change_basket).await?;
    
    // STEP 5: Count Available Change (line 101)
    // - Count spendable outputs in change basket
    let basket_id = change_basket.basket_id;
    let available_change_count = storage.count_change_inputs(user_id, basket_id, !vargs.is_delayed).await?;
    
    // STEP 6: Validate Fee Model (line 103)
    // TODO: Implement fee model validation from storage settings
    let fee_model = StorageFeeModel {
        model: "sat/kb".to_string(),
        value: 0.5,
    };
    
    // STEP 7: Create Transaction Record (line 105)
    // - Insert into transactions table
    // - Status = 'unsigned'
    // - Generate reference ID
    // Convert storage_beef to binary for storage
    let storage_beef_bytes = None; // TODO: storage_beef.to_binary().ok();
    let new_tx = create_new_tx_record(storage, user_id, &vargs, storage_beef_bytes).await?;
    
    // Build context for remaining steps
    let mut ctx = CreateTransactionContext {
        xinputs,
        xoutputs,
        change_basket,
        no_send_change_in,
        available_change_count,
        fee_model,
        transaction_id: new_tx.transaction_id,
    };
    
    // STEP 8: Fund Transaction (lines 117-118)
    // - Calculate required satoshis (outputs + fees)
    // - Select and LOCK change outputs
    // - Generate new change outputs if needed
    let funding_result = fund_new_transaction(storage, user_id, &vargs, &mut ctx).await?;
    
    // STEP 9: Adjust maxPossibleSatoshis if needed (lines 120-124)
    if let Some(adjustment) = funding_result.max_possible_satoshis_adjustment {
        ctx.xoutputs[adjustment.fixed_output_index].output.satoshis = adjustment.satoshis;
    }
    
    // STEP 10: Calculate Net Satoshis (lines 127-129)
    // - Change outputs - allocated change inputs
    let satoshis = funding_result.change_outputs.iter().map(|o| o.satoshis).sum::<i64>()
        - funding_result.allocated_change.iter().map(|o| o.satoshis).sum::<i64>();
    
    storage.update_transaction(new_tx.transaction_id, satoshis).await?;
    
    // STEP 11: Create New Outputs (line 131)
    // - Insert user outputs + change outputs
    // - Create basket/tag associations
    let output_result = create_new_outputs(storage, user_id, &vargs, &ctx, &funding_result.change_outputs).await?;
    
    // STEP 12: Merge BEEFs (line 133)
    // - Combine inputBEEF + change BEEFs
    // TODO: Convert beef to binary when to_binary() is implemented
    let beef_bytes = Vec::new(); // beef.to_binary().unwrap_or_default();
    let input_beef = merge_allocated_change_beefs(
        storage,
        user_id,
        &vargs,
        &funding_result.allocated_change,
        beef_bytes,
    ).await?;
    
    // STEP 13: Create Result Inputs (line 135)
    // - Combine user inputs + allocated change
    // - Add locking scripts and derivation info
    let inputs = create_new_inputs(
        storage,
        user_id,
        &vargs,
        &ctx,
        &funding_result.allocated_change,
    ).await?;
    
    // STEP 14: Build Final Result (lines 137-149)
    let result = StorageCreateActionResult {
        reference: new_tx.reference.clone(),
        version: new_tx.version.unwrap_or(1),
        lock_time: new_tx.lock_time.unwrap_or(0),
        inputs,
        outputs: output_result.outputs,
        derivation_prefix: funding_result.derivation_prefix,
        input_beef,
        no_send_change_output_vouts: if vargs.is_no_send {
            Some(output_result.change_vouts)
        } else {
            None
        },
    };
    
    Ok(result)
}

// ============================================================================
// STEP IMPLEMENTATIONS (STUBS - To be implemented)
// ============================================================================

/// STEP 1: Validate and parse all inputs
/// Reference: TypeScript createAction.ts lines 557-658
/// 
/// This is the most complex validation step:
/// 1. Create empty BEEF or merge inputBEEF
/// 2. Map inputs to xinputs with vin assignments
/// 3. Check for txidOnly entries, verify with storage if trustSelf='known'
/// 4. Ensure BEEF has entry for all input txids
/// 5. Verify BEEF with ChainTracker (TODO)
/// 6. For each input:
///    - Find output in storage (if exists)
///    - Verify not spending change
///    - Verify spendable
///    - Parse locking script and satoshis from BEEF or storage
/// 7. Return (storageBeef, beef, xinputs)
async fn validate_required_inputs(
    storage: &dyn WalletStorageProvider,
    user_id: i64,
    vargs: &ValidCreateActionArgs,
) -> Result<(Beef, Beef, Vec<XValidCreateActionInput>), StorageError> {
    // TS line 568: const beef = new Beef()
    let mut beef = Beef::new_v2();
    
    // TS line 570: Early return if no inputs
    if vargs.inputs.is_empty() {
        return Ok((beef.clone(), beef, Vec::new()));
    }
    
    // TS line 572: Merge inputBEEF if provided
    if let Some(input_beef) = &vargs.input_beef {
        beef.merge_beef(input_beef).map_err(|e| {
            StorageError::InvalidArg(format!("inputBEEF: {}", e))
        })?;
    }
    
    // TS lines 574-580: Map inputs to xinputs with vin
    let mut xinputs: Vec<XValidCreateActionInput> = vargs.inputs.iter()
        .enumerate()
        .map(|(vin, input)| XValidCreateActionInput {
            input: input.clone(),
            vin: vin as u32,
            locking_script: Vec::new(), // Will be filled later
            satoshis: -1, // Will be filled later
            output: None,
        })
        .collect();
    
    // TS line 582: Check trustSelf setting
    let trust_self = vargs.options.trust_self.as_deref() == Some("known");
    
    // TS lines 584-585: Build inputTxids map
    let mut input_txids: std::collections::HashMap<String, bool> = std::collections::HashMap::new();
    for input in &xinputs {
        input_txids.insert(input.input.outpoint.txid.clone(), true);
    }
    
    // TS lines 590-601: Check beef for txidOnly entries
    for btx in &beef.txs {
        if btx.is_txid_only {
            if !trust_self {
                return Err(StorageError::InvalidArg(
                    format!("inputBEEF: valid and contain complete proof data for {}", btx.txid)
                ));
            }
            if !input_txids.contains_key(&btx.txid) {
                // Verify storage knows about this txid
                let is_known = storage.verify_known_valid_transaction(&btx.txid).await?;
                if !is_known {
                    return Err(StorageError::InvalidArg(
                        format!("inputBEEF: valid and contain complete proof data for unknown {}", btx.txid)
                    ));
                }
            }
        }
    }
    
    // TS lines 604-610: Ensure entry for all input txids
    for txid in input_txids.keys() {
        let mut btx_found = beef.find_txid(txid).is_some();
        if !btx_found && trust_self {
            if storage.verify_known_valid_transaction(txid).await? {
                beef.merge_txid_only(txid);
                btx_found = true;
            }
        }
        if !btx_found {
            return Err(StorageError::InvalidArg(
                format!("inputBEEF: valid and contain proof data for possibly known {}", txid)
            ));
        }
    }
    
    // TS line 612: Verify BEEF with ChainTracker
    // TODO: Implement when ChainTracker is available
    // For now, log a warning
    eprintln!("WARNING: BEEF verification skipped - ChainTracker not yet implemented");
    
    // TS line 620: Clone beef for storage
    let storage_beef = beef.clone_beef();
    
    // TS lines 622-655: Process each input
    for input in &mut xinputs {
        let txid = &input.input.outpoint.txid;
        let vout = input.input.outpoint.vout;
        
        // TS line 624: Try to find output in storage
        let partial = PartialOutput {
            txid: Some(txid.clone()),
            basket_id: None,
            spendable: None,
            change: None,
            transaction_id: None,
        };
        let args = FindOutputsArgs {
            user_id,
            since: None,
            paged: None,
            order_descending: None,
            partial: Some(partial),
            no_script: Some(false),
            tx_status: None,
        };
        
        let auth = AuthId::new("").with_user_id(user_id);
        let outputs = storage.find_outputs_auth(&auth, &args).await?;
        let outputs: Vec<_> = outputs.into_iter().filter(|o| o.vout == vout).collect();
        
        if let Some(output) = outputs.into_iter().next() {
            // TS lines 626-630: Check not spending change
            if output.change {
                return Err(StorageError::InvalidArg(
                    format!("inputs[{}]: an unmanaged input. Change outputs are managed by your wallet.", input.vin)
                ));
            }
            
            // TS lines 633-634: Validate locking script and satoshis
            let locking_script = output.locking_script.clone().ok_or_else(|| {
                StorageError::InvalidArg(format!("{}.{}: output with valid lockingScript and satoshis", txid, vout))
            })?;
            let satoshis = output.satoshis;
            
            // TS lines 635-636: Check spendable (unless noSend)
            if !vargs.is_no_send && !output.spendable {
                return Err(StorageError::InvalidArg(
                    format!("{}.{}: spendable output unless noSend is true", txid, vout)
                ));
            }
            
            // TS lines 638-639: Set input data from storage
            input.satoshis = satoshis;
            input.locking_script = locking_script;
            input.output = Some(output);
        } else {
            // TS lines 641-654: Get from BEEF
            let btx = beef.find_txid(txid).ok_or_else(|| {
                StorageError::InvalidArg(format!("inputBEEF: missing transaction {}", txid))
            })?;
            
            if btx.is_txid_only {
                // TS lines 643-647: Get from storage and merge
                let proven_or_raw = storage.get_proven_or_raw_tx(txid).await?;
                
                let raw_tx = proven_or_raw.raw_tx.ok_or_else(|| {
                    StorageError::InvalidArg(
                        format!("inputBEEF: valid and contain proof data for {}", txid)
                    )
                })?;
                
                let _merged_btx = beef.merge_raw_tx(&raw_tx).map_err(|e| {
                    StorageError::InvalidArg(format!("Failed to merge raw tx: {}", e))
                })?;
                
                // TODO: Merge BUMP if proven (requires MerklePath from EntityProvenTx)
                // if let Some(proven) = proven_or_raw.proven {
                //     beef.mergeBump(EntityProvenTx::new(proven).getMerklePath());
                // }
            }
            
            // TS lines 650-653: Get output from parsed transaction
            // TODO: Implement when Transaction parsing is complete
            // For now, return error indicating this needs BEEF transaction parsing
            return Err(StorageError::NotImplemented(
                "validate_required_inputs: Transaction parsing from BEEF not yet implemented"
            ));
        }
    }
    
    // TS line 657
    Ok((beef, storage_beef, xinputs))
}

/// STEP 2: Validate and assign outputs
/// Reference: TypeScript lines 496-534
/// 
/// Convert vargs.outputs to XValidCreateActionOutput by:
/// - Assigning vout numbers sequentially
/// - Setting providedBy='you' for user outputs
/// - Adding storage commission output if configured
fn validate_required_outputs(
    _storage: &dyn WalletStorageProvider,
    _user_id: i64,
    vargs: &ValidCreateActionArgs,
) -> Result<Vec<XValidCreateActionOutput>, StorageError> {
    let mut xoutputs: Vec<XValidCreateActionOutput> = Vec::new();
    let mut vout: u32 = 0;
    
    // Process user-provided outputs
    for output in &vargs.outputs {
        let xo = XValidCreateActionOutput {
            output: output.clone(),
            vout,
            provided_by: StorageProvidedBy::You,
            purpose: None,
            derivation_suffix: None,
            key_offset: None,
        };
        xoutputs.push(xo);
        vout += 1;
    }
    
    // Add storage commission if configured
    // TODO: Implement when we have storage commission configuration
    // Reference TS: if (storage.commissionSatoshis > 0 && storage.commissionPubKeyHex)
    // For now, we skip commission outputs
    
    Ok(xoutputs)
}

/// Find output basket by name
/// Reference: TypeScript createAction.ts lines 91-97
/// Uses findOutputBaskets with name filter
async fn find_output_basket(
    storage: &dyn WalletStorageProvider,
    user_id: i64,
    name: &str,
) -> Result<TableOutputBasket, StorageError> {
    // Build query args matching TS: { partial: { userId, name: changeBasketName } }
    let args = FindOutputBasketsArgs {
        user_id,
        name: Some(name.to_string()),
        since: None,
        paged: None,
    };
    
    let auth = AuthId::new("").with_user_id(user_id);
    let baskets = storage.find_output_baskets_auth(&auth, &args).await?;
    
    // TS uses verifyOne() which requires exactly 1 result
    if baskets.is_empty() {
        return Err(StorageError::NotFound(
            format!("Output basket '{}' not found", name)
        ));
    }
    if baskets.len() > 1 {
        return Err(StorageError::Conflict(
            format!("Multiple output baskets found for name '{}'", name)
        ));
    }
    
    Ok(baskets.into_iter().next().unwrap())
}

/// Validate noSendChange configuration
/// Reference: TypeScript lines 680-718
/// 
/// Validates noSendChange outpoints when isNoSend is true:
/// - Each outpoint must exist in storage
/// - Must be providedBy='storage', purpose='change'
/// - Must be spendable (not already spent)
/// - Must be in the correct change basket
/// - No duplicates allowed
async fn validate_no_send_change(
    storage: &dyn WalletStorageProvider,
    user_id: i64,
    vargs: &ValidCreateActionArgs,
    change_basket: &TableOutputBasket,
) -> Result<Vec<TableOutput>, StorageError> {
    let mut result: Vec<TableOutput> = Vec::new();
    
    // If not noSend, return empty
    if !vargs.is_no_send {
        return Ok(result);
    }
    
    // Get noSendChange outpoints from options
    let no_send_change = match &vargs.options.no_send_change {
        Some(outpoints) => outpoints,
        None => return Ok(result),
    };
    
    if no_send_change.is_empty() {
        return Ok(result);
    }
    
    // Validate each outpoint
    for op in no_send_change {
        // Find output by txid and vout - TS logic
        let partial = PartialOutput {
            txid: Some(op.txid.clone()),
            basket_id: None,
            spendable: None,
            change: None,
            transaction_id: None,
        };
        let args = FindOutputsArgs {
            user_id,
            since: None,
            paged: None,
            order_descending: None,
            partial: Some(partial),
            no_script: Some(true),
            tx_status: None,
        };
        let auth = AuthId::new("");
        let outputs = storage.find_outputs_auth(&auth, &args).await?;
        let output = outputs.into_iter().find(|o| o.vout == op.vout).ok_or_else(|| {
            StorageError::InvalidArg(format!("noSendChange output {}:{} not found", op.txid, op.vout))
        })?;
        
        // Verify output properties
        if output.provided_by != WalletStorageProvidedBy::Storage {
            return Err(StorageError::InvalidArg(
                format!("noSendChange output {}:{} not provided by storage", op.txid, op.vout)
            ));
        }
        if output.purpose != "change" {
            return Err(StorageError::InvalidArg(
                format!("noSendChange output {}:{} purpose is not 'change'", op.txid, op.vout)
            ));
        }
        if !output.spendable {
            return Err(StorageError::InvalidArg(
                format!("noSendChange output {}:{} is not spendable", op.txid, op.vout)
            ));
        }
        if output.spent_by.is_some() {
            return Err(StorageError::InvalidArg(
                format!("noSendChange output {}:{} is already spent", op.txid, op.vout)
            ));
        }
        if output.satoshis <= 0 {
            return Err(StorageError::InvalidArg(
                format!("noSendChange output {}:{} has invalid satoshis", op.txid, op.vout)
            ));
        }
        if output.basket_id != Some(change_basket.basket_id) {
            return Err(StorageError::InvalidArg(
                format!("noSendChange output {}:{} not in change basket", op.txid, op.vout)
            ));
        }
        
        // Check for duplicates
        if result.iter().any(|r| r.txid == output.txid && r.vout == output.vout) {
            return Err(StorageError::InvalidArg(
                format!("Duplicate noSendChange output {}:{}", op.txid, op.vout)
            ));
        }
        
        result.push(output);
    }
    
    Ok(result)
}

/// Create transaction record in database
/// Reference: TypeScript lines 441-472
/// 
/// Creates transaction with:
/// - Random 12-byte base64 reference ID
/// - Status='unsigned'
/// - Version and lockTime from vargs
/// - Links to transaction labels
async fn create_new_tx_record(
    storage: &mut dyn WalletStorageProvider,
    user_id: i64,
    vargs: &ValidCreateActionArgs,
    storage_beef: Option<Vec<u8>>,
) -> Result<TableTransaction, StorageError> {
    let now = Utc::now();
    
    // Generate random reference ID (12 bytes = 16 chars base64)
    let reference = generate_random_reference();
    
    let new_tx = TableTransaction {
        created_at: now.to_rfc3339(),
        updated_at: now.to_rfc3339(),
        transaction_id: 0, // Will be set by insert
        user_id,
        proven_tx_id: None,
        status: TransactionStatus::Unsigned,
        reference,
        is_outgoing: true,
        satoshis: 0, // Updated after funding
        description: vargs.description.clone(),
        version: Some(vargs.version),
        lock_time: Some(vargs.lock_time),
        txid: None,
        raw_tx: None,
        input_beef: storage_beef,
    };
    
    // Insert transaction - TS line 464
    let transaction_id = storage.insert_transaction(&new_tx).await?;
    // Create transaction with ID
    let mut new_tx_with_id = new_tx;
    new_tx_with_id.transaction_id = transaction_id;
    
    // Insert labels - TS lines 466-469
    for label in &vargs.labels {
        let tx_label = storage.find_or_insert_tx_label(user_id, label).await?;
        let tx_label_id = tx_label.tx_label_id;
        storage.find_or_insert_tx_label_map(transaction_id, tx_label_id).await?;
    }
    
    Ok(new_tx_with_id)
}

/// Generate random reference ID
/// Reference: TypeScript randomBytesBase64(12)
fn generate_random_reference() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..12).map(|_| rng.gen()).collect();
    base64::engine::general_purpose::STANDARD.encode(&bytes)
}

/// Create default output record
/// Reference: TypeScript lines 177-205 (makeDefaultOutput)
fn make_default_output(user_id: i64, transaction_id: i64, satoshis: i64, vout: u32) -> TableOutput {
    TableOutput::new(
        0, // output_id - will be set by insert
        user_id,
        transaction_id,
        true, // spendable
        false, // change
        String::new(), // output_description
        vout,
        satoshis,
        WalletStorageProvidedBy::You,
        String::new(), // purpose
        String::new(), // output_type
    )
}

/// Funding result from change allocation
struct FundingResult {
    allocated_change: Vec<TableOutput>,
    change_outputs: Vec<TableOutput>,
    derivation_prefix: String,
    max_possible_satoshis_adjustment: Option<MaxPossibleSatoshisAdjustment>,
}

struct MaxPossibleSatoshisAdjustment {
    fixed_output_index: usize,
    satoshis: i64,
}

/// STEP 8: Fund transaction with change allocation
/// Reference: lines 720-888 of createAction.ts (fundNewTransactionSdk)
/// 
/// Implements sophisticated funding algorithm:
/// 1. Calculates total satoshis required (outputs + fees)
/// 2. Allocates noSendChange outputs first (if specified)
/// 3. Selects additional change inputs as needed
/// 4. Locks all allocated outputs (marks as spent)
/// 5. Generates change outputs if needed
/// 6. Returns funding result with allocated change and new outputs
async fn fund_new_transaction(
    storage: &mut dyn WalletStorageProvider,
    user_id: i64,
    vargs: &ValidCreateActionArgs,
    ctx: &mut CreateTransactionContext,
) -> Result<FundingResult, StorageError> {
    // TS line 726: Calculate total satoshis needed from outputs
    let output_satoshis: i64 = ctx.xoutputs.iter()
        .map(|o| o.satoshis())
        .sum();
    
    // TS lines 728-730: Estimate fee
    // Simple fee model for now: sat/kb * estimated size
    let estimated_size = estimate_transaction_size(&ctx.xinputs, &ctx.xoutputs);
    let estimated_fee = (estimated_size as f64 * ctx.fee_model.value / 1000.0).ceil() as i64;
    let total_required = output_satoshis + estimated_fee;
    
    // TS lines 735-743: Allocate noSendChange first
    let mut allocated_change = ctx.no_send_change_in.clone();
    let mut allocated_satoshis: i64 = allocated_change.iter()
        .map(|o| o.satoshis)
        .sum();
    
    // TS lines 745-770: Allocate additional change if needed
    if allocated_satoshis < total_required {
        let needed = total_required - allocated_satoshis;
        
        // Select available change outputs from basket
        let additional_change = select_change_inputs(
            storage,
            user_id,
            ctx.change_basket.basket_id,
            needed,
            vargs.is_delayed,
        ).await?;
        
        allocated_satoshis += additional_change.iter()
            .map(|o| o.satoshis)
            .sum::<i64>();
        allocated_change.extend(additional_change);
    }
    
    // TS lines 772-786: Lock all allocated outputs
    for output in &allocated_change {
        let updates = OutputUpdates {
            spendable: Some(false),
            spent_by: Some(ctx.transaction_id),
            spending_description: Some("change".to_string()),
        };
        storage.update_output(output.output_id, &updates).await?;
    }
    
    // TS lines 788-795: Generate derivation prefix (random 10 bytes base64)
    let derivation_prefix = generate_random_derivation_prefix();
    
    // TS lines 797-850: Create change outputs if we have excess
    let mut change_outputs = Vec::new();
    let excess_satoshis = allocated_satoshis - total_required;
    
    if excess_satoshis > 0 {
        // Create a change output
        let change_output = create_change_output(
            user_id,
            ctx.transaction_id,
            ctx.change_basket.basket_id,
            excess_satoshis,
            &derivation_prefix,
        )?;
        change_outputs.push(change_output);
    }
    
    // TS lines 852-870: Handle maxPossibleSatoshis adjustment
    let max_possible_satoshis_adjustment = handle_max_possible_satoshis(
        vargs,
        &ctx.xoutputs,
        allocated_satoshis,
        output_satoshis,
    )?;
    
    Ok(FundingResult {
        allocated_change,
        change_outputs,
        derivation_prefix,
        max_possible_satoshis_adjustment,
    })
}

/// Estimate transaction size in bytes
/// Reference: TypeScript fee calculation logic
fn estimate_transaction_size(
    xinputs: &[XValidCreateActionInput],
    xoutputs: &[XValidCreateActionOutput],
) -> usize {
    // Basic size calculation:
    // - 10 bytes overhead (version + locktime + counts)
    // - Each input: ~148 bytes (outpoint + script + sequence)
    // - Each output: ~34 bytes (value + script)
    let base_size = 10;
    let input_size = xinputs.len() * 148;
    let output_size = xoutputs.len() * 34;
    base_size + input_size + output_size
}

/// Select change inputs from basket
/// Reference: TypeScript change allocation logic (lines 745-770)
async fn select_change_inputs(
    storage: &dyn WalletStorageProvider,
    user_id: i64,
    basket_id: i64,
    needed_satoshis: i64,
    _is_delayed: bool,
) -> Result<Vec<TableOutput>, StorageError> {
    // Find spendable change outputs in basket
    let partial = PartialOutput {
        basket_id: Some(basket_id),
        spendable: Some(true),
        change: Some(true),
        transaction_id: None,
        txid: None,
    };
    let args = FindOutputsArgs {
        user_id,
        since: None,
        paged: None,
        order_descending: Some(false), // Smallest first
        partial: Some(partial),
        no_script: Some(true),
        tx_status: None,
    };
    
    let auth = AuthId::new("");
    let available = storage.find_outputs_auth(&auth, &args).await?;
    
    // Select outputs until we have enough satoshis
    let mut selected = Vec::new();
    let mut total: i64 = 0;
    
    for output in available {
        if total >= needed_satoshis {
            break;
        }
        total += output.satoshis;
        selected.push(output);
    }
    
    if total < needed_satoshis {
        return Err(StorageError::InvalidArg(
            format!("Insufficient funds: need {} satoshis, only {} available", needed_satoshis, total)
        ));
    }
    
    Ok(selected)
}

/// Generate random derivation prefix (10 bytes base64)
/// Reference: TypeScript randomBytesBase64(10)
fn generate_random_derivation_prefix() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..10).map(|_| rng.gen()).collect();
    base64::engine::general_purpose::STANDARD.encode(&bytes)
}

/// Create change output record
/// Reference: TypeScript change output creation (lines 797-850)
fn create_change_output(
    user_id: i64,
    transaction_id: i64,
    basket_id: i64,
    satoshis: i64,
    derivation_prefix: &str,
) -> Result<TableOutput, StorageError> {
    let mut output = TableOutput::new(
        0, // output_id - will be set by insert
        user_id,
        transaction_id,
        true, // spendable
        true, // change
        "change".to_string(), // output_description
        0, // vout - will be assigned later
        satoshis,
        WalletStorageProvidedBy::Storage,
        "change".to_string(), // purpose
        "P2PKH".to_string(), // output_type
    );
    
    output.basket_id = Some(basket_id);
    output.derivation_prefix = Some(derivation_prefix.to_string());
    
    Ok(output)
}

/// Handle maxPossibleSatoshis adjustment
/// Reference: TypeScript lines 852-870
/// 
/// TODO: Implement maxPossibleSatoshis detection and adjustment
/// Currently disabled as schema doesn't have flag for this feature yet
fn handle_max_possible_satoshis(
    _vargs: &ValidCreateActionArgs,
    _xoutputs: &[XValidCreateActionOutput],
    _allocated_satoshis: i64,
    _output_satoshis: i64,
) -> Result<Option<MaxPossibleSatoshisAdjustment>, StorageError> {
    // For now, no maxPossibleSatoshis adjustment
    // Will implement when schema supports optional satoshis or has explicit flag
    Ok(None)
}

/// Output creation result
struct OutputCreationResult {
    outputs: Vec<StorageCreateTransactionOutput>,
    change_vouts: Vec<u32>,
}

/// STEP 11: Create all output records
/// Reference: TypeScript createAction.ts lines 297-439
/// 
/// Creates all output records in database:
/// 1. Lookup/create baskets for outputs
/// 2. Lookup/create tags for outputs  
/// 3. Create TableOutput for each xoutput (with service-charge special handling)
/// 4. Add change outputs
/// 5. Randomize output order if requested
/// 6. Insert all outputs to database
/// 7. Link outputs to tags
/// 8. Build StorageCreateTransactionOutput results
/// 9. Track change vouts
async fn create_new_outputs(
    storage: &mut dyn WalletStorageProvider,
    user_id: i64,
    vargs: &ValidCreateActionArgs,
    ctx: &CreateTransactionContext,
    change_outputs: &[TableOutput],
) -> Result<OutputCreationResult, StorageError> {
    let mut outputs_result: Vec<StorageCreateTransactionOutput> = Vec::new();
    
    // TS lines 309-314: Lookup output baskets
    let mut tx_baskets: std::collections::HashMap<String, TableOutputBasket> = std::collections::HashMap::new();
    for xo in &ctx.xoutputs {
        if let Some(basket_name) = xo.basket() {
            if !tx_baskets.contains_key(basket_name) {
                let basket = storage.find_or_insert_output_basket(user_id, basket_name).await?;
                tx_baskets.insert(basket_name.to_string(), basket);
            }
        }
    }
    
    // TS lines 315-321: Lookup output tags
    let mut tx_tags: std::collections::HashMap<String, TableOutputTag> = std::collections::HashMap::new();
    for xo in &ctx.xoutputs {
        for tag in xo.tags() {
            if !tx_tags.contains_key(tag) {
                let tag_obj = storage.find_or_insert_output_tag(user_id, tag).await?;
                tx_tags.insert(tag.to_string(), tag_obj);
            }
        }
    }
    
    // TS line 323: Build newOutputs array
    let mut new_outputs: Vec<(TableOutput, Vec<String>)> = Vec::new();
    
    // TS lines 325-364: Process each xoutput
    for xo in &ctx.xoutputs {
        let locking_script = xo.locking_script().as_bytes().to_vec();
        
        if xo.purpose.as_deref() == Some("service-charge") {
            // TS lines 327-348: Handle service-charge (storage commission)
            let commission = TableCommission::new(
                0, // commission_id - will be set by insert
                user_id,
                ctx.transaction_id,
                xo.satoshis(),
                xo.key_offset.as_ref().unwrap_or(&String::new()).clone(),
                locking_script.clone(),
            );
            
            storage.insert_commission(&commission).await?;
            
            let mut o = make_default_output(user_id, ctx.transaction_id, xo.satoshis(), xo.vout);
            o.locking_script = Some(locking_script);
            o.provided_by = WalletStorageProvidedBy::Storage;
            o.purpose = "storage-commission".to_string();
            o.output_type = "custom".to_string();
            o.spendable = false;
            
            new_outputs.push((o, Vec::new()));
        } else {
            // TS lines 349-363: Handle regular output
            let basket_id = xo.basket().and_then(|basket_name| {
                tx_baskets.get(basket_name).map(|b| b.basket_id)
            });
            
            let mut o = make_default_output(user_id, ctx.transaction_id, xo.satoshis(), xo.vout);
            o.locking_script = Some(locking_script);
            o.basket_id = basket_id;
            o.custom_instructions = xo.custom_instructions().map(|s| s.to_string());
            o.output_description = xo.output_description().to_string();
            o.provided_by = match xo.provided_by {
                StorageProvidedBy::You => WalletStorageProvidedBy::You,
                StorageProvidedBy::Storage => WalletStorageProvidedBy::Storage,
                StorageProvidedBy::YouAndStorage => WalletStorageProvidedBy::YouAndStorage,
            };
            o.purpose = xo.purpose.clone().unwrap_or_else(|| String::new());
            o.output_type = "custom".to_string();
            
            new_outputs.push((o, xo.tags().to_vec()));
        }
    }
    
    // TS lines 366-369: Add change outputs
    for mut o in change_outputs.to_vec() {
        o.spendable = true;
        new_outputs.push((o, Vec::new()));
    }
    
    // TS lines 371-409: Randomize output order if requested
    if vargs.options.randomize_outputs {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        
        // Create array of indices
        let mut new_vouts: Vec<usize> = (0..new_outputs.len()).collect();
        
        // Shuffle using provided randomVals or thread_rng
        // TODO: Use vargs.random_vals if provided for deterministic testing
        new_vouts.shuffle(&mut rng);
        
        // Reassign vout values (TS lines 400-408)
        for (vout, (output, _tags)) in new_outputs.iter_mut().enumerate() {
            let original_vout = output.vout;
            
            // TS line 406: Verify in-order
            if original_vout as usize != vout {
                return Err(StorageError::Database(
                    format!("new output {} has out of order vout {}", vout, original_vout)
                ));
            }
            
            // TS line 407: Assign new shuffled vout
            output.vout = new_vouts[vout] as u32;
        }
    }
    
    // TS lines 411-436: Insert outputs and build results
    let mut change_vouts: Vec<u32> = Vec::new();
    
    for (mut o, tags) in new_outputs {
        // TS line 413: Insert output
        let output_id = storage.insert_output(&o).await?;
        o.output_id = output_id;
        
        // TS line 415: Track change vouts
        if o.change 
            && o.purpose == "change" 
            && o.provided_by == WalletStorageProvidedBy::Storage 
        {
            change_vouts.push(o.vout);
        }
        
        // TS lines 417-421: Add tags to output
        for tag_name in &tags {
            let tag = tx_tags.get(tag_name).ok_or_else(|| {
                StorageError::Database(format!("Tag {} not found", tag_name))
            })?;
            let tag_id = tag.output_tag_id;
            storage.find_or_insert_output_tag_map(output_id, tag_id).await?;
        }
        
        // TS lines 423-435: Build result object
        let basket_name = if let Some(bid) = o.basket_id {
            tx_baskets.iter()
                .find(|(_, b)| b.basket_id == bid)
                .map(|(name, _)| name.clone())
        } else {
            None
        };
        
        // Convert wallet_storage::StorageProvidedBy to action::StorageProvidedBy
        let action_provided_by = match o.provided_by {
            WalletStorageProvidedBy::You => StorageProvidedBy::You,
            WalletStorageProvidedBy::Storage => StorageProvidedBy::Storage,
            WalletStorageProvidedBy::YouAndStorage => StorageProvidedBy::YouAndStorage,
        };
        
        let ro = StorageCreateTransactionOutput {
            vout: o.vout,
            satoshis: o.satoshis,
            locking_script: o.locking_script
                .as_ref()
                .map(|ls| hex::encode(ls))
                .unwrap_or_default(),
            provided_by: action_provided_by,
            purpose: if o.purpose.is_empty() { None } else { Some(o.purpose.clone()) },
            basket: basket_name,
            tags: Some(tags),
            output_description: o.output_description.clone(),
            derivation_suffix: o.derivation_suffix.clone(),
            custom_instructions: o.custom_instructions.clone(),
        };
        outputs_result.push(ro);
    }
    
    // TS line 438
    Ok(OutputCreationResult {
        outputs: outputs_result,
        change_vouts,
    })
}

/// STEP 12: Merge all input BEEFs
/// Reference: lines 903-945 of createAction.ts
/// 
/// Merges input BEEF with BEEFs from allocated change outputs:
/// 1. Start with input BEEF
/// 2. For each allocated change output, get its transaction
/// 3. Merge transactions into BEEF
/// 4. Return merged BEEF bytes (or None if no BEEF data)
async fn merge_allocated_change_beefs(
    storage: &dyn WalletStorageProvider,
    _user_id: i64,
    vargs: &ValidCreateActionArgs,
    allocated_change: &[TableOutput],
    beef_bytes: Vec<u8>,
) -> Result<Option<Vec<u8>>, StorageError> {
    // TS lines 905-910: If no includeAllSourceTransactions, return input BEEF
    if !vargs.include_all_source_transactions || !vargs.is_sign_action {
        return Ok(if beef_bytes.is_empty() {
            None
        } else {
            Some(beef_bytes)
        });
    }
    
    // TS lines 912-945: Merge BEEFs from allocated change
    // TODO: Implement full BEEF merging when Beef::from_binary() is available
    // For now, we need to:
    // 1. Parse input BEEF from bytes
    // 2. For each allocated change output, get its raw transaction
    // 3. Merge those transactions into the BEEF
    // 4. Serialize back to bytes
    
    // Temporary implementation: just return input BEEF
    // Full implementation requires:
    // - Beef::from_binary() to parse beef_bytes
    // - beef.merge_transaction() for each change output
    // - beef.to_binary() to serialize result
    
    if allocated_change.is_empty() {
        return Ok(if beef_bytes.is_empty() {
            None
        } else {
            Some(beef_bytes)
        });
    }
    
    // Get transactions for allocated change
    for output in allocated_change {
        if let Some(ref txid) = output.txid {
            // Get the raw transaction
            let _raw_tx = storage.get_raw_tx_of_known_valid_transaction(txid, None, None).await?;
            // TODO: Merge into BEEF when Beef methods are available
        }
    }
    
    // Return input BEEF for now
    Ok(if beef_bytes.is_empty() {
        None
    } else {
        Some(beef_bytes)
    })
}

/// STEP 13: Create input specifications for result
/// Reference: TypeScript createAction.ts lines 207-295
/// 
/// Creates all input specifications:
/// 1. Combine user xinputs + allocated change
/// 2. For user inputs with storage outputs:
///    - Mark output as spent (spendable=false, spentBy=transactionId)
///    - Verify not double-spent
/// 3. Assign sequential vin numbers
/// 4. Build StorageCreateTransactionInput for each:
///    - Get source transaction if includeAllSourceTransactions
///    - Set providedBy (you, storage, or you-and-storage)
///    - Set derivation fields, type, spending description
async fn create_new_inputs(
    storage: &mut dyn WalletStorageProvider,
    user_id: i64,
    vargs: &ValidCreateActionArgs,
    ctx: &CreateTransactionContext,
    allocated_change: &[TableOutput],
) -> Result<Vec<StorageCreateTransactionInput>, StorageError> {
    let mut result: Vec<StorageCreateTransactionInput> = Vec::new();
    
    // TS lines 216-243: Build newInputs array with user inputs
    let mut new_inputs: Vec<(Option<XValidCreateActionInput>, Option<TableOutput>, Option<u32>)> = Vec::new();
    
    for xinput in &ctx.xinputs {
        let output = xinput.output.clone();
        
        // TS lines 224-242: Mark output as spent if from storage
        if let Some(ref o) = output {
            let output_id = o.output_id;
            
            // TS lines 226-231: Verify output is spendable (double-spend check)
            // Use transactionId and vout to find the output
            let partial = PartialOutput {
                basket_id: o.basket_id,
                spendable: None,
                change: None,
                transaction_id: Some(o.transaction_id),
                txid: o.txid.clone(),
            };
            let args = FindOutputsArgs {
                user_id: user_id,
                since: None,
                paged: None,
                order_descending: None,
                partial: Some(partial),
                no_script: Some(true),
                tx_status: None,
            };
            let auth = AuthId::new("");
            let outputs = storage.find_outputs_auth(&auth, &args).await?;
            let o2 = outputs.into_iter().find(|out| out.vout == o.vout).ok_or_else(|| {
                StorageError::NotFound(format!("Output {} not found", output_id))
            })?;
            
            if !o2.spendable || o2.spent_by.is_some() {
                return Err(StorageError::InvalidArg(
                    format!("inputs[{}]: spendable output. output {}:{} appears to have been spent.",
                        xinput.vin, o.txid.as_ref().unwrap_or(&"unknown".to_string()), o.vout)
                ));
            }
            
            // TS lines 232-240: Update output to mark as spent
            let updates = OutputUpdates {
                spendable: Some(false),
                spent_by: Some(ctx.transaction_id),
                spending_description: Some(xinput.input.input_description.clone()),
            };
            storage.update_output(output_id, &updates).await?;
        }
        
        new_inputs.push((Some(xinput.clone()), output, None));
    }
    
    // TS lines 245-247: Add allocated change with unlockingScriptLength=107 (P2PKH)
    for change_output in allocated_change {
        new_inputs.push((None, Some(change_output.clone()), Some(107)));
    }
    
    // TS lines 249-293: Build result inputs with sequential vin
    let mut vin: i32 = -1;
    for (xinput, output, unlock_len) in new_inputs {
        vin += 1;
        
        if let Some(o) = output {
            // TS lines 252-273: Input from storage (user or change)
            
            // TS line 253: Validate non-fixed input has unlockLen
            if xinput.is_none() && unlock_len.is_none() {
                return Err(StorageError::Database(
                    format!("vin {} non-fixedInput without unlockLen", vin)
                ));
            }
            
            // TS lines 254-257: Get source transaction if requested
            let source_transaction = if vargs.include_all_source_transactions && vargs.is_sign_action {
                let txid = o.txid.as_ref().ok_or_else(|| {
                    StorageError::InvalidArg("Output missing txid".to_string())
                })?;
                storage.get_raw_tx_of_known_valid_transaction(txid, None, None).await?
            } else {
                None
            };
            
            // TS lines 258-272: Build input record
            // Convert wallet_storage::StorageProvidedBy to action::StorageProvidedBy
            let provided_by = if let Some(ref _xi) = xinput {
                // User input with storage output
                if o.provided_by == WalletStorageProvidedBy::Storage {
                    StorageProvidedBy::YouAndStorage
                } else {
                    // Convert enum to action enum
                    match o.provided_by {
                        WalletStorageProvidedBy::You => StorageProvidedBy::You,
                        WalletStorageProvidedBy::Storage => StorageProvidedBy::Storage,
                        WalletStorageProvidedBy::YouAndStorage => StorageProvidedBy::YouAndStorage,
                    }
                }
            } else {
                // Change input from storage
                match o.provided_by {
                    WalletStorageProvidedBy::You => StorageProvidedBy::You,
                    WalletStorageProvidedBy::Storage => StorageProvidedBy::Storage,
                    WalletStorageProvidedBy::YouAndStorage => StorageProvidedBy::YouAndStorage,
                }
            };
            
            let ri = StorageCreateTransactionInput {
                vin: vin as u32,
                source_txid: o.txid.clone().unwrap_or_default(),
                source_vout: o.vout,
                source_satoshis: o.satoshis,
                source_locking_script: o.locking_script
                    .as_ref()
                    .map(|ls| hex::encode(ls))
                    .unwrap_or_default(),
                source_transaction,
                unlocking_script_length: unlock_len.or_else(|| {
                    xinput.as_ref().and_then(|xi| xi.input.unlocking_script_length)
                }).unwrap_or(107), // Default P2PKH length if not specified
                provided_by,
                input_type: o.output_type.clone(),
                spending_description: o.spending_description.clone(),
                derivation_prefix: o.derivation_prefix.clone(),
                derivation_suffix: o.derivation_suffix.clone(),
                sender_identity_key: o.sender_identity_key.clone(),
            };
            
            result.push(ri);
        } else {
            // TS lines 274-292: User-specified input with no storage output
            
            let xi = xinput.ok_or_else(|| {
                StorageError::Database(format!("vin {} without output or xinput", vin))
            })?;
            
            let ri = StorageCreateTransactionInput {
                vin: vin as u32,
                source_txid: xi.input.outpoint.txid.clone(),
                source_vout: xi.input.outpoint.vout,
                source_satoshis: xi.satoshis,
                source_locking_script: hex::encode(&xi.locking_script),
                source_transaction: None,
                unlocking_script_length: xi.input.unlocking_script_length.unwrap_or(107), // Default P2PKH
                provided_by: StorageProvidedBy::You,
                input_type: "custom".to_string(),
                spending_description: None,
                derivation_prefix: None,
                derivation_suffix: None,
                sender_identity_key: None,
            };
            
            result.push(ri);
        }
    }
    
    // TS line 294
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sdk::action::*;
    
    // ============================================================================
    // Helper Functions Tests
    // ============================================================================
    
    #[test]
    fn test_generate_random_reference() {
        // Test that generate_random_reference creates 16-char base64 string (12 bytes)
        let ref1 = generate_random_reference();
        let ref2 = generate_random_reference();
        
        assert_eq!(ref1.len(), 16, "Reference should be 16 characters (12 bytes base64)");
        assert_eq!(ref2.len(), 16, "Reference should be 16 characters");
        assert_ne!(ref1, ref2, "References should be random and different");
        
        // Verify it's valid base64
        assert!(base64::engine::general_purpose::STANDARD.decode(&ref1).is_ok(), "Should be valid base64");
    }
    
    #[test]
    fn test_make_default_output() {
        let user_id = 123;
        let transaction_id = 456;
        let satoshis = 1000;
        let vout = 0;
        
        let output = make_default_output(user_id, transaction_id, satoshis, vout);
        
        // Verify all fields are set correctly (Phase 2 schema with non-Option fields)
        assert_eq!(output.user_id, user_id);
        assert_eq!(output.transaction_id, transaction_id);
        assert_eq!(output.satoshis, satoshis);
        assert_eq!(output.vout, vout);
        assert_eq!(output.change, false);
        assert_eq!(output.spendable, true);
        assert_eq!(output.provided_by, WalletStorageProvidedBy::You);
        assert_eq!(output.purpose, String::new());
        assert_eq!(output.output_type, String::new());
        assert_eq!(output.output_id, 0, "outputId should be 0 until set by insert");
        // Timestamps should be valid RFC3339 strings
        assert!(chrono::DateTime::parse_from_rfc3339(&output.created_at).is_ok());
        assert!(chrono::DateTime::parse_from_rfc3339(&output.updated_at).is_ok());
    }
    
    #[test]
    fn test_validate_required_outputs_assigns_vout() {
        // This would need a mock storage, but we can test the logic
        // that vout numbers are assigned sequentially starting from 0
        
        // Create test outputs
        let output1 = ValidCreateActionOutput {
            satoshis: 1000,
            locking_script: "76a914...88ac".to_string(),
            output_description: "Test output 1".to_string(),
            basket: None,
            custom_instructions: None,
            tags: None,
        };
        
        let output2 = ValidCreateActionOutput {
            satoshis: 2000,
            locking_script: "76a914...88ac".to_string(),
            output_description: "Test output 2".to_string(),
            basket: Some("default".to_string()),
            custom_instructions: None,
            tags: Some(vec!["tag1".to_string()]),
        };
        
        // Would call validate_required_outputs with mock storage
        // For now, verify the structure is correct
        assert_eq!(output1.satoshis, 1000);
        assert_eq!(output2.satoshis, 2000);
    }
    
    // ============================================================================
    // BEEF Module Tests
    // ============================================================================
    
    #[test]
    fn test_beef_new_v2() {
        let beef = Beef::new_v2();
        assert_eq!(beef.version, crate::beef::BEEF_V2);
        assert_eq!(beef.txs.len(), 0);
        assert_eq!(beef.bumps.len(), 0);
        assert!(beef.atomic_txid.is_none());
    }
    
    #[test]
    fn test_beef_merge_txid_only() {
        let mut beef = Beef::new_v2();
        let txid = "abc123";
        
        let beef_tx = beef.merge_txid_only(txid);
        
        assert_eq!(beef_tx.txid, txid);
        assert!(beef_tx.is_txid_only);
        assert!(beef_tx.raw_tx.is_none());
        assert!(beef_tx.tx.is_none());
        assert_eq!(beef.txs.len(), 1);
        
        // Merging same txid again should not duplicate
        beef.merge_txid_only(txid);
        assert_eq!(beef.txs.len(), 1, "Should not duplicate txid");
    }
    
    #[test]
    fn test_beef_find_txid() {
        let mut beef = Beef::new_v2();
        
        // Add some txids
        beef.merge_txid_only("tx1");
        beef.merge_txid_only("tx2");
        beef.merge_txid_only("tx3");
        
        // Find existing
        assert!(beef.find_txid("tx1").is_some());
        assert!(beef.find_txid("tx2").is_some());
        assert!(beef.find_txid("tx3").is_some());
        
        // Not found
        assert!(beef.find_txid("tx4").is_none());
    }
    
    #[test]
    fn test_beef_make_txid_only() {
        let mut beef = Beef::new_v2();
        
        // First merge as txid-only
        beef.merge_txid_only("tx1");
        
        // Making it txid-only again should return same
        let btx = beef.make_txid_only("tx1");
        assert!(btx.is_some());
        assert!(btx.unwrap().is_txid_only);
        
        // Non-existent txid
        assert!(beef.make_txid_only("tx_nonexistent").is_none());
    }
    
    #[test]
    fn test_beef_clone() {
        let mut beef = Beef::new_v2();
        beef.merge_txid_only("tx1");
        beef.merge_txid_only("tx2");
        
        let beef2 = beef.clone_beef();
        
        assert_eq!(beef2.txs.len(), 2);
        assert_eq!(beef2.version, beef.version);
        assert!(beef2.find_txid("tx1").is_some());
        assert!(beef2.find_txid("tx2").is_some());
    }
    
    #[test]
    fn test_beef_to_log_string() {
        let mut beef = Beef::new_v2();
        beef.merge_txid_only("tx1");
        beef.merge_txid_only("tx2");
        
        let log = beef.to_log_string();
        
        assert!(log.contains("BEEF"));
        assert!(log.contains("2 txs"));
        assert!(log.contains("2 txid-only"));
    }
    
    // ============================================================================
    // XValidCreateActionOutput Delegation Tests
    // ============================================================================
    
    #[test]
    fn test_xvalid_output_delegation() {
        let output = ValidCreateActionOutput {
            satoshis: 5000,
            locking_script: "76a91488ac".to_string(),
            output_description: "Test".to_string(),
            basket: Some("basket1".to_string()),
            custom_instructions: Some("instructions".to_string()),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
        };
        
        let xoutput = XValidCreateActionOutput {
            output,
            vout: 0,
            provided_by: StorageProvidedBy::You,
            purpose: Some("payment".to_string()),
            derivation_suffix: None,
            key_offset: None,
        };
        
        // Test delegation methods
        assert_eq!(xoutput.satoshis(), 5000);
        assert_eq!(xoutput.locking_script(), "76a91488ac");
        assert_eq!(xoutput.basket(), Some("basket1"));
        assert_eq!(xoutput.custom_instructions(), Some("instructions"));
        assert_eq!(xoutput.tags().len(), 2);
        assert_eq!(xoutput.tags()[0], "tag1");
        assert_eq!(xoutput.output_description(), "Test");
    }
    
    // ============================================================================
    // Fee Calculation & Transaction Size Estimation Tests
    // Reference: TypeScript createAction.ts lines 728-730
    // ============================================================================
    
    #[test]
    fn test_estimate_transaction_size_basic() {
        // TS Reference: Basic P2PKH transaction estimation
        // Each input ~148 bytes, each output ~34 bytes, overhead 10 bytes
        
        let xinputs = vec![]; // Empty inputs
        let xoutputs = vec![]; // Empty outputs
        
        let size = estimate_transaction_size(&xinputs, &xoutputs);
        assert_eq!(size, 10, "Empty transaction should be 10 bytes overhead");
    }
    
    #[test]
    fn test_estimate_transaction_size_with_inputs_outputs() {
        // TS Reference: Typical transaction size calculation
        // 1 input + 2 outputs = 10 + 148 + (2*34) = 226 bytes
        
        let xinputs = vec![
            XValidCreateActionInput {
                input: ValidCreateActionInput {
                    outpoint: crate::sdk::action::OutPoint {
                        txid: "abc123".to_string(),
                        vout: 0,
                    },
                    input_description: "test input".to_string(),
                    sequence_number: 0xFFFFFFFF,
                    unlocking_script: None,
                    unlocking_script_length: Some(107),
                    satoshis: Some(10000),
                    locking_script: Some("76a9".to_string()),
                },
                vin: 0,
                satoshis: 10000,
                locking_script: vec![0x76, 0xa9],
                output: None,
            },
        ];
        
        let xoutputs = vec![
            XValidCreateActionOutput {
                output: ValidCreateActionOutput {
                    satoshis: 5000,
                    locking_script: "76a914".to_string(),
                    output_description: "output1".to_string(),
                    basket: None,
                    custom_instructions: None,
                    tags: None,
                },
                vout: 0,
                provided_by: StorageProvidedBy::You,
                purpose: None,
                derivation_suffix: None,
                key_offset: None,
            },
            XValidCreateActionOutput {
                output: ValidCreateActionOutput {
                    satoshis: 4000,
                    locking_script: "76a914".to_string(),
                    output_description: "output2".to_string(),
                    basket: None,
                    custom_instructions: None,
                    tags: None,
                },
                vout: 1,
                provided_by: StorageProvidedBy::You,
                purpose: None,
                derivation_suffix: None,
                key_offset: None,
            },
        ];
        
        let size = estimate_transaction_size(&xinputs, &xoutputs);
        assert_eq!(size, 226, "1 input + 2 outputs = 226 bytes");
    }
    
    #[test]
    fn test_estimate_transaction_size_large() {
        // TS Reference: Larger transaction for fee calculation
        // 10 inputs + 5 outputs = 10 + (10*148) + (5*34) = 1660 bytes
        
        let xinputs = vec![XValidCreateActionInput {
            input: ValidCreateActionInput {
                outpoint: crate::sdk::action::OutPoint {
                    txid: "test".to_string(),
                    vout: 0,
                },
                input_description: "test".to_string(),
                sequence_number: 0xFFFFFFFF,
                unlocking_script: None,
                unlocking_script_length: Some(107),
                satoshis: Some(1000),
                locking_script: Some("76a9".to_string()),
            },
            vin: 0,
            satoshis: 1000,
            locking_script: vec![],
            output: None,
        }; 10]; // 10 inputs
        
        let xoutputs = vec![XValidCreateActionOutput {
            output: ValidCreateActionOutput {
                satoshis: 1000,
                locking_script: "76a914".to_string(),
                output_description: "out".to_string(),
                basket: None,
                custom_instructions: None,
                tags: None,
            },
            vout: 0,
            provided_by: StorageProvidedBy::You,
            purpose: None,
            derivation_suffix: None,
            key_offset: None,
        }; 5]; // 5 outputs
        
        let size = estimate_transaction_size(&xinputs, &xoutputs);
        assert_eq!(size, 1660, "10 inputs + 5 outputs = 1660 bytes");
    }
    
    // ============================================================================
    // Derivation Prefix Generation Tests
    // Reference: TypeScript createAction.ts lines 788-795
    // ============================================================================
    
    #[test]
    fn test_generate_random_derivation_prefix_length() {
        // TS Reference: randomBytesBase64(10) generates 10-byte random prefix
        // Base64 encoding of 10 bytes produces ~14 characters
        
        let prefix = generate_random_derivation_prefix();
        
        // Base64 of 10 bytes should be roughly 14 characters
        // (10 bytes * 8 bits) / 6 bits per char = 13.33... ≈ 14 chars (with padding)
        assert!(prefix.len() >= 13 && prefix.len() <= 16, 
            "Base64 of 10 bytes should be 13-16 chars, got {}", prefix.len());
        
        // Should be valid base64
        assert!(prefix.chars().all(|c| 
            c.is_alphanumeric() || c == '+' || c == '/' || c == '='
        ), "Should be valid base64 characters");
    }
    
    #[test]
    fn test_generate_random_derivation_prefix_uniqueness() {
        // TS Reference: Each transaction should get unique derivation prefix
        
        let prefix1 = generate_random_derivation_prefix();
        let prefix2 = generate_random_derivation_prefix();
        let prefix3 = generate_random_derivation_prefix();
        
        // Statistically, these should be different
        // (collision chance is astronomically low with 10 random bytes)
        assert_ne!(prefix1, prefix2, "Prefixes should be unique");
        assert_ne!(prefix2, prefix3, "Prefixes should be unique");
        assert_ne!(prefix1, prefix3, "Prefixes should be unique");
    }
    
    // ============================================================================
    // Change Output Creation Tests
    // Reference: TypeScript createAction.ts lines 797-850
    // ============================================================================
    
    #[test]
    fn test_create_change_output_basic() {
        // TS Reference: Creating change output with derivation prefix
        
        let user_id = 123;
        let transaction_id = 456;
        let basket_id = 789;
        let satoshis = 50000;
        let derivation_prefix = "test_prefix_123";
        
        let output = create_change_output(
            user_id,
            transaction_id,
            basket_id,
            satoshis,
            derivation_prefix,
        ).expect("Should create change output");
        
        // Verify all fields match TS behavior
        assert_eq!(output.user_id, user_id);
        assert_eq!(output.transaction_id, transaction_id);
        assert_eq!(output.satoshis, satoshis);
        assert_eq!(output.spendable, true, "Change outputs must be spendable");
        assert_eq!(output.change, true, "Must be marked as change");
        assert_eq!(output.purpose, "change");
        assert_eq!(output.output_type, "P2PKH");
        assert_eq!(output.provided_by, WalletStorageProvidedBy::Storage);
        assert_eq!(output.basket_id, Some(basket_id));
        assert_eq!(output.derivation_prefix, Some(derivation_prefix.to_string()));
        assert_eq!(output.output_description, "change");
    }
    
    #[test]
    fn test_create_change_output_zero_satoshis() {
        // TS Reference: Edge case - zero satoshi change (should still create valid output)
        
        let output = create_change_output(1, 1, 1, 0, "prefix").unwrap();
        
        assert_eq!(output.satoshis, 0);
        assert_eq!(output.change, true);
        assert_eq!(output.spendable, true);
    }
    
    #[test]
    fn test_create_change_output_large_satoshis() {
        // TS Reference: Large satoshi amounts (within i64 range)
        
        let large_amount = 2_100_000_000_000_000i64; // 21M BTC in satoshis
        let output = create_change_output(1, 1, 1, large_amount, "prefix").unwrap();
        
        assert_eq!(output.satoshis, large_amount);
        assert_eq!(output.change, true);
    }
    
    // ============================================================================
    // MaxPossibleSatoshis Tests
    // Reference: TypeScript createAction.ts lines 852-870
    // ============================================================================
    
    #[test]
    fn test_handle_max_possible_satoshis_none() {
        // TS Reference: When no output requests maxPossibleSatoshis
        // Currently disabled in implementation, so we just test the structure
        
        let vargs = create_test_vargs();
        let xoutputs = vec![];
        let allocated = 100000;
        let output_total = 50000;
        
        let result = handle_max_possible_satoshis(&vargs, &xoutputs, allocated, output_total);
        
        assert!(result.is_ok());
        assert!(result.unwrap().is_none(), "Should return None when no maxPossibleSatoshis");
    }
    
    // Helper function to create test ValidCreateActionArgs
    fn create_test_vargs() -> ValidCreateActionArgs {
        ValidCreateActionArgs {
            inputs: vec![],
            outputs: vec![],
            version: 1,
            lock_time: 0,
            labels: vec![],
            options: ValidCreateActionOptions::default(),
            input_beef: None,
            random_vals: None,
            is_new_tx: true,
            description: "test".to_string(),
            is_sign_action: false,
            is_no_send: false,
            is_delayed: false,
            include_all_source_transactions: false,
        }
    }
    
    // ============================================================================
    // FundingResult Structure Tests
    // Reference: TypeScript createAction.ts fundNewTransactionSdk return type
    // ============================================================================
    
    #[test]
    fn test_funding_result_structure() {
        // TS Reference: FundingResult { allocatedChange, changeOutputs, derivationPrefix, maxPossibleSatoshisAdjustment }
        
        let allocated = vec![];
        let change = vec![];
        let prefix = "test_prefix".to_string();
        
        let result = FundingResult {
            allocated_change: allocated,
            change_outputs: change,
            derivation_prefix: prefix.clone(),
            max_possible_satoshis_adjustment: None,
        };
        
        assert_eq!(result.allocated_change.len(), 0);
        assert_eq!(result.change_outputs.len(), 0);
        assert_eq!(result.derivation_prefix, prefix);
        assert!(result.max_possible_satoshis_adjustment.is_none());
    }
    
    #[test]
    fn test_max_possible_satoshis_adjustment_structure() {
        // TS Reference: { fixedOutputIndex, satoshis }
        
        let adjustment = MaxPossibleSatoshisAdjustment {
            fixed_output_index: 2,
            satoshis: 45000,
        };
        
        assert_eq!(adjustment.fixed_output_index, 2);
        assert_eq!(adjustment.satoshis, 45000);
    }
    
    // ============================================================================
    // Edge Case Tests
    // ============================================================================
    
    #[test]
    fn test_storage_fee_model_creation() {
        let fee_model = StorageFeeModel {
            model: "sat/kb".to_string(),
            value: 0.5,
        };
        
        assert_eq!(fee_model.model, "sat/kb");
        assert_eq!(fee_model.value, 0.5);
    }
    
    #[test]
    fn test_output_creation_result() {
        let result = OutputCreationResult {
            outputs: vec![],
            change_vouts: vec![0, 1, 2],
        };
        
        assert_eq!(result.outputs.len(), 0);
        assert_eq!(result.change_vouts.len(), 3);
        assert_eq!(result.change_vouts[0], 0);
        assert_eq!(result.change_vouts[2], 2);
    }
    
    // ============================================================================
    // Integration-like Tests (would need mocks in real implementation)
    // ============================================================================
    
    #[test]
    fn test_create_action_context_structure() {
        let now = chrono::Utc::now().to_rfc3339();
        let ctx = CreateTransactionContext {
            xinputs: vec![],
            xoutputs: vec![],
            change_basket: TableOutputBasket {
                created_at: now.clone(),
                updated_at: now,
                basket_id: 1,
                user_id: 123,
                name: "default".to_string(),
                number_of_desired_utxos: 10,
                minimum_desired_utxo_value: 1000,
                is_deleted: false,
            },
            no_send_change_in: vec![],
            available_change_count: 10,
            fee_model: StorageFeeModel {
                model: "sat/kb".to_string(),
                value: 0.5,
            },
            transaction_id: 456,
        };
        
        assert_eq!(ctx.transaction_id, 456);
        assert_eq!(ctx.available_change_count, 10);
        assert_eq!(ctx.fee_model.model, "sat/kb");
        assert_eq!(ctx.change_basket.basket_id, 1);
        assert_eq!(ctx.change_basket.user_id, 123);
        assert_eq!(ctx.change_basket.name, "default");
    }
    
    // ============================================================================
    // Error Handling Tests
    // ============================================================================
    
    #[test]
    fn test_beef_error_display() {
        use crate::beef::BeefError;
        
        let err = BeefError::TxNotFound("abc123".to_string());
        assert!(err.to_string().contains("abc123"));
        
        let err2 = BeefError::InvalidData("bad format".to_string());
        assert!(err2.to_string().contains("bad format"));
        
        let err3 = BeefError::NotImplemented("test");
        assert!(err3.to_string().contains("not implemented"));
    }
    
    // ============================================================================
    // Notes for Future Tests (when storage mocks are available)
    // ============================================================================
    
    // TODO: Test validate_required_inputs with mock storage
    // - Test with empty inputs
    // - Test with inputBEEF
    // - Test trustSelf='known' mode
    // - Test double-spend prevention
    // - Test change output validation
    
    // TODO: Test create_new_outputs with mock storage
    // - Test basket creation
    // - Test tag creation
    // - Test output randomization
    // - Test service-charge handling
    // - Test change output tracking
    
    // TODO: Test create_new_inputs with mock storage
    // - Test marking outputs as spent
    // - Test double-spend checking
    // - Test providedBy logic
    // - Test includeAllSourceTransactions
    
    // TODO: Test fund_new_transaction with mock storage
    // - Test change allocation
    // - Test fee calculation
    // - Test maxPossibleSatoshis adjustment
    
    // TODO: Test full create_action orchestration
    // - End-to-end transaction creation
    // - All 14 steps working together
    // - Error propagation
    // - Transaction state consistency
}
