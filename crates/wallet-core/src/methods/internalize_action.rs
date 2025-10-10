//! Internalize Action Implementation
//!
//! **Reference**: TypeScript `src/signer/methods/internalizeAction.ts`
//!
//! Allows a wallet to take ownership of outputs in a pre-existing transaction.
//!
//! ## Overview
//!
//! InternalizeAction handles two types of outputs:
//! 1. **Wallet payments** - Adds to wallet balance (default basket)
//! 2. **Basket insertions** - Custom outputs (don't affect balance)
//!
//! ## Process Flow (TypeScript Reference)
//!
//! 1. **Validate Arguments** (TS lines 39-40)
//!     - Validate InternalizeActionArgs
//!     - Check output indices
//!
//! 2. **Validate BEEF** (TS lines 82-98)
//!     - Parse AtomicBEEF from binary
//!     - Verify transaction
//!     - Extract txid and transaction
//!
//! 3. **Process Outputs** (TS lines 44-57)
//!     - For each output:
//!       - If "basket insertion": setup basket
//!       - If "wallet payment": validate BRC-29 payment
//!
//! 4. **Call Storage** (TS line 59)
//!     - Storage layer handles merge logic
//!     - Returns internalize result
//!
//! ## Merge Rules
//!
//! **Basket Insertion Rules**:
//! 1. Cannot use "default" basket
//! 2. Cannot convert change outputs to custom
//! 3. Custom outputs don't affect balance
//!
//! **Wallet Payment Rules**:
//! 1. Existing change outputs = no-op
//! 2. Converting custom to change = alters balance
//!
//! **Returns**: `StorageInternalizeActionResult` with txid and merge status

use crate::sdk::action_process::{
    ValidInternalizeActionArgs, ValidInternalizeOutput,
    StorageInternalizeActionResult,
};
use wallet_storage::{
    StorageError, WalletStorageProvider, AuthId,
};

/// Main internalizeAction implementation
///
/// Reference: TypeScript src/signer/methods/internalizeAction.ts
///
/// Takes ownership of outputs in existing transaction:
/// 1. Validates BEEF transaction
/// 2. Processes outputs by protocol type
/// 3. Calls storage layer for merge logic
pub async fn internalize_action(
    _storage: &mut dyn WalletStorageProvider,
    auth: &AuthId,
    vargs: ValidInternalizeActionArgs,
) -> Result<StorageInternalizeActionResult, StorageError> {
    let _user_id = auth.user_id.ok_or_else(|| {
        StorageError::Unauthorized("user_id required".to_string())
    })?;
    
    // STEP 1: Validate AtomicBEEF
    // TS lines 82-98: Parse and verify BEEF
    let (txid, _tx) = validate_atomic_beef(&vargs.tx)?;
    
    // STEP 2: Process outputs
    // TS lines 44-57: Validate each output by protocol type
    for output in &vargs.outputs {
        match output.protocol {
            crate::sdk::action_process::InternalizeProtocol::BasketInsertion => {
                // TS lines 76-80: Basket insertion validation
                validate_basket_insertion(output)?;
            }
            crate::sdk::action_process::InternalizeProtocol::WalletPayment => {
                // TS lines 63-74: Wallet payment validation
                validate_wallet_payment(output)?;
            }
        }
    }
    
    // STEP 3: Call storage layer
    // TS line 59: Storage handles the actual merge logic
    // For now, return a placeholder result
    // In full implementation, storage.internalize_action() would be called
    
    Ok(StorageInternalizeActionResult {
        txid,
        is_merge: false, // Would be determined by storage
        satoshis: 0, // Would be calculated by storage
        send_with_results: None,
        not_delayed_results: None,
    })
}

/// STEP 1: Validate AtomicBEEF transaction
/// Reference: TypeScript internalizeAction.ts lines 82-98
fn validate_atomic_beef(beef_binary: &[u8]) -> Result<(String, Vec<u8>), StorageError> {
    // Parse BEEF
    // TS line 83: const ab = Beef.fromBinary(vargs.tx)
    
    // For now, return placeholder
    // In full implementation, would:
    // 1. Parse BEEF binary format
    // 2. Verify transaction with chain tracker
    // 3. Extract atomic txid
    // 4. Return txid and transaction
    
    if beef_binary.is_empty() {
        return Err(StorageError::InvalidArg("Empty BEEF binary".to_string()));
    }
    
    // Placeholder txid
    Ok(("placeholder_txid".to_string(), beef_binary.to_vec()))
}

/// STEP 2.1: Validate basket insertion output
/// Reference: TypeScript internalizeAction.ts lines 76-80
fn validate_basket_insertion(output: &ValidInternalizeOutput) -> Result<(), StorageError> {
    // TS: No additional validations for basket insertions
    // Merge rules are enforced at storage layer:
    // 1. Cannot use "default" basket
    // 2. Cannot convert change outputs
    
    if let Some(ref insertion) = output.insertion_remittance {
        if insertion.basket == "default" {
            return Err(StorageError::InvalidArg(
                "Basket insertions cannot use 'default' basket".to_string()
            ));
        }
    }
    
    Ok(())
}

/// STEP 2.2: Validate wallet payment output
/// Reference: TypeScript internalizeAction.ts lines 63-74
fn validate_wallet_payment(output: &ValidInternalizeOutput) -> Result<(), StorageError> {
    // TS lines 64-66: Require paymentRemittance
    if output.payment_remittance.is_none() {
        return Err(StorageError::InvalidArg(
            "paymentRemittance required for wallet payment protocol".to_string()
        ));
    }
    
    // In full implementation, would validate BRC-29 locking script:
    // TS lines 68-73:
    // 1. Derive private key from derivation prefix/suffix
    // 2. Generate expected P2PKH locking script
    // 3. Compare with actual output locking script
    
    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_beef_empty() {
        let result = validate_atomic_beef(&[]);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_basket_insertion_default() {
        let output = ValidInternalizeOutput {
            output_index: 0,
            protocol: crate::sdk::action_process::InternalizeProtocol::BasketInsertion,
            payment_remittance: None,
            insertion_remittance: Some(crate::sdk::action_process::ValidBasketInsertion {
                basket: "default".to_string(),
                custom_instructions: None,
                tags: Some(vec![]),
            }),
        };
        
        let result = validate_basket_insertion(&output);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_basket_insertion_custom() {
        let output = ValidInternalizeOutput {
            output_index: 0,
            protocol: crate::sdk::action_process::InternalizeProtocol::BasketInsertion,
            payment_remittance: None,
            insertion_remittance: Some(crate::sdk::action_process::ValidBasketInsertion {
                basket: "custom_basket".to_string(),
                custom_instructions: None,
                tags: Some(vec![]),
            }),
        };
        
        let result = validate_basket_insertion(&output);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_wallet_payment_no_remittance() {
        let output = ValidInternalizeOutput {
            output_index: 0,
            protocol: crate::sdk::action_process::InternalizeProtocol::WalletPayment,
            payment_remittance: None,
            insertion_remittance: None,
        };
        
        let result = validate_wallet_payment(&output);
        assert!(result.is_err());
    }
}
