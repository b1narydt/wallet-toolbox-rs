//! Validation helper functions
//!
//! Translates TypeScript validation helpers to Rust.
//! Reference: wallet-toolbox/src/sdk/validationHelpers.ts

use crate::sdk::errors::*;
use crate::sdk::types::OutPoint;

/// Parse wallet outpoint string format "txid.vout"
///
/// Matches TypeScript `parseWalletOutpoint` function
pub fn parse_wallet_outpoint(outpoint: &str) -> Result<OutPoint, WalletError> {
    let parts: Vec<&str> = outpoint.split('.').collect();
    if parts.len() != 2 {
        return Err(WErrInvalidParameter::new(
            "outpoint",
            Some("format 'txid.vout'".to_string()),
        ));
    }
    
    let txid = parts[0].to_string();
    let vout = parts[1].parse::<u32>().map_err(|_| {
        WErrInvalidParameter::new("outpoint vout", Some("a valid number".to_string()))
    })?;
    
    Ok(OutPoint::new(txid, vout))
}

/// Validate satoshis value
///
/// Matches TypeScript `validateSatoshis` function
pub fn validate_satoshis(
    v: Option<i64>,
    name: &str,
    min: Option<i64>,
) -> Result<i64, WalletError> {
    match v {
        None => Err(WErrInvalidParameter::new(
            name,
            Some("a valid number of satoshis".to_string()),
        )),
        Some(val) => {
            // Check bounds: 0 to 21e14 (max Bitcoin supply)
            if val < 0 || val > 21_000_000_000_000_00 {
                return Err(WErrInvalidParameter::new(
                    name,
                    Some("a valid number of satoshis".to_string()),
                ));
            }
            
            if let Some(min_val) = min {
                if val < min_val {
                    return Err(WErrInvalidParameter::new(
                        name,
                        Some(format!("at least {} satoshis", min_val)),
                    ));
                }
            }
            
            Ok(val)
        }
    }
}

/// Validate integer value with optional bounds
///
/// Matches TypeScript `validateInteger` function
pub fn validate_integer(
    v: Option<i64>,
    name: &str,
    default_value: Option<i64>,
    min: Option<i64>,
    max: Option<i64>,
) -> Result<i64, WalletError> {
    let val = match v {
        Some(v) => v,
        None => match default_value {
            Some(d) => return Ok(d),
            None => {
                return Err(WErrInvalidParameter::new(
                    name,
                    Some("a valid integer".to_string()),
                ))
            }
        },
    };
    
    if let Some(min_val) = min {
        if val < min_val {
            return Err(WErrInvalidParameter::new(
                name,
                Some(format!("at least {} length", min_val)),
            ));
        }
    }
    
    if let Some(max_val) = max {
        if val > max_val {
            return Err(WErrInvalidParameter::new(
                name,
                Some(format!("no more than {} length", max_val)),
            ));
        }
    }
    
    Ok(val)
}

/// Validate optional integer
pub fn validate_optional_integer(
    v: Option<i64>,
    name: &str,
    min: Option<i64>,
    max: Option<i64>,
) -> Result<Option<i64>, WalletError> {
    match v {
        None => Ok(None),
        Some(_) => Ok(Some(validate_integer(v, name, None, min, max)?)),
    }
}

/// Validate positive integer or zero
///
/// Matches TypeScript `validatePositiveIntegerOrZero` function
pub fn validate_positive_integer_or_zero(v: i64, name: &str) -> Result<i64, WalletError> {
    validate_integer(Some(v), name, Some(0), Some(0), None)
}

/// Validate string length in bytes
///
/// Matches TypeScript `validateStringLength` function
pub fn validate_string_length(
    s: &str,
    name: &str,
    min: Option<usize>,
    max: Option<usize>,
) -> Result<String, WalletError> {
    let bytes = s.as_bytes().len();
    
    if let Some(min_len) = min {
        if bytes < min_len {
            return Err(WErrInvalidParameter::new(
                name,
                Some(format!("at least {} length", min_len)),
            ));
        }
    }
    
    if let Some(max_len) = max {
        if bytes > max_len {
            return Err(WErrInvalidParameter::new(
                name,
                Some(format!("no more than {} length", max_len)),
            ));
        }
    }
    
    Ok(s.to_string())
}

/// Validate optional string length
pub fn validate_optional_string_length(
    s: Option<&str>,
    name: &str,
    min: Option<usize>,
    max: Option<usize>,
) -> Result<Option<String>, WalletError> {
    match s {
        None => Ok(None),
        Some(val) => Ok(Some(validate_string_length(val, name, min, max)?)),
    }
}

/// Validate hex string
///
/// Matches TypeScript `validateHexString` function
pub fn validate_hex_string(
    s: &str,
    name: &str,
    min: Option<usize>,
    max: Option<usize>,
) -> Result<String, WalletError> {
    let trimmed = s.trim().to_lowercase();
    
    // Check even length
    if trimmed.len() % 2 != 0 {
        return Err(WErrInvalidParameter::new(
            name,
            Some(format!("even length, not {}", trimmed.len())),
        ));
    }
    
    // Check hex characters
    if !trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(WErrInvalidParameter::new(
            name,
            Some("hexadecimal string".to_string()),
        ));
    }
    
    if let Some(min_len) = min {
        if trimmed.len() < min_len {
            return Err(WErrInvalidParameter::new(
                name,
                Some(format!("at least {} length", min_len)),
            ));
        }
    }
    
    if let Some(max_len) = max {
        if trimmed.len() > max_len {
            return Err(WErrInvalidParameter::new(
                name,
                Some(format!("no more than {} length", max_len)),
            ));
        }
    }
    
    Ok(trimmed)
}

/// Validate optional hex string
pub fn validate_optional_hex_string(
    s: Option<&str>,
    name: &str,
    min: Option<usize>,
    max: Option<usize>,
) -> Result<Option<String>, WalletError> {
    match s {
        None => Ok(None),
        Some(val) => Ok(Some(validate_hex_string(val, name, min, max)?)),
    }
}

/// Check if string is hex
///
/// Matches TypeScript `isHexString` function
pub fn is_hex_string(s: &str) -> bool {
    let trimmed = s.trim();
    if trimmed.len() % 2 != 0 {
        return false;
    }
    trimmed.chars().all(|c| c.is_ascii_hexdigit())
}

/// Validate base64 string
///
/// Matches TypeScript `validateBase64String` function
pub fn validate_base64_string(
    s: &str,
    name: &str,
    min: Option<usize>,
    max: Option<usize>,
) -> Result<String, WalletError> {
    use base64::{Engine as _, engine::general_purpose};
    
    let trimmed = s.trim();
    
    // Try to decode to verify it's valid base64
    let decoded = general_purpose::STANDARD.decode(trimmed).map_err(|_| {
        WErrInvalidParameter::new(name, Some("valid base64 string".to_string()))
    })?;
    
    let bytes = decoded.len();
    
    if let Some(min_len) = min {
        if bytes < min_len {
            return Err(WErrInvalidParameter::new(
                name,
                Some(format!("at least {} length", min_len)),
            ));
        }
    }
    
    if let Some(max_len) = max {
        if bytes > max_len {
            return Err(WErrInvalidParameter::new(
                name,
                Some(format!("no more than {} length", max_len)),
            ));
        }
    }
    
    Ok(trimmed.to_string())
}

/// Validate optional base64 string
pub fn validate_optional_base64_string(
    s: Option<&str>,
    name: &str,
    min: Option<usize>,
    max: Option<usize>,
) -> Result<Option<String>, WalletError> {
    match s {
        None => Ok(None),
        Some(val) => Ok(Some(validate_base64_string(val, name, min, max)?)),
    }
}

/// Validate identifier (trimmed and lowercased)
///
/// Matches TypeScript `validateIdentifier` function
pub fn validate_identifier(
    s: &str,
    name: &str,
    min: Option<usize>,
    max: Option<usize>,
) -> Result<String, WalletError> {
    let normalized = s.trim().to_lowercase();
    validate_string_length(&normalized, name, min, max)
}

/// Validate basket name
pub fn validate_basket(s: &str) -> Result<String, WalletError> {
    validate_identifier(s, "basket", Some(1), Some(300))
}

/// Validate optional basket
pub fn validate_optional_basket(s: Option<&str>) -> Result<Option<String>, WalletError> {
    match s {
        None => Ok(None),
        Some(val) => Ok(Some(validate_basket(val)?)),
    }
}

/// Validate label
pub fn validate_label(s: &str) -> Result<String, WalletError> {
    validate_identifier(s, "label", Some(1), Some(300))
}

/// Validate tag
pub fn validate_tag(s: &str) -> Result<String, WalletError> {
    validate_identifier(s, "tag", Some(1), Some(300))
}

/// Validate outpoint string format "txid.vout"
///
/// Matches TypeScript `validateOutpointString` function
pub fn validate_outpoint_string(outpoint: &str, name: &str) -> Result<String, WalletError> {
    let parts: Vec<&str> = outpoint.split('.').collect();
    if parts.len() != 2 {
        return Err(WErrInvalidParameter::new(
            name,
            Some("txid as hex string and numeric output index joined with '.'".to_string()),
        ));
    }
    
    let txid = validate_hex_string(parts[0], &format!("{} txid", name), None, Some(64))?;
    let vout = parts[1].parse::<u32>().map_err(|_| {
        WErrInvalidParameter::new(&format!("{} vout", name), Some("a valid number".to_string()))
    })?;
    
    validate_positive_integer_or_zero(vout as i64, &format!("{} vout", name))?;
    
    Ok(format!("{}.{}", txid, vout))
}

/// Validate optional outpoint string
pub fn validate_optional_outpoint_string(
    outpoint: Option<&str>,
    name: &str,
) -> Result<Option<String>, WalletError> {
    match outpoint {
        None => Ok(None),
        Some(val) => Ok(Some(validate_outpoint_string(val, name)?)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wallet_outpoint() {
        let result = parse_wallet_outpoint("abc123.5").unwrap();
        assert_eq!(result.txid, "abc123");
        assert_eq!(result.vout, 5);
    }

    #[test]
    fn test_parse_wallet_outpoint_invalid() {
        assert!(parse_wallet_outpoint("invalid").is_err());
        assert!(parse_wallet_outpoint("abc.notanumber").is_err());
    }

    #[test]
    fn test_validate_satoshis() {
        assert_eq!(validate_satoshis(Some(1000), "amount", None).unwrap(), 1000);
        assert!(validate_satoshis(Some(-1), "amount", None).is_err());
        assert!(validate_satoshis(Some(100), "amount", Some(500)).is_err());
        assert!(validate_satoshis(None, "amount", None).is_err());
    }

    #[test]
    fn test_validate_integer() {
        assert_eq!(validate_integer(Some(5), "val", None, Some(1), Some(10)).unwrap(), 5);
        assert_eq!(validate_integer(None, "val", Some(42), None, None).unwrap(), 42);
        assert!(validate_integer(Some(0), "val", None, Some(1), None).is_err());
        assert!(validate_integer(Some(100), "val", None, None, Some(50)).is_err());
    }

    #[test]
    fn test_validate_string_length() {
        assert_eq!(
            validate_string_length("hello", "str", Some(3), Some(10)).unwrap(),
            "hello"
        );
        assert!(validate_string_length("ab", "str", Some(3), None).is_err());
        assert!(validate_string_length("toolong", "str", None, Some(5)).is_err());
    }

    #[test]
    fn test_validate_hex_string() {
        assert_eq!(
            validate_hex_string("deadbeef", "hex", None, None).unwrap(),
            "deadbeef"
        );
        assert_eq!(
            validate_hex_string("DeAdBeEf", "hex", None, None).unwrap(),
            "deadbeef"
        );
        assert!(validate_hex_string("notahex", "hex", None, None).is_err());
        assert!(validate_hex_string("abc", "hex", None, None).is_err()); // odd length
    }

    #[test]
    fn test_is_hex_string() {
        assert!(is_hex_string("deadbeef"));
        assert!(is_hex_string("DEADBEEF"));
        assert!(!is_hex_string("xyz"));
        assert!(!is_hex_string("abc")); // odd length
    }

    #[test]
    fn test_validate_base64_string() {
        assert!(validate_base64_string("SGVsbG8=", "b64", None, None).is_ok());
        assert!(validate_base64_string("invalid!!!", "b64", None, None).is_err());
    }

    #[test]
    fn test_validate_identifier() {
        assert_eq!(
            validate_identifier("  MyBASKET  ", "id", None, None).unwrap(),
            "mybasket"
        );
    }

    #[test]
    fn test_validate_basket() {
        assert_eq!(validate_basket("  Change  ").unwrap(), "change");
        assert!(validate_basket("").is_err());
    }

    #[test]
    fn test_validate_outpoint_string() {
        let result = validate_outpoint_string(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef.42",
            "outpoint"
        ).unwrap();
        assert!(result.contains(".42"));
    }
}
