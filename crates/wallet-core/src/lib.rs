//! Core domain types and wallet logic (placeholder)

pub fn version() -> &'static str { "0.0.1-dev" }

// SDK namespace
pub mod sdk;

// Utility namespace
pub mod utility;

// Core wallet namespaces
pub mod wallet;
pub mod setup;
pub mod managers;
pub mod signer; // signer has methods/* and WalletSigner type

// Public re-exports to mirror TS index boundaries
pub use wallet::Wallet;
pub use setup::{Setup, SetupClient, SetupWallet};
pub use signer::WalletSigner;
pub use managers::{
    WalletPermissionsManager,
    WalletAuthenticationManager,
    SimpleWalletManager,
    CWIStyleWalletManager,
};

// Storage namespace (placeholder)
pub mod storage {
    pub mod index_all {}
    pub mod index_client {}
    pub mod index_mobile {}
}

// Services namespace
pub mod services;

// Monitor namespace
pub mod monitor;

// WAB client namespace (placeholder)
pub mod wab_client;
