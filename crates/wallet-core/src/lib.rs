//! Core domain types and wallet logic

pub fn version() -> &'static str { "0.1.0" }

// SDK types and interfaces
pub mod sdk;

// Core wallet methods (createAction, signAction, etc.)
pub mod methods;

// BEEF (Background Evaluation Extended Format) implementation
pub mod beef;

// Bitcoin transaction primitives (pure Rust for performance)
pub mod transaction;

// Cryptographic operations (ECDSA signing, key derivation)
pub mod crypto;

// BRC-42/43 key derivation
pub mod keys;

// Wallet managers (SimpleWalletManager, WalletSettingsManager, etc.)
pub mod managers;

// Signer methods (buildSignableTransaction, completeSignedTransaction, etc.)
pub mod signer;

// Utility functions and helpers
pub mod utility;

// WAB (Wallet Authentication Bridge) client
pub mod wab_client;

// Main wallet orchestration
pub mod wallet;

// Monitor for transaction tracking
pub mod monitor;

// Setup and initialization
pub mod setup;

// Service integrations (placeholder - actual services in wallet-services crate)
pub mod services;

// Tauri command handlers for metanet-desktop integration
#[cfg(feature = "tauri")]
pub mod tauri_commands;
