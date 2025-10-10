//! List Actions Implementation
//!
//! **Reference**: TypeScript `src/storage/methods/listActionsKnex.ts`
//!
//! Lists wallet actions (transactions) with filtering and pagination.
//!
//! ## Overview
//!
//! The listActions method queries wallet transactions with:
//! 1. Label filtering
//! 2. Status filtering  
//! 3. Pagination (limit/offset)
//! 4. Optional BEEF inclusion
//!
//! ## Process Flow (TypeScript Reference)
//!
//! 1. **Validate Arguments** - Parse labels and filters
//! 2. **Resolve Labels** - Find label IDs from names
//! 3. **Query Transactions** - Build and execute query
//! 4. **Build Result** - Transform to WalletAction format
//!
//! **Returns**: `ListActionsResult` with actions array and total count

use crate::sdk::action_list::{ValidListActionsArgs, WalletAction};
use wallet_storage::{
    StorageError, WalletStorageProvider, AuthId,
    TableTransaction, TransactionStatus,
};

/// List actions result
/// Matches TypeScript `ListActionsResult`
#[derive(Debug, Clone)]
pub struct ListActionsResult {
    /// Total number of actions matching query (before pagination)
    pub total_actions: i64,
    
    /// Array of wallet actions
    pub actions: Vec<WalletAction>,
    
    /// Optional BEEF (if includeTransactions was requested)
    pub beef: Option<Vec<u8>>,
}

/// Main listActions implementation
///
/// Reference: TypeScript src/storage/methods/listActionsKnex.ts
///
/// Lists wallet actions with filtering:
/// 1. By labels
/// 2. By status
/// 3. With pagination
pub async fn list_actions(
    storage: &mut dyn WalletStorageProvider,
    auth: &AuthId,
    vargs: ValidListActionsArgs,
) -> Result<ListActionsResult, StorageError> {
    let user_id = auth.user_id.ok_or_else(|| {
        StorageError::Unauthorized("user_id required".to_string())
    })?;
    
    // STEP 1: Setup pagination
    let limit = vargs.limit as i64;
    let offset = vargs.offset as i64;
    let _order_desc = false;
    
    // STEP 2: Resolve labels if specified
    let label_ids = if !vargs.labels.is_empty() {
        resolve_labels(storage, user_id, &vargs.labels).await?
    } else {
        Vec::new()
    };
    
    // STEP 3: Query transactions
    let transactions = query_transactions(
        storage,
        user_id,
        &label_ids,
        &vargs,
        limit,
        offset,
    ).await?;
    
    // STEP 4: Build result
    let actions = transform_transactions(&transactions, storage, &vargs).await?;
    
    // Calculate total
    let total = if actions.len() < limit as usize {
        actions.len() as i64
    } else {
        count_transactions(storage, user_id, &label_ids, &vargs).await?
    };
    
    Ok(ListActionsResult {
        total_actions: total,
        actions,
        beef: None, // TODO: Add BEEF support when needed
    })
}

/// STEP 2: Resolve label names to label IDs
async fn resolve_labels(
    storage: &mut dyn WalletStorageProvider,
    user_id: i64,
    label_names: &[String],
) -> Result<Vec<i64>, StorageError> {
    let mut label_ids = Vec::new();
    
    for label_name in label_names {
        let label = storage.find_or_insert_tx_label(user_id, label_name).await?;
        label_ids.push(label.tx_label_id);
    }
    
    Ok(label_ids)
}

/// STEP 3: Query transactions with all filters
async fn query_transactions(
    _storage: &dyn WalletStorageProvider,
    _user_id: i64,
    _label_ids: &[i64],
    _vargs: &ValidListActionsArgs,
    _limit: i64,
    _offset: i64,
) -> Result<Vec<TableTransaction>, StorageError> {
    // For now, use a simplified query
    // In a real implementation, we'd pass all these filters to storage
    
    // This is a placeholder - we'd need to add find_transactions_with_filters to storage trait
    Err(StorageError::NotImplemented("query_transactions with full filtering"))
}

/// STEP 3.1: Count total transactions matching query
async fn count_transactions(
    _storage: &dyn WalletStorageProvider,
    _user_id: i64,
    _label_ids: &[i64],
    _vargs: &ValidListActionsArgs,
) -> Result<i64, StorageError> {
    // Placeholder
    Ok(0)
}

/// STEP 4: Transform TableTransaction to WalletAction
async fn transform_transactions(
    transactions: &[TableTransaction],
    _storage: &dyn WalletStorageProvider,
    _vargs: &ValidListActionsArgs,
) -> Result<Vec<WalletAction>, StorageError> {
    let mut actions = Vec::new();
    
    for tx in transactions {
        let wa = WalletAction {
            txid: tx.txid.clone(),
            satoshis: Some(tx.satoshis),
            status: format!("{:?}", tx.status),
            is_outgoing: tx.is_outgoing,
            description: tx.description.clone(),
            labels: None,
            version: tx.version.unwrap_or(1) as i32,
            lock_time: tx.lock_time.unwrap_or(0),
            inputs: None,
            outputs: None,
        };
        
        actions.push(wa);
    }
    
    Ok(actions)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_list_actions_structure() {
        // Basic structure test
        let result = ListActionsResult {
            total_actions: 10,
            actions: vec![],
            beef: None,
        };
        
        assert_eq!(result.total_actions, 10);
        assert_eq!(result.actions.len(), 0);
    }
}
