//! Wallet Managers Module
//!
//! Reference: TypeScript src/ manager files
//!
//! Wallet managers provide high-level wallet orchestration and authentication

pub mod simple_wallet_manager;
pub mod wallet_settings_manager;
pub mod wallet_auth_manager;
pub mod wallet_permissions_manager;

// Re-exports
pub use simple_wallet_manager::{
    SimpleWalletManager,
    WalletInterface,
    PrivilegedKeyManager,
    WalletBuilder,
    OriginatorDomainName,
};

pub use wallet_settings_manager::{
    WalletSettingsManager,
    WalletSettings,
    TrustSettings,
    Certifier,
    WalletTheme,
    WalletSettingsManagerConfig,
    LocalKVStore,
    default_settings,
    testnet_default_settings,
    SETTINGS_BASKET,
};

pub use wallet_auth_manager::{
    WalletAuthenticationManager,
    PresentationKeyHex,
};

pub use wallet_permissions_manager::{
    WalletPermissionsManager,
    PermissionType,
    PermissionRequest,
    PermissionToken,
    GroupedPermissions,
    GroupedPermissionRequest,
    PermissionsManagerConfig,
};

// Stubs for remaining managers (to be implemented)
#[derive(Debug, Default)]
pub struct CWIStyleWalletManager;
