//! Permission Manager Utilities
//!
//! **Reference**: TypeScript `src/WalletPermissionsManager.ts` lines 20-45
//!
//! Helper functions for permission management

use crate::sdk::errors::{WalletError, WalletResult};
use serde_json::Value;
use uuid::Uuid;

/// Deep equality comparison for JSON values
///
/// Reference: TS deepEqual (WalletPermissionsManager.ts lines 20-41)
///
/// Recursively compares two objects for deep equality, checking all nested properties.
///
/// # Arguments
/// * `object1` - First value to compare
/// * `object2` - Second value to compare
///
/// # Returns
/// `true` if objects are deeply equal, `false` otherwise
pub fn deep_equal(object1: &Value, object2: &Value) -> bool {
    // TS lines 21-23: Handle null/undefined
    match (object1, object2) {
        (Value::Null, Value::Null) => return true,
        (Value::Null, _) | (_, Value::Null) => return false,
        _ => {}
    }
    
    // If both are objects, do deep comparison
    if let (Value::Object(map1), Value::Object(map2)) = (object1, object2) {
        // TS lines 24-29: Check key count
        if map1.len() != map2.len() {
            return false;
        }
        
        // TS lines 31-38: Compare each key-value pair
        for (key, val1) in map1.iter() {
            match map2.get(key) {
                Some(val2) => {
                    let are_objects = is_object(val1) && is_object(val2);
                    // TS line 35: Recursive comparison for objects, direct for primitives
                    if (are_objects && !deep_equal(val1, val2)) || (!are_objects && val1 != val2) {
                        return false;
                    }
                }
                None => return false,
            }
        }
        
        // TS line 40: All checks passed
        true
    } else if let (Value::Array(arr1), Value::Array(arr2)) = (object1, object2) {
        // Handle array comparison
        if arr1.len() != arr2.len() {
            return false;
        }
        for (v1, v2) in arr1.iter().zip(arr2.iter()) {
            if !deep_equal(v1, v2) {
                return false;
            }
        }
        true
    } else {
        // For primitives, use direct equality
        object1 == object2
    }
}

/// Check if a value is an object
///
/// Reference: TS isObject (WalletPermissionsManager.ts lines 43-45)
///
/// # Arguments
/// * `value` - Value to check
///
/// # Returns
/// `true` if value is an object (not null), `false` otherwise
pub fn is_object(value: &Value) -> bool {
    // TS line 44: object != null && typeof object === 'object'
    matches!(value, Value::Object(_))
}

/// Generate a unique request ID
///
/// Reference: Used throughout WalletPermissionsManager for tracking requests
///
/// # Returns
/// A unique UUID string for tracking permission requests
pub fn create_request_id() -> String {
    Uuid::new_v4().to_string()
}

/// Sanitize an originator string
///
/// Reference: Implicit validation throughout permission request handling
///
/// Ensures the originator is a valid domain/FQDN format.
///
/// # Arguments
/// * `originator` - The originator string to sanitize
///
/// # Returns
/// Sanitized originator or error if invalid
pub fn sanitize_originator(originator: &str) -> WalletResult<String> {
    let trimmed = originator.trim();
    
    // Basic validation - must not be empty
    if trimmed.is_empty() {
        return Err(WalletError::invalid_parameter(
            "originator",
            "must not be empty"
        ));
    }
    
    // Must not contain whitespace
    if trimmed.contains(char::is_whitespace) {
        return Err(WalletError::invalid_parameter(
            "originator",
            "must not contain whitespace"
        ));
    }
    
    // Should look like a domain (contains at least one dot or is "localhost")
    if trimmed != "localhost" && !trimmed.contains('.') {
        return Err(WalletError::invalid_parameter(
            "originator",
            "must be a valid domain or FQDN"
        ));
    }
    
    Ok(trimmed.to_string())
}

/// Validate token expiry time
///
/// Reference: Token expiry validation used in permission token checking
///
/// # Arguments
/// * `expiry` - Expiry timestamp in UNIX epoch seconds
///
/// # Returns
/// `true` if token is still valid, `false` if expired
pub fn is_token_expired(expiry: i64) -> bool {
    if expiry == 0 {
        // Spending authorizations with expiry=0 are indefinite
        return false;
    }
    
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    now > expiry
}

/// Get current month identifier for spending tracking
///
/// Reference: Used in spending authorization to track monthly limits
///
/// # Returns
/// Month identifier in format "YYYY-MM"
pub fn get_current_month() -> String {
    use chrono::prelude::*;
    let now = Utc::now();
    format!("{:04}-{:02}", now.year(), now.month())
}

/// Parse protocol ID into components
///
/// Reference: Protocol ID format [securityLevel, protocolName] used throughout
///
/// # Arguments
/// * `protocol_id` - Protocol ID array with [securityLevel, protocolName]
///
/// # Returns
/// Tuple of (security_level, protocol_name) or error
pub fn parse_protocol_id(protocol_id: &[String]) -> WalletResult<(String, String)> {
    if protocol_id.len() != 2 {
        return Err(WalletError::invalid_parameter(
            "protocolID",
            "must have exactly 2 elements: [securityLevel, protocolName]"
        ));
    }
    
    Ok((protocol_id[0].clone(), protocol_id[1].clone()))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_deep_equal_primitives() {
        // TS deepEqual tests
        assert!(deep_equal(&json!(42), &json!(42)));
        assert!(deep_equal(&json!("hello"), &json!("hello")));
        assert!(deep_equal(&json!(true), &json!(true)));
        assert!(deep_equal(&json!(null), &json!(null)));
        
        assert!(!deep_equal(&json!(42), &json!(43)));
        assert!(!deep_equal(&json!("hello"), &json!("world")));
        assert!(!deep_equal(&json!(true), &json!(false)));
    }
    
    #[test]
    fn test_deep_equal_objects() {
        // TS deepEqual object comparison (lines 20-41)
        let obj1 = json!({"a": 1, "b": 2});
        let obj2 = json!({"a": 1, "b": 2});
        let obj3 = json!({"a": 1, "b": 3});
        let obj4 = json!({"a": 1, "b": 2, "c": 3});
        
        assert!(deep_equal(&obj1, &obj2));
        assert!(!deep_equal(&obj1, &obj3));
        assert!(!deep_equal(&obj1, &obj4)); // Different key counts
    }
    
    #[test]
    fn test_deep_equal_nested() {
        // TS deepEqual recursive comparison (line 35)
        let nested1 = json!({"a": {"b": {"c": 1}}});
        let nested2 = json!({"a": {"b": {"c": 1}}});
        let nested3 = json!({"a": {"b": {"c": 2}}});
        
        assert!(deep_equal(&nested1, &nested2));
        assert!(!deep_equal(&nested1, &nested3));
    }
    
    #[test]
    fn test_is_object() {
        // TS isObject (lines 43-45)
        assert!(is_object(&json!({"a": 1})));
        assert!(!is_object(&json!(null)));
        assert!(!is_object(&json!(42)));
        assert!(!is_object(&json!("hello")));
        assert!(!is_object(&json!([1, 2, 3])));
    }
    
    #[test]
    fn test_create_request_id() {
        let id1 = create_request_id();
        let id2 = create_request_id();
        
        // Should be valid UUIDs
        assert!(Uuid::parse_str(&id1).is_ok());
        assert!(Uuid::parse_str(&id2).is_ok());
        
        // Should be unique
        assert_ne!(id1, id2);
    }
    
    #[test]
    fn test_sanitize_originator() {
        // Valid originators
        assert!(sanitize_originator("example.com").is_ok());
        assert!(sanitize_originator("sub.example.com").is_ok());
        assert!(sanitize_originator("localhost").is_ok());
        
        // Invalid originators
        assert!(sanitize_originator("").is_err());
        assert!(sanitize_originator("  ").is_err());
        assert!(sanitize_originator("has spaces").is_err());
        assert!(sanitize_originator("nodotin").is_err());
    }
    
    #[test]
    fn test_is_token_expired() {
        // Indefinite token (expiry = 0)
        assert!(!is_token_expired(0));
        
        // Far future token
        let far_future = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64) + 86400 * 365; // 1 year from now
        assert!(!is_token_expired(far_future));
        
        // Past token
        assert!(is_token_expired(1000000000)); // Way in the past
    }
    
    #[test]
    fn test_get_current_month() {
        let month = get_current_month();
        // Should match YYYY-MM format
        assert_eq!(month.len(), 7);
        assert_eq!(&month[4..5], "-");
    }
    
    #[test]
    fn test_parse_protocol_id() {
        // Valid protocol ID
        let valid = vec!["2".to_string(), "myProtocol".to_string()];
        let result = parse_protocol_id(&valid);
        assert!(result.is_ok());
        let (level, name) = result.unwrap();
        assert_eq!(level, "2");
        assert_eq!(name, "myProtocol");
        
        // Invalid - wrong length
        let invalid = vec!["2".to_string()];
        assert!(parse_protocol_id(&invalid).is_err());
        
        let invalid = vec!["2".to_string(), "protocol".to_string(), "extra".to_string()];
        assert!(parse_protocol_id(&invalid).is_err());
    }
}
