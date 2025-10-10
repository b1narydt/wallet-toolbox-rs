//! Wallet interface types for cryptographic operations
//!
//! Mirrors TypeScript SDK types from @bsv/sdk Wallet.interfaces.ts
//! Reference: ts-sdk/src/wallet/Wallet.interfaces.ts

use serde::{Deserialize, Serialize};

// ============================================================================
// Protocol and Key Types
// ============================================================================

/// Protocol ID: [security_level, protocol_string]
/// Security level: 0 = open, 1 = authenticated, 2 = confidential
pub type WalletProtocol = (u8, String);

/// Public key in hex format (compressed, 33 bytes = 66 hex chars)
pub type PubKeyHex = String;

/// Key ID string (max 800 bytes)
pub type KeyIDString = String;

/// Wallet counterparty - public key or 'self' or 'anyone'
pub type WalletCounterparty = String;

/// ISO 8601 timestamp string
pub type ISOTimestampString = String;

// ============================================================================
// Base Encryption Args
// ============================================================================

/// Base arguments for wallet encryption operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletEncryptionArgs {
    /// Protocol identifier [security_level, protocol_string]
    #[serde(rename = "protocolID")]
    pub protocol_id: WalletProtocol,
    
    /// Key identifier string (max 800 bytes)
    #[serde(rename = "keyID")]
    pub key_id: KeyIDString,
    
    /// Optional counterparty public key (defaults to 'self')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<WalletCounterparty>,
    
    /// Whether this is a privileged operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    
    /// Reason for privileged access (required if privileged=true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_reason: Option<String>,
    
    /// Whether to seek user permission (default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seek_permission: Option<bool>,
}

// ============================================================================
// Get Public Key
// ============================================================================

/// Arguments for getting a public key
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPublicKeyArgs {
    /// Get identity key (overrides other fields if true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_key: Option<bool>,
    
    /// Whether to get key for self vs counterparty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_self: Option<bool>,
    
    /// Protocol ID (flattened from WalletEncryptionArgs)
    #[serde(rename = "protocolID", skip_serializing_if = "Option::is_none")]
    pub protocol_id: Option<WalletProtocol>,
    
    /// Key ID (flattened from WalletEncryptionArgs)
    #[serde(rename = "keyID", skip_serializing_if = "Option::is_none")]
    pub key_id: Option<KeyIDString>,
    
    /// Counterparty (flattened from WalletEncryptionArgs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<WalletCounterparty>,
    
    /// Privileged operation flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    
    /// Privileged reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_reason: Option<String>,
}

/// Result from getting a public key
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPublicKeyResult {
    /// The public key in hex format
    pub public_key: PubKeyHex,
}

// ============================================================================
// HMAC Operations
// ============================================================================

/// Arguments for creating an HMAC
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateHmacArgs {
    /// Protocol identifier
    #[serde(rename = "protocolID")]
    pub protocol_id: WalletProtocol,
    
    /// Key identifier
    #[serde(rename = "keyID")]
    pub key_id: KeyIDString,
    
    /// Data to create HMAC for
    pub data: Vec<u8>,
    
    /// Optional counterparty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<WalletCounterparty>,
    
    /// Privileged operation flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    
    /// Privileged reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_reason: Option<String>,
}

/// Result from creating an HMAC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateHmacResult {
    /// The HMAC bytes
    pub hmac: Vec<u8>,
}

/// Arguments for verifying an HMAC
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyHmacArgs {
    /// Protocol identifier
    #[serde(rename = "protocolID")]
    pub protocol_id: WalletProtocol,
    
    /// Key identifier
    #[serde(rename = "keyID")]
    pub key_id: KeyIDString,
    
    /// Data to verify HMAC for
    pub data: Vec<u8>,
    
    /// HMAC to verify
    pub hmac: Vec<u8>,
    
    /// Optional counterparty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<WalletCounterparty>,
    
    /// Privileged operation flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    
    /// Privileged reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_reason: Option<String>,
}

/// Result from verifying an HMAC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyHmacResult {
    /// Whether the HMAC is valid (always true on success, error on failure)
    pub valid: bool,
}

// ============================================================================
// Signature Operations
// ============================================================================

/// Arguments for creating a signature
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSignatureArgs {
    /// Protocol identifier
    #[serde(rename = "protocolID")]
    pub protocol_id: WalletProtocol,
    
    /// Key identifier
    #[serde(rename = "keyID")]
    pub key_id: KeyIDString,
    
    /// Data to sign (required unless hashToDirectlySign is provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    
    /// Pre-hashed data to sign directly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_to_directly_sign: Option<Vec<u8>>,
    
    /// Optional counterparty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<WalletCounterparty>,
    
    /// Privileged operation flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    
    /// Privileged reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_reason: Option<String>,
}

/// Result from creating a signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSignatureResult {
    /// The DER-encoded ECDSA signature
    pub signature: Vec<u8>,
}

/// Arguments for verifying a signature
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifySignatureArgs {
    /// Protocol identifier
    #[serde(rename = "protocolID")]
    pub protocol_id: WalletProtocol,
    
    /// Key identifier
    #[serde(rename = "keyID")]
    pub key_id: KeyIDString,
    
    /// Data that was signed (required unless hashToDirectlyVerify is provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    
    /// Pre-hashed data to verify directly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_to_directly_verify: Option<Vec<u8>>,
    
    /// The signature to verify
    pub signature: Vec<u8>,
    
    /// Whether signature was created by self (vs counterparty)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_self: Option<bool>,
    
    /// Optional counterparty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<WalletCounterparty>,
    
    /// Privileged operation flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    
    /// Privileged reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_reason: Option<String>,
}

/// Result from verifying a signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifySignatureResult {
    /// Whether the signature is valid (always true on success, error on failure)
    pub valid: bool,
}

// ============================================================================
// Key Linkage Operations
// ============================================================================

/// Arguments for revealing counterparty key linkage
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevealCounterpartyKeyLinkageArgs {
    /// The counterparty public key
    pub counterparty: PubKeyHex,
    
    /// The verifier public key (who will receive the linkage)
    pub verifier: PubKeyHex,
    
    /// Whether this is a privileged operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    
    /// Reason for privileged access
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_reason: Option<String>,
}

/// Base key linkage result fields
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyLinkageResult {
    /// Encrypted linkage data
    pub encrypted_linkage: Vec<u8>,
    
    /// Encrypted linkage proof
    pub encrypted_linkage_proof: Vec<u8>,
    
    /// Prover's public key (identity key)
    pub prover: PubKeyHex,
    
    /// Verifier's public key
    pub verifier: PubKeyHex,
    
    /// Counterparty's public key
    pub counterparty: PubKeyHex,
}

/// Result from revealing counterparty key linkage
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevealCounterpartyKeyLinkageResult {
    /// Encrypted linkage data
    pub encrypted_linkage: Vec<u8>,
    
    /// Encrypted linkage proof
    pub encrypted_linkage_proof: Vec<u8>,
    
    /// Prover's public key (identity key)
    pub prover: PubKeyHex,
    
    /// Verifier's public key
    pub verifier: PubKeyHex,
    
    /// Counterparty's public key
    pub counterparty: PubKeyHex,
    
    /// ISO timestamp of when the revelation was created
    pub revelation_time: ISOTimestampString,
}

/// Arguments for revealing specific key linkage
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevealSpecificKeyLinkageArgs {
    /// The counterparty public key or special value
    pub counterparty: WalletCounterparty,
    
    /// The verifier public key (who will receive the linkage)
    pub verifier: PubKeyHex,
    
    /// Protocol identifier for the specific key
    #[serde(rename = "protocolID")]
    pub protocol_id: WalletProtocol,
    
    /// Key identifier for the specific key
    #[serde(rename = "keyID")]
    pub key_id: KeyIDString,
    
    /// Whether this is a privileged operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    
    /// Reason for privileged access
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_reason: Option<String>,
}

/// Result from revealing specific key linkage
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevealSpecificKeyLinkageResult {
    /// Encrypted linkage data
    pub encrypted_linkage: Vec<u8>,
    
    /// Encrypted linkage proof
    pub encrypted_linkage_proof: Vec<u8>,
    
    /// Prover's public key (identity key)
    pub prover: PubKeyHex,
    
    /// Verifier's public key
    pub verifier: PubKeyHex,
    
    /// Counterparty's public key or special value
    pub counterparty: PubKeyHex,
    
    /// Protocol identifier
    #[serde(rename = "protocolID")]
    pub protocol_id: WalletProtocol,
    
    /// Key identifier
    #[serde(rename = "keyID")]
    pub key_id: KeyIDString,
    
    /// Proof type (0 = no proof)
    pub proof_type: u8,
}

// ============================================================================
// Encryption/Decryption Operations
// ============================================================================

/// Arguments for wallet encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletEncryptArgs {
    /// Protocol identifier
    #[serde(rename = "protocolID")]
    pub protocol_id: WalletProtocol,
    
    /// Key identifier
    #[serde(rename = "keyID")]
    pub key_id: KeyIDString,
    
    /// Plaintext data to encrypt
    pub plaintext: Vec<u8>,
    
    /// Optional counterparty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<WalletCounterparty>,
    
    /// Privileged operation flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    
    /// Privileged reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_reason: Option<String>,
}

/// Result from wallet encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletEncryptResult {
    /// Encrypted ciphertext (includes IV)
    pub ciphertext: Vec<u8>,
}

/// Arguments for wallet decryption
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletDecryptArgs {
    /// Protocol identifier
    #[serde(rename = "protocolID")]
    pub protocol_id: WalletProtocol,
    
    /// Key identifier
    #[serde(rename = "keyID")]
    pub key_id: KeyIDString,
    
    /// Ciphertext to decrypt (includes IV)
    pub ciphertext: Vec<u8>,
    
    /// Optional counterparty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<WalletCounterparty>,
    
    /// Privileged operation flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    
    /// Privileged reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_reason: Option<String>,
}

/// Result from wallet decryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletDecryptResult {
    /// Decrypted plaintext
    pub plaintext: Vec<u8>,
}

// ============================================================================
// Blockchain Query Operations
// ============================================================================

/// Arguments for getting a block header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHeaderArgs {
    /// Block height
    pub height: u32,
}

/// Result from getting a block header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHeaderResult {
    /// Serialized block header in hex format
    pub header: String,
}

/// Result from getting blockchain height
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHeightResult {
    /// Current blockchain height
    pub height: u32,
}

/// Result from getting network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNetworkResult {
    /// Network name ("main" or "test")
    pub network: String,
}

/// Result from getting wallet version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVersionResult {
    /// Wallet version string
    pub version: String,
}

// ============================================================================
// Output Management
// ============================================================================

/// Arguments for relinquishing an output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelinquishOutputArgs {
    /// Transaction ID containing the output
    pub txid: String,
    
    /// Output index
    pub vout: u32,
    
    /// Optional basket name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basket: Option<String>,
}

/// Result from relinquishing an output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelinquishOutputResult {
    /// Whether the output was relinquished
    pub relinquished: bool,
}

// ============================================================================
// Authentication
// ============================================================================

/// Result from authentication check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedResult {
    /// Whether authenticated
    pub authenticated: bool,
}
