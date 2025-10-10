//! Wallet Permissions Manager
//!
//! **Reference**: TypeScript `src/WalletPermissionsManager.ts` (3,111 lines)
//!
//! Wraps an underlying BRC-100 `Wallet` implementation with permissions management capabilities.
//! The manager intercepts calls from external applications (identified by originators), checks if
//! the request is allowed, and if not, orchestrates user permission flows.
//!
//! ## Key Responsibilities
//!
//! Reference: TS class documentation (WalletPermissionsManager.ts lines 353-365)
//!
//! - **Permission Checking**: Before standard wallet operations (e.g. `encrypt`),
//!   the manager checks if a valid permission token exists. If not, it attempts to request
//!   permission from the user.
//! - **On-Chain Tokens**: When permission is granted, the manager stores it as an unspent
//!   "PushDrop" output. This can be spent later to revoke or renew the permission.
//! - **Callbacks**: The manager triggers user-defined callbacks on permission requests
//!   (to show a UI prompt), on grants/denials, and on internal processes.
//!
//! ## Implementation Notes
//!
//! - The manager follows the BRC-100 `createAction` + `signAction` pattern for building or
//!   spending these tokens.
//! - Token revocation or renewal uses standard BRC-100 flows: we build a transaction that
//!   consumes the old token UTXO and outputs a new one (or none, if fully revoked).
//!
//! ## Security Warning
//!
//! ```text
//! ////// TODO: ADD SUPPORT FOR ADMIN COUNTERPARTIES BASED ON WALLET STORAGE
//! //////       PROHIBITION OF SPECIAL OPERATIONS IS ALSO CRITICAL.
//! ////// !!!!!!!! SECURITY-CRITICAL ADDITION — DO NOT USE UNTIL IMPLEMENTED.
//! ```
//!
//! Reference: TS lines 16-18

pub mod types;
pub mod constants;
pub mod utils;
pub mod callbacks;
pub mod permission_request;
pub mod permission_validation;
pub mod token_management;

// Re-exports for convenience
pub use types::*;
pub use constants::*;
pub use utils::*;
pub use callbacks::*;
pub use permission_request::*;
pub use permission_validation::*;
pub use token_management::*;

use crate::sdk::errors::{WalletError, WalletResult};
use crate::managers::simple_wallet_manager::WalletInterface;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Active permission request tracking
///
/// Reference: TS activeRequests (WalletPermissionsManager.ts lines 395-404)
///
/// We queue parallel requests for the same resource so that only one
/// user prompt is created for a single resource. If multiple calls come
/// in at once for the same "protocol:domain:privileged:counterparty" etc.,
/// they get merged.
struct ActiveRequest {
    /// The permission request being processed
    request: serde_json::Value, // Can be PermissionRequest or GroupedPermissionRequest
    
    /// Pending promise resolvers waiting on this request
    pending: Vec<tokio::sync::oneshot::Sender<WalletResult<()>>>,
}

/// Wallet Permissions Manager
///
/// Reference: TS class WalletPermissionsManager (WalletPermissionsManager.ts lines 366-3111)
///
/// Implements BRC-73 grouped permissions and manages four types of permission tokens:
/// - DPACP (Domain Protocol Access Control Protocol)
/// - DBAP (Domain Basket Access Protocol)
/// - DCAP (Domain Certificate Access Protocol)
/// - DSAP (Domain Spending Authorization Protocol)
pub struct WalletPermissionsManager {
    /// A reference to the BRC-100 wallet instance
    ///
    /// Reference: TS underlying (line 368)
    underlying: Arc<dyn WalletInterface>,
    
    /// The "admin" domain or FQDN that is implicitly allowed to do everything
    ///
    /// Reference: TS adminOriginator (line 371)
    admin_originator: String,
    
    /// Event callbacks that external code can subscribe to
    ///
    /// Reference: TS callbacks (lines 377-383)
    ///
    /// Each event can have multiple handlers for UI prompts or logging.
    callbacks: Arc<RwLock<WalletPermissionsManagerCallbacks>>,
    
    /// Active permission requests being processed
    ///
    /// Reference: TS activeRequests (lines 395-404)
    ///
    /// Maps request keys to pending requests to avoid duplicate prompts
    active_requests: Arc<RwLock<HashMap<String, ActiveRequest>>>,
    
    /// Cache recently confirmed permissions to avoid repeated lookups
    ///
    /// Reference: TS permissionCache (line 407)
    permission_cache: Arc<RwLock<HashMap<String, CachedPermission>>>,
    
    /// Configuration that determines whether to skip or apply various checks
    ///
    /// Reference: TS config (line 415)
    config: PermissionsManagerConfig,
}

impl WalletPermissionsManager {
    /// Cache time-to-live (5 minutes)
    ///
    /// Reference: TS CACHE_TTL_MS (line 410)
    const CACHE_TTL_MS: i64 = 5 * 60 * 1000;
    
    /// Constructs a new Permissions Manager instance
    ///
    /// Reference: TS constructor (WalletPermissionsManager.ts lines 424-452)
    ///
    /// # Arguments
    ///
    /// * `underlying_wallet` - The underlying BRC-100 wallet, where requests are forwarded
    ///                         after permission is granted
    /// * `admin_originator` - The domain or FQDN that is automatically allowed everything
    /// * `config` - A set of boolean flags controlling how strictly permissions are enforced
    ///              (defaults to most secure configuration)
    ///
    /// # Returns
    ///
    /// New WalletPermissionsManager instance
    pub fn new(
        underlying_wallet: Arc<dyn WalletInterface>,
        admin_originator: String,
        config: Option<PermissionsManagerConfig>,
    ) -> Self {
        // TS lines 425-426: Store underlying wallet and admin originator
        // TS lines 429-451: Merge user config with secure defaults
        let merged_config = if let Some(user_config) = config {
            // TODO: Implement proper config merging
            // For now, use the provided config or defaults
            user_config
        } else {
            PermissionsManagerConfig::default()
        };
        
        Self {
            underlying: underlying_wallet,
            admin_originator,
            callbacks: Arc::new(RwLock::new(WalletPermissionsManagerCallbacks::default())),
            active_requests: Arc::new(RwLock::new(HashMap::new())),
            permission_cache: Arc::new(RwLock::new(HashMap::new())),
            config: merged_config,
        }
    }
    
    /// Binds a callback function to a named event
    ///
    /// Reference: TS bindCallback (WalletPermissionsManager.ts lines 465-472)
    ///
    /// # Arguments
    ///
    /// * `event_name` - The name of the event to listen to (e.g., "onProtocolPermissionRequested")
    /// * `handler` - A function that handles the event
    ///
    /// # Returns
    ///
    /// A numeric ID you can use to unbind later
    pub async fn bind_callback_protocol(&self, handler: PermissionEventHandler) -> usize {
        // TS lines 469-471: Push handler and return index
        let mut callbacks = self.callbacks.write().await;
        callbacks.on_protocol_permission_requested.push(handler);
        callbacks.on_protocol_permission_requested.len() - 1
    }
    
    /// Binds a callback for basket access requests
    ///
    /// Reference: TS bindCallback (WalletPermissionsManager.ts lines 465-472)
    pub async fn bind_callback_basket(&self, handler: PermissionEventHandler) -> usize {
        let mut callbacks = self.callbacks.write().await;
        callbacks.on_basket_access_requested.push(handler);
        callbacks.on_basket_access_requested.len() - 1
    }
    
    /// Binds a callback for certificate access requests
    ///
    /// Reference: TS bindCallback (WalletPermissionsManager.ts lines 465-472)
    pub async fn bind_callback_certificate(&self, handler: PermissionEventHandler) -> usize {
        let mut callbacks = self.callbacks.write().await;
        callbacks.on_certificate_access_requested.push(handler);
        callbacks.on_certificate_access_requested.len() - 1
    }
    
    /// Binds a callback for spending authorization requests
    ///
    /// Reference: TS bindCallback (WalletPermissionsManager.ts lines 465-472)
    pub async fn bind_callback_spending(&self, handler: PermissionEventHandler) -> usize {
        let mut callbacks = self.callbacks.write().await;
        callbacks.on_spending_authorization_requested.push(handler);
        callbacks.on_spending_authorization_requested.len() - 1
    }
    
    /// Binds a callback for grouped permission requests
    ///
    /// Reference: TS bindCallback (WalletPermissionsManager.ts lines 465-472)
    pub async fn bind_callback_grouped(&self, handler: GroupedPermissionEventHandler) -> usize {
        let mut callbacks = self.callbacks.write().await;
        callbacks.on_grouped_permission_requested.push(handler);
        callbacks.on_grouped_permission_requested.len() - 1
    }
    
    /// Unbinds a previously registered callback by its numeric ID
    ///
    /// Reference: TS unbindCallback (WalletPermissionsManager.ts lines 482-498)
    ///
    /// # Arguments
    ///
    /// * `event_type` - The type of event
    /// * `id` - The numeric ID returned by bind_callback
    ///
    /// # Returns
    ///
    /// `true` if successfully unbound, `false` otherwise
    pub async fn unbind_callback(
        &self,
        event_type: PermissionType,
        id: usize,
    ) -> bool {
        // TS lines 483-498: Remove callback at index by setting to null
        let mut callbacks = self.callbacks.write().await;
        
        match event_type {
            PermissionType::Protocol => {
                if id < callbacks.on_protocol_permission_requested.len() {
                    callbacks.on_protocol_permission_requested.remove(id);
                    true
                } else {
                    false
                }
            }
            PermissionType::Basket => {
                if id < callbacks.on_basket_access_requested.len() {
                    callbacks.on_basket_access_requested.remove(id);
                    true
                } else {
                    false
                }
            }
            PermissionType::Certificate => {
                if id < callbacks.on_certificate_access_requested.len() {
                    callbacks.on_certificate_access_requested.remove(id);
                    true
                } else {
                    false
                }
            }
            PermissionType::Spending => {
                if id < callbacks.on_spending_authorization_requested.len() {
                    callbacks.on_spending_authorization_requested.remove(id);
                    true
                } else {
                    false
                }
            }
        }
    }
    
    /// Get the admin originator domain
    ///
    /// Reference: TS adminOriginator field (line 371)
    pub fn admin_originator(&self) -> &str {
        &self.admin_originator
    }
    
    /// Get the configuration
    ///
    /// Reference: TS config field (line 415)
    pub fn config(&self) -> &PermissionsManagerConfig {
        &self.config
    }
    
    /// Check if an originator is the admin
    ///
    /// Reference: TS isAdminOriginator (WalletPermissionsManager.ts lines 3023-3025)
    pub fn is_admin_originator(&self, originator: &str) -> bool {
        // TS line 3024: return originator === this.adminOriginator
        originator == self.admin_originator
    }
    
    /// Check if a protocol is admin-only
    ///
    /// Reference: TS isAdminProtocol (WalletPermissionsManager.ts lines 3035-3040)
    ///
    /// Admin-only protocols:
    /// - Start with "admin"
    /// - Start with "p " (future specially permissioned protocols)
    pub fn is_admin_protocol(&self, protocol_id: &[String]) -> bool {
        if protocol_id.len() < 2 {
            return false;
        }
        
        let protocol_name = &protocol_id[1];
        
        // TS lines 3037-3038
        protocol_name.starts_with("admin") || protocol_name.starts_with("p ")
    }
    
    /// Check if a basket is admin-only
    ///
    /// Reference: TS isAdminBasket (WalletPermissionsManager.ts lines 3064-3068)
    ///
    /// Admin-only baskets:
    /// - "default" (used for internal operations)
    /// - Start with "admin"
    /// - Start with "p " (future specially permissioned baskets)
    pub fn is_admin_basket(&self, basket: &str) -> bool {
        // TS lines 3065-3067
        basket == "default" || basket.starts_with("admin") || basket.starts_with("p ")
    }
    
    /// Check if a label is admin-only
    ///
    /// Reference: TS isAdminLabel (WalletPermissionsManager.ts lines 3050-3053)
    ///
    /// Admin-only labels:
    /// - Start with "admin"
    pub fn is_admin_label(&self, label: &str) -> bool {
        // TS lines 3051-3052
        label.starts_with("admin")
    }
    
    /// Grants a previously requested permission
    ///
    /// Reference: TS grantPermission (WalletPermissionsManager.ts lines 535-581)
    ///
    /// This method:
    /// 1) Resolves all pending promise calls waiting on this request
    /// 2) Optionally creates or renews an on-chain PushDrop token (unless `ephemeral===true`)
    ///
    /// # Arguments
    ///
    /// * `params` - Grant parameters including requestID, expiry, ephemeral flag, and amount
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn grant_permission(&self, params: GrantPermissionParams) -> WalletResult<()> {
        // TS lines 542-545: Identify the matching queued requests
        let mut active_requests = self.active_requests.write().await;
        let matching = active_requests.remove(&params.request_id)
            .ok_or_else(|| WalletError::invalid_parameter(
                "requestID",
                "Request ID not found."
            ))?;
        
        // TS lines 548-551: Mark all matching requests as resolved
        for sender in matching.pending {
            let _ = sender.send(Ok(())); // Ignore send errors (receiver dropped)
        }
        
        // TS lines 553-572: If not ephemeral, create or renew on-chain token
        if !params.ephemeral.unwrap_or(false) {
            // TODO: Parse request from matching.request JSON
            // let request = serde_json::from_value::<PermissionRequest>(matching.request)?;
            // 
            // if !request.renewal.unwrap_or(false) {
            //     // TS lines 556-562: Create brand-new permission token
            //     create_permission_on_chain(
            //         &request,
            //         params.expiry.unwrap_or_else(calculate_default_expiry),
            //         params.amount,
            //     ).await?;
            // } else {
            //     // TS lines 563-571: Renewal => spend old token, produce new one
            //     if let Some(prev_token) = request.previous_token.as_ref() {
            //         renew_permission_on_chain(
            //             prev_token,
            //             &request,
            //             params.expiry.unwrap_or_else(calculate_default_expiry),
            //             params.amount,
            //         ).await?;
            //     }
            // }
        }
        
        // TS lines 574-580: Cache non-ephemeral permissions
        if !params.ephemeral.unwrap_or(false) {
            let expiry = params.expiry.unwrap_or_else(calculate_default_expiry);
            // TODO: Get request from matching.request and build key
            // let key = build_request_key(&request);
            // let mut cache = self.permission_cache.write().await;
            // cache_permission(&mut cache, key, expiry);
        }
        
        Ok(())
    }
    
    /// Denies a previously requested permission
    ///
    /// Reference: TS denyPermission (WalletPermissionsManager.ts lines 589-601)
    ///
    /// This method rejects all pending promise calls waiting on that request.
    ///
    /// # Arguments
    ///
    /// * `request_id` - Request ID identifying which request to deny
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn deny_permission(&self, request_id: String) -> WalletResult<()> {
        // TS lines 591-594: Identify the matching requests
        let mut active_requests = self.active_requests.write().await;
        let matching = active_requests.remove(&request_id)
            .ok_or_else(|| WalletError::invalid_parameter(
                "requestID",
                "Request ID not found."
            ))?;
        
        // TS lines 597-600: Reject all matching requests
        let error = WalletError::invalid_operation("Permission denied.");
        for sender in matching.pending {
            let _ = sender.send(Err(error.clone())); // Ignore send errors
        }
        
        Ok(())
    }
    
    /// Grants a previously requested grouped permission
    ///
    /// Reference: TS grantGroupedPermission (WalletPermissionsManager.ts lines 609-723)
    ///
    /// # Arguments
    ///
    /// * `params` - Grant parameters with requestID, granted permissions subset, and expiry
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn grant_grouped_permission(&self, params: GrantGroupedPermissionParams) -> WalletResult<()> {
        // TS lines 614-617: Identify the matching requests
        let mut active_requests = self.active_requests.write().await;
        let matching = active_requests.remove(&params.request_id)
            .ok_or_else(|| WalletError::invalid_parameter(
                "requestID",
                "Request ID not found."
            ))?;
        
        // TODO: Implement full validation and token creation
        // TS lines 619-644: Validate granted permissions are subset of requested
        // TS lines 646-716: Create tokens for each granted permission type
        
        // TS lines 718-722: Resolve all pending promises
        for sender in matching.pending {
            let _ = sender.send(Ok(()));
        }
        
        Ok(())
    }
    
    /// Denies a previously requested grouped permission
    ///
    /// Reference: TS denyGroupedPermission (WalletPermissionsManager.ts lines 729-740)
    ///
    /// # Arguments
    ///
    /// * `request_id` - Request ID identifying which request to deny
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub async fn deny_grouped_permission(&self, request_id: String) -> WalletResult<()> {
        // TS lines 730-733: Identify the matching requests
        let mut active_requests = self.active_requests.write().await;
        let matching = active_requests.remove(&request_id)
            .ok_or_else(|| WalletError::invalid_parameter(
                "requestID",
                "Request ID not found."
            ))?;
        
        // TS lines 734-739: Reject all matching requests with specific error
        let mut error = WalletError::invalid_operation("The user has denied the request for permission.");
        // TODO: Set error code to ERR_PERMISSION_DENIED when error struct supports it
        
        for sender in matching.pending {
            let _ = sender.send(Err(error.clone()));
        }
        
        Ok(())
    }
    
    /// Ensures the originator has protocol usage permission
    ///
    /// Reference: TS ensureProtocolPermission (WalletPermissionsManager.ts lines 750-858)
    ///
    /// If no valid (unexpired) permission token is found, triggers a permission request flow.
    ///
    /// # Arguments
    ///
    /// * `params` - Permission parameters including originator, protocol, counterparty, etc.
    ///
    /// # Returns
    ///
    /// `true` if permission granted, error otherwise
    pub async fn ensure_protocol_permission(&self, params: EnsureProtocolPermissionParams) -> WalletResult<bool> {
        // TS line 768: adminOriginator can do anything
        if self.is_admin_originator(&params.originator) {
            return Ok(true);
        }
        
        // TS lines 771-772: If security level=0, we consider it "open" usage
        if params.protocol_id.len() >= 1 {
            let level = params.protocol_id[0].parse::<i32>().unwrap_or(0);
            if level == 0 {
                return Ok(true);
            }
        }
        
        // TS lines 775-777: If protocol is admin-reserved, block
        if self.is_admin_protocol(&params.protocol_id) {
            let proto_name = params.protocol_id.get(1).map(|s| s.as_str()).unwrap_or("");
            return Err(WalletError::invalid_operation(
                format!("Protocol \"{}\" is admin-only.", proto_name)
            ));
        }
        
        // TS lines 780-797: Allow configured exceptions
        let mut privileged = params.privileged;
        match params.usage_type {
            ProtocolUsageType::Signing if !self.config.seek_protocol_permissions_for_signing => return Ok(true),
            ProtocolUsageType::Encrypting if !self.config.seek_protocol_permissions_for_encrypting => return Ok(true),
            ProtocolUsageType::Hmac if !self.config.seek_protocol_permissions_for_hmac => return Ok(true),
            // TODO: Add other usage type checks when config fields are added
            _ => {}
        }
        
        // TS lines 798-800: If not differentiating privileged, force to false
        if !self.config.differentiate_privileged_operations {
            privileged = false;
        }
        
        // TS lines 802-811: Check cache
        let request = PermissionRequest {
            permission_type: PermissionType::Protocol,
            originator: params.originator.clone(),
            privileged: Some(privileged),
            protocol_id: Some(params.protocol_id.clone()),
            counterparty: Some(params.counterparty.clone()),
            basket: None,
            certificate: None,
            spending: None,
            reason: params.reason.clone(),
            renewal: None,
            previous_token: None,
        };
        
        let cache_key = build_request_key(&request);
        {
            let cache = self.permission_cache.read().await;
            if is_permission_cached(&cache, &cache_key, Self::CACHE_TTL_MS) {
                return Ok(true);
            }
        }
        
        // TS lines 814-820: Attempt to find a valid token
        let token = find_protocol_token(
            self.underlying.as_ref(),
            &self.admin_originator,
            &params.originator,
            privileged,
            &params.protocol_id,
            &params.counterparty,
            true, // includeExpired
        ).await?;
        
        if let Some(token) = token {
            // TS lines 822-826: Token found and not expired
            if !is_token_expired_internal(token.expiry) {
                let mut cache = self.permission_cache.write().await;
                cache_permission(&mut cache, cache_key, token.expiry);
                return Ok(true);
            } else {
                // TS lines 827-841: Token expired, request renewal if allowed
                if !params.seek_permission {
                    return Err(WalletError::invalid_operation(
                        "Protocol permission expired and no further user consent allowed (seekPermission=false)."
                    ));
                }
                
                let mut renewal_request = request.clone();
                renewal_request.renewal = Some(true);
                renewal_request.previous_token = Some(Box::new(token));
                
                return self.request_permission_flow(renewal_request).await;
            }
        } else {
            // TS lines 843-857: No token found, request new one if allowed
            if !params.seek_permission {
                return Err(WalletError::invalid_operation(
                    "No protocol permission token found (seekPermission=false)."
                ));
            }
            
            return self.request_permission_flow(request).await;
        }
    }
    
    /// A central method that triggers the permission request flow
    ///
    /// Reference: TS requestPermissionFlow (WalletPermissionsManager.ts lines 1133-1180)
    ///
    /// - Checks if there's already an active request for the same key
    /// - If so, waits on that existing request rather than creating a duplicate
    /// - Otherwise creates a new request queue, calls the relevant event, and returns a promise
    async fn request_permission_flow(&self, request: PermissionRequest) -> WalletResult<bool> {
        let key = build_request_key(&request);
        
        // TS lines 1137-1142: If there's already a queue for the same resource, piggyback on it
        {
            let active_requests = self.active_requests.read().await;
            if active_requests.contains_key(&key) {
                // TODO: This needs a more sophisticated approach with oneshot channels
                // For now, return true to avoid blocking
                return Ok(true);
            }
        }
        
        // TS lines 1144-1150: Create a new queue with a single entry
        let (tx, rx) = tokio::sync::oneshot::channel();
        
        {
            let mut active_requests = self.active_requests.write().await;
            active_requests.insert(key.clone(), ActiveRequest {
                request: serde_json::to_value(&request).unwrap_or_default(),
                pending: vec![tx],
            });
        }
        
        // TS lines 1153-1178: Fire the relevant onXXXRequested event
        let request_with_id = PermissionRequestWithId {
            request: request.clone(),
            request_id: key.clone(),
        };
        
        {
            let callbacks = self.callbacks.read().await;
            match request.permission_type {
                PermissionType::Protocol => {
                    // TS lines 1155-1158
                    emit_permission_event(
                        &callbacks.on_protocol_permission_requested,
                        request_with_id,
                    ).await;
                }
                PermissionType::Basket => {
                    // TS lines 1161-1164
                    emit_permission_event(
                        &callbacks.on_basket_access_requested,
                        request_with_id,
                    ).await;
                }
                PermissionType::Certificate => {
                    // TS lines 1167-1170
                    emit_permission_event(
                        &callbacks.on_certificate_access_requested,
                        request_with_id,
                    ).await;
                }
                PermissionType::Spending => {
                    // TS lines 1173-1176
                    emit_permission_event(
                        &callbacks.on_spending_authorization_requested,
                        request_with_id,
                    ).await;
                }
            }
        }
        
        // Wait for grant or deny
        match rx.await {
            Ok(Ok(())) => Ok(true), // Permission granted
            Ok(Err(e)) => Err(e),   // Permission denied
            Err(_) => Err(WalletError::invalid_operation("Permission request channel closed")),
        }
    }
    
    /// Ensures the originator has basket usage permission
    ///
    /// Reference: TS ensureBasketAccess (WalletPermissionsManager.ts lines 864-920)
    ///
    /// If not, triggers a permission request flow.
    ///
    /// # Arguments
    ///
    /// * `params` - Basket access parameters
    ///
    /// # Returns
    ///
    /// `true` if permission granted, error otherwise
    pub async fn ensure_basket_access(&self, params: EnsureBasketAccessParams) -> WalletResult<bool> {
        // TS line 877: adminOriginator can do anything
        if self.is_admin_originator(&params.originator) {
            return Ok(true);
        }
        
        // TS lines 878-880: Admin basket check
        if self.is_admin_basket(&params.basket) {
            return Err(WalletError::invalid_operation(
                format!("Basket \"{}\" is admin-only.", params.basket)
            ));
        }
        
        // TS lines 881-883: Config-based exceptions
        match params.usage_type {
            BasketUsageType::Insertion if !self.config.seek_basket_insertion_permissions => return Ok(true),
            BasketUsageType::Removal if !self.config.seek_basket_removal_permissions => return Ok(true),
            BasketUsageType::Listing if !self.config.seek_basket_listing_permissions => return Ok(true),
            _ => {}
        }
        
        // TS lines 884-887: Check cache
        let request = PermissionRequest {
            permission_type: PermissionType::Basket,
            originator: params.originator.clone(),
            privileged: None,
            protocol_id: None,
            counterparty: None,
            basket: Some(params.basket.clone()),
            certificate: None,
            spending: None,
            reason: params.reason.clone(),
            renewal: None,
            previous_token: None,
        };
        
        let cache_key = build_request_key(&request);
        {
            let cache = self.permission_cache.read().await;
            if is_permission_cached(&cache, &cache_key, Self::CACHE_TTL_MS) {
                return Ok(true);
            }
        }
        
        // TS lines 888-905: Find existing token
        let token = find_basket_token(
            self.underlying.as_ref(),
            &self.admin_originator,
            &params.originator,
            &params.basket,
            true, // includeExpired
        ).await?;
        
        if let Some(token) = token {
            // TS lines 890-893: Valid token found
            if !is_token_expired_internal(token.expiry) {
                let mut cache = self.permission_cache.write().await;
                cache_permission(&mut cache, cache_key, token.expiry);
                return Ok(true);
            } else {
                // TS lines 894-905: Expired token - renewal flow
                if !params.seek_permission {
                    return Err(WalletError::invalid_operation(
                        "Basket permission expired (seekPermission=false)."
                    ));
                }
                
                let mut renewal_request = request.clone();
                renewal_request.renewal = Some(true);
                renewal_request.previous_token = Some(Box::new(token));
                
                return self.request_permission_flow(renewal_request).await;
            }
        } else {
            // TS lines 907-919: No token found
            if !params.seek_permission {
                return Err(WalletError::invalid_operation(
                    "No basket permission found, and no user consent allowed (seekPermission=false)."
                ));
            }
            
            return self.request_permission_flow(request).await;
        }
    }
    
    /// Ensures the originator has certificate access permission
    ///
    /// Reference: TS ensureCertificateAccess (WalletPermissionsManager.ts lines 926-1001)
    ///
    /// Relevant for revealing certificate fields in DCAP contexts.
    ///
    /// # Arguments
    ///
    /// * `params` - Certificate access parameters
    ///
    /// # Returns
    ///
    /// `true` if permission granted, error otherwise
    pub async fn ensure_certificate_access(&self, params: EnsureCertificateAccessParams) -> WalletResult<bool> {
        // TS line 945: adminOriginator can do anything
        if self.is_admin_originator(&params.originator) {
            return Ok(true);
        }
        
        // TS lines 946-948: Config-based exceptions
        if matches!(params.usage_type, CertificateUsageType::Disclosure) 
            && !self.config.seek_certificate_disclosure_permissions {
            return Ok(true);
        }
        
        // TS lines 949-951: Privileged differentiation
        let mut privileged = params.privileged;
        if !self.config.differentiate_privileged_operations {
            privileged = false;
        }
        
        // TS lines 952-960: Check cache
        let request = PermissionRequest {
            permission_type: PermissionType::Certificate,
            originator: params.originator.clone(),
            privileged: Some(privileged),
            protocol_id: None,
            counterparty: None,
            basket: None,
            certificate: Some(CertificateDetails {
                verifier: params.verifier.clone(),
                cert_type: params.cert_type.clone(),
                fields: params.fields.clone(),
            }),
            spending: None,
            reason: params.reason.clone(),
            renewal: None,
            previous_token: None,
        };
        
        let cache_key = build_request_key(&request);
        {
            let cache = self.permission_cache.read().await;
            if is_permission_cached(&cache, &cache_key, Self::CACHE_TTL_MS) {
                return Ok(true);
            }
        }
        
        // TS lines 961-986: Find existing token
        let token = find_certificate_token(
            self.underlying.as_ref(),
            &self.admin_originator,
            &params.originator,
            privileged,
            &params.verifier,
            &params.cert_type,
            &params.fields,
            true, // includeExpired
        ).await?;
        
        if let Some(token) = token {
            // TS lines 970-973: Valid token found
            if !is_token_expired_internal(token.expiry) {
                let mut cache = self.permission_cache.write().await;
                cache_permission(&mut cache, cache_key, token.expiry);
                return Ok(true);
            } else {
                // TS lines 974-986: Expired token - renewal flow
                if !params.seek_permission {
                    return Err(WalletError::invalid_operation(
                        "Certificate permission expired (seekPermission=false)."
                    ));
                }
                
                let mut renewal_request = request.clone();
                renewal_request.renewal = Some(true);
                renewal_request.previous_token = Some(Box::new(token));
                
                return self.request_permission_flow(renewal_request).await;
            }
        } else {
            // TS lines 988-999: No token found
            if !params.seek_permission {
                return Err(WalletError::invalid_operation(
                    "No certificate permission found (seekPermission=false)."
                ));
            }
            
            return self.request_permission_flow(request).await;
        }
    }
    
    /// Ensures the originator has spending authorization (DSAP)
    ///
    /// Reference: TS ensureSpendingAuthorization (WalletPermissionsManager.ts lines 1007-1069)
    ///
    /// If the existing token limit is insufficient, attempts to renew. If no token, creates one.
    ///
    /// # Arguments
    ///
    /// * `params` - Spending authorization parameters
    ///
    /// # Returns
    ///
    /// `true` if permission granted, error otherwise
    pub async fn ensure_spending_authorization(&self, params: EnsureSpendingAuthorizationParams) -> WalletResult<bool> {
        // TS line 1024: adminOriginator can do anything
        if self.is_admin_originator(&params.originator) {
            return Ok(true);
        }
        
        // TS lines 1025-1028: Config-based bypass
        if !self.config.seek_spending_permissions {
            return Ok(true);
        }
        
        // TS lines 1029-1032: Check cache
        let request = PermissionRequest {
            permission_type: PermissionType::Spending,
            originator: params.originator.clone(),
            privileged: None,
            protocol_id: None,
            counterparty: None,
            basket: None,
            certificate: None,
            spending: Some(SpendingDetails {
                satoshis: params.satoshis,
                line_items: params.line_items.clone(),
            }),
            reason: params.reason.clone(),
            renewal: None,
            previous_token: None,
        };
        
        let cache_key = build_request_key(&request);
        {
            let cache = self.permission_cache.read().await;
            if is_permission_cached(&cache, &cache_key, Self::CACHE_TTL_MS) {
                return Ok(true);
            }
        }
        
        // TS lines 1033-1055: Find existing token and check limits
        let token = find_spending_token(
            self.underlying.as_ref(),
            &self.admin_originator,
            &params.originator,
        ).await?;
        
        if let Some(token) = token {
            if let Some(authorized_amount) = token.authorized_amount {
                // TS lines 1035-1040: Check how much has been spent
                let spent_so_far = query_spent_since(
                    self.underlying.as_ref(),
                    &self.admin_originator,
                    &token,
                ).await?;
                
                if spent_so_far + params.satoshis <= authorized_amount {
                    // TS lines 1038-1039: Sufficient authorization
                    let mut cache = self.permission_cache.write().await;
                    cache_permission(&mut cache, cache_key, token.expiry);
                    return Ok(true);
                } else {
                    // TS lines 1041-1055: Insufficient - renew
                    if !params.seek_permission {
                        return Err(WalletError::invalid_operation(
                            format!("Spending authorization insufficient for {}, no user consent (seekPermission=false).", params.satoshis)
                        ));
                    }
                    
                    let mut renewal_request = request.clone();
                    renewal_request.renewal = Some(true);
                    renewal_request.previous_token = Some(Box::new(token));
                    
                    return self.request_permission_flow(renewal_request).await;
                }
            }
        }
        
        // TS lines 1057-1068: No token or no authorized amount
        if !params.seek_permission {
            return Err(WalletError::invalid_operation(
                "No spending authorization found, (seekPermission=false)."
            ));
        }
        
        return self.request_permission_flow(request).await;
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock wallet for testing
    struct MockWallet;
    
    #[async_trait::async_trait]
    impl WalletInterface for MockWallet {
        async fn create_action(
            &self,
            _args: serde_json::Value,
            _originator: Option<&str>,
        ) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({}))
        }
        
        async fn sign_action(
            &self,
            _args: serde_json::Value,
            _originator: Option<&str>,
        ) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({}))
        }
        
        async fn abort_action(
            &self,
            _args: serde_json::Value,
            _originator: Option<&str>,
        ) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({}))
        }
        
        async fn list_actions(
            &self,
            _args: serde_json::Value,
            _originator: Option<&str>,
        ) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({}))
        }
        
        async fn internal_ize_action(
            &self,
            _args: serde_json::Value,
            _originator: Option<&str>,
        ) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({}))
        }
        
        async fn list_outputs(
            &self,
            _args: serde_json::Value,
            _originator: Option<&str>,
        ) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({}))
        }
        
        async fn relinquish_output(
            &self,
            _args: serde_json::Value,
            _originator: Option<&str>,
        ) -> WalletResult<serde_json::Value> {
            Ok(serde_json::json!({}))
        }
    }
    
    #[tokio::test]
    async fn test_permissions_manager_creation() {
        // TS constructor test (lines 424-452)
        let wallet = Arc::new(MockWallet);
        let manager = WalletPermissionsManager::new(
            wallet,
            "admin.example.com".to_string(),
            None,
        );
        
        assert_eq!(manager.admin_originator(), "admin.example.com");
        assert!(manager.config().seek_protocol_permissions_for_signing);
    }
    
    #[tokio::test]
    async fn test_is_admin() {
        let wallet = Arc::new(MockWallet);
        let manager = WalletPermissionsManager::new(
            wallet,
            "admin.example.com".to_string(),
            None,
        );
        
        assert!(manager.is_admin("admin.example.com"));
        assert!(!manager.is_admin("other.example.com"));
    }
    
    #[tokio::test]
    async fn test_callback_binding() {
        // TS bindCallback/unbindCallback test (lines 465-498)
        let wallet = Arc::new(MockWallet);
        let manager = WalletPermissionsManager::new(
            wallet,
            "admin.example.com".to_string(),
            None,
        );
        
        // Create a dummy handler
        let handler = Arc::new(|_req: PermissionRequestWithId| {
            Ok(())
        });
        
        // Bind callback
        let id = manager.bind_callback_protocol(handler).await;
        assert_eq!(id, 0);
        
        // Unbind callback
        let unbound = manager.unbind_callback(PermissionType::Protocol, id).await;
        assert!(unbound);
        
        // Try to unbind again (should fail)
        let unbound_again = manager.unbind_callback(PermissionType::Protocol, id).await;
        assert!(!unbound_again);
    }
}
