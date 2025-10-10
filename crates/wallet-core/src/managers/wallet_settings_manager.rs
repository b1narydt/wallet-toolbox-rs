//! Wallet Settings Manager
//!
//! **Reference**: TypeScript `src/WalletSettingsManager.ts` (113 lines)
//!
//! Manages wallet settings including trust settings, theme, and currency preferences

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::sdk::errors::{WalletError, WalletResult};

/// Public key in hex format
///
/// Reference: TS PubKeyHex from @bsv/sdk
pub type PubKeyHex = String;

/// Settings basket name
///
/// Reference: TS SETTINGS_BASKET (WalletSettingsManager.ts line 27)
pub const SETTINGS_BASKET: &str = "wallet settings";

/// Certifier information
///
/// Reference: TS Certifier interface (WalletSettingsManager.ts lines 3-10)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Certifier {
    /// Certifier name
    pub name: String,
    
    /// Description of certifier
    pub description: String,
    
    /// Identity key (public key hex)
    pub identity_key: PubKeyHex,
    
    /// Trust level
    pub trust: u32,
    
    /// Icon URL (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    
    /// Base URL (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
}

/// Trust settings
///
/// Reference: TS TrustSettings interface (WalletSettingsManager.ts lines 11-14)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrustSettings {
    /// General trust level
    pub trust_level: u32,
    
    /// List of trusted certifiers
    pub trusted_certifiers: Vec<Certifier>,
}

/// Wallet theme settings
///
/// Reference: TS WalletTheme interface (WalletSettingsManager.ts lines 15-17)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalletTheme {
    /// Theme mode (e.g., "dark", "light")
    pub mode: String,
}

/// Complete wallet settings
///
/// Reference: TS WalletSettings interface (WalletSettingsManager.ts lines 18-22)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalletSettings {
    /// Trust settings
    pub trust_settings: TrustSettings,
    
    /// Theme settings (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<WalletTheme>,
    
    /// Preferred currency (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Configuration for wallet settings manager
///
/// Reference: TS WalletSettingsManagerConfig interface (WalletSettingsManager.ts lines 23-25)
#[derive(Debug, Clone)]
pub struct WalletSettingsManagerConfig {
    /// Default settings to use
    pub default_settings: WalletSettings,
}

impl Default for WalletSettingsManagerConfig {
    fn default() -> Self {
        Self {
            default_settings: default_settings(),
        }
    }
}

/// Get default wallet settings
///
/// Reference: TS DEFAULT_SETTINGS (WalletSettingsManager.ts lines 30-51)
pub fn default_settings() -> WalletSettings {
    WalletSettings {
        trust_settings: TrustSettings {
            trust_level: 2,
            trusted_certifiers: vec![
                Certifier {
                    name: "Metanet Trust Services".to_string(),
                    description: "Registry for protocols, baskets, and certificates types".to_string(),
                    icon_url: Some("https://bsvblockchain.org/favicon.ico".to_string()),
                    identity_key: "03daf815fe38f83da0ad83b5bedc520aa488aef5cbc93a93c67a7fe60406cbffe8".to_string(),
                    trust: 4,
                    base_url: None,
                },
                Certifier {
                    name: "SocialCert".to_string(),
                    description: "Certifies social media handles, phone numbers and emails".to_string(),
                    icon_url: Some("https://socialcert.net/favicon.ico".to_string()),
                    trust: 3,
                    identity_key: "02cf6cdf466951d8dfc9e7c9367511d0007ed6fba35ed42d425cc412fd6cfd4a17".to_string(),
                    base_url: None,
                },
            ],
        },
        theme: Some(WalletTheme {
            mode: "dark".to_string(),
        }),
        currency: None,
    }
}

/// Testnet identity key mapping
///
/// Reference: TS TESTNET_IDENTITY_KEYS (WalletSettingsManager.ts lines 54-58)
fn testnet_identity_keys() -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert(
        "Babbage Trust Services".to_string(),
        "03d0b36b5c98b000ec9ffed9a2cf005e279244edf6a19cf90545cdebe873162761".to_string(),
    );
    keys.insert(
        "IdentiCert".to_string(),
        "036dc48522aba1705afbb43df3c04dbd1da373b6154341a875bceaa2a3e7f21528".to_string(),
    );
    keys.insert(
        "SocialCert".to_string(),
        "02cf6cdf466951d8dfc9e7c9367511d0007ed6fba35ed42d425cc412fd6cfd4a17".to_string(),
    );
    keys
}

/// Get testnet default settings
///
/// Reference: TS TESTNET_DEFAULT_SETTINGS (WalletSettingsManager.ts lines 61-71)
pub fn testnet_default_settings() -> WalletSettings {
    let mut settings = default_settings();
    let testnet_keys = testnet_identity_keys();
    
    // Map certifiers to use testnet keys if available (TS lines 65-69)
    settings.trust_settings.trusted_certifiers = settings
        .trust_settings
        .trusted_certifiers
        .into_iter()
        .map(|mut certifier| {
            if let Some(testnet_key) = testnet_keys.get(&certifier.name) {
                certifier.identity_key = testnet_key.clone();
            }
            certifier
        })
        .collect();
    
    settings
}

/// Local key-value store interface
///
/// Reference: TS LocalKVStore from @bsv/sdk
#[async_trait::async_trait]
pub trait LocalKVStore: Send + Sync {
    /// Get a value from the store
    async fn get(&self, key: &str, default: &str) -> WalletResult<String>;
    
    /// Set a value in the store
    async fn set(&self, key: &str, value: &str) -> WalletResult<()>;
    
    /// Remove a value from the store
    async fn remove(&self, key: &str) -> WalletResult<()>;
}

/// Wallet Settings Manager
///
/// Reference: TS WalletSettingsManager class (WalletSettingsManager.ts lines 76-112)
///
/// ## Purpose
///
/// Manages wallet settings including:
/// - Trust settings (trust level, trusted certifiers)
/// - Theme preferences
/// - Currency preferences
///
/// ## Storage
///
/// Uses LocalKVStore for persistent storage in the "wallet settings" basket
pub struct WalletSettingsManager {
    /// Key-value store for settings persistence
    kv: Box<dyn LocalKVStore>,
    
    /// Configuration with defaults
    config: WalletSettingsManagerConfig,
}

impl WalletSettingsManager {
    /// Create a new WalletSettingsManager
    ///
    /// Reference: TS constructor (WalletSettingsManager.ts lines 79-86)
    ///
    /// # Arguments
    /// * `kv` - Key-value store implementation
    /// * `config` - Optional configuration (defaults to DEFAULT_SETTINGS)
    pub fn new(
        kv: Box<dyn LocalKVStore>,
        config: Option<WalletSettingsManagerConfig>,
    ) -> Self {
        Self {
            kv,
            config: config.unwrap_or_default(),
        }
    }
    
    /// Get the user's wallet settings
    ///
    /// Reference: TS get() (WalletSettingsManager.ts lines 93-95)
    ///
    /// Returns the stored settings, or default settings if none exist.
    ///
    /// # Returns
    /// Wallet settings object
    pub async fn get(&self) -> WalletResult<WalletSettings> {
        // Get from KV store with default as fallback (TS line 94)
        let default_json = serde_json::to_string(&self.config.default_settings)
            .map_err(|e| WalletError::invalid_operation(&format!("Failed to serialize defaults: {}", e)))?;
        
        let settings_json = self.kv.get("settings", &default_json).await?;
        
        // Parse JSON to WalletSettings
        let settings: WalletSettings = serde_json::from_str(&settings_json)
            .map_err(|e| WalletError::invalid_operation(&format!("Failed to parse settings: {}", e)))?;
        
        Ok(settings)
    }
    
    /// Set (create or update) the user's wallet settings
    ///
    /// Reference: TS set() (WalletSettingsManager.ts lines 102-104)
    ///
    /// # Arguments
    /// * `settings` - The wallet settings to store
    pub async fn set(&self, settings: WalletSettings) -> WalletResult<()> {
        // Serialize to JSON (TS line 103)
        let settings_json = serde_json::to_string(&settings)
            .map_err(|e| WalletError::invalid_operation(&format!("Failed to serialize settings: {}", e)))?;
        
        // Store in KV
        self.kv.set("settings", &settings_json).await
    }
    
    /// Delete the user's wallet settings
    ///
    /// Reference: TS delete() (WalletSettingsManager.ts lines 109-111)
    pub async fn delete(&self) -> WalletResult<()> {
        self.kv.remove("settings").await
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    
    // Mock KV store for testing
    struct MockKVStore {
        data: Arc<RwLock<HashMap<String, String>>>,
    }
    
    impl MockKVStore {
        fn new() -> Self {
            Self {
                data: Arc::new(RwLock::new(HashMap::new())),
            }
        }
    }
    
    #[async_trait::async_trait]
    impl LocalKVStore for MockKVStore {
        async fn get(&self, key: &str, default: &str) -> WalletResult<String> {
            let data = self.data.read().await;
            Ok(data.get(key).cloned().unwrap_or_else(|| default.to_string()))
        }
        
        async fn set(&self, key: &str, value: &str) -> WalletResult<()> {
            let mut data = self.data.write().await;
            data.insert(key.to_string(), value.to_string());
            Ok(())
        }
        
        async fn remove(&self, key: &str) -> WalletResult<()> {
            let mut data = self.data.write().await;
            data.remove(key);
            Ok(())
        }
    }
    
    #[test]
    fn test_default_settings() {
        let settings = default_settings();
        assert_eq!(settings.trust_settings.trust_level, 2);
        assert_eq!(settings.trust_settings.trusted_certifiers.len(), 2);
        assert_eq!(settings.theme.as_ref().unwrap().mode, "dark");
    }
    
    #[test]
    fn test_testnet_settings() {
        let settings = testnet_default_settings();
        assert_eq!(settings.trust_settings.trust_level, 2);
        
        // Check that SocialCert uses testnet key
        let socialcert = settings.trust_settings.trusted_certifiers
            .iter()
            .find(|c| c.name == "SocialCert")
            .unwrap();
        assert_eq!(
            socialcert.identity_key,
            "02cf6cdf466951d8dfc9e7c9367511d0007ed6fba35ed42d425cc412fd6cfd4a17"
        );
    }
    
    #[test]
    fn test_certifier_serde() {
        let certifier = Certifier {
            name: "Test".to_string(),
            description: "Test certifier".to_string(),
            identity_key: "abc123".to_string(),
            trust: 3,
            icon_url: Some("https://test.com/icon.png".to_string()),
            base_url: None,
        };
        
        let json = serde_json::to_string(&certifier).unwrap();
        let deserialized: Certifier = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized, certifier);
    }
    
    #[tokio::test]
    async fn test_settings_manager_get_default() {
        let kv = Box::new(MockKVStore::new());
        let manager = WalletSettingsManager::new(kv, None);
        
        let settings = manager.get().await.unwrap();
        assert_eq!(settings.trust_settings.trust_level, 2);
        assert_eq!(settings.theme.as_ref().unwrap().mode, "dark");
    }
    
    #[tokio::test]
    async fn test_settings_manager_set_get() {
        let kv = Box::new(MockKVStore::new());
        let manager = WalletSettingsManager::new(kv, None);
        
        // Create custom settings
        let mut custom_settings = default_settings();
        custom_settings.trust_settings.trust_level = 5;
        custom_settings.currency = Some("EUR".to_string());
        
        // Set the settings
        manager.set(custom_settings.clone()).await.unwrap();
        
        // Get them back
        let retrieved = manager.get().await.unwrap();
        assert_eq!(retrieved.trust_settings.trust_level, 5);
        assert_eq!(retrieved.currency, Some("EUR".to_string()));
    }
    
    #[tokio::test]
    async fn test_settings_manager_delete() {
        let kv = Box::new(MockKVStore::new());
        let manager = WalletSettingsManager::new(kv, None);
        
        // Set custom settings
        let mut custom_settings = default_settings();
        custom_settings.trust_settings.trust_level = 7;
        manager.set(custom_settings).await.unwrap();
        
        // Verify it's there
        let retrieved = manager.get().await.unwrap();
        assert_eq!(retrieved.trust_settings.trust_level, 7);
        
        // Delete
        manager.delete().await.unwrap();
        
        // Get should return defaults now
        let after_delete = manager.get().await.unwrap();
        assert_eq!(after_delete.trust_settings.trust_level, 2); // Default
    }
    
    #[tokio::test]
    async fn test_settings_manager_custom_defaults() {
        let kv = Box::new(MockKVStore::new());
        
        // Create custom defaults
        let custom_defaults = WalletSettings {
            trust_settings: TrustSettings {
                trust_level: 10,
                trusted_certifiers: vec![],
            },
            theme: Some(WalletTheme {
                mode: "light".to_string(),
            }),
            currency: Some("GBP".to_string()),
        };
        
        let config = WalletSettingsManagerConfig {
            default_settings: custom_defaults,
        };
        
        let manager = WalletSettingsManager::new(kv, Some(config));
        
        // Get should return custom defaults
        let settings = manager.get().await.unwrap();
        assert_eq!(settings.trust_settings.trust_level, 10);
        assert_eq!(settings.theme.as_ref().unwrap().mode, "light");
        assert_eq!(settings.currency, Some("GBP".to_string()));
    }
}
