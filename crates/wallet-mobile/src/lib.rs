//! Mobile wrapper (placeholder)

pub use wallet_core::sdk;
pub use wallet_core::utility::index_client as utility;
pub use wallet_core::WalletSigner;
pub use wallet_core::WalletPermissionsManager;
pub use wallet_core::CWIStyleWalletManager;
pub use wallet_core::WalletAuthenticationManager;
pub use wallet_core::wab_client::WABClient;
pub use wallet_core::wab_client::auth_method_interactors::TwilioPhoneInteractor;
pub use wallet_core::wab_client::auth_method_interactors::PersonaIDInteractor;
pub use wallet_core::wab_client::auth_method_interactors::AuthMethodInteractor;
pub use wallet_core::services::Services;
pub use wallet_core::sdk::PrivilegedKeyManager;
pub use wallet_core::SimpleWalletManager;
pub use wallet_core::storage::index_mobile as storage;
pub use wallet_core::Wallet;
pub use wallet_core::monitor::Monitor;

pub fn init() {}
