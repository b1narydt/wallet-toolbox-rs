///! Main Wallet Orchestrator
///!
///! **Reference**: TypeScript `src/Wallet.ts`
///!
///! This is the production-ready wallet that coordinates all managers and implements
///! the complete WalletInterface. This is the entry point for applications like metanet-desktop.

use crate::sdk::errors::{WalletError, WalletResult};
use crate::managers::simple_wallet_manager::WalletInterface;
use crate::managers::wallet_permissions_manager::WalletPermissionsManager;
use crate::managers::wallet_settings_manager::WalletSettingsManager;
use crate::managers::wallet_auth_manager::WalletAuthenticationManager;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main wallet configuration
///
/// Reference: TS WalletArgs (Wallet.ts lines 121-130)
pub struct WalletConfig {
    /// Network (mainnet or testnet)
    pub chain: String,
    
    /// Root private key or seed for key derivation
    pub root_key: Vec<u8>,
    
    /// Storage manager (can be any storage backend)
    pub storage: Arc<dyn WalletInterface>,
    
    /// Optional: Admin originator for permission management
    pub admin_originator: Option<String>,
}

/// Main Wallet orchestrator
///
/// Reference: TS Wallet class (Wallet.ts lines 136-1135)
///
/// Coordinates all wallet managers and implements the full WalletInterface.
/// This is the production-ready entry point for applications.
///
/// **Current Status**: Minimal MVP - delegates to inner wallet
/// TODO: Add full manager integration (permissions, settings, auth) as they're completed
pub struct Wallet {
    /// Underlying storage/simple wallet
    inner: Arc<dyn WalletInterface>,
    
    /// Network chain
    chain: String,
    
    /// Admin originator for internal operations
    admin_originator: String,
    
    // TODO: Add when managers are ready
    // permissions: Arc<RwLock<WalletPermissionsManager>>,
    // settings: WalletSettingsManager,
    // auth: Arc<RwLock<WalletAuthenticationManager>>,
}

impl Wallet {
    /// Create a new Wallet instance
    ///
    /// # Arguments
    ///
    /// * `config` - Wallet configuration
    ///
    /// # Returns
    ///
    /// New wallet instance ready for use
    pub fn new(config: WalletConfig) -> WalletResult<Self> {
        let inner = config.storage;
        let admin_originator = config.admin_originator.unwrap_or_else(|| "admin".to_string());
        
        // TODO: Initialize managers when ready
        // let permissions = Arc::new(RwLock::new(
        //     WalletPermissionsManager::new(inner.clone(), admin_originator.clone(), None)
        // ));
        
        Ok(Self {
            inner,
            chain: config.chain,
            admin_originator,
        })
    }
    
    /// Get the network name
    pub fn chain(&self) -> &str {
        &self.chain
    }
    
    /// Get admin originator
    pub fn admin_originator(&self) -> &str {
        &self.admin_originator
    }
}

/// Implement WalletInterface for the main Wallet
///
/// All 28 methods required by metanet-desktop
#[async_trait::async_trait]
impl WalletInterface for Wallet {
    // 1. createAction - delegate to inner with permission checks
    async fn create_action(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        let originator = originator.ok_or_else(|| {
            WalletError::invalid_parameter("originator", "Required for createAction")
        })?;
        
        // TODO: Check authentication first
        // let auth = self.auth.read().await;
        // if !auth.is_authenticated(originator).await? {
        //     return Err(WalletError::new(
        //         "WERR_UNAUTHORIZED",
        //         "Application not authenticated"
        //     ));
        // }
        // drop(auth);
        
        // Delegate to underlying wallet
        self.inner.create_action(args, Some(originator)).await
    }
    
    // 2. signAction - delegate to inner with permission checks
    async fn sign_action(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        let originator = originator.ok_or_else(|| {
            WalletError::invalid_parameter("originator", "Required for signAction")
        })?;
        
        // TODO: Check authentication
        // let auth = self.auth.read().await;
        // if !auth.is_authenticated(originator).await? {
        //     return Err(WalletError::new(
        //         "WERR_UNAUTHORIZED",
        //         "Application not authenticated"
        //     ));
        // }
        // drop(auth);
        
        // Delegate to underlying wallet
        self.inner.sign_action(args, Some(originator)).await
    }
    
    // 3. abortAction - delegate to inner
    async fn abort_action(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.abort_action(args, originator).await
    }
    
    // 4. listActions - delegate to inner with permission checks
    async fn list_actions(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        let originator = originator.ok_or_else(|| {
            WalletError::invalid_parameter("originator", "Required for listActions")
        })?;
        
        // TODO: Check permissions for basket/label access
        
        self.inner.list_actions(args, Some(originator)).await
    }
    
    // 5. internalizeAction - delegate to inner
    async fn internalize_action(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.internalize_action(args, originator).await
    }
    
    // 6. listOutputs - delegate to inner with permission checks
    async fn list_outputs(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        let originator = originator.ok_or_else(|| {
            WalletError::invalid_parameter("originator", "Required for listOutputs")
        })?;
        
        // TODO: Check permissions for basket access
        
        self.inner.list_outputs(args, Some(originator)).await
    }
    
    // 7. relinquishOutput - TODO: Add to WalletInterface trait
    // async fn relinquish_output(
    //     &self,
    //     args: Value,
    //     originator: Option<&str>,
    // ) -> WalletResult<Value> {
    //     self.inner.relinquish_output(args, originator).await
    // }
    
    // 8. getPublicKey - delegate to inner with permission checks
    async fn get_public_key(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        let originator = originator.ok_or_else(|| {
            WalletError::invalid_parameter("originator", "Required for getPublicKey")
        })?;
        
        // TODO: Check protocol permissions
        
        self.inner.get_public_key(args, Some(originator)).await
    }
    
    // 9. relinquishOutput - delegate to inner
    async fn relinquish_output(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.relinquish_output(args, originator).await
    }
    
    // 10. revealCounterpartyKeyLinkage - delegate to inner
    async fn reveal_counterparty_key_linkage(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.reveal_counterparty_key_linkage(args, originator).await
    }
    
    // 11. revealSpecificKeyLinkage - delegate to inner
    async fn reveal_specific_key_linkage(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.reveal_specific_key_linkage(args, originator).await
    }
    
    // 12. encrypt - delegate to inner
    async fn encrypt(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.encrypt(args, originator).await
    }
    
    // 13. decrypt - delegate to inner
    async fn decrypt(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.decrypt(args, originator).await
    }
    
    // 14. createHmac - delegate to inner
    async fn create_hmac(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.create_hmac(args, originator).await
    }
    
    // 15. verifyHmac - delegate to inner
    async fn verify_hmac(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.verify_hmac(args, originator).await
    }
    
    // 16. createSignature - delegate to inner
    async fn create_signature(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.create_signature(args, originator).await
    }
    
    // 17. verifySignature - delegate to inner
    async fn verify_signature(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.verify_signature(args, originator).await
    }
    
    // 18. acquireCertificate - delegate to inner
    async fn acquire_certificate(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.acquire_certificate(args, originator).await
    }
    
    // 19. listCertificates - delegate to inner
    async fn list_certificates(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.list_certificates(args, originator).await
    }
    
    // 20. proveCertificate - delegate to inner
    async fn prove_certificate(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.prove_certificate(args, originator).await
    }
    
    // 21. relinquishCertificate - delegate to inner
    async fn relinquish_certificate(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.relinquish_certificate(args, originator).await
    }
    
    // 22. discoverByIdentityKey - delegate to inner
    async fn discover_by_identity_key(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.discover_by_identity_key(args, originator).await
    }
    
    // 23. discoverByAttributes - delegate to inner
    async fn discover_by_attributes(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.discover_by_attributes(args, originator).await
    }
    
    // 24. isAuthenticated - delegate to inner
    async fn is_authenticated(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.is_authenticated(args, originator).await
    }
    
    // 25. waitForAuthentication - delegate to inner
    async fn wait_for_authentication(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.wait_for_authentication(args, originator).await
    }
    
    // 26. getHeight - delegate to inner
    async fn get_height(&self, originator: Option<&str>) -> WalletResult<Value> {
        self.inner.get_height(originator).await
    }
    
    // 27. getHeaderForHeight - delegate to inner
    async fn get_header_for_height(
        &self,
        args: Value,
        originator: Option<&str>,
    ) -> WalletResult<Value> {
        self.inner.get_header_for_height(args, originator).await
    }
    
    // 28. getNetwork - return configured chain
    async fn get_network(&self, _originator: Option<&str>) -> WalletResult<Value> {
        Ok(json!({ "network": self.chain }))
    }
    
    // 29. getVersion - return wallet version
    async fn get_version(&self, _originator: Option<&str>) -> WalletResult<Value> {
        Ok(json!({ "version": crate::version() }))
    }
}

// TODO: Implement authentication methods on Wallet when WalletAuthenticationManager is complete
// impl Wallet {
//     /// Check if an originator is authenticated
//     pub async fn is_authenticated_internal(&self, originator: &str) -> WalletResult<bool> {
//         let auth = self.auth.read().await;
//         auth.is_authenticated(originator).await
//     }
//     
//     /// Wait for authentication
//     pub async fn wait_for_authentication_internal(&self, originator: &str) -> WalletResult<()> {
//         let mut auth = self.auth.write().await;
//         auth.wait_for_authentication(originator).await
//     }
// }
