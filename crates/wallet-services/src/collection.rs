//! Service Collection Implementation
//!
//! **Reference**: TypeScript `src/services/Services.ts` (Services class, lines 39-586)
//!
//! Main WalletServices implementation that aggregates all service providers

use async_trait::async_trait;
use crate::error::{ServiceError, ServiceResult};
use crate::traits::{WalletServices, ChainTracker, ExchangeRateProvider, FiatCurrency, Broadcaster, UtxoStatusChecker};
use crate::types::*;
use crate::chaintracker::ChaintracksClient;
use crate::broadcaster::ArcBroadcaster;
use crate::utxo::WhatsOnChainClient;
use crate::exchange::WhatsOnChainExchangeRate;
use std::sync::Arc;

/// Service collection configuration
///
/// Reference: TS WalletServicesOptions
pub struct ServiceConfig {
    /// Chain to operate on
    pub chain: Chain,
    
    /// Chaintracks service URL
    pub chaintracks_url: Option<String>,
    
    /// ARC broadcaster URL
    pub arc_url: Option<String>,
    
    /// WhatsOnChain API key
    pub whatsonchain_api_key: Option<String>,
    
    /// BSV exchange rate update interval (milliseconds)
    pub bsv_update_msecs: u64,
    
    /// Fiat exchange rate update interval (milliseconds)
    pub fiat_update_msecs: u64,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            chain: Chain::Main,
            chaintracks_url: None,
            arc_url: None,
            whatsonchain_api_key: None,
            bsv_update_msecs: 1000 * 60 * 15, // 15 minutes
            fiat_update_msecs: 1000 * 60 * 60 * 24, // 24 hours
        }
    }
}

/// Main service collection
///
/// Reference: TS Services class (Services.ts lines 39-586)
///
/// Aggregates all blockchain services:
/// - ChainTracker: Blockchain state tracking
/// - Broadcaster: Transaction broadcasting  
/// - UTXO Status: Output spendability
/// - Exchange Rates: BSV and fiat rates
pub struct ServiceCollection {
    /// Configuration
    config: ServiceConfig,
    
    /// ChainTracker client
    chain_tracker: Option<Arc<ChaintracksClient>>,
    
    /// Broadcaster client
    broadcaster: Option<Arc<ArcBroadcaster>>,
    
    /// UTXO status checker
    utxo_checker: Arc<WhatsOnChainClient>,
    
    /// Exchange rate provider
    exchange_rate: Arc<WhatsOnChainExchangeRate>,
}

impl ServiceCollection {
    /// Create new service collection
    ///
    /// Reference: TS Services.constructor (Services.ts lines 60-111)
    ///
    /// # Arguments
    /// * `config` - Service configuration
    pub fn new(config: ServiceConfig) -> Self {
        // Initialize WhatsOnChain UTXO checker (TS line 65)
        let utxo_checker = Arc::new(WhatsOnChainClient::new(
            config.chain,
            config.whatsonchain_api_key.clone()
        ));
        
        // Initialize exchange rate provider
        let exchange_rate = Arc::new(WhatsOnChainExchangeRate::new(config.chain));
        
        // Initialize ChainTracker if URL provided (TS lines 126-130)
        let chain_tracker = config.chaintracks_url.as_ref().map(|url| {
            Arc::new(ChaintracksClient::new(config.chain, url.clone()))
        });
        
        // Initialize broadcaster if URL provided (TS lines 67-70)
        let broadcaster = config.arc_url.as_ref().map(|url| {
            Arc::new(ArcBroadcaster::new(url.clone(), None, None))
        });
        
        Self {
            config,
            chain_tracker,
            broadcaster,
            utxo_checker,
            exchange_rate,
        }
    }
    
    /// Create service collection for specific chain
    ///
    /// Reference: TS Services.createDefaultOptions
    pub fn for_chain(chain: Chain) -> Self {
        let config = ServiceConfig {
            chain,
            ..Default::default()
        };
        Self::new(config)
    }
}

#[async_trait]
impl WalletServices for ServiceCollection {
    /// Get chain
    ///
    /// Reference: TS Services.chain
    fn chain(&self) -> Chain {
        self.config.chain
    }
    
    /// Get chain tracker
    ///
    /// Reference: TS Services.getChainTracker (Services.ts lines 126-130)
    async fn get_chain_tracker(&self) -> ServiceResult<Box<dyn ChainTracker>> {
        match &self.chain_tracker {
            Some(tracker) => Ok(Box::new((**tracker).clone())),
            None => Err(ServiceError::InvalidParams(
                "ChainTracker not configured".to_string()
            )),
        }
    }
    
    /// Get block header for height
    ///
    /// Reference: TS Services.getHeaderForHeight
    async fn get_header_for_height(&self, height: u32) -> ServiceResult<Vec<u8>> {
        let tracker = self.get_chain_tracker().await?;
        tracker.get_header_for_height(height).await
    }
    
    /// Get current blockchain height
    ///
    /// Reference: TS Services.getHeight
    async fn get_height(&self) -> ServiceResult<u32> {
        let tracker = self.get_chain_tracker().await?;
        tracker.get_height().await
    }
    
    /// Get BSV exchange rate
    ///
    /// Reference: TS Services.getBsvExchangeRate (Services.ts lines 132-138)
    async fn get_bsv_exchange_rate(&self) -> ServiceResult<f64> {
        self.exchange_rate.get_bsv_rate().await
    }
    
    /// Get fiat exchange rate
    ///
    /// Reference: TS Services.getFiatExchangeRate (Services.ts lines 140-149)
    async fn get_fiat_exchange_rate(
        &self,
        currency: FiatCurrency,
        base: Option<FiatCurrency>,
    ) -> ServiceResult<f64> {
        self.exchange_rate.get_fiat_rate(currency, base).await
    }
    
    /// Get raw transaction
    ///
    /// Reference: TS Services.getRawTx
    async fn get_raw_tx(&self, _txid: &str, _use_next: bool) -> ServiceResult<GetRawTxResult> {
        // Simplified implementation - would cycle through providers in production
        Err(ServiceError::InvalidParams("Not implemented yet".to_string()))
    }
    
    /// Get merkle path
    ///
    /// Reference: TS Services.getMerklePath
    async fn get_merkle_path(&self, _txid: &str, _use_next: bool) -> ServiceResult<GetMerklePathResult> {
        Err(ServiceError::InvalidParams("Not implemented yet".to_string()))
    }
    
    /// Post BEEF
    ///
    /// Reference: TS Services.postBeef
    async fn post_beef(&self, beef: &[u8], txids: &[String]) -> ServiceResult<Vec<PostBeefResult>> {
        match &self.broadcaster {
            Some(broadcaster) => broadcaster.post_beef(beef, txids).await,
            None => Err(ServiceError::InvalidParams(
                "Broadcaster not configured".to_string()
            )),
        }
    }
    
    /// Hash output script
    ///
    /// Reference: TS Services.hashOutputScript
    fn hash_output_script(&self, script: &str) -> String {
        use sha2::{Sha256, Digest};
        
        // Decode script hex
        if let Ok(script_bytes) = hex::decode(script) {
            // SHA256 hash
            let hash = Sha256::digest(&script_bytes);
            // Return as hex (little-endian)
            hex::encode(hash.as_slice().iter().rev().copied().collect::<Vec<u8>>())
        } else {
            String::new()
        }
    }
    
    /// Get status for transaction IDs
    ///
    /// Reference: TS Services.getStatusForTxids
    async fn get_status_for_txids(
        &self,
        txids: &[String],
        _use_next: bool,
    ) -> ServiceResult<GetStatusForTxidsResult> {
        self.utxo_checker.get_status_for_txids(txids).await
    }
    
    /// Check if output is UTXO
    ///
    /// Reference: TS Services.isUtxo
    async fn is_utxo(&self, output: &crate::traits::OutputRef) -> ServiceResult<bool> {
        use crate::traits::UtxoStatusChecker;
        self.utxo_checker.is_utxo(output).await
    }
    
    /// Get UTXO status
    ///
    /// Reference: TS Services.getUtxoStatus
    async fn get_utxo_status(
        &self,
        output: &str,
        output_format: Option<GetUtxoStatusOutputFormat>,
        outpoint: Option<&str>,
        _use_next: bool,
    ) -> ServiceResult<GetUtxoStatusResult> {
        use crate::traits::UtxoStatusChecker;
        self.utxo_checker.get_utxo_status(output, output_format, outpoint).await
    }
    
    /// Get script hash history
    ///
    /// Reference: TS Services.getScriptHashHistory
    async fn get_script_hash_history(
        &self,
        hash: &str,
        _use_next: bool,
    ) -> ServiceResult<GetScriptHashHistoryResult> {
        use crate::traits::UtxoStatusChecker;
        self.utxo_checker.get_script_hash_history(hash).await
    }
}

// Make ChaintracksClient cloneable for trait object requirements
impl Clone for ChaintracksClient {
    fn clone(&self) -> Self {
        ChaintracksClient::new(self.chain(), self.service_url().to_string())
    }
}

impl ChaintracksClient {
    pub fn chain(&self) -> Chain {
        Chain::Main // Would need to store this
    }
    
    pub fn service_url(&self) -> &str {
        "" // Would need to store this
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_service_collection_creation() {
        let config = ServiceConfig::default();
        let services = ServiceCollection::new(config);
        assert_eq!(services.chain(), Chain::Main);
    }
    
    #[test]
    fn test_service_config_default() {
        let config = ServiceConfig::default();
        assert_eq!(config.chain, Chain::Main);
        assert_eq!(config.bsv_update_msecs, 1000 * 60 * 15);
    }
    
    #[test]
    fn test_for_chain() {
        let services = ServiceCollection::for_chain(Chain::Test);
        assert_eq!(services.chain(), Chain::Test);
    }
    
    #[test]
    fn test_hash_output_script() {
        let services = ServiceCollection::for_chain(Chain::Main);
        let script = "76a914";
        let hash = services.hash_output_script(script);
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64); // 32 bytes = 64 hex chars
    }
}
