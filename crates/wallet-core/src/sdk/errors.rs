//! Error types for wallet operations
//! 
//! Translates TypeScript WalletError and WERR_* error classes to Rust.
//! Reference: wallet-toolbox/src/sdk/WalletError.ts and WERR_errors.ts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Base error type for all wallet operations.
/// 
/// Derived from TypeScript WalletError class which extends Error.
/// Provides code, description, details, and stack trace capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletError {
    /// Error code (10-40 bytes, matches ErrorCodeString10To40Bytes)
    pub code: String,
    
    /// Error description (20-200 bytes, matches ErrorDescriptionString20To200Bytes)
    pub description: String,
    
    /// Optional additional details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, String>>,
    
    /// Optional stack trace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
}

impl WalletError {
    /// Create a new WalletError with code and description
    pub fn new(code: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            description: description.into(),
            details: None,
            stack: None,
        }
    }

    /// Create a new WalletError with all fields
    pub fn with_details(
        code: impl Into<String>,
        description: impl Into<String>,
        details: Option<HashMap<String, String>>,
        stack: Option<String>,
    ) -> Self {
        Self {
            code: code.into(),
            description: description.into(),
            details,
            stack,
        }
    }

    /// Convert to HTTP status object
    /// 
    /// Matches TypeScript asStatus() method
    pub fn as_status(&self) -> serde_json::Value {
        serde_json::json!({
            "status": "error",
            "code": self.code,
            "description": self.description
        })
    }

    /// Recover error from unknown source
    /// 
    /// Matches TypeScript WalletError.fromUnknown() static method
    pub fn from_unknown(err: &dyn std::error::Error) -> Self {
        let code = "WERR_UNKNOWN".to_string();
        let description = err.to_string();
        
        Self {
            code,
            description,
            details: None,
            stack: None,
        }
    }

    /// Create error from dynamic error type
    pub fn from_dyn(err: Box<dyn std::error::Error>) -> Self {
        Self::new("WERR_UNKNOWN", err.to_string())
    }
    
    /// Create an invalid parameter error
    pub fn invalid_parameter(parameter: impl Into<String>, must_be: impl Into<String>) -> Self {
        WErrInvalidParameter::new(parameter, Some(must_be.into()))
    }
    
    /// Create an invalid operation error
    pub fn invalid_operation(message: impl Into<String>) -> Self {
        WErrInvalidOperation::new(Some(message.into()))
    }
    
    /// Create a not implemented error
    pub fn not_implemented(message: impl Into<String>) -> Self {
        WErrNotImplemented::new(Some(message.into()))
    }
    
    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        WErrInternal::new(Some(message.into()))
    }
    
    /// Create a missing parameter error
    pub fn missing_parameter(parameter: impl Into<String>) -> Self {
        WErrMissingParameter::new(parameter)
    }
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.description)
    }
}

impl std::error::Error for WalletError {}

impl From<std::io::Error> for WalletError {
    fn from(err: std::io::Error) -> Self {
        Self::new("WERR_IO", err.to_string())
    }
}

impl From<serde_json::Error> for WalletError {
    fn from(err: serde_json::Error) -> Self {
        Self::new("WERR_JSON", err.to_string())
    }
}

/// Result type for wallet operations
pub type WalletResult<T> = Result<T, WalletError>;

// ============================================================================
// WERR Error Variants
// ============================================================================

/// Not implemented error
#[derive(Debug, Clone)]
pub struct WErrNotImplemented {
    pub message: Option<String>,
}

impl WErrNotImplemented {
    pub fn new(message: Option<String>) -> WalletError {
        WalletError::new(
            "WERR_NOT_IMPLEMENTED",
            message.unwrap_or_else(|| "Not implemented.".to_string()),
        )
    }
}

/// Internal error
#[derive(Debug, Clone)]
pub struct WErrInternal {
    pub message: Option<String>,
}

impl WErrInternal {
    pub fn new(message: Option<String>) -> WalletError {
        WalletError::new(
            "WERR_INTERNAL",
            message.unwrap_or_else(|| "An internal error has occurred.".to_string()),
        )
    }
}

/// Invalid operation error
#[derive(Debug, Clone)]
pub struct WErrInvalidOperation {
    pub message: Option<String>,
}

impl WErrInvalidOperation {
    pub fn new(message: Option<String>) -> WalletError {
        WalletError::new(
            "WERR_INVALID_OPERATION",
            message.unwrap_or_else(|| "An invalid operation was requested.".to_string()),
        )
    }
}

/// Broadcast unavailable error
#[derive(Debug, Clone)]
pub struct WErrBroadcastUnavailable;

impl WErrBroadcastUnavailable {
    pub fn new(_message: Option<String>) -> WalletError {
        WalletError::new(
            "WERR_BROADCAST_UNAVAILABLE",
            "Unable to broadcast transaction at this time.",
        )
    }
}

/// Invalid parameter error
#[derive(Debug, Clone)]
pub struct WErrInvalidParameter {
    pub parameter: String,
    pub must_be: Option<String>,
}

impl WErrInvalidParameter {
    pub fn new(parameter: impl Into<String>, must_be: Option<String>) -> WalletError {
        let param = parameter.into();
        let requirement = must_be.unwrap_or_else(|| "valid.".to_string());
        WalletError::new(
            "WERR_INVALID_PARAMETER",
            format!("The {} parameter must be {}", param, requirement),
        )
    }
}

/// Missing parameter error
#[derive(Debug, Clone)]
pub struct WErrMissingParameter {
    pub parameter: String,
}

impl WErrMissingParameter {
    pub fn new(parameter: impl Into<String>) -> WalletError {
        let param = parameter.into();
        WalletError::new(
            "WERR_MISSING_PARAMETER",
            format!("The required {} parameter is missing.", param),
        )
    }
}

/// Bad request error
#[derive(Debug, Clone)]
pub struct WErrBadRequest {
    pub message: Option<String>,
}

impl WErrBadRequest {
    pub fn new(message: Option<String>) -> WalletError {
        WalletError::new(
            "WERR_BAD_REQUEST",
            message.unwrap_or_else(|| "The request is invalid.".to_string()),
        )
    }
}

/// Network chain error
#[derive(Debug, Clone)]
pub struct WErrNetworkChain {
    pub message: Option<String>,
}

impl WErrNetworkChain {
    pub fn new(message: Option<String>) -> WalletError {
        WalletError::new(
            "WERR_NETWORK_CHAIN",
            message.unwrap_or_else(|| {
                "Configured network chain is invalid or does not match across services.".to_string()
            }),
        )
    }
}

/// Unauthorized error
#[derive(Debug, Clone)]
pub struct WErrUnauthorized {
    pub message: Option<String>,
}

impl WErrUnauthorized {
    pub fn new(message: Option<String>) -> WalletError {
        WalletError::new(
            "WERR_UNAUTHORIZED",
            message.unwrap_or_else(|| "Access is denied due to an authorization error.".to_string()),
        )
    }
}

/// Not active storage error
#[derive(Debug, Clone)]
pub struct WErrNotActive {
    pub message: Option<String>,
}

impl WErrNotActive {
    pub fn new(message: Option<String>) -> WalletError {
        WalletError::new(
            "WERR_NOT_ACTIVE",
            message.unwrap_or_else(|| {
                "WalletStorageManager is not accessing user's active storage or there are conflicting active stores configured.".to_string()
            }),
        )
    }
}

/// Insufficient funds error
#[derive(Debug, Clone)]
pub struct WErrInsufficientFunds {
    pub total_satoshis_needed: u64,
    pub more_satoshis_needed: u64,
}

impl WErrInsufficientFunds {
    pub fn new(total_satoshis_needed: u64, more_satoshis_needed: u64) -> WalletError {
        WalletError::new(
            "WERR_INSUFFICIENT_FUNDS",
            format!(
                "Insufficient funds in the available inputs to cover the cost of the required outputs and the transaction fee ({} more satoshis are needed, for a total of {}), plus whatever would be required in order to pay the fee to unlock and spend the outputs used to provide the additional satoshis.",
                more_satoshis_needed, total_satoshis_needed
            ),
        )
    }
}

/// Wallet network type (matches TypeScript WalletNetwork)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WalletNetwork {
    Mainnet,
    Testnet,
}

/// Invalid public key error
#[derive(Debug, Clone)]
pub struct WErrInvalidPublicKey {
    pub key: String,
    pub network: WalletNetwork,
}

impl WErrInvalidPublicKey {
    pub fn new(key: impl Into<String>, network: WalletNetwork) -> WalletError {
        let key_str = key.into();
        let message = match network {
            WalletNetwork::Mainnet => {
                format!("The provided public key \"{}\" is invalid or malformed.", key_str)
            }
            WalletNetwork::Testnet => {
                "The provided public key is invalid or malformed.".to_string()
            }
        };
        WalletError::new("WERR_INVALID_PUBLIC_KEY", message)
    }
}

/// Review actions error - thrown when createAction or signAction requires review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewActionResult {
    // Placeholder - will be fully defined when implementing storage types
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendWithResult {
    // Placeholder - will be fully defined when implementing action types
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct WErrReviewActions {
    pub review_action_results: Vec<ReviewActionResult>,
    pub send_with_results: Vec<SendWithResult>,
    pub txid: Option<String>,
    pub tx: Option<Vec<u8>>, // AtomicBEEF placeholder
    pub no_send_change: Option<Vec<String>>, // OutpointString placeholder
}

impl WErrReviewActions {
    pub fn new(
        _review_action_results: Vec<ReviewActionResult>,
        _send_with_results: Vec<SendWithResult>,
        _txid: Option<String>,
        _tx: Option<Vec<u8>>,
        _no_send_change: Option<Vec<String>>,
    ) -> WalletError {
        // Note: In full implementation, these parameters would be stored in the error
        // For now, we just create the basic error message matching TypeScript behavior
        WalletError::new(
            "WERR_REVIEW_ACTIONS",
            "Undelayed createAction or signAction results require review.",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_error_new() {
        let err = WalletError::new("TEST_CODE", "Test description");
        assert_eq!(err.code, "TEST_CODE");
        assert_eq!(err.description, "Test description");
        assert!(err.details.is_none());
        assert!(err.stack.is_none());
    }

    #[test]
    fn test_wallet_error_with_details() {
        let mut details = HashMap::new();
        details.insert("key".to_string(), "value".to_string());
        
        let err = WalletError::with_details(
            "TEST_CODE",
            "Test description",
            Some(details.clone()),
            Some("stack trace".to_string()),
        );
        
        assert_eq!(err.code, "TEST_CODE");
        assert_eq!(err.description, "Test description");
        assert_eq!(err.details, Some(details));
        assert_eq!(err.stack, Some("stack trace".to_string()));
    }

    #[test]
    fn test_wallet_error_as_status() {
        let err = WalletError::new("TEST_ERROR", "Test message");
        let status = err.as_status();
        
        assert_eq!(status["status"], "error");
        assert_eq!(status["code"], "TEST_ERROR");
        assert_eq!(status["description"], "Test message");
    }

    #[test]
    fn test_wallet_error_display() {
        let err = WalletError::new("TEST_CODE", "Test message");
        assert_eq!(format!("{}", err), "TEST_CODE: Test message");
    }

    #[test]
    fn test_werr_not_implemented() {
        let err = WErrNotImplemented::new(None);
        assert_eq!(err.code, "WERR_NOT_IMPLEMENTED");
        assert_eq!(err.description, "Not implemented.");
    }

    #[test]
    fn test_werr_not_implemented_custom() {
        let err = WErrNotImplemented::new(Some("Custom message".to_string()));
        assert_eq!(err.code, "WERR_NOT_IMPLEMENTED");
        assert_eq!(err.description, "Custom message");
    }

    #[test]
    fn test_werr_internal() {
        let err = WErrInternal::new(None);
        assert_eq!(err.code, "WERR_INTERNAL");
        assert_eq!(err.description, "An internal error has occurred.");
    }

    #[test]
    fn test_werr_invalid_operation() {
        let err = WErrInvalidOperation::new(None);
        assert_eq!(err.code, "WERR_INVALID_OPERATION");
        assert_eq!(err.description, "An invalid operation was requested.");
    }

    #[test]
    fn test_werr_broadcast_unavailable() {
        let err = WErrBroadcastUnavailable::new(None);
        assert_eq!(err.code, "WERR_BROADCAST_UNAVAILABLE");
        assert_eq!(err.description, "Unable to broadcast transaction at this time.");
    }

    #[test]
    fn test_werr_invalid_parameter() {
        let err = WErrInvalidParameter::new("testParam", None);
        assert_eq!(err.code, "WERR_INVALID_PARAMETER");
        assert_eq!(err.description, "The testParam parameter must be valid.");
    }

    #[test]
    fn test_werr_invalid_parameter_with_requirement() {
        let err = WErrInvalidParameter::new("age", Some("a positive number".to_string()));
        assert_eq!(err.code, "WERR_INVALID_PARAMETER");
        assert_eq!(err.description, "The age parameter must be a positive number");
    }

    #[test]
    fn test_werr_missing_parameter() {
        let err = WErrMissingParameter::new("userId");
        assert_eq!(err.code, "WERR_MISSING_PARAMETER");
        assert_eq!(err.description, "The required userId parameter is missing.");
    }

    #[test]
    fn test_werr_bad_request() {
        let err = WErrBadRequest::new(None);
        assert_eq!(err.code, "WERR_BAD_REQUEST");
        assert_eq!(err.description, "The request is invalid.");
    }

    #[test]
    fn test_werr_network_chain() {
        let err = WErrNetworkChain::new(None);
        assert_eq!(err.code, "WERR_NETWORK_CHAIN");
        assert!(err.description.contains("network chain"));
    }

    #[test]
    fn test_werr_unauthorized() {
        let err = WErrUnauthorized::new(None);
        assert_eq!(err.code, "WERR_UNAUTHORIZED");
        assert_eq!(err.description, "Access is denied due to an authorization error.");
    }

    #[test]
    fn test_werr_not_active() {
        let err = WErrNotActive::new(None);
        assert_eq!(err.code, "WERR_NOT_ACTIVE");
        assert!(err.description.contains("WalletStorageManager"));
    }

    #[test]
    fn test_werr_insufficient_funds() {
        let err = WErrInsufficientFunds::new(10000, 5000);
        assert_eq!(err.code, "WERR_INSUFFICIENT_FUNDS");
        assert!(err.description.contains("5000 more satoshis"));
        assert!(err.description.contains("total of 10000"));
    }

    #[test]
    fn test_werr_invalid_public_key_mainnet() {
        let err = WErrInvalidPublicKey::new("badkey123", WalletNetwork::Mainnet);
        assert_eq!(err.code, "WERR_INVALID_PUBLIC_KEY");
        assert!(err.description.contains("badkey123"));
        assert!(err.description.contains("invalid or malformed"));
    }

    #[test]
    fn test_werr_invalid_public_key_testnet() {
        let err = WErrInvalidPublicKey::new("testkey", WalletNetwork::Testnet);
        assert_eq!(err.code, "WERR_INVALID_PUBLIC_KEY");
        assert!(!err.description.contains("testkey")); // Should not include key on testnet
        assert!(err.description.contains("invalid or malformed"));
    }

    #[test]
    fn test_wallet_error_serialization() {
        let err = WalletError::new("TEST_CODE", "Test message");
        let json = serde_json::to_string(&err).unwrap();
        let deserialized: WalletError = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.code, err.code);
        assert_eq!(deserialized.description, err.description);
    }

    #[test]
    fn test_wallet_network_serialization() {
        let mainnet = WalletNetwork::Mainnet;
        let json = serde_json::to_string(&mainnet).unwrap();
        assert_eq!(json, "\"mainnet\"");
        
        let testnet = WalletNetwork::Testnet;
        let json = serde_json::to_string(&testnet).unwrap();
        assert_eq!(json, "\"testnet\"");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let wallet_err: WalletError = io_err.into();
        
        assert_eq!(wallet_err.code, "WERR_IO");
        assert!(wallet_err.description.contains("file not found"));
    }

    #[test]
    fn test_json_error_conversion() {
        let json_err = serde_json::from_str::<serde_json::Value>("{invalid}").unwrap_err();
        let wallet_err: WalletError = json_err.into();
        
        assert_eq!(wallet_err.code, "WERR_JSON");
    }
}
