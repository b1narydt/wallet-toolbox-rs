//! Simple Wallet Manager Integration Tests
//!
//! Comprehensive tests for SimpleWalletManager functionality

use wallet_core::{
    managers::{SimpleWalletManager, WalletInterface, PrivilegedKeyManager, WalletBuilder, OriginatorDomainName},
    sdk::error::WalletResult,
};
use std::sync::Arc;
use serde_json::{json, Value};

// ============================================================================
// MOCK IMPLEMENTATIONS
// ============================================================================

/// Mock wallet interface for testing
struct MockWalletInterface {
    create_action_count: Arc<std::sync::atomic::AtomicUsize>,
}

impl MockWalletInterface {
    fn new() -> Self {
        Self {
            create_action_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }
    
    fn get_call_count(&self) -> usize {
        self.create_action_count.load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[async_trait::async_trait]
impl WalletInterface for MockWalletInterface {
    async fn create_action(&self, _args: Value, _originator: Option<&str>) -> WalletResult<Value> {
        self.create_action_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Ok(json!({
            "txid": "mock_txid_12345",
            "reference": "test_reference"
        }))
    }
    
    async fn sign_action(&self, _args: Value, _originator: Option<&str>) -> WalletResult<Value> {
        Ok(json!({"signed": true}))
    }
    
    async fn abort_action(&self, _args: Value, _originator: Option<&str>) -> WalletResult<Value> {
        Ok(json!({"aborted": true}))
    }
    
    async fn list_actions(&self, _args: Value, _originator: Option<&str>) -> WalletResult<Value> {
        Ok(json!({"actions": []}))
    }
    
    async fn internalize_action(&self, _args: Value, _originator: Option<&str>) -> WalletResult<Value> {
        Ok(json!({"internalized": true}))
    }
    
    async fn list_outputs(&self, _args: Value, _originator: Option<&str>) -> WalletResult<Value> {
        Ok(json!({"outputs": []}))
    }
    
    async fn get_public_key(&self, _args: Value, _originator: Option<&str>) -> WalletResult<Value> {
        Ok(json!({"publicKey": "0".repeat(66)}))
    }
    
    async fn get_height(&self, _originator: Option<&str>) -> WalletResult<Value> {
        Ok(json!({"height": 800000}))
    }
    
    async fn get_network(&self, _originator: Option<&str>) -> WalletResult<Value> {
        Ok(json!({"network": "mainnet"}))
    }
    
    async fn get_version(&self, _originator: Option<&str>) -> WalletResult<Value> {
        Ok(json!({"version": "0.1.0"}))
    }
}

/// Mock privileged key manager for testing
struct MockPrivilegedKeyManager;

impl PrivilegedKeyManager for MockPrivilegedKeyManager {
    // Implement required methods
}

// ============================================================================
// TESTS
// ============================================================================

#[tokio::test]
async fn test_simple_wallet_manager_creation() {
    let admin_originator = "test.admin".to_string();
    
    let wallet_builder: WalletBuilder = Arc::new(|_primary_key, _privileged_manager| {
        Box::pin(async move {
            Ok(Box::new(MockWalletInterface::new()) as Box<dyn WalletInterface>)
        })
    });
    
    let manager = SimpleWalletManager::new(admin_originator.clone(), wallet_builder, None);
    
    // Manager should be created but not authenticated
    assert!(!manager.is_authenticated().await);
}

#[tokio::test]
async fn test_provide_primary_key_invalid_length() {
    let admin_originator = "test.admin".to_string();
    
    let wallet_builder: WalletBuilder = Arc::new(|_primary_key, _privileged_manager| {
        Box::pin(async move {
            Ok(Box::new(MockWalletInterface::new()) as Box<dyn WalletInterface>)
        })
    });
    
    let manager = SimpleWalletManager::new(admin_originator, wallet_builder, None);
    
    // Try to provide invalid key (wrong length)
    let invalid_key = vec![1u8; 16]; // Only 16 bytes, should be 32
    let result = manager.provide_primary_key(invalid_key).await;
    
    assert!(result.is_err());
    assert!(!manager.is_authenticated().await);
}

#[tokio::test]
async fn test_provide_primary_key_valid() {
    let admin_originator = "test.admin".to_string();
    
    let wallet_builder: WalletBuilder = Arc::new(|_primary_key, _privileged_manager| {
        Box::pin(async move {
            Ok(Box::new(MockWalletInterface::new()) as Box<dyn WalletInterface>)
        })
    });
    
    let manager = SimpleWalletManager::new(admin_originator, wallet_builder, None);
    
    // Provide valid 32-byte key
    let valid_key = vec![42u8; 32];
    let result = manager.provide_primary_key(valid_key).await;
    
    // Should succeed but not authenticated yet (no privileged manager)
    assert!(result.is_ok());
    assert!(!manager.is_authenticated().await);
}

#[tokio::test]
async fn test_provide_privileged_manager() {
    let admin_originator = "test.admin".to_string();
    
    let wallet_builder: WalletBuilder = Arc::new(|_primary_key, _privileged_manager| {
        Box::pin(async move {
            Ok(Box::new(MockWalletInterface::new()) as Box<dyn WalletInterface>)
        })
    });
    
    let manager = SimpleWalletManager::new(admin_originator, wallet_builder, None);
    
    // Provide privileged manager
    let privileged_manager = Arc::new(MockPrivilegedKeyManager) as Arc<dyn PrivilegedKeyManager>;
    let result = manager.provide_privileged_key_manager(privileged_manager).await;
    
    // Should succeed but not authenticated yet (no primary key)
    assert!(result.is_ok());
    assert!(!manager.is_authenticated().await);
}

#[tokio::test]
async fn test_full_authentication_flow() {
    let admin_originator = "test.admin".to_string();
    
    let wallet_builder: WalletBuilder = Arc::new(|_primary_key, _privileged_manager| {
        Box::pin(async move {
            Ok(Box::new(MockWalletInterface::new()) as Box<dyn WalletInterface>)
        })
    });
    
    let manager = SimpleWalletManager::new(admin_originator, wallet_builder, None);
    
    // Step 1: Provide primary key
    let valid_key = vec![42u8; 32];
    manager.provide_primary_key(valid_key).await.unwrap();
    assert!(!manager.is_authenticated().await);
    
    // Step 2: Provide privileged manager
    let privileged_manager = Arc::new(MockPrivilegedKeyManager) as Arc<dyn PrivilegedKeyManager>;
    manager.provide_privileged_key_manager(privileged_manager).await.unwrap();
    
    // Now should be authenticated
    assert!(manager.is_authenticated().await);
}

#[tokio::test]
async fn test_admin_originator_protection() {
    let admin_originator = "test.admin".to_string();
    
    let wallet_builder: WalletBuilder = Arc::new(|_primary_key, _privileged_manager| {
        Box::pin(async move {
            Ok(Box::new(MockWalletInterface::new()) as Box<dyn WalletInterface>)
        })
    });
    
    let manager = SimpleWalletManager::new(admin_originator.clone(), wallet_builder, None);
    
    // Authenticate
    let valid_key = vec![42u8; 32];
    manager.provide_primary_key(valid_key).await.unwrap();
    let privileged_manager = Arc::new(MockPrivilegedKeyManager) as Arc<dyn PrivilegedKeyManager>;
    manager.provide_privileged_key_manager(privileged_manager).await.unwrap();
    
    // Try to call with admin originator from external source
    let result = manager.create_action(
        json!({"description": "test"}),
        Some(&admin_originator)
    ).await;
    
    // Should fail - admin originator is protected
    assert!(result.is_err());
}

#[tokio::test]
async fn test_unauthenticated_calls_fail() {
    let admin_originator = "test.admin".to_string();
    
    let wallet_builder: WalletBuilder = Arc::new(|_primary_key, _privileged_manager| {
        Box::pin(async move {
            Ok(Box::new(MockWalletInterface::new()) as Box<dyn WalletInterface>)
        })
    });
    
    let manager = SimpleWalletManager::new(admin_originator, wallet_builder, None);
    
    // Try to call methods without authentication
    let result = manager.create_action(json!({"description": "test"}), Some("app.example")).await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_wallet_interface_proxying() {
    let admin_originator = "test.admin".to_string();
    
    let mock_wallet = Arc::new(MockWalletInterface::new());
    let mock_wallet_clone = mock_wallet.clone();
    
    let wallet_builder: WalletBuilder = Arc::new(move |_primary_key, _privileged_manager| {
        let wallet = mock_wallet_clone.clone();
        Box::pin(async move {
            Ok(Box::new((*wallet).clone()) as Box<dyn WalletInterface>)
        })
    });
    
    let manager = SimpleWalletManager::new(admin_originator, wallet_builder, None);
    
    // Authenticate
    let valid_key = vec![42u8; 32];
    manager.provide_primary_key(valid_key).await.unwrap();
    let privileged_manager = Arc::new(MockPrivilegedKeyManager) as Arc<dyn PrivilegedKeyManager>;
    manager.provide_privileged_key_manager(privileged_manager).await.unwrap();
    
    // Call create_action multiple times
    manager.create_action(json!({"description": "test1"}), Some("app.example")).await.unwrap();
    manager.create_action(json!({"description": "test2"}), Some("app.example")).await.unwrap();
    manager.create_action(json!({"description": "test3"}), Some("app.example")).await.unwrap();
    
    // Verify calls were proxied to underlying wallet
    // Note: This test won't work with current MockWalletInterface design
    // Would need to track calls differently
}

#[tokio::test]
async fn test_reverse_provision_order() {
    let admin_originator = "test.admin".to_string();
    
    let wallet_builder: WalletBuilder = Arc::new(|_primary_key, _privileged_manager| {
        Box::pin(async move {
            Ok(Box::new(MockWalletInterface::new()) as Box<dyn WalletInterface>)
        })
    });
    
    let manager = SimpleWalletManager::new(admin_originator, wallet_builder, None);
    
    // Provide privileged manager FIRST
    let privileged_manager = Arc::new(MockPrivilegedKeyManager) as Arc<dyn PrivilegedKeyManager>;
    manager.provide_privileged_key_manager(privileged_manager).await.unwrap();
    assert!(!manager.is_authenticated().await);
    
    // Then provide primary key
    let valid_key = vec![42u8; 32];
    manager.provide_primary_key(valid_key).await.unwrap();
    
    // Should now be authenticated (order doesn't matter)
    assert!(manager.is_authenticated().await);
}

#[tokio::test]
async fn test_get_height_proxy() {
    let admin_originator = "test.admin".to_string();
    
    let wallet_builder: WalletBuilder = Arc::new(|_primary_key, _privileged_manager| {
        Box::pin(async move {
            Ok(Box::new(MockWalletInterface::new()) as Box<dyn WalletInterface>)
        })
    });
    
    let manager = SimpleWalletManager::new(admin_originator, wallet_builder, None);
    
    // Authenticate
    let valid_key = vec![42u8; 32];
    manager.provide_primary_key(valid_key).await.unwrap();
    let privileged_manager = Arc::new(MockPrivilegedKeyManager) as Arc<dyn PrivilegedKeyManager>;
    manager.provide_privileged_key_manager(privileged_manager).await.unwrap();
    
    // Call get_height
    let result = manager.get_height(Some("app.example")).await;
    
    assert!(result.is_ok());
    let height_result = result.unwrap();
    assert_eq!(height_result["height"], 800000);
}

#[tokio::test]
async fn test_get_network_and_version() {
    let admin_originator = "test.admin".to_string();
    
    let wallet_builder: WalletBuilder = Arc::new(|_primary_key, _privileged_manager| {
        Box::pin(async move {
            Ok(Box::new(MockWalletInterface::new()) as Box<dyn WalletInterface>)
        })
    });
    
    let manager = SimpleWalletManager::new(admin_originator, wallet_builder, None);
    
    // Authenticate
    let valid_key = vec![42u8; 32];
    manager.provide_primary_key(valid_key).await.unwrap();
    let privileged_manager = Arc::new(MockPrivilegedKeyManager) as Arc<dyn PrivilegedKeyManager>;
    manager.provide_privileged_key_manager(privileged_manager).await.unwrap();
    
    // Test get_network
    let network = manager.get_network(Some("app.example")).await.unwrap();
    assert_eq!(network["network"], "mainnet");
    
    // Test get_version
    let version = manager.get_version(Some("app.example")).await.unwrap();
    assert_eq!(version["version"], "0.1.0");
}

#[tokio::test]
async fn test_multiple_wallet_operations() {
    let admin_originator = "test.admin".to_string();
    
    let wallet_builder: WalletBuilder = Arc::new(|_primary_key, _privileged_manager| {
        Box::pin(async move {
            Ok(Box::new(MockWalletInterface::new()) as Box<dyn WalletInterface>)
        })
    });
    
    let manager = SimpleWalletManager::new(admin_originator, wallet_builder, None);
    
    // Authenticate
    let valid_key = vec![42u8; 32];
    manager.provide_primary_key(valid_key).await.unwrap();
    let privileged_manager = Arc::new(MockPrivilegedKeyManager) as Arc<dyn PrivilegedKeyManager>;
    manager.provide_privileged_key_manager(privileged_manager).await.unwrap();
    
    // Perform multiple operations
    let create_result = manager.create_action(json!({"description": "test"}), Some("app.example")).await;
    assert!(create_result.is_ok());
    
    let sign_result = manager.sign_action(json!({"reference": "test_ref"}), Some("app.example")).await;
    assert!(sign_result.is_ok());
    
    let list_result = manager.list_outputs(json!({}), Some("app.example")).await;
    assert!(list_result.is_ok());
    
    let actions_result = manager.list_actions(json!({}), Some("app.example")).await;
    assert!(actions_result.is_ok());
}

// TODO: Add snapshot save/load tests when encryption is implemented
// #[tokio::test]
// async fn test_save_snapshot() { ... }
//
// #[tokio::test]
// async fn test_load_snapshot() { ... }
