//! WhatsOnChain UTXO Service
//!
//! **Reference**: TypeScript `src/services/providers/WhatsOnChain.ts`
//!
//! WhatsOnChain API client for UTXO status and script hash history

use async_trait::async_trait;
use reqwest::Client;
use crate::error::{ServiceError, ServiceResult};
use crate::traits::UtxoStatusChecker;
use crate::types::{
    Chain, GetUtxoStatusResult, GetUtxoStatusOutputFormat,
    GetScriptHashHistoryResult, HistoryEntry, GetStatusForTxidsResult,
    TxStatus, TxStatusType,
};
use crate::traits::OutputRef;
use super::types::*;
use super::script_hash::validate_script_hash;

/// WhatsOnChain client
///
/// Reference: TypeScript WhatsOnChainNoServices class
///
/// Provides UTXO status checking via WhatsOnChain API
pub struct WhatsOnChainClient {
    /// Service name
    name: String,
    
    /// Chain (main or test)
    chain: Chain,
    
    /// Base URL
    url: String,
    
    /// HTTP client
    client: Client,
    
    /// API key (optional)
    api_key: Option<String>,
    
    /// Maximum retries
    max_retries: usize,
}

impl WhatsOnChainClient {
    /// Create new WhatsOnChain client
    ///
    /// Reference: TS WhatsOnChainNoServices.constructor (lines 25-28)
    ///
    /// # Arguments
    /// * `chain` - Chain to query (main or test)
    /// * `api_key` - Optional API key for rate limiting
    pub fn new(chain: Chain, api_key: Option<String>) -> Self {
        let url = match chain {
            Chain::Main => "https://api.whatsonchain.com/v1/bsv/main",
            Chain::Test => "https://api.whatsonchain.com/v1/bsv/test",
        };
        
        Self {
            name: "WoC".to_string(),
            chain,
            url: url.to_string(),
            client: Client::new(),
            api_key,
            max_retries: 3,
        }
    }
    
    /// Get HTTP headers
    ///
    /// Reference: TS getHttpHeaders() method
    fn get_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(ref api_key) = self.api_key {
            headers.insert(
                "woc-api-key",
                reqwest::header::HeaderValue::from_str(api_key).unwrap(),
            );
        }
        headers
    }
    
    /// Check if error is transient (should retry)
    ///
    /// Reference: TS retry logic checking for ECONNRESET
    fn is_transient_error(&self, _error: &ServiceError) -> bool {
        // Could check for specific error types that should retry
        true
    }
    
    /// Parse outpoint string
    ///
    /// Reference: TS parseWalletOutpoint (validationHelpers.ts)
    fn parse_outpoint(outpoint: &str) -> ServiceResult<(String, u32)> {
        let parts: Vec<&str> = outpoint.split('.').collect();
        if parts.len() != 2 {
            return Err(ServiceError::InvalidParams(
                "Outpoint must be in format txid.vout".to_string()
            ));
        }
        
        let txid = parts[0].to_string();
        let vout = parts[1].parse::<u32>()
            .map_err(|_| ServiceError::InvalidParams("Invalid vout".to_string()))?;
        
        Ok((txid, vout))
    }
}

#[async_trait]
impl UtxoStatusChecker for WhatsOnChainClient {
    /// Check if output is unspent
    ///
    /// Reference: TS WhatsOnChain.isUtxo
    async fn is_utxo(&self, output: &OutputRef) -> ServiceResult<bool> {
        let script = output.script.as_ref()
            .ok_or_else(|| ServiceError::InvalidParams("Script required".to_string()))?;
        
        let outpoint = format!("{}.{}", output.txid, output.vout);
        let result = self.get_utxo_status(
            script,
            Some(GetUtxoStatusOutputFormat::Script),
            Some(&outpoint)
        ).await?;
        
        Ok(result.is_utxo)
    }
    
    /// Get UTXO status
    ///
    /// Reference: TS WhatsOnChain.getUtxoStatus (lines 350-422)
    async fn get_utxo_status(
        &self,
        output: &str,
        output_format: Option<GetUtxoStatusOutputFormat>,
        outpoint: Option<&str>,
    ) -> ServiceResult<GetUtxoStatusResult> {
        let mut result = GetUtxoStatusResult {
            is_utxo: false,
            name: Some(self.name.clone()),
            error: None,
        };
        
        // Retry loop (TS lines 362-421)
        for retry in 0..self.max_retries {
            match self.try_get_utxo_status(output, output_format, outpoint).await {
                Ok(r) => return Ok(r),
                Err(e) => {
                    // Check if we should retry (TS lines 416-420)
                    if !self.is_transient_error(&e) || retry >= self.max_retries - 1 {
                        result.error = Some(crate::types::ServiceError {
                            service: self.name.clone(),
                            message: e.to_string(),
                            status_code: None,
                        });
                        return Ok(result);
                    }
                    // Retry on transient error
                }
            }
        }
        
        Ok(result)
    }
    
    /// Get script hash history
    ///
    /// Reference: TS WhatsOnChain.getScriptHashConfirmedHistory (lines 424-484)
    async fn get_script_hash_history(&self, hash: &str) -> ServiceResult<GetScriptHashHistoryResult> {
        // Reverse hash from LE to BE for WoC (TS lines 432-433)
        let hash_bytes = hex::decode(hash)
            .map_err(|_| ServiceError::InvalidParams("Invalid hash".to_string()))?;
        let hash_be = hex::encode(hash_bytes.iter().rev().copied().collect::<Vec<u8>>());
        
        let url = format!("{}/script/{}/confirmed/history", self.url, hash_be);
        
        // Retry loop (TS lines 437-480)
        for retry in 0..self.max_retries {
            let headers = self.get_headers();
            
            match self.client.get(&url).headers(headers).send().await {
                Ok(response) => {
                    // Handle 404 - no history (TS lines 450-454)
                    if response.status() == reqwest::StatusCode::NOT_FOUND {
                        return Ok(GetScriptHashHistoryResult {
                            script_hash: hash.to_string(),
                            history: vec![],
                            name: Some(self.name.clone()),
                        });
                    }
                    
                    // Handle 429 - rate limit (TS lines 445-448)
                    if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS && retry < 2 {
                        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
                        continue;
                    }
                    
                    if !response.status().is_success() {
                        return Err(ServiceError::ServiceFailed {
                            service: self.name.clone(),
                            message: format!("HTTP {}", response.status()),
                        });
                    }
                    
                    let data: WhatsOnChainScriptHashHistory = response.json().await
                        .map_err(ServiceError::Http)?;
                    
                    // Check for error in response (TS lines 464-467)
                    if let Some(error) = data.error {
                        return Err(ServiceError::ServiceFailed {
                            service: self.name.clone(),
                            message: error,
                        });
                    }
                    
                    // Map results (TS line 469)
                    let history: Vec<HistoryEntry> = data.result.into_iter()
                        .map(|d| HistoryEntry {
                            txid: d.tx_hash,
                            height: Some(d.height),
                        })
                        .collect();
                    
                    return Ok(GetScriptHashHistoryResult {
                        script_hash: hash.to_string(),
                        history,
                        name: Some(self.name.clone()),
                    });
                }
                Err(e) => {
                    if retry >= self.max_retries - 1 {
                        return Err(ServiceError::Http(e));
                    }
                }
            }
        }
        
        Err(ServiceError::Timeout)
    }
}

impl WhatsOnChainClient {
    /// Try to get UTXO status (single attempt)
    ///
    /// Reference: TS getUtxoStatus inner logic (lines 365-411)
    async fn try_get_utxo_status(
        &self,
        output: &str,
        output_format: Option<GetUtxoStatusOutputFormat>,
        outpoint: Option<&str>,
    ) -> ServiceResult<GetUtxoStatusResult> {
        // Validate script hash (TS line 366)
        let script_hash = validate_script_hash(output, output_format)?;
        
        // Build URL (TS lines 373-375)
        let url = format!("{}/script/{}/unspent/all", self.url, script_hash);
        let headers = self.get_headers();
        
        // Make request (TS lines 373-376)
        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .map_err(ServiceError::Http)?;
        
        // Check for success (TS lines 383-384)
        if !response.status().is_success() {
            return Err(ServiceError::ServiceFailed {
                service: self.name.clone(),
                message: format!("HTTP {}", response.status()),
            });
        }
        
        // Parse response (TS lines 386-390)
        let data: WhatsOnChainUtxoStatus = response.json().await
            .map_err(ServiceError::Http)?;
        
        if data.script != script_hash {
            return Err(ServiceError::InvalidResponse("Script hash mismatch".to_string()));
        }
        
        // Build result (TS lines 392-411)
        let mut result = GetUtxoStatusResult {
            is_utxo: false,
            name: Some(self.name.clone()),
            error: None,
        };
        
        if data.result.is_empty() {
            // No UTXOs (TS lines 392-396)
            result.is_utxo = false;
        } else {
            // Has UTXOs (TS lines 397-411)
            let details: Vec<UtxoDetail> = data.result.into_iter()
                .map(|s| UtxoDetail {
                    txid: s.tx_hash,
                    satoshis: s.value,
                    height: s.height,
                    index: s.tx_pos,
                })
                .collect();
            
            // Check if specific outpoint is UTXO (TS lines 407-410)
            if let Some(outpoint_str) = outpoint {
                let (txid, vout) = Self::parse_outpoint(outpoint_str)?;
                result.is_utxo = details.iter()
                    .any(|d| d.txid == txid && d.index == vout);
            } else {
                result.is_utxo = !details.is_empty();
            }
        }
        
        Ok(result)
    }
    
    /// Get status for multiple transaction IDs
    ///
    /// Reference: TS WhatsOnChain.getStatusForTxids (lines 51-90)
    pub async fn get_status_for_txids(&self, txids: &[String]) -> ServiceResult<GetStatusForTxidsResult> {
        let url = format!("{}/txs/status", self.url);
        let headers = self.get_headers();
        
        // Build request body (TS lines 59-63)
        let body = serde_json::json!({ "txids": txids });
        
        // Make request (TS lines 68-71)
        let response = self.client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(ServiceError::Http)?;
        
        if !response.status().is_success() {
            return Err(ServiceError::ServiceFailed {
                service: self.name.clone(),
                message: format!("HTTP {}", response.status()),
            });
        }
        
        // Parse response (TS lines 68-82)
        let data: Vec<WhatsOnChainTxStatus> = response.json().await
            .map_err(ServiceError::Http)?;
        
        // Map results (TS lines 74-82)
        let mut statuses = Vec::new();
        for txid in txids {
            let d = data.iter().find(|d| &d.txid == txid);
            
            let status = match d {
                // Unknown or error (TS lines 76-79)
                None | Some(WhatsOnChainTxStatus { error: Some(_), .. }) => {
                    TxStatus {
                        txid: txid.clone(),
                        status: TxStatusType::Unknown,
                        depth: None,
                    }
                }
                // Known (in mempool) (TS line 80)
                Some(WhatsOnChainTxStatus { confirmations: None, .. }) => {
                    TxStatus {
                        txid: txid.clone(),
                        status: TxStatusType::Known,
                        depth: Some(0),
                    }
                }
                // Mined (TS line 81)
                Some(WhatsOnChainTxStatus { confirmations: Some(conf), .. }) => {
                    TxStatus {
                        txid: txid.clone(),
                        status: TxStatusType::Mined,
                        depth: Some(*conf),
                    }
                }
            };
            
            statuses.push(status);
        }
        
        Ok(GetStatusForTxidsResult {
            statuses,
            name: Some(self.name.clone()),
        })
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_whatsonchain_client_creation() {
        let client = WhatsOnChainClient::new(Chain::Main, None);
        assert_eq!(client.chain, Chain::Main);
        assert_eq!(client.url, "https://api.whatsonchain.com/v1/bsv/main");
    }
    
    #[test]
    fn test_whatsonchain_testnet() {
        let client = WhatsOnChainClient::new(Chain::Test, None);
        assert_eq!(client.url, "https://api.whatsonchain.com/v1/bsv/test");
    }
    
    #[test]
    fn test_parse_outpoint() {
        let result = WhatsOnChainClient::parse_outpoint("abc123.0");
        assert!(result.is_ok());
        let (txid, vout) = result.unwrap();
        assert_eq!(txid, "abc123");
        assert_eq!(vout, 0);
    }
    
    #[test]
    fn test_parse_outpoint_invalid() {
        let result = WhatsOnChainClient::parse_outpoint("invalid");
        assert!(result.is_err());
    }
}
