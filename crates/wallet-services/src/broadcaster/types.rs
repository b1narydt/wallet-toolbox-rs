//! Broadcaster types
//!
//! **Reference**: TypeScript `src/services/providers/ARC.ts`

use serde::{Deserialize, Serialize};

/// ARC configuration
/// Reference: TypeScript ArcConfig
#[derive(Debug, Clone)]
pub struct ArcConfig {
    /// Authentication token for ARC API
    pub api_key: Option<String>,
    
    /// Deployment ID for tracking
    pub deployment_id: Option<String>,
    
    /// Callback URL for notifications
    pub callback_url: Option<String>,
    
    /// Callback authentication token
    pub callback_token: Option<String>,
    
    /// Additional headers
    pub headers: Option<std::collections::HashMap<String, String>>,
}

impl Default for ArcConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            deployment_id: Some(Self::generate_deployment_id()),
            callback_url: None,
            callback_token: None,
            headers: None,
        }
    }
}

impl ArcConfig {
    /// Generate random deployment ID
    /// Reference: TS defaultDeploymentId()
    fn generate_deployment_id() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
        format!("rs-sdk-{}", hex::encode(random_bytes))
    }
}

/// ARC API response
/// Reference: TypeScript ARC response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcResponse {
    /// Block hash (if mined)
    #[serde(rename = "blockHash", skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    
    /// Block height (if mined)
    #[serde(rename = "blockHeight", skip_serializing_if = "Option::is_none")]
    pub block_height: Option<u32>,
    
    /// Extra info
    #[serde(rename = "extraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String>,
    
    /// Status
    pub status: i32,
    
    /// Timestamp
    pub timestamp: String,
    
    /// Title (status description)
    pub title: String,
    
    /// Transaction ID
    pub txid: String,
    
    /// Txid (duplicate field for compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid_field: Option<String>,
    
    /// Competing transactions (if double spend)
    #[serde(rename = "competingTxs", skip_serializing_if = "Option::is_none")]
    pub competing_txs: Option<Vec<String>>,
}

impl ArcResponse {
    /// Check if response indicates success
    pub fn is_success(&self) -> bool {
        self.status == 200 || self.status == 409 // 409 = already in mempool
    }
    
    /// Check if response indicates double spend
    pub fn is_double_spend(&self) -> bool {
        self.competing_txs.is_some() && !self.competing_txs.as_ref().unwrap().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_arc_config_default() {
        let config = ArcConfig::default();
        assert!(config.deployment_id.is_some());
        assert!(config.deployment_id.unwrap().starts_with("rs-sdk-"));
    }
    
    #[test]
    fn test_arc_response_success() {
        let response = ArcResponse {
            block_hash: None,
            block_height: None,
            extra_info: None,
            status: 200,
            timestamp: "2025-01-07T00:00:00Z".to_string(),
            title: "OK".to_string(),
            txid: "abc123".to_string(),
            txid_field: None,
            competing_txs: None,
        };
        
        assert!(response.is_success());
        assert!(!response.is_double_spend());
    }
    
    #[test]
    fn test_arc_response_double_spend() {
        let response = ArcResponse {
            block_hash: None,
            block_height: None,
            extra_info: None,
            status: 409,
            timestamp: "2025-01-07T00:00:00Z".to_string(),
            title: "Conflict".to_string(),
            txid: "abc123".to_string(),
            txid_field: None,
            competing_txs: Some(vec!["def456".to_string()]),
        };
        
        assert!(response.is_success()); // 409 is still "success" (already in mempool)
        assert!(response.is_double_spend());
    }
}
