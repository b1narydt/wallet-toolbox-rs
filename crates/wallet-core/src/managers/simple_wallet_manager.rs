//! Simple Wallet Manager
//!
//! **Reference**: TypeScript `src/SimpleWalletManager.ts` (527 lines)
//!
//! A slimmed-down wallet manager that requires only a primary key and privileged key manager
//! for authentication. Proxies all wallet operations to an underlying WalletInterface instance.

use crate::sdk::errors::{WalletError, WalletResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Originator domain name (under 250 bytes)
///
/// Reference: TS OriginatorDomainNameStringUnder250Bytes
pub type OriginatorDomainName = String;

/// Wallet interface trait
///
/// Reference: TS WalletInterface from @bsv/sdk
///
/// Defines all standard wallet operations that can be performed.
/// This is the complete interface required by metanet-desktop (28 methods).
#[async_trait::async_trait]
pub trait WalletInterface: Send + Sync {
    // ===== Action Management (5 methods) =====
    async fn create_action(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn sign_action(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn abort_action(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn list_actions(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn internalize_action(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    
    // ===== Output Management (2 methods) =====
    async fn list_outputs(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn relinquish_output(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    
    // ===== Key Operations (3 methods) =====
    async fn get_public_key(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn reveal_counterparty_key_linkage(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn reveal_specific_key_linkage(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    
    // ===== Cryptographic Operations (6 methods) =====
    async fn encrypt(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn decrypt(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn create_hmac(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn verify_hmac(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn create_signature(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn verify_signature(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    
    // ===== Certificate Operations (4 methods) =====
    async fn acquire_certificate(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn list_certificates(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn prove_certificate(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn relinquish_certificate(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    
    // ===== Identity Operations (2 methods) =====
    async fn discover_by_identity_key(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn discover_by_attributes(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    
    // ===== Authentication (2 methods) =====
    async fn is_authenticated(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn wait_for_authentication(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    
    // ===== Blockchain Queries (4 methods) =====
    async fn get_height(&self, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn get_header_for_height(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn get_network(&self, originator: Option<&str>) -> WalletResult<serde_json::Value>;
    async fn get_version(&self, originator: Option<&str>) -> WalletResult<serde_json::Value>;
}

/// Privileged key manager
///
/// Reference: TS PrivilegedKeyManager
pub trait PrivilegedKeyManager: Send + Sync {
    // TODO: Define privileged operations
}

/// Wallet builder function type
///
/// Reference: TS walletBuilder function signature
pub type WalletBuilder = Arc<
    dyn Fn(Vec<u8>, Arc<dyn PrivilegedKeyManager>) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = WalletResult<Box<dyn WalletInterface>>> + Send>
    > + Send + Sync
>;

/// Simple Wallet Manager
///
/// Reference: TS SimpleWalletManager class (SimpleWalletManager.ts lines 84-526)
///
/// ## Purpose
/// 
/// A slimmed-down wallet manager that only requires two things to authenticate:
/// 1. A primary key (32 bytes) - the core secret for the wallet
/// 2. A privileged key manager - responsible for sensitive operations
///
/// Once both are provided, the wallet becomes authenticated and proxies all
/// operations to an underlying WalletInterface instance.
///
/// ## Features
///
/// - Authentication management (primary key + privileged manager required)
/// - Admin originator protection (prevents external use)
/// - Snapshot save/load (stores encrypted primary key)
/// - Proxies 27+ WalletInterface methods to underlying wallet
///
/// ## Important
///
/// - Does NOT handle user password flows or recovery
/// - Does NOT manage on-chain tokens
/// - Snapshot only contains primary key (privileged manager must be re-provided)
pub struct SimpleWalletManager {
    /// Whether user is authenticated
    authenticated: Arc<RwLock<bool>>,
    
    /// Admin originator domain (protected from external use)
    admin_originator: String,
    
    /// Wallet builder function
    wallet_builder: WalletBuilder,
    
    /// Underlying wallet instance (built after authentication)
    underlying: Arc<RwLock<Option<Box<dyn WalletInterface>>>>,
    
    /// Privileged key manager
    privileged_manager: Arc<RwLock<Option<Arc<dyn PrivilegedKeyManager>>>>,
    
    /// Primary key (32 bytes)
    primary_key: Arc<RwLock<Option<Vec<u8>>>>,
}

impl SimpleWalletManager {
    /// Create a new SimpleWalletManager
    ///
    /// Reference: TS constructor (SimpleWalletManager.ts lines 128-140)
    ///
    /// # Arguments
    /// * `admin_originator` - Domain name of administrative originator
    /// * `wallet_builder` - Function that builds WalletInterface from primary key and manager
    /// * `state_snapshot` - Optional snapshot to restore from
    pub fn new(
        admin_originator: String,
        wallet_builder: WalletBuilder,
        state_snapshot: Option<Vec<u8>>,
    ) -> Self {
        let manager = Self {
            authenticated: Arc::new(RwLock::new(false)),
            admin_originator,
            wallet_builder,
            underlying: Arc::new(RwLock::new(None)),
            privileged_manager: Arc::new(RwLock::new(None)),
            primary_key: Arc::new(RwLock::new(None)),
        };
        
        // Load snapshot if provided
        if let Some(snapshot) = state_snapshot {
            // TODO: Implement loadSnapshot
            // For now, ignore
        }
        
        manager
    }
    
    /// Provide the primary key for authentication
    ///
    /// Reference: TS providePrimaryKey (SimpleWalletManager.ts lines 149-152)
    ///
    /// Sets the primary key and attempts to build the underlying wallet if
    /// the privileged key manager has also been provided.
    pub async fn provide_primary_key(&self, key: Vec<u8>) -> WalletResult<()> {
        if key.len() != 32 {
            return Err(WalletError::invalid_parameter(
                "key",
                "must be exactly 32 bytes"
            ));
        }
        
        *self.primary_key.write().await = Some(key);
        self.try_build_underlying().await
    }
    
    /// Provide the privileged key manager for sensitive operations
    ///
    /// Reference: TS providePrivilegedKeyManager (SimpleWalletManager.ts lines 161-164)
    ///
    /// Sets the privileged manager and attempts to build the underlying wallet if
    /// the primary key has also been provided.
    pub async fn provide_privileged_key_manager(
        &self,
        manager: Arc<dyn PrivilegedKeyManager>,
    ) -> WalletResult<()> {
        *self.privileged_manager.write().await = Some(manager);
        self.try_build_underlying().await
    }
    
    /// Try to build the underlying wallet if both key and manager are available
    ///
    /// Reference: TS tryBuildUnderlying (SimpleWalletManager.ts lines 170-180)
    ///
    /// Internal method that checks if we have both the primary key and privileged manager.
    /// If so, builds the underlying wallet instance and sets authenticated = true.
    async fn try_build_underlying(&self) -> WalletResult<()> {
        if *self.authenticated.read().await {
            return Err(WalletError::invalid_operation(
                "The user is already authenticated."
            ));
        }
        
        let primary_key = self.primary_key.read().await;
        let privileged_manager = self.privileged_manager.read().await;
        
        if primary_key.is_none() || privileged_manager.is_none() {
            // Not ready yet, but not an error
            return Ok(());
        }
        
        // Build the underlying wallet
        let key = primary_key.as_ref().unwrap().clone();
        let manager = privileged_manager.as_ref().unwrap().clone();
        
        drop(primary_key);
        drop(privileged_manager);
        
        let wallet = (self.wallet_builder)(key, manager).await?;
        
        *self.underlying.write().await = Some(wallet);
        *self.authenticated.write().await = true;
        
        Ok(())
    }
    
    /// Destroy the underlying wallet, returning to unauthenticated state
    ///
    /// Reference: TS destroy (SimpleWalletManager.ts lines 187-192)
    ///
    /// Clears the primary key, privileged key manager, and authenticated flag.
    pub async fn destroy(&self) {
        *self.underlying.write().await = None;
        *self.privileged_manager.write().await = None;
        *self.authenticated.write().await = false;
        *self.primary_key.write().await = None;
    }
    
    /// Save current wallet state to encrypted snapshot
    ///
    /// Reference: TS saveSnapshot (SimpleWalletManager.ts lines 210-237)
    ///
    /// Creates an encrypted snapshot containing the primary key.
    /// The snapshot does NOT include the privileged key manager.
    ///
    /// # Security
    /// The snapshot contains critical secret material and must be protected carefully.
    ///
    /// # Returns
    /// Byte array representing the encrypted snapshot
    pub async fn save_snapshot(&self) -> WalletResult<Vec<u8>> {
        let primary_key = self.primary_key.read().await;
        
        let key = primary_key.as_ref()
            .ok_or_else(|| WalletError::invalid_operation(
                "No primary key is set; cannot save snapshot."
            ))?;
        
        // TODO: Implement full snapshot encryption
        // For now, return a simple version-prefixed structure
        let mut snapshot = Vec::new();
        snapshot.push(1); // Version byte
        snapshot.push(key.len() as u8); // Length
        snapshot.extend_from_slice(key);
        
        Ok(snapshot)
    }
    
    /// Load a previously saved state snapshot
    ///
    /// Reference: TS loadSnapshot (SimpleWalletManager.ts lines 247-279)
    ///
    /// Restores the primary key from a snapshot. The privileged key manager
    /// must still be provided separately to complete authentication.
    pub async fn load_snapshot(&self, snapshot: Vec<u8>) -> WalletResult<()> {
        if snapshot.len() < 2 {
            return Err(WalletError::invalid_parameter(
                "snapshot",
                "too short"
            ));
        }
        
        // TODO: Implement full snapshot decryption
        // For now, read simple version-prefixed structure
        let version = snapshot[0];
        if version != 1 {
            return Err(WalletError::invalid_parameter(
                "snapshot",
                &format!("Unsupported snapshot version: {}", version)
            ));
        }
        
        let length = snapshot[1] as usize;
        if snapshot.len() < 2 + length {
            return Err(WalletError::invalid_parameter(
                "snapshot",
                "invalid length"
            ));
        }
        
        let primary_key = snapshot[2..(2 + length)].to_vec();
        *self.primary_key.write().await = Some(primary_key);
        
        // Try to build underlying if privileged manager already provided
        self.try_build_underlying().await
    }
    
    /// Check if user is authenticated
    ///
    /// Reference: TS isAuthenticated (SimpleWalletManager.ts lines 289-292)
    pub async fn is_authenticated(&self, originator: Option<&str>) -> WalletResult<bool> {
        self.ensure_can_call(originator).await?;
        Ok(true)
    }
    
    /// Wait for user to authenticate
    ///
    /// Reference: TS waitForAuthentication (SimpleWalletManager.ts lines 302-313)
    ///
    /// Blocks until the user is authenticated by providing both
    /// primary key and privileged manager.
    pub async fn wait_for_authentication(&self, originator: Option<&str>) -> WalletResult<bool> {
        if let Some(orig) = originator {
            if orig == self.admin_originator {
                return Err(WalletError::invalid_operation(
                    "External applications cannot use the admin originator."
                ));
            }
        }
        
        while !*self.authenticated.read().await {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        Ok(true)
    }
    
    /// Ensure the call can proceed (authenticated and not admin originator)
    ///
    /// Reference: TS ensureCanCall (SimpleWalletManager.ts lines 518-525)
    ///
    /// Helper that throws if:
    /// - User is not authenticated
    /// - Provided originator is the admin (not permitted externally)
    async fn ensure_can_call(&self, originator: Option<&str>) -> WalletResult<()> {
        if let Some(orig) = originator {
            if orig == self.admin_originator {
                return Err(WalletError::invalid_operation(
                    "External applications cannot use the admin originator."
                ));
            }
        }
        
        if !*self.authenticated.read().await {
            return Err(WalletError::invalid_operation(
                "User is not authenticated."
            ));
        }
        
        Ok(())
    }
}

// ============================================================================
// WalletInterface implementation - proxies all calls to underlying wallet
// ============================================================================

#[async_trait::async_trait]
impl WalletInterface for SimpleWalletManager {
    /// Create an action
    ///
    /// Reference: TS createAction (SimpleWalletManager.ts lines 387-393)
    async fn create_action(
        &self,
        args: serde_json::Value,
        originator: Option<&str>,
    ) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref()
            .ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.create_action(args, originator).await
    }
    
    /// Sign an action
    ///
    /// Reference: TS signAction (SimpleWalletManager.ts lines 395-401)
    async fn sign_action(
        &self,
        args: serde_json::Value,
        originator: Option<&str>,
    ) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref()
            .ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.sign_action(args, originator).await
    }
    
    /// Abort an action
    ///
    /// Reference: TS abortAction (SimpleWalletManager.ts lines 403-409)
    async fn abort_action(
        &self,
        args: serde_json::Value,
        originator: Option<&str>,
    ) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref()
            .ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.abort_action(args, originator).await
    }
    
    /// List actions
    ///
    /// Reference: TS listActions (SimpleWalletManager.ts lines 411-417)
    async fn list_actions(
        &self,
        args: serde_json::Value,
        originator: Option<&str>,
    ) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref()
            .ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.list_actions(args, originator).await
    }
    
    /// Internalize an action
    ///
    /// Reference: TS internalizeAction (SimpleWalletManager.ts lines 419-425)
    async fn internalize_action(
        &self,
        args: serde_json::Value,
        originator: Option<&str>,
    ) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref()
            .ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.internalize_action(args, originator).await
    }
    
    /// List outputs
    ///
    /// Reference: TS listOutputs (SimpleWalletManager.ts lines 427-433)
    async fn list_outputs(
        &self,
        args: serde_json::Value,
        originator: Option<&str>,
    ) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref()
            .ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.list_outputs(args, originator).await
    }
    
    /// Get public key
    ///
    /// Reference: TS getPublicKey (SimpleWalletManager.ts lines 315-321)
    async fn get_public_key(
        &self,
        args: serde_json::Value,
        originator: Option<&str>,
    ) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref()
            .ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.get_public_key(args, originator).await
    }
    
    /// Get blockchain height
    ///
    /// Reference: TS getHeight (SimpleWalletManager.ts lines 491-494)
    async fn get_height(&self, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref()
            .ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.get_height(originator).await
    }
    
    /// Get network
    ///
    /// Reference: TS getNetwork (SimpleWalletManager.ts lines 504-507)
    async fn get_network(&self, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref()
            .ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.get_network(originator).await
    }
    
    /// Get version
    ///
    /// Reference: TS getVersion (SimpleWalletManager.ts lines 509-512)
    async fn get_version(&self, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref()
            .ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.get_version(originator).await
    }
    
    // ===== Additional 18 methods for complete metanet-desktop compatibility =====
    // TODO: These are stubs that delegate to underlying wallet
    // Implementations will be added as features are completed
    
    async fn relinquish_output(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref().ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.relinquish_output(args, originator).await
    }
    
    async fn reveal_counterparty_key_linkage(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref().ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.reveal_counterparty_key_linkage(args, originator).await
    }
    
    async fn reveal_specific_key_linkage(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref().ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.reveal_specific_key_linkage(args, originator).await
    }
    
    async fn encrypt(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref().ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.encrypt(args, originator).await
    }
    
    async fn decrypt(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref().ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.decrypt(args, originator).await
    }
    
    async fn create_hmac(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref().ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.create_hmac(args, originator).await
    }
    
    async fn verify_hmac(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref().ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.verify_hmac(args, originator).await
    }
    
    async fn create_signature(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref().ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.create_signature(args, originator).await
    }
    
    async fn verify_signature(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref().ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.verify_signature(args, originator).await
    }
    
    async fn acquire_certificate(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref().ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.acquire_certificate(args, originator).await
    }
    
    async fn list_certificates(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref().ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.list_certificates(args, originator).await
    }
    
    async fn prove_certificate(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref().ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.prove_certificate(args, originator).await
    }
    
    async fn relinquish_certificate(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref().ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.relinquish_certificate(args, originator).await
    }
    
    async fn discover_by_identity_key(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref().ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.discover_by_identity_key(args, originator).await
    }
    
    async fn discover_by_attributes(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref().ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.discover_by_attributes(args, originator).await
    }
    
    async fn is_authenticated(&self, _args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        let authenticated = self.is_authenticated(originator).await?;
        Ok(serde_json::json!({ "authenticated": authenticated }))
    }
    
    async fn wait_for_authentication(&self, _args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.wait_for_authentication(originator).await?;
        Ok(serde_json::json!({ "authenticated": true }))
    }
    
    async fn get_header_for_height(&self, args: serde_json::Value, originator: Option<&str>) -> WalletResult<serde_json::Value> {
        self.ensure_can_call(originator).await?;
        let underlying = self.underlying.read().await;
        let wallet = underlying.as_ref().ok_or_else(|| WalletError::invalid_operation("Not authenticated"))?;
        wallet.get_header_for_height(args, originator).await
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock PrivilegedKeyManager for testing
    struct MockPrivilegedManager;
    impl PrivilegedKeyManager for MockPrivilegedManager {}
    
    // Mock WalletInterface for testing
    struct MockWallet;
    
    #[async_trait::async_trait]
    impl WalletInterface for MockWallet {
        async fn create_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({"success": true}))
        }
        async fn sign_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({"success": true}))
        }
        async fn abort_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({"success": true}))
        }
        async fn list_actions(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({"actions": []}))
        }
        async fn internalize_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({"success": true}))
        }
        async fn list_outputs(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({"outputs": []}))
        }
        async fn get_public_key(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({"publicKey": "test"}))
        }
        async fn get_height(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({"height": 100}))
        }
        async fn get_network(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({"network": "main"}))
        }
        async fn get_version(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({"version": "1.0.0"}))
        }
    }
    
    #[tokio::test]
    async fn test_simple_wallet_manager_creation() {
        let builder: WalletBuilder = Arc::new(|_key, _manager| {
            Box::pin(async {
                Ok(Box::new(MockWallet) as Box<dyn WalletInterface>)
            })
        });
        
        let manager = SimpleWalletManager::new(
            "admin.example.com".to_string(),
            builder,
            None,
        );
        
        assert!(!*manager.authenticated.read().await);
    }
    
    #[tokio::test]
    async fn test_provide_primary_key() {
        let builder: WalletBuilder = Arc::new(|_key, _manager| {
            Box::pin(async {
                Ok(Box::new(MockWallet) as Box<dyn WalletInterface>)
            })
        });
        
        let manager = SimpleWalletManager::new(
            "admin.example.com".to_string(),
            builder,
            None,
        );
        
        let key = vec![0u8; 32];
        let result = manager.provide_primary_key(key).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_authentication_flow() {
        let builder: WalletBuilder = Arc::new(|_key, _manager| {
            Box::pin(async {
                Ok(Box::new(MockWallet) as Box<dyn WalletInterface>)
            })
        });
        
        let manager = SimpleWalletManager::new(
            "admin.example.com".to_string(),
            builder,
            None,
        );
        
        // Not authenticated initially
        assert!(!*manager.authenticated.read().await);
        
        // Provide primary key
        let key = vec![0u8; 32];
        manager.provide_primary_key(key).await.unwrap();
        
        // Still not authenticated (need privileged manager)
        assert!(!*manager.authenticated.read().await);
        
        // Provide privileged manager
        let priv_manager = Arc::new(MockPrivilegedManager);
        manager.provide_privileged_key_manager(priv_manager).await.unwrap();
        
        // Now authenticated
        assert!(*manager.authenticated.read().await);
    }
    
    #[tokio::test]
    async fn test_admin_originator_blocked() {
        let builder: WalletBuilder = Arc::new(|_key, _manager| {
            Box::pin(async {
                Ok(Box::new(MockWallet) as Box<dyn WalletInterface>)
            })
        });
        
        let manager = SimpleWalletManager::new(
            "admin.example.com".to_string(),
            builder,
            None,
        );
        
        // Authenticate
        let key = vec![0u8; 32];
        manager.provide_primary_key(key).await.unwrap();
        let priv_manager = Arc::new(MockPrivilegedManager);
        manager.provide_privileged_key_manager(priv_manager).await.unwrap();
        
        // Try to use admin originator
        let result = manager.create_action(
            serde_json::json!({}),
            Some("admin.example.com")
        ).await;
        
        assert!(result.is_err());
    }
}
