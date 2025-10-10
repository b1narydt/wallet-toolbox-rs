//! Service trait definitions
//!
//! **Reference**: TypeScript `src/sdk/WalletServices.interfaces.ts`
//!
//! Defines the main WalletServices trait and sub-traits

use async_trait::async_trait;
use crate::types::*;
use crate::error::ServiceResult;

/// Main wallet services trait
///
/// Reference: TypeScript WalletServices interface
///
/// Provides access to external blockchain services:
/// - Chain tracking and block headers
/// - Transaction broadcasting
/// - UTXO status checking
/// - Exchange rates
/// - Merkle proofs
#[async_trait]
pub trait WalletServices: Send + Sync {
    /// Get the chain this service operates on
    fn chain(&self) -> Chain;
    
    /// Get chain tracker service
    async fn get_chain_tracker(&self) -> ServiceResult<Box<dyn ChainTracker>>;
    
    /// Get block header for height
    /// Reference: TS WalletServices.getHeaderForHeight
    async fn get_header_for_height(&self, height: u32) -> ServiceResult<Vec<u8>>;
    
    /// Get current blockchain height
    /// Reference: TS WalletServices.getHeight
    async fn get_height(&self) -> ServiceResult<u32>;
    
    /// Get BSV exchange rate (USD per BSV)
    /// Reference: TS WalletServices.getBsvExchangeRate
    async fn get_bsv_exchange_rate(&self) -> ServiceResult<f64>;
    
    /// Get fiat exchange rate
    /// Reference: TS WalletServices.getFiatExchangeRate
    async fn get_fiat_exchange_rate(
        &self,
        currency: FiatCurrency,
        base: Option<FiatCurrency>,
    ) -> ServiceResult<f64>;
    
    /// Get raw transaction bytes
    /// Reference: TS WalletServices.getRawTx
    async fn get_raw_tx(
        &self,
        txid: &str,
        use_next: bool,
    ) -> ServiceResult<GetRawTxResult>;
    
    /// Get merkle proof for transaction
    /// Reference: TS WalletServices.getMerklePath
    async fn get_merkle_path(
        &self,
        txid: &str,
        use_next: bool,
    ) -> ServiceResult<GetMerklePathResult>;
    
    /// Post BEEF transaction(s)
    /// Reference: TS WalletServices.postBeef
    async fn post_beef(
        &self,
        beef: &[u8],
        txids: &[String],
    ) -> ServiceResult<Vec<PostBeefResult>>;
    
    /// Hash output script for UTXO queries
    /// Reference: TS WalletServices.hashOutputScript
    fn hash_output_script(&self, script: &str) -> String;
    
    /// Get status for multiple transaction IDs
    /// Reference: TS WalletServices.getStatusForTxids
    async fn get_status_for_txids(
        &self,
        txids: &[String],
        use_next: bool,
    ) -> ServiceResult<GetStatusForTxidsResult>;
    
    /// Check if output is currently unspent
    /// Reference: TS WalletServices.isUtxo
    async fn is_utxo(&self, output: &OutputRef) -> ServiceResult<bool>;
    
    /// Get UTXO status
    /// Reference: TS WalletServices.getUtxoStatus
    async fn get_utxo_status(
        &self,
        output: &str,
        output_format: Option<GetUtxoStatusOutputFormat>,
        outpoint: Option<&str>,
        use_next: bool,
    ) -> ServiceResult<GetUtxoStatusResult>;
    
    /// Get script hash history
    /// Reference: TS WalletServices.getScriptHashHistory
    async fn get_script_hash_history(
        &self,
        hash: &str,
        use_next: bool,
    ) -> ServiceResult<GetScriptHashHistoryResult>;
}

/// Chain tracker trait
///
/// Reference: TypeScript ChainTracker from @bsv/sdk
///
/// Tracks blockchain state and validates proofs
#[async_trait]
pub trait ChainTracker: Send + Sync {
    /// Check if merkle root is valid for height
    async fn is_valid_root_for_height(&self, root: &str, height: u32) -> ServiceResult<bool>;
    
    /// Get header for block height
    async fn get_header_for_height(&self, height: u32) -> ServiceResult<Vec<u8>>;
    
    /// Get current blockchain height
    async fn get_height(&self) -> ServiceResult<u32>;
    
    /// Get merkle path for transaction
    async fn get_merkle_path(&self, txid: &str) -> ServiceResult<MerklePath>;
}

/// Broadcaster trait
///
/// Handles transaction broadcasting to the network
#[async_trait]
pub trait Broadcaster: Send + Sync {
    /// Post raw transaction
    async fn post_raw_tx(&self, raw_tx: &[u8]) -> ServiceResult<PostRawTxResult>;
    
    /// Post BEEF transaction(s)
    async fn post_beef(
        &self,
        beef: &[u8],
        txids: &[String],
    ) -> ServiceResult<Vec<PostBeefResult>>;
    
    /// Get status for multiple transactions
    async fn get_status_for_txids(
        &self,
        txids: &[String],
    ) -> ServiceResult<GetStatusForTxidsResult>;
}

/// UTXO status checker trait
///
/// Checks if outputs are currently spendable
#[async_trait]
pub trait UtxoStatusChecker: Send + Sync {
    /// Check if output is unspent
    async fn is_utxo(&self, output: &OutputRef) -> ServiceResult<bool>;
    
    /// Get detailed UTXO status
    async fn get_utxo_status(
        &self,
        output: &str,
        output_format: Option<GetUtxoStatusOutputFormat>,
        outpoint: Option<&str>,
    ) -> ServiceResult<GetUtxoStatusResult>;
    
    /// Get script hash history
    async fn get_script_hash_history(&self, hash: &str) -> ServiceResult<GetScriptHashHistoryResult>;
}

/// Exchange rate provider trait
///
/// Provides BSV and fiat exchange rates
#[async_trait]
pub trait ExchangeRateProvider: Send + Sync {
    /// Get BSV/USD exchange rate
    async fn get_bsv_rate(&self) -> ServiceResult<f64>;
    
    /// Get fiat exchange rate
    async fn get_fiat_rate(
        &self,
        currency: FiatCurrency,
        base: Option<FiatCurrency>,
    ) -> ServiceResult<f64>;
}

/// Fiat currency codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FiatCurrency {
    USD,
    GBP,
    EUR,
}

impl FiatCurrency {
    pub fn as_str(&self) -> &'static str {
        match self {
            FiatCurrency::USD => "USD",
            FiatCurrency::GBP => "GBP",
            FiatCurrency::EUR => "EUR",
        }
    }
}

/// Output reference for UTXO checks
#[derive(Debug, Clone)]
pub struct OutputRef {
    pub txid: String,
    pub vout: u32,
    pub script: Option<String>,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fiat_currency() {
        assert_eq!(FiatCurrency::USD.as_str(), "USD");
        assert_eq!(FiatCurrency::GBP.as_str(), "GBP");
        assert_eq!(FiatCurrency::EUR.as_str(), "EUR");
    }
    
    #[test]
    fn test_output_ref() {
        let output = OutputRef {
            txid: "abc123".to_string(),
            vout: 0,
            script: Some("76a914...88ac".to_string()),
        };
        
        assert_eq!(output.txid, "abc123");
        assert_eq!(output.vout, 0);
        assert!(output.script.is_some());
    }
}
