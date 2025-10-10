//! WAB (Wallet Authentication Bridge) Client
//!
//! **Reference**: TypeScript `src/wab-client/WABClient.ts`
//!
//! Client for interacting with Wallet Authentication Bridge servers
//! for user authentication flows (phone, email, ID verification, etc.)

use crate::sdk::errors::{WalletError, WalletResult};
use serde::{Deserialize, Serialize};

/// Auth method interactor trait
///
/// Reference: TS AuthMethodInteractor interface
pub trait AuthMethodInteractor: Send + Sync {
    /// Get the method name (e.g., "twilio", "persona")
    fn method_name(&self) -> &str;
}

/// Result from starting an auth method
///
/// Reference: TS AuthStartResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthStartResult {
    pub success: bool,
    pub message: Option<String>,
}

/// Result from completing an auth method
///
/// Reference: TS AuthCompleteResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCompleteResult {
    pub success: bool,
    pub presentation_key: Option<String>,
    pub message: Option<String>,
}

/// Faucet request result
///
/// Reference: TS FaucetResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaucetResult {
    pub payment_data: serde_json::Value,
}

/// WAB Client trait
///
/// Reference: TS WABClient class methods
#[async_trait::async_trait]
pub trait WABClientTrait: Send + Sync {
    /// Start an authentication method
    async fn start_auth_method(
        &self,
        method: &dyn AuthMethodInteractor,
        presentation_key: &str,
        payload: serde_json::Value,
    ) -> WalletResult<AuthStartResult>;
    
    /// Complete an authentication method
    async fn complete_auth_method(
        &self,
        method: &dyn AuthMethodInteractor,
        temp_key: &str,
        payload: serde_json::Value,
    ) -> WalletResult<AuthCompleteResult>;
    
    /// Request faucet funding
    async fn request_faucet(&self, presentation_key: &str) -> WalletResult<FaucetResult>;
}

/// WAB Client implementation
///
/// Reference: TS WABClient class (src/wab-client/WABClient.ts)
#[derive(Debug, Clone)]
pub struct WABClient {
    /// Base URL of the WAB server
    base_url: String,
}

impl WABClient {
    /// Create a new WAB client
    ///
    /// # Arguments
    /// * `base_url` - Base URL of the WAB server (e.g., "https://wab.example.com")
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
    
    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[async_trait::async_trait]
impl WABClientTrait for WABClient {
    async fn start_auth_method(
        &self,
        method: &dyn AuthMethodInteractor,
        presentation_key: &str,
        payload: serde_json::Value,
    ) -> WalletResult<AuthStartResult> {
        // TODO: Implement HTTP request to WAB server
        // POST /auth/{method_name}/start
        // Body: { presentationKey, ...payload }
        Err(WalletError::not_implemented("WABClient.start_auth_method requires HTTP client"))
    }
    
    async fn complete_auth_method(
        &self,
        method: &dyn AuthMethodInteractor,
        temp_key: &str,
        payload: serde_json::Value,
    ) -> WalletResult<AuthCompleteResult> {
        // TODO: Implement HTTP request to WAB server
        // POST /auth/{method_name}/complete
        // Body: { tempKey, ...payload }
        Err(WalletError::not_implemented("WABClient.complete_auth_method requires HTTP client"))
    }
    
    async fn request_faucet(&self, presentation_key: &str) -> WalletResult<FaucetResult> {
        // TODO: Implement HTTP request to WAB server
        // POST /faucet/request
        // Body: { presentationKey }
        Err(WalletError::not_implemented("WABClient.request_faucet requires HTTP client"))
    }
}

pub mod auth_method_interactors {
    #[derive(Debug, Default)]
    pub struct TwilioPhoneInteractor;
    #[derive(Debug, Default)]
    pub struct PersonaIDInteractor;
    #[derive(Debug, Default)]
    pub struct AuthMethodInteractor; // placeholder marker type; real trait in wallet-wab-client
}
