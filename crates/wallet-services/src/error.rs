//! Service error types
//!
//! Defines errors that can occur when interacting with external services

use thiserror::Error;

/// Service error type
#[derive(Debug, Error)]
pub enum ServiceError {
    /// HTTP request error
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    /// JSON parsing error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// Service not available
    #[error("Service unavailable: {0}")]
    Unavailable(String),
    
    /// Service returned an error
    #[error("Service error from {service}: {message}")]
    ServiceFailed {
        service: String,
        message: String,
    },
    
    /// Transaction not found
    #[error("Transaction not found: {0}")]
    TxNotFound(String),
    
    /// Block not found
    #[error("Block not found at height {0}")]
    BlockNotFound(u32),
    
    /// Invalid response from service
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    
    /// Timeout waiting for response
    #[error("Request timeout")]
    Timeout,
    
    /// Rate limit exceeded
    #[error("Rate limit exceeded for service: {0}")]
    RateLimitExceeded(String),
    
    /// Invalid parameters
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),
    
    /// No services configured
    #[error("No services configured")]
    NoServices,
    
    /// All services failed
    #[error("All services failed")]
    AllServicesFailed,
}

/// Result type for service operations
pub type ServiceResult<T> = Result<T, ServiceError>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_display() {
        let error = ServiceError::TxNotFound("abc123".to_string());
        assert!(error.to_string().contains("abc123"));
        
        let error = ServiceError::ServiceFailed {
            service: "test".to_string(),
            message: "failed".to_string(),
        };
        assert!(error.to_string().contains("test"));
        assert!(error.to_string().contains("failed"));
    }
}
