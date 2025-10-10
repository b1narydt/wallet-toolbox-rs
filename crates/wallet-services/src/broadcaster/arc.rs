//! ARC Broadcaster
//!
//! **Reference**: TypeScript `src/services/providers/ARC.ts`
//!
//! ARC (BSV Blockchain Transaction Processor) broadcaster implementation

use async_trait::async_trait;
use reqwest::Client;
use crate::error::{ServiceError, ServiceResult};
use crate::traits::Broadcaster;
use crate::types::{PostRawTxResult, PostBeefResult, GetStatusForTxidsResult, TxStatus, TxStatusType};
use super::types::{ArcConfig, ArcResponse};

/// ARC broadcaster client
///
/// Reference: TypeScript ARC class
///
/// Broadcasts transactions to BSV network via ARC service
pub struct ArcBroadcaster {
    /// Service name
    name: String,
    
    /// ARC service URL
    url: String,
    
    /// Configuration
    config: ArcConfig,
    
    /// HTTP client
    client: Client,
}

impl ArcBroadcaster {
    /// Create new ARC broadcaster
    ///
    /// Reference: TS ARC.constructor
    ///
    /// # Arguments
    /// * `url` - Base URL of ARC service
    /// * `config` - Optional configuration
    /// * `name` - Optional service name
    pub fn new(url: String, config: Option<ArcConfig>, name: Option<String>) -> Self {
        Self {
            name: name.unwrap_or_else(|| "ARC".to_string()),
            url,
            config: config.unwrap_or_default(),
            client: Client::new(),
        }
    }
    
    /// Post transaction to ARC
    ///
    /// Reference: TS ARC.postRawTx (lines 129-234)
    async fn post_tx_to_arc(&self, raw_tx_hex: &str, txid: &str) -> ServiceResult<ArcResponse> {
        let url = format!("{}/v1/tx", self.url);
        
        // Build headers (TS lines 95-111)
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        
        // Add deployment ID header
        if let Some(ref deployment_id) = self.config.deployment_id {
            headers.insert(
                reqwest::header::HeaderName::from_static("x-deployment-id"),
                reqwest::header::HeaderValue::from_str(deployment_id)
                    .map_err(|_| ServiceError::InvalidParams("Invalid deployment ID".to_string()))?,
            );
        }
        
        // Add API key if provided
        if let Some(ref api_key) = self.config.api_key {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", api_key))
                    .map_err(|_| ServiceError::InvalidParams("Invalid API key".to_string()))?,
            );
        }
        
        // Add custom headers
        if let Some(ref custom_headers) = self.config.headers {
            for (key, value) in custom_headers {
                headers.insert(
                    reqwest::header::HeaderName::from_bytes(key.as_bytes())
                        .map_err(|_| ServiceError::InvalidParams(format!("Invalid header key: {}", key)))?,
                    reqwest::header::HeaderValue::from_str(value)
                        .map_err(|_| ServiceError::InvalidParams(format!("Invalid header value: {}", value)))?,
                );
            }
        }
        
        // Build request body (TS lines 150-155)
        let mut body = serde_json::json!({
            "rawTx": raw_tx_hex,
        });
        
        // Add callback info if configured
        if let Some(ref callback_url) = self.config.callback_url {
            body["callbackUrl"] = serde_json::json!(callback_url);
            if let Some(ref callback_token) = self.config.callback_token {
                body["callbackToken"] = serde_json::json!(callback_token);
            }
        }
        
        // Make request
        let response = self.client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(ServiceError::Http)?;
        
        // Parse response (TS lines 156-180)
        let arc_response: ArcResponse = response
            .json()
            .await
            .map_err(ServiceError::Http)?;
        
        Ok(arc_response)
    }
    
    /// Calculate transaction ID from raw hex
    ///
    /// Reference: TS line 130
    fn calculate_txid(raw_tx_hex: &str) -> ServiceResult<String> {
        // Decode hex
        let tx_bytes = hex::decode(raw_tx_hex)
            .map_err(|_| ServiceError::InvalidParams("Invalid hex string".to_string()))?;
        
        // Double SHA-256
        use sha2::{Sha256, Digest};
        let hash1 = Sha256::digest(&tx_bytes);
        let hash2 = Sha256::digest(&hash1);
        
        // Convert to hex (big-endian)
        let txid = hex::encode(hash2.as_slice().iter().rev().copied().collect::<Vec<u8>>());
        
        Ok(txid)
    }
}

#[async_trait]
impl Broadcaster for ArcBroadcaster {
    /// Post raw transaction
    ///
    /// Reference: TS ARC.postRawTx
    async fn post_raw_tx(&self, raw_tx: &[u8]) -> ServiceResult<PostRawTxResult> {
        let raw_tx_hex = hex::encode(raw_tx);
        let txid = Self::calculate_txid(&raw_tx_hex)?;
        
        match self.post_tx_to_arc(&raw_tx_hex, &txid).await {
            Ok(arc_response) => {
                Ok(PostRawTxResult {
                    txid: arc_response.txid.clone(),
                    success: arc_response.is_success(),
                    name: Some(self.name.clone()),
                    error: if arc_response.is_success() {
                        None
                    } else {
                        Some(crate::types::ServiceError {
                            service: self.name.clone(),
                            message: arc_response.title.clone(),
                            status_code: Some(arc_response.status as u16),
                        })
                    },
                })
            }
            Err(e) => {
                Ok(PostRawTxResult {
                    txid: txid.clone(),
                    success: false,
                    name: Some(self.name.clone()),
                    error: Some(crate::types::ServiceError {
                        service: self.name.clone(),
                        message: e.to_string(),
                        status_code: None,
                    }),
                })
            }
        }
    }
    
    /// Post BEEF transaction(s)
    ///
    /// Reference: TS ARC.postBeef (lines 241-276)
    async fn post_beef(&self, beef: &[u8], txids: &[String]) -> ServiceResult<Vec<PostBeefResult>> {
        // Convert BEEF to hex
        let beef_hex = hex::encode(beef);
        
        // Get last txid (primary transaction)
        let primary_txid = txids.last()
            .ok_or_else(|| ServiceError::InvalidParams("No txids provided".to_string()))?;
        
        // Post the BEEF as raw tx
        let result = self.post_tx_to_arc(&beef_hex, primary_txid).await?;
        
        // Build results for all txids
        let mut results = Vec::new();
        for txid in txids {
            results.push(PostBeefResult {
                txid: txid.clone(),
                status: if result.is_success() { "success" } else { "error" }.to_string(),
                name: Some(self.name.clone()),
                error: if result.is_success() {
                    None
                } else {
                    Some(crate::types::ServiceError {
                        service: self.name.clone(),
                        message: result.title.clone(),
                        status_code: Some(result.status as u16),
                    })
                },
            });
        }
        
        Ok(results)
    }
    
    /// Get status for multiple transactions
    async fn get_status_for_txids(&self, txids: &[String]) -> ServiceResult<GetStatusForTxidsResult> {
        // ARC doesn't have a dedicated status endpoint
        // This would typically query the transaction status via separate API calls
        // For now, return a basic implementation
        
        let mut statuses = Vec::new();
        for txid in txids {
            statuses.push(TxStatus {
                txid: txid.clone(),
                status: TxStatusType::Unknown,
                depth: None,
            });
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
    fn test_arc_broadcaster_creation() {
        let broadcaster = ArcBroadcaster::new(
            "https://arc.example.com".to_string(),
            None,
            Some("TestARC".to_string()),
        );
        
        assert_eq!(broadcaster.name, "TestARC");
        assert_eq!(broadcaster.url, "https://arc.example.com");
    }
    
    #[test]
    fn test_calculate_txid() {
        // Simple test - would need actual transaction hex for real test
        let hex = "0100000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        let result = ArcBroadcaster::calculate_txid(hex);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_arc_broadcaster_with_config() {
        let mut config = ArcConfig::default();
        config.api_key = Some("test-key".to_string());
        config.callback_url = Some("https://callback.example.com".to_string());
        
        let broadcaster = ArcBroadcaster::new(
            "https://arc.example.com".to_string(),
            Some(config),
            None,
        );
        
        assert!(broadcaster.config.api_key.is_some());
        assert!(broadcaster.config.callback_url.is_some());
    }
}
