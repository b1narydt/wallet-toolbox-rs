//! Tauri Command Handlers for metanet-desktop Integration
//!
//! This module provides all 28 WalletInterface methods as Tauri commands
//! that can be called from the TypeScript frontend.
//!
//! ## Usage in Tauri App
//!
//! ```rust,no_run
//! use wallet_core::tauri_commands::*;
//! use wallet_core::wallet::{Wallet, WalletConfig};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Initialize wallet
//!     let wallet = Wallet::new(config).unwrap();
//!     
//!     tauri::Builder::default()
//!         .manage(wallet)
//!         .invoke_handler(tauri::generate_handler![
//!             wallet_create_action,
//!             wallet_sign_action,
//!             // ... all 28 commands
//!         ])
//!         .run(tauri::generate_context!())
//!         .expect("error while running tauri application");
//! }
//! ```

use crate::wallet::Wallet;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Type alias for managed Wallet state in Tauri
pub type WalletState = Arc<Mutex<Wallet>>;

// ============================================================================
// ACTION MANAGEMENT COMMANDS (5)
// ============================================================================

/// Create a new transaction action
#[tauri::command]
pub async fn wallet_create_action(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .create_action(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Sign a transaction action
#[tauri::command]
pub async fn wallet_sign_action(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .sign_action(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Abort a pending action
#[tauri::command]
pub async fn wallet_abort_action(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .abort_action(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// List transaction actions
#[tauri::command]
pub async fn wallet_list_actions(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .list_actions(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Internalize an incoming action
#[tauri::command]
pub async fn wallet_internalize_action(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .internalize_action(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

// ============================================================================
// OUTPUT MANAGEMENT COMMANDS (2)
// ============================================================================

/// List unspent transaction outputs
#[tauri::command]
pub async fn wallet_list_outputs(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .list_outputs(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Relinquish control of an output
#[tauri::command]
pub async fn wallet_relinquish_output(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .relinquish_output(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

// ============================================================================
// KEY OPERATIONS COMMANDS (3)
// ============================================================================

/// Get a public key for a specific purpose
#[tauri::command]
pub async fn wallet_get_public_key(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .get_public_key(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Reveal counterparty key linkage
#[tauri::command]
pub async fn wallet_reveal_counterparty_key_linkage(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .reveal_counterparty_key_linkage(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Reveal specific key linkage
#[tauri::command]
pub async fn wallet_reveal_specific_key_linkage(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .reveal_specific_key_linkage(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

// ============================================================================
// CRYPTOGRAPHIC OPERATIONS COMMANDS (6)
// ============================================================================

/// Encrypt data
#[tauri::command]
pub async fn wallet_encrypt(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .encrypt(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Decrypt data
#[tauri::command]
pub async fn wallet_decrypt(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .decrypt(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Create an HMAC
#[tauri::command]
pub async fn wallet_create_hmac(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .create_hmac(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Verify an HMAC
#[tauri::command]
pub async fn wallet_verify_hmac(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .verify_hmac(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Create a signature
#[tauri::command]
pub async fn wallet_create_signature(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .create_signature(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Verify a signature
#[tauri::command]
pub async fn wallet_verify_signature(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .verify_signature(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

// ============================================================================
// CERTIFICATE OPERATIONS COMMANDS (4)
// ============================================================================

/// Acquire a certificate
#[tauri::command]
pub async fn wallet_acquire_certificate(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .acquire_certificate(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// List certificates
#[tauri::command]
pub async fn wallet_list_certificates(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .list_certificates(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Prove certificate ownership
#[tauri::command]
pub async fn wallet_prove_certificate(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .prove_certificate(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Relinquish a certificate
#[tauri::command]
pub async fn wallet_relinquish_certificate(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .relinquish_certificate(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

// ============================================================================
// IDENTITY OPERATIONS COMMANDS (2)
// ============================================================================

/// Discover by identity key
#[tauri::command]
pub async fn wallet_discover_by_identity_key(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .discover_by_identity_key(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Discover by attributes
#[tauri::command]
pub async fn wallet_discover_by_attributes(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .discover_by_attributes(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

// ============================================================================
// AUTHENTICATION COMMANDS (2)
// ============================================================================

/// Check if authenticated
#[tauri::command]
pub async fn wallet_is_authenticated(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .is_authenticated(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Wait for authentication
#[tauri::command]
pub async fn wallet_wait_for_authentication(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .wait_for_authentication(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

// ============================================================================
// BLOCKCHAIN QUERY COMMANDS (4)
// ============================================================================

/// Get current blockchain height
#[tauri::command]
pub async fn wallet_get_height(
    wallet: tauri::State<'_, WalletState>,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .get_height(Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Get block header for specific height
#[tauri::command]
pub async fn wallet_get_header_for_height(
    wallet: tauri::State<'_, WalletState>,
    args: Value,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .get_header_for_height(args, Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Get network information
#[tauri::command]
pub async fn wallet_get_network(
    wallet: tauri::State<'_, WalletState>,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .get_network(Some(&originator))
        .await
        .map_err(|e| e.to_string())
}

/// Get wallet version
#[tauri::command]
pub async fn wallet_get_version(
    wallet: tauri::State<'_, WalletState>,
    originator: String,
) -> Result<Value, String> {
    let wallet = wallet.lock().await;
    wallet
        .get_version(Some(&originator))
        .await
        .map_err(|e| e.to_string())
}
