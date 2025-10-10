//! Chaintracks HTTP Client
//!
//! **Reference**: TypeScript `src/services/chaintracker/chaintracks/ChaintracksServiceClient.ts`
//!
//! HTTP client for Chaintracks blockchain state tracking service

use async_trait::async_trait;
use reqwest::Client;
use crate::error::{ServiceError, ServiceResult};
use crate::traits::ChainTracker;
use crate::types::{Chain, MerklePath};
use super::types::{BlockHeader, ChaintracksInfo, FetchStatus};

/// Chaintracks service client
///
/// Reference: TypeScript ChaintracksServiceClient
///
/// Connects to a Chaintracks service to track blockchain state
pub struct ChaintracksClient {
    /// Chain being tracked
    chain: Chain,
    
    /// Service base URL
    service_url: String,
    
    /// HTTP client
    client: Client,
    
    /// Maximum retries for transient errors
    max_retries: usize,
}

impl ChaintracksClient {
    /// Create new Chaintracks client
    ///
    /// Reference: TS ChaintracksServiceClient.constructor
    ///
    /// # Arguments
    /// * `chain` - Chain to track (main or test)
    /// * `service_url` - Base URL of Chaintracks service
    pub fn new(chain: Chain, service_url: String) -> Self {
        Self {
            chain,
            service_url,
            client: Client::new(),
            max_retries: 3,
        }
    }
    
    /// Get JSON from service endpoint
    ///
    /// Reference: TS ChaintracksServiceClient.getJson
    ///
    /// Makes HTTP GET request and parses JSON response with retries
    async fn get_json<T>(&self, path: &str) -> ServiceResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let value = self.get_json_or_none(path).await?;
        value.ok_or_else(|| {
            ServiceError::InvalidResponse("Value was undefined. Requested object may not exist.".to_string())
        })
    }
    
    /// Get JSON from service endpoint, returns None if not found
    ///
    /// Reference: TS ChaintracksServiceClient.getJsonOrUndefined
    ///
    /// Retries on transient errors (ECONNRESET)
    async fn get_json_or_none<T>(&self, path: &str) -> ServiceResult<Option<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.service_url, path);
        let mut last_error: Option<ServiceError> = None;
        
        // Retry loop (TS lines 60-70)
        for retry in 0..self.max_retries {
            match self.try_get_json(&url).await {
                Ok(Some(value)) => return Ok(Some(value)),
                Ok(None) => return Ok(None),
                Err(e) => {
                    // Don't retry on non-transient errors
                    if !self.is_transient_error(&e) {
                        return Err(e);
                    }
                    last_error = Some(e);
                }
            }
        }
        
        // All retries exhausted
        Err(last_error.unwrap_or(ServiceError::Timeout))
    }
    
    /// Try single GET request
    async fn try_get_json<T>(&self, url: &str) -> ServiceResult<Option<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(ServiceError::Http)?;
        
        let status: FetchStatus<T> = response
            .json()
            .await
            .map_err(ServiceError::Http)?;
        
        if status.is_success() {
            Ok(status.value)
        } else {
            Err(ServiceError::ServiceFailed {
                service: "chaintracks".to_string(),
                message: status.description.unwrap_or_else(|| "Unknown error".to_string()),
            })
        }
    }
    
    /// Check if error is transient (should retry)
    fn is_transient_error(&self, error: &ServiceError) -> bool {
        matches!(error, ServiceError::Timeout | ServiceError::Http(_))
    }
    
    /// Get current blockchain height
    ///
    /// Reference: TS ChaintracksServiceClient.getPresentHeight
    pub async fn get_present_height(&self) -> ServiceResult<u32> {
        self.get_json("/getPresentHeight").await
    }
    
    /// Get service info
    ///
    /// Reference: TS ChaintracksServiceClient.getInfo
    pub async fn get_info(&self) -> ServiceResult<ChaintracksInfo> {
        self.get_json("/getInfo").await
    }
    
    /// Find header for specific height
    ///
    /// Reference: TS ChaintracksServiceClient.findHeaderForHeight
    pub async fn find_header_for_height(&self, height: u32) -> ServiceResult<Option<BlockHeader>> {
        self.get_json_or_none(&format!("/findHeaderHexForHeight?height={}", height)).await
    }
    
    /// Find header for block hash
    ///
    /// Reference: TS ChaintracksServiceClient.findHeaderForBlockHash
    pub async fn find_header_for_block_hash(&self, hash: &str) -> ServiceResult<Option<BlockHeader>> {
        self.get_json_or_none(&format!("/findHeaderHexForBlockHash?hash={}", hash)).await
    }
    
    /// Find chain tip header
    ///
    /// Reference: TS ChaintracksServiceClient.findChainTipHeader
    pub async fn find_chain_tip_header(&self) -> ServiceResult<BlockHeader> {
        self.get_json("/findChainTipHeaderHex").await
    }
    
    /// Find chain tip hash
    ///
    /// Reference: TS ChaintracksServiceClient.findChainTipHash
    pub async fn find_chain_tip_hash(&self) -> ServiceResult<String> {
        self.get_json("/findChainTipHashHex").await
    }
    
    /// Check if service is listening
    ///
    /// Reference: TS ChaintracksServiceClient.isListening
    pub async fn is_listening(&self) -> bool {
        self.get_present_height().await.is_ok()
    }
}

#[async_trait]
impl ChainTracker for ChaintracksClient {
    /// Check if merkle root is valid for height
    ///
    /// Reference: TS ChaintracksServiceClient.isValidRootForHeight
    async fn is_valid_root_for_height(&self, root: &str, height: u32) -> ServiceResult<bool> {
        let header = self.find_header_for_height(height).await?;
        match header {
            Some(h) => Ok(root == h.merkle_root),
            None => Ok(false),
        }
    }
    
    /// Get header for block height
    async fn get_header_for_height(&self, height: u32) -> ServiceResult<Vec<u8>> {
        let header = self.find_header_for_height(height).await?;
        match header {
            Some(_h) => {
                // TODO: Convert BlockHeader to raw bytes
                // For now, return placeholder
                Ok(vec![])
            }
            None => Err(ServiceError::BlockNotFound(height)),
        }
    }
    
    /// Get current blockchain height
    ///
    /// Reference: TS ChaintracksServiceClient.currentHeight
    async fn get_height(&self) -> ServiceResult<u32> {
        self.get_present_height().await
    }
    
    /// Get merkle path for transaction
    async fn get_merkle_path(&self, _txid: &str) -> ServiceResult<MerklePath> {
        // TODO: Implement merkle path retrieval
        // Chaintracks doesn't provide this directly, might need WhatsOnChain
        Err(ServiceError::InvalidParams("Merkle path not supported by Chaintracks".to_string()))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chaintracks_client_creation() {
        let client = ChaintracksClient::new(
            Chain::Main,
            "https://chaintracks.example.com".to_string(),
        );
        
        assert_eq!(client.chain, Chain::Main);
        assert_eq!(client.service_url, "https://chaintracks.example.com");
        assert_eq!(client.max_retries, 3);
    }
    
    #[test]
    fn test_is_transient_error() {
        let client = ChaintracksClient::new(Chain::Main, "http://test".to_string());
        
        assert!(client.is_transient_error(&ServiceError::Timeout));
        assert!(!client.is_transient_error(&ServiceError::InvalidParams("test".to_string())));
    }
    
    // Integration tests would require a real Chaintracks service
    // or mock server. For now, we have unit tests only.
}
