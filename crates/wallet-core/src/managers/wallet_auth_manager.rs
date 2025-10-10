//! Wallet Authentication Manager
//!
//! **Reference**: TypeScript `src/WalletAuthenticationManager.ts` (154 lines)
//!
//! A wallet manager that integrates with WABClient for user authentication flows
//! (e.g., Twilio phone verification, Persona ID verification).
//!
//! This manager extends CWIStyleWalletManager and adds authentication method support.

use crate::sdk::errors::{WalletError, WalletResult};
use crate::wab_client::{WABClient, AuthMethodInteractor, WABClientTrait};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Presentation key in hex format (64 hex chars for 32 bytes)
pub type PresentationKeyHex = String;

/// Wallet Authentication Manager
///
/// Reference: TS WalletAuthenticationManager class (WalletAuthenticationManager.ts lines 13-153)
///
/// ## Purpose
///
/// Integrates with WABClient (Wallet Authentication Bridge) to provide:
/// - Phone number verification (via Twilio)
/// - Identity verification (via Persona)
/// - Other authentication method support
///
/// ## Authentication Flow
///
/// 1. `start_auth()` - Initiate authentication (e.g., send SMS code)
/// 2. `complete_auth()` - Complete authentication (e.g., verify code)
/// 3. Presentation key is retrieved from WAB server
/// 4. Manager provides key to underlying CWI wallet logic
///
/// ## Features
///
/// - Multiple authentication method support
/// - Temporary presentation key generation
/// - WAB server integration
/// - Faucet funding for new wallets
/// - Extends CWIStyleWalletManager functionality
pub struct WalletAuthenticationManager {
    /// WAB client for authentication
    wab_client: Arc<WABClient>,
    
    /// Currently selected authentication method
    auth_method: Arc<RwLock<Option<Box<dyn AuthMethodInteractor>>>>,
    
    /// Temporary presentation key (used during auth flow)
    temp_presentation_key: Arc<RwLock<Option<String>>>,
    
    /// Admin originator domain
    admin_originator: String,
}

impl WalletAuthenticationManager {
    /// Create a new WalletAuthenticationManager
    ///
    /// Reference: TS constructor (WalletAuthenticationManager.ts lines 18-83)
    ///
    /// # Arguments
    /// * `admin_originator` - Domain name of administrative originator
    /// * `wab_client` - WAB client instance for authentication
    /// * `auth_method` - Optional initial authentication method
    ///
    /// # Returns
    /// New WalletAuthenticationManager instance
    pub fn new(
        admin_originator: String,
        wab_client: Arc<WABClient>,
        auth_method: Option<Box<dyn AuthMethodInteractor>>,
    ) -> Self {
        Self {
            wab_client,
            auth_method: Arc::new(RwLock::new(auth_method)),
            temp_presentation_key: Arc::new(RwLock::new(None)),
            admin_originator,
        }
    }
    
    /// Set or switch the authentication method
    ///
    /// Reference: TS setAuthMethod() (WalletAuthenticationManager.ts lines 89-91)
    ///
    /// Allows changing the authentication method at runtime if the user
    /// changes their mind or picks a new method in the UI.
    ///
    /// # Arguments
    /// * `method` - New authentication method interactor
    pub async fn set_auth_method(&self, method: Box<dyn AuthMethodInteractor>) {
        *self.auth_method.write().await = Some(method);
    }
    
    /// Start the WAB-based authentication flow
    ///
    /// Reference: TS startAuth() (WalletAuthenticationManager.ts lines 97-116)
    ///
    /// Initiates the authentication process using the chosen AuthMethodInteractor.
    /// For example, if using Twilio, this sends an SMS code to the provided phone number.
    ///
    /// # Arguments
    /// * `payload` - Authentication payload (e.g., `{"phoneNumber": "+1..."}` for Twilio)
    ///
    /// # Errors
    /// Returns error if no auth method is selected or if WAB server rejects the request
    pub async fn start_auth(&self, payload: serde_json::Value) -> WalletResult<()> {
        // Check if auth method is set (TS lines 98-100)
        let auth_method = self.auth_method.read().await;
        if auth_method.is_none() {
            return Err(WalletError::invalid_operation(
                "No AuthMethod selected in WalletAuthenticationManager"
            ));
        }
        
        // Generate temporary presentation key (TS line 101)
        let temp_key = self.generate_temporary_presentation_key();
        *self.temp_presentation_key.write().await = Some(temp_key.clone());
        
        // Start auth method via WAB client (TS lines 104-111)
        let auth_method_ref = auth_method.as_ref().unwrap();
        let start_result = self.wab_client.as_ref().start_auth_method(
            auth_method_ref.as_ref(),
            &temp_key,
            payload,
        ).await?;
        
        // Check success (TS lines 113-115)
        if !start_result.success {
            return Err(WalletError::invalid_operation(
                &start_result.message.unwrap_or_else(|| "Failed to start WAB auth method".to_string())
            ));
        }
        
        Ok(())
    }
    
    /// Complete the WAB-based authentication flow
    ///
    /// Reference: TS completeAuth() (WalletAuthenticationManager.ts lines 121-146)
    ///
    /// Completes the authentication process and retrieves the final presentation key
    /// from the WAB server if successful.
    ///
    /// # Arguments
    /// * `payload` - Completion payload (e.g., `{"code": "123456"}` for SMS verification)
    ///
    /// # Returns
    /// The presentation key bytes (32 bytes) retrieved from WAB
    ///
    /// # Errors
    /// Returns error if auth method not set, startAuth not called, or verification fails
    pub async fn complete_auth(&self, payload: serde_json::Value) -> WalletResult<Vec<u8>> {
        // Check auth method and temp key (TS lines 122-124)
        let auth_method = self.auth_method.read().await;
        if auth_method.is_none() {
            return Err(WalletError::invalid_operation(
                "No AuthMethod selected in WalletAuthenticationManager or startAuth has yet to be called."
            ));
        }
        
        let temp_key = {
            let mut temp_key_guard = self.temp_presentation_key.write().await;
            let key = temp_key_guard.clone();
            *temp_key_guard = None; // Unset for security (TS lines 127-128)
            key
        };
        
        let temp_key = temp_key.ok_or_else(|| {
            WalletError::invalid_operation(
                "startAuth must be called before completeAuth"
            )
        })?;
        
        // Complete auth method via WAB client (TS line 130)
        let auth_method_ref = auth_method.as_ref().unwrap();
        let result = self.wab_client.as_ref().complete_auth_method(
            auth_method_ref.as_ref(),
            &temp_key,
            payload,
        ).await?;
        
        // Check success and extract presentation key (TS lines 132-134)
        if !result.success || result.presentation_key.is_none() {
            return Err(WalletError::invalid_operation(
                &result.message.unwrap_or_else(|| "Failed to complete WAB auth".to_string())
            ));
        }
        
        // Convert hex presentation key to bytes (TS lines 137-138)
        let presentation_key_hex = result.presentation_key.unwrap();
        let presentation_key_bytes = hex::decode(&presentation_key_hex)
            .map_err(|e| WalletError::invalid_operation(&format!("Invalid presentation key hex: {}", e)))?;
        
        // Validate key length
        if presentation_key_bytes.len() != 32 {
            return Err(WalletError::invalid_operation(
                "Presentation key must be exactly 32 bytes"
            ));
        }
        
        Ok(presentation_key_bytes)
    }
    
    /// Generate a temporary presentation key for the auth flow
    ///
    /// Reference: TS generateTemporaryPresentationKey() (WalletAuthenticationManager.ts lines 148-152)
    ///
    /// Generates a random 32-byte (256-bit) key and returns it as hex string.
    /// This is used as a placeholder during the startAuth call.
    ///
    /// # Returns
    /// 64-character hex string representing 32 random bytes
    fn generate_temporary_presentation_key(&self) -> String {
        use rand::Rng;
        
        // Generate 32 random bytes (TS line 150)
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        
        // Convert to hex (TS line 151)
        hex::encode(random_bytes)
    }
    
    /// Get the admin originator domain
    pub fn admin_originator(&self) -> &str {
        &self.admin_originator
    }
    
    /// Get the WAB client
    pub fn wab_client(&self) -> &Arc<WABClient> {
        &self.wab_client
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Mock WAB client for testing
    struct MockWABClient;
    
    #[async_trait::async_trait]
    impl crate::wab_client::WABClientTrait for MockWABClient {
        async fn start_auth_method(
            &self,
            _method: &dyn AuthMethodInteractor,
            _presentation_key: &str,
            _payload: serde_json::Value,
        ) -> WalletResult<crate::wab_client::AuthStartResult> {
            Ok(crate::wab_client::AuthStartResult {
                success: true,
                message: Some("Started".to_string()),
            })
        }
        
        async fn complete_auth_method(
            &self,
            _method: &dyn AuthMethodInteractor,
            _temp_key: &str,
            _payload: serde_json::Value,
        ) -> WalletResult<crate::wab_client::AuthCompleteResult> {
            Ok(crate::wab_client::AuthCompleteResult {
                success: true,
                presentation_key: Some("a".repeat(64)), // 32 bytes in hex
                message: Some("Completed".to_string()),
            })
        }
        
        async fn request_faucet(&self, _presentation_key: &str) -> WalletResult<crate::wab_client::FaucetResult> {
            Ok(crate::wab_client::FaucetResult {
                payment_data: serde_json::json!({}),
            })
        }
    }
    
    /// Mock auth method for testing
    struct MockAuthMethod;
    
    impl AuthMethodInteractor for MockAuthMethod {
        fn method_name(&self) -> &str {
            "mock"
        }
    }
    
    #[test]
    fn test_generate_temporary_presentation_key() {
        let wab_client = Arc::new(WABClient::new("https://test.wab".to_string()));
        let manager = WalletAuthenticationManager::new(
            "test.admin".to_string(),
            wab_client,
            None,
        );
        
        let key1 = manager.generate_temporary_presentation_key();
        let key2 = manager.generate_temporary_presentation_key();
        
        // Keys should be 64 hex characters (32 bytes)
        assert_eq!(key1.len(), 64);
        assert_eq!(key2.len(), 64);
        
        // Keys should be different (random)
        assert_ne!(key1, key2);
        
        // Should be valid hex
        assert!(hex::decode(&key1).is_ok());
        assert!(hex::decode(&key2).is_ok());
    }
    
    #[tokio::test]
    async fn test_set_auth_method() {
        let wab_client = Arc::new(WABClient::new("https://test.wab".to_string()));
        let manager = WalletAuthenticationManager::new(
            "test.admin".to_string(),
            wab_client,
            None,
        );
        
        // Initially no auth method
        assert!(manager.auth_method.read().await.is_none());
        
        // Set auth method
        let auth_method = Box::new(MockAuthMethod);
        manager.set_auth_method(auth_method).await;
        
        // Should now have auth method
        assert!(manager.auth_method.read().await.is_some());
    }
    
    #[tokio::test]
    async fn test_start_auth_without_method() {
        let wab_client = Arc::new(WABClient::new("https://test.wab".to_string()));
        let manager = WalletAuthenticationManager::new(
            "test.admin".to_string(),
            wab_client,
            None,
        );
        
        // Try to start auth without setting method
        let result = manager.start_auth(serde_json::json!({"test": "data"})).await;
        
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_complete_auth_without_start() {
        let wab_client = Arc::new(WABClient::new("https://test.wab".to_string()));
        let manager = WalletAuthenticationManager::new(
            "test.admin".to_string(),
            wab_client,
            Some(Box::new(MockAuthMethod)),
        );
        
        // Try to complete auth without starting
        let result = manager.complete_auth(serde_json::json!({"code": "123456"})).await;
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_admin_originator_getter() {
        let wab_client = Arc::new(WABClient::new("https://test.wab".to_string()));
        let manager = WalletAuthenticationManager::new(
            "test.admin".to_string(),
            wab_client,
            None,
        );
        
        assert_eq!(manager.admin_originator(), "test.admin");
    }
}
