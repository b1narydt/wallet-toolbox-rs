//! Permission Manager Callbacks
//!
//! **Reference**: TypeScript `src/WalletPermissionsManager.ts` lines 501-520
//!
//! Event emission and callback handling for permission requests

use super::types::*;
use crate::sdk::errors::WalletResult;

/// Emit a permission request event to all registered callbacks
///
/// Reference: TS callEvent (WalletPermissionsManager.ts lines 509-520)
///
/// Internally triggers a named event, calling all subscribed listeners.
/// Each callback is awaited in turn (though errors are swallowed so that
/// one failing callback doesn't prevent the others).
///
/// # Arguments
///
/// * `callbacks` - The callback array for this event type
/// * `param` - The parameter object passed to all listeners
pub async fn emit_permission_event(
    callbacks: &[PermissionEventHandler],
    param: PermissionRequestWithId,
) {
    // TS lines 510-519: Iterate through callbacks
    for cb in callbacks {
        // TS lines 512-517: Try to call each callback, swallowing errors
        if let Err(_e) = cb(param.clone()) {
            // Intentionally swallow errors from user-provided callbacks (TS line 516)
            // This ensures one failing callback doesn't prevent others from running
        }
    }
}

/// Emit a grouped permission request event to all registered callbacks
///
/// Reference: TS callEvent (WalletPermissionsManager.ts lines 509-520)
///
/// # Arguments
///
/// * `callbacks` - The callback array for this event type
/// * `param` - The grouped permission request
pub async fn emit_grouped_permission_event(
    callbacks: &[GroupedPermissionEventHandler],
    param: GroupedPermissionRequest,
) {
    // TS lines 510-519: Iterate through callbacks
    for cb in callbacks {
        // TS lines 512-517: Try to call each callback, swallowing errors
        if let Err(_e) = cb(param.clone()) {
            // Intentionally swallow errors from user-provided callbacks (TS line 516)
        }
    }
}

/// Build a request key for caching and deduplication
///
/// Reference: TS buildRequestKey usage throughout the file
///
/// Creates a unique string key from a permission request for use in:
/// - Permission caching
/// - Active request deduplication
/// - Token lookup
///
/// # Arguments
///
/// * `request` - The permission request
///
/// # Returns
///
/// A unique key string for this request
pub fn build_request_key(request: &PermissionRequest) -> String {
    // Build key based on permission type and relevant fields
    match request.permission_type {
        PermissionType::Protocol => {
            // For protocol permissions: type:originator:privileged:protocolID:counterparty
            let privileged = request.privileged.unwrap_or(false);
            let protocol_id = request.protocol_id.as_ref()
                .map(|p| p.join(":"))
                .unwrap_or_else(|| "".to_string());
            let counterparty = request.counterparty.as_deref().unwrap_or("");
            
            format!(
                "protocol:{}:{}:{}:{}",
                request.originator,
                privileged,
                protocol_id,
                counterparty
            )
        }
        PermissionType::Basket => {
            // For basket permissions: type:originator:basketName
            let basket = request.basket.as_deref().unwrap_or("");
            format!("basket:{}:{}", request.originator, basket)
        }
        PermissionType::Certificate => {
            // For certificate permissions: type:originator:privileged:verifier:certType
            let privileged = request.privileged.unwrap_or(false);
            let cert_details = request.certificate.as_ref();
            let verifier = cert_details.map(|c| c.verifier.as_str()).unwrap_or("");
            let cert_type = cert_details.map(|c| c.cert_type.as_str()).unwrap_or("");
            
            format!(
                "certificate:{}:{}:{}:{}",
                request.originator,
                privileged,
                verifier,
                cert_type
            )
        }
        PermissionType::Spending => {
            // For spending permissions: type:originator
            format!("spending:{}", request.originator)
        }
    }
}

/// Cached permission entry  
///
/// Reference: TS permissionCache (WalletPermissionsManager.ts line 407)
#[derive(Debug, Clone)]
pub struct CachedPermission {
    /// When this permission expires
    pub expiry: i64,
    
    /// When this permission was cached
    pub cached_at: i64,
}

/// Check if a permission is currently cached
///
/// Reference: TS isPermissionCached usage (around line 809)
///
/// # Arguments
///
/// * `cache` - The permission cache
/// * `key` - The request key
/// * `cache_ttl_ms` - Cache time-to-live in milliseconds
///
/// # Returns
///
/// `true` if cached and not expired, `false` otherwise
pub fn is_permission_cached(
    cache: &std::collections::HashMap<String, CachedPermission>,
    key: &str,
    cache_ttl_ms: i64,
) -> bool {
    if let Some(cached) = cache.get(key) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        
        // Check if cache entry is still valid (not expired by TTL)
        if now - cached.cached_at < cache_ttl_ms {
            // Check if permission itself hasn't expired
            if !super::utils::is_token_expired(cached.expiry) {
                return true;
            }
        }
    }
    false
}

/// Add a permission to the cache
///
/// Reference: TS cachePermission (around line 824)
///
/// # Arguments
///
/// * `cache` - The permission cache
/// * `key` - The request key
/// * `expiry` - When this permission expires
pub fn cache_permission(
    cache: &mut std::collections::HashMap<String, CachedPermission>,
    key: String,
    expiry: i64,
) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    
    cache.insert(key, CachedPermission {
        expiry,
        cached_at: now,
    });
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    
    #[tokio::test]
    async fn test_emit_permission_event() {
        // Create a callback that sets a flag
        let called = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let called_clone = called.clone();
        
        let handler: PermissionEventHandler = Arc::new(move |_req| {
            called_clone.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(())
        });
        
        let request = PermissionRequestWithId {
            request: PermissionRequest {
                permission_type: PermissionType::Protocol,
                originator: "test.com".to_string(),
                privileged: None,
                protocol_id: Some(vec!["2".to_string(), "test".to_string()]),
                counterparty: None,
                basket: None,
                certificate: None,
                spending: None,
                reason: None,
                renewal: None,
                previous_token: None,
            },
            request_id: "test-123".to_string(),
        };
        
        emit_permission_event(&[handler], request).await;
        
        assert!(called.load(std::sync::atomic::Ordering::SeqCst));
    }
    
    #[test]
    fn test_build_request_key_protocol() {
        let request = PermissionRequest {
            permission_type: PermissionType::Protocol,
            originator: "example.com".to_string(),
            privileged: Some(false),
            protocol_id: Some(vec!["2".to_string(), "myProtocol".to_string()]),
            counterparty: Some("self".to_string()),
            basket: None,
            certificate: None,
            spending: None,
            reason: None,
            renewal: None,
            previous_token: None,
        };
        
        let key = build_request_key(&request);
        assert_eq!(key, "protocol:example.com:false:2:myProtocol:self");
    }
    
    #[test]
    fn test_build_request_key_basket() {
        let request = PermissionRequest {
            permission_type: PermissionType::Basket,
            originator: "example.com".to_string(),
            privileged: None,
            protocol_id: None,
            counterparty: None,
            basket: Some("myBasket".to_string()),
            certificate: None,
            spending: None,
            reason: None,
            renewal: None,
            previous_token: None,
        };
        
        let key = build_request_key(&request);
        assert_eq!(key, "basket:example.com:myBasket");
    }
    
    #[test]
    fn test_build_request_key_spending() {
        let request = PermissionRequest {
            permission_type: PermissionType::Spending,
            originator: "example.com".to_string(),
            privileged: None,
            protocol_id: None,
            counterparty: None,
            basket: None,
            certificate: None,
            spending: Some(SpendingDetails {
                satoshis: 1000,
                line_items: None,
            }),
            reason: None,
            renewal: None,
            previous_token: None,
        };
        
        let key = build_request_key(&request);
        assert_eq!(key, "spending:example.com");
    }
    
    #[test]
    fn test_is_permission_cached() {
        use std::collections::HashMap;
        
        let mut cache = HashMap::new();
        let key = "test-key".to_string();
        
        // Not cached initially
        assert!(!is_permission_cached(&cache, &key, 5 * 60 * 1000));
        
        // Add to cache with far future expiry
        let future_expiry = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64) + 86400; // 1 day from now
        
        cache_permission(&mut cache, key.clone(), future_expiry);
        
        // Should be cached now
        assert!(is_permission_cached(&cache, &key, 5 * 60 * 1000));
    }
}
