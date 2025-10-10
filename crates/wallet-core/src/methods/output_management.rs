//! Output Management Operations
//!
//! Manage UTXOs (relinquish, etc.).
//! Reference: wallet-toolbox SDK output management methods

use crate::sdk::{RelinquishOutputArgs, RelinquishOutputResult, WalletError, WalletResult};

/// Relinquish an output (mark as no longer owned)
///
/// Removes a UTXO from the wallet's management.
///
/// Reference: TypeScript `relinquishOutput()` in SDK
pub async fn relinquish_output(args: &RelinquishOutputArgs) -> WalletResult<RelinquishOutputResult> {
    let _ = args;
    // TODO: Implement actual output relinquishment
    // This requires:
    // 1. Find the output in storage by txid + vout
    // 2. Mark it as relinquished (spend_able = false, or delete)
    // 3. Update storage
    Err(WalletError::not_implemented("relinquishOutput"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_relinquish_output_not_implemented() {
        let args = RelinquishOutputArgs {
            txid: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            vout: 0,
            basket: None,
        };
        
        let result = relinquish_output(&args).await;
        assert!(result.is_err());
    }
}
