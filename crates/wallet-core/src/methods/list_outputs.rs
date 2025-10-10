//! List Outputs Implementation
//!
//! **Reference**: TypeScript `src/storage/methods/listOutputsKnex.ts`
//!
//! Lists wallet outputs (UTXOs) with filtering and pagination.
//!
//! ## Overview
//!
//! The listOutputs method queries wallet outputs with:
//! 1. Basket filtering (e.g., "default" basket)
//! 2. Tag filtering (with 'all' or 'any' mode)
//! 3. Spendability filtering
//! 4. Pagination (limit/offset)
//! 5. Optional BEEF inclusion
//!
//! ## Process Flow (TypeScript Reference)
//!
//! 1. **Validate Arguments** (TS lines 12-45)
//!     - Parse basket and tags
//!     - Handle special operations
//!     - Setup pagination
//!
//! 2. **Resolve Basket** (TS lines 47-67)
//!     - Find basket by name
//!     - Get basketId for filtering
//!     - Handle missing basket
//!
//! 3. **Resolve Tags** (TS lines 69-116)
//!     - Find tag IDs from tag names
//!     - Handle tag query mode (all/any)
//!     - Validate tag existence
//!
//! 4. **Query Outputs** (TS lines 118-189)
//!     - Build SQL query with filters
//!     - Apply pagination
//!     - Sort by outputId
//!     - Execute query
//!
//! 5. **Build Result** (TS lines 222-262)
//!     - Transform to WalletOutput format
//!     - Add optional fields (tags, labels, scripts)
//!     - Include BEEF if requested
//!     - Return formatted result
//!
//! **Returns**: `ListOutputsResult` with outputs array and total count

use crate::sdk::action_list::{ValidListOutputsArgs, WalletOutput};
use wallet_storage::{
    StorageError, WalletStorageProvider, AuthId,
    TableOutput, TableOutputBasket,
};

/// List outputs result
/// Matches TypeScript `ListOutputsResult`
#[derive(Debug, Clone)]
pub struct ListOutputsResult {
    /// Total number of outputs matching query (before pagination)
    pub total_outputs: i64,
    
    /// Array of wallet outputs
    pub outputs: Vec<WalletOutput>,
    
    /// Optional BEEF (if includeTransactions was requested)
    pub beef: Option<Vec<u8>>,
}

/// Main listOutputs implementation
///
/// Reference: TypeScript src/storage/methods/listOutputsKnex.ts
///
/// Lists wallet outputs with filtering:
/// 1. By basket name
/// 2. By tags (with all/any mode)
/// 3. By spendability
/// 4. With pagination
pub async fn list_outputs(
    storage: &mut dyn WalletStorageProvider,
    auth: &AuthId,
    vargs: ValidListOutputsArgs,
) -> Result<ListOutputsResult, StorageError> {
    let user_id = auth.user_id.ok_or_else(|| {
        StorageError::Unauthorized("user_id required".to_string())
    })?;
    
    // STEP 1: Setup pagination
    // TS lines 19-26: Handle limit/offset
    let limit = vargs.limit as i64;
    let offset = vargs.offset as i64;
    let order_desc = false;
    
    // STEP 2: Resolve basket if specified
    // TS lines 48-67: Find basket by name
    let basket_id = if !vargs.basket.is_empty() {
        resolve_basket(storage, user_id, &vargs.basket).await?
    } else {
        None
    };
    
    // STEP 3: Resolve tags if specified  
    // TS lines 96-116: Find tag IDs
    let tag_ids = if !vargs.tags.is_empty() {
        resolve_tags(storage, user_id, &vargs.tags).await?
    } else {
        Vec::new()
    };
    
    // STEP 4: Query outputs
    // TS lines 165-181: Build and execute query
    let mut outputs = query_outputs(
        storage,
        user_id,
        basket_id,
        &tag_ids,
        &vargs,
        limit,
        offset,
        order_desc,
    ).await?;
    
    // STEP 5: Build result
    // TS lines 222-262: Transform to WalletOutput format
    let wallet_outputs = transform_outputs(&mut outputs, storage, &vargs).await?;
    
    // Calculate total
    let total = if wallet_outputs.len() < limit as usize {
        wallet_outputs.len() as i64
    } else {
        count_outputs(storage, user_id, basket_id, &tag_ids, &vargs).await?
    };
    
    Ok(ListOutputsResult {
        total_outputs: total,
        outputs: wallet_outputs,
        beef: None, // TODO: Add BEEF support when needed
    })
}

/// STEP 2: Resolve basket name to basket ID
/// Reference: TypeScript listOutputsKnex.ts lines 48-67
async fn resolve_basket(
    storage: &mut dyn WalletStorageProvider,
    user_id: i64,
    basket_name: &str,
) -> Result<Option<i64>, StorageError> {
    // TS line 55: Find basket by user and name
    let basket = storage.find_or_insert_output_basket(user_id, basket_name).await?;
    Ok(Some(basket.basket_id))
}

/// STEP 3: Resolve tag names to tag IDs
/// Reference: TypeScript listOutputsKnex.ts lines 96-107
async fn resolve_tags(
    storage: &mut dyn WalletStorageProvider,
    user_id: i64,
    tag_names: &[String],
) -> Result<Vec<i64>, StorageError> {
    let mut tag_ids = Vec::new();
    
    for tag_name in tag_names {
        let tag = storage.find_or_insert_output_tag(user_id, tag_name).await?;
        tag_ids.push(tag.output_tag_id);
    }
    
    Ok(tag_ids)
}

/// STEP 4: Query outputs with all filters
/// Reference: TypeScript listOutputsKnex.ts lines 165-181
async fn query_outputs(
    _storage: &dyn WalletStorageProvider,
    _user_id: i64,
    _basket_id: Option<i64>,
    _tag_ids: &[i64],
    _vargs: &ValidListOutputsArgs,
    _limit: i64,
    _offset: i64,
    _order_desc: bool,
) -> Result<Vec<TableOutput>, StorageError> {
    // TODO: Implement full filtering query
    // Need to add storage method that accepts basket_id, tag_ids, limit, offset
    // For now, return empty to allow compilation
    Ok(Vec::new())
}

/// STEP 4.1: Count total outputs matching query
/// Reference: TypeScript listOutputsKnex.ts lines 193-194
async fn count_outputs(
    _storage: &dyn WalletStorageProvider,
    _user_id: i64,
    _basket_id: Option<i64>,
    _tag_ids: &[i64],
    _vargs: &ValidListOutputsArgs,
) -> Result<i64, StorageError> {
    // For now, return a placeholder
    // In a real implementation, we'd run a COUNT query
    Ok(0)
}

/// STEP 5: Transform TableOutput to WalletOutput
/// Reference: TypeScript listOutputsKnex.ts lines 227-257
async fn transform_outputs(
    outputs: &mut [TableOutput],
    storage: &dyn WalletStorageProvider,
    vargs: &ValidListOutputsArgs,
) -> Result<Vec<WalletOutput>, StorageError> {
    let mut wallet_outputs = Vec::new();
    
    for output in outputs {
        let outpoint = format!("{}.{}", 
            output.txid.as_ref().ok_or_else(|| StorageError::InvalidArg("missing txid".to_string()))?,
            output.vout
        );
        
        let mut wo = WalletOutput {
            outpoint,
            satoshis: output.satoshis,
            spendable: output.spendable,
            custom_instructions: None,
            locking_script: None,
            tags: None,
            labels: None,
        };
        
        // Add optional fields based on request
        if vargs.include_custom_instructions {
            wo.custom_instructions = output.custom_instructions.clone();
        }
        
        if vargs.include_locking_scripts {
            wo.locking_script = output.locking_script.as_ref().map(|s| hex::encode(s));
        }
        
        // TODO: Add tags and labels when storage methods are available
        
        wallet_outputs.push(wo);
    }
    
    Ok(wallet_outputs)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_list_outputs_structure() {
        // Basic structure test
        let result = ListOutputsResult {
            total_outputs: 5,
            outputs: vec![],
            beef: None,
        };
        
        assert_eq!(result.total_outputs, 5);
        assert_eq!(result.outputs.len(), 0);
    }
    
    #[test]
    fn test_wallet_output_creation() {
        let wo = WalletOutput {
            outpoint: "abc123.0".to_string(),
            satoshis: 1000,
            spendable: true,
            custom_instructions: None,
            locking_script: None,
            tags: None,
            labels: None,
        };
        
        assert_eq!(wo.satoshis, 1000);
        assert!(wo.spendable);
    }
}
