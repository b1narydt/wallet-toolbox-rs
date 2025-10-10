//! Validation for complex action arguments
//!
//! Translates TypeScript argument validation to Rust.
//! Reference: wallet-toolbox/src/sdk/validationHelpers.ts

use crate::sdk::errors::*;
use crate::sdk::types::OutPoint;
use crate::sdk::validation::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Type Definitions
// ============================================================================

/// Description string (5 to 2000 bytes)
pub type DescriptionString5to2000Bytes = String;

/// Validated create action input
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidCreateActionInput {
    pub outpoint: OutPoint,
    
    #[serde(rename = "inputDescription")]
    pub input_description: DescriptionString5to2000Bytes,
    
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: u32,
    
    #[serde(rename = "unlockingScript", skip_serializing_if = "Option::is_none")]
    pub unlocking_script: Option<String>,
    
    #[serde(rename = "unlockingScriptLength")]
    pub unlocking_script_length: usize,
}

/// Validated create action output
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidCreateActionOutput {
    #[serde(rename = "lockingScript")]
    pub locking_script: String,
    
    pub satoshis: i64,
    
    #[serde(rename = "outputDescription")]
    pub output_description: DescriptionString5to2000Bytes,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basket: Option<String>,
    
    #[serde(rename = "customInstructions", skip_serializing_if = "Option::is_none")]
    pub custom_instructions: Option<String>,
    
    pub tags: Vec<String>,
}

/// Validated wallet payment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidWalletPayment {
    #[serde(rename = "derivationPrefix")]
    pub derivation_prefix: String,
    
    #[serde(rename = "derivationSuffix")]
    pub derivation_suffix: String,
    
    #[serde(rename = "senderIdentityKey")]
    pub sender_identity_key: String,
}

/// Validated basket insertion
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidBasketInsertion {
    pub basket: String,
    
    #[serde(rename = "customInstructions", skip_serializing_if = "Option::is_none")]
    pub custom_instructions: Option<String>,
    
    pub tags: Vec<String>,
}

/// Validated internalize output
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidInternalizeOutput {
    #[serde(rename = "outputIndex")]
    pub output_index: u32,
    
    pub protocol: String, // "wallet payment" or "basket insertion"
    
    #[serde(rename = "paymentRemittance", skip_serializing_if = "Option::is_none")]
    pub payment_remittance: Option<ValidWalletPayment>,
    
    #[serde(rename = "insertionRemittance", skip_serializing_if = "Option::is_none")]
    pub insertion_remittance: Option<ValidBasketInsertion>,
}

// ============================================================================
// Validation Functions
// ============================================================================

/// Validate create action input
///
/// Matches TypeScript `validateCreateActionInput` function
pub fn validate_create_action_input(
    outpoint: &str,
    input_description: &str,
    sequence_number: Option<u32>,
    unlocking_script: Option<&str>,
    unlocking_script_length: Option<usize>,
) -> Result<ValidCreateActionInput, WalletError> {
    // At least one of unlockingScript or unlockingScriptLength must be provided
    if unlocking_script.is_none() && unlocking_script_length.is_none() {
        return Err(WErrInvalidParameter::new(
            "unlockingScript, unlockingScriptLength",
            Some("at least one valid value".to_string()),
        ));
    }
    
    let validated_unlocking_script = validate_optional_hex_string(unlocking_script, "unlockingScript", None, None)?;
    
    let computed_length = match &validated_unlocking_script {
        Some(script) => script.len() / 2,
        None => unlocking_script_length.unwrap(),
    };
    
    // If both provided, they must match
    if let Some(script) = &validated_unlocking_script {
        if let Some(length) = unlocking_script_length {
            if length != script.len() / 2 {
                return Err(WErrInvalidParameter::new(
                    "unlockingScriptLength",
                    Some("length unlockingScript if both valid".to_string()),
                ));
            }
        }
    }
    
    Ok(ValidCreateActionInput {
        outpoint: parse_wallet_outpoint(outpoint)?,
        input_description: validate_string_length(input_description, "inputDescription", Some(5), Some(2000))?,
        sequence_number: sequence_number.unwrap_or(0xffffffff),
        unlocking_script: validated_unlocking_script,
        unlocking_script_length: computed_length,
    })
}

/// Validate create action output
///
/// Matches TypeScript `validateCreateActionOutput` function
pub fn validate_create_action_output(
    locking_script: &str,
    satoshis: i64,
    output_description: &str,
    basket: Option<&str>,
    custom_instructions: Option<&str>,
    tags: &[String],
) -> Result<ValidCreateActionOutput, WalletError> {
    let validated_tags: Result<Vec<String>, WalletError> = tags
        .iter()
        .map(|t| validate_tag(t))
        .collect();
    
    Ok(ValidCreateActionOutput {
        locking_script: validate_hex_string(locking_script, "lockingScript", None, None)?,
        satoshis: validate_satoshis(Some(satoshis), "satoshis", None)?,
        output_description: validate_string_length(output_description, "outputDescription", Some(5), Some(2000))?,
        basket: validate_optional_basket(basket)?,
        custom_instructions: custom_instructions.map(|s| s.to_string()),
        tags: validated_tags?,
    })
}

/// Validate wallet payment
///
/// Matches TypeScript `validateWalletPayment` function
pub fn validate_wallet_payment(
    derivation_prefix: &str,
    derivation_suffix: &str,
    sender_identity_key: &str,
) -> Result<ValidWalletPayment, WalletError> {
    Ok(ValidWalletPayment {
        derivation_prefix: validate_base64_string(derivation_prefix, "derivationPrefix", None, None)?,
        derivation_suffix: validate_base64_string(derivation_suffix, "derivationSuffix", None, None)?,
        sender_identity_key: validate_hex_string(sender_identity_key, "senderIdentityKey", None, None)?,
    })
}

/// Validate basket insertion
///
/// Matches TypeScript `validateBasketInsertion` function
pub fn validate_basket_insertion(
    basket: &str,
    custom_instructions: Option<&str>,
    tags: &[String],
) -> Result<ValidBasketInsertion, WalletError> {
    let validated_tags: Result<Vec<String>, WalletError> = tags
        .iter()
        .map(|t| validate_tag(t))
        .collect();
    
    Ok(ValidBasketInsertion {
        basket: validate_basket(basket)?,
        custom_instructions: validate_optional_string_length(custom_instructions, "customInstructions", Some(0), Some(1000))?,
        tags: validated_tags?,
    })
}

/// Validate internalize output
///
/// Matches TypeScript `validateInternalizeOutput` function
pub fn validate_internalize_output(
    output_index: u32,
    protocol: &str,
    payment_remittance: Option<ValidWalletPayment>,
    insertion_remittance: Option<ValidBasketInsertion>,
) -> Result<ValidInternalizeOutput, WalletError> {
    if protocol != "basket insertion" && protocol != "wallet payment" {
        return Err(WErrInvalidParameter::new(
            "protocol",
            Some("'basket insertion' or 'wallet payment'".to_string()),
        ));
    }
    
    Ok(ValidInternalizeOutput {
        output_index: validate_positive_integer_or_zero(output_index as i64, "outputIndex")? as u32,
        protocol: protocol.to_string(),
        payment_remittance,
        insertion_remittance,
    })
}

/// Validate originator string (domain-like format)
///
/// Matches TypeScript `validateOriginator` function
pub fn validate_originator(s: Option<&str>) -> Result<Option<String>, WalletError> {
    match s {
        None => Ok(None),
        Some(val) => {
            let normalized = val.trim().to_lowercase();
            validate_string_length(&normalized, "originator", Some(1), Some(250))?;
            
            // Validate each part
            let parts: Vec<&str> = normalized.split('.').collect();
            for part in parts {
                validate_string_length(part, "originator part", Some(1), Some(63))?;
            }
            
            Ok(Some(normalized))
        }
    }
}

/// Validate certificate fields
pub fn validate_certificate_fields(
    fields: &std::collections::HashMap<String, String>,
) -> Result<std::collections::HashMap<String, String>, WalletError> {
    let mut validated = std::collections::HashMap::new();
    
    for (field_name, field_value) in fields {
        validate_string_length(field_name, "field name", Some(1), Some(50))?;
        validated.insert(field_name.clone(), field_value.clone());
    }
    
    Ok(validated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_create_action_input_with_script() {
        let result = validate_create_action_input(
            "abc123.0",
            "test input",
            None,
            Some("deadbeef"),
            None,
        ).unwrap();
        
        assert_eq!(result.outpoint.txid, "abc123");
        assert_eq!(result.outpoint.vout, 0);
        assert_eq!(result.input_description, "test input");
        assert_eq!(result.sequence_number, 0xffffffff);
        assert_eq!(result.unlocking_script, Some("deadbeef".to_string()));
        assert_eq!(result.unlocking_script_length, 4);
    }

    #[test]
    fn test_validate_create_action_input_with_length_only() {
        let result = validate_create_action_input(
            "abc.1",
            "test input",
            Some(100),
            None,
            Some(50),
        ).unwrap();
        
        assert_eq!(result.sequence_number, 100);
        assert_eq!(result.unlocking_script_length, 50);
        assert!(result.unlocking_script.is_none());
    }

    #[test]
    fn test_validate_create_action_input_neither_provided() {
        let result = validate_create_action_input(
            "abc.0",
            "test input",
            None,
            None,
            None,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_create_action_input_length_mismatch() {
        let result = validate_create_action_input(
            "abc.0",
            "test input",
            None,
            Some("deadbeef"), // 4 bytes
            Some(10),         // but says 10
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_create_action_output() {
        let tags = vec!["tag1".to_string(), "tag2".to_string()];
        let result = validate_create_action_output(
            "deadbeef",
            5000,
            "test output",
            Some("my-basket"),
            Some("custom instructions"),
            &tags,
        ).unwrap();
        
        assert_eq!(result.locking_script, "deadbeef");
        assert_eq!(result.satoshis, 5000);
        assert_eq!(result.output_description, "test output");
        assert_eq!(result.basket, Some("my-basket".to_string()));
        assert_eq!(result.custom_instructions, Some("custom instructions".to_string()));
        assert_eq!(result.tags.len(), 2);
    }

    #[test]
    fn test_validate_create_action_output_invalid_satoshis() {
        let result = validate_create_action_output(
            "deadbeef",
            -100,
            "test output",
            None,
            None,
            &[],
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_wallet_payment() {
        let result = validate_wallet_payment(
            "SGVsbG8=",      // base64
            "V29ybGQ=",      // base64
            "deadbeef",      // hex
        ).unwrap();
        
        assert_eq!(result.derivation_prefix, "SGVsbG8=");
        assert_eq!(result.derivation_suffix, "V29ybGQ=");
        assert_eq!(result.sender_identity_key, "deadbeef");
    }

    #[test]
    fn test_validate_wallet_payment_invalid_base64() {
        let result = validate_wallet_payment(
            "not-base64!!!",
            "V29ybGQ=",
            "deadbeef",
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_basket_insertion() {
        let tags = vec!["important".to_string(), "verified".to_string()];
        let result = validate_basket_insertion(
            "my-basket",
            Some("custom"),
            &tags,
        ).unwrap();
        
        assert_eq!(result.basket, "my-basket");
        assert_eq!(result.custom_instructions, Some("custom".to_string()));
        assert_eq!(result.tags.len(), 2);
    }

    #[test]
    fn test_validate_internalize_output_wallet_payment() {
        let payment = ValidWalletPayment {
            derivation_prefix: "prefix".to_string(),
            derivation_suffix: "suffix".to_string(),
            sender_identity_key: "key".to_string(),
        };
        
        let result = validate_internalize_output(
            0,
            "wallet payment",
            Some(payment.clone()),
            None,
        ).unwrap();
        
        assert_eq!(result.output_index, 0);
        assert_eq!(result.protocol, "wallet payment");
        assert!(result.payment_remittance.is_some());
        assert!(result.insertion_remittance.is_none());
    }

    #[test]
    fn test_validate_internalize_output_basket_insertion() {
        let insertion = ValidBasketInsertion {
            basket: "my-basket".to_string(),
            custom_instructions: None,
            tags: vec![],
        };
        
        let result = validate_internalize_output(
            1,
            "basket insertion",
            None,
            Some(insertion.clone()),
        ).unwrap();
        
        assert_eq!(result.output_index, 1);
        assert_eq!(result.protocol, "basket insertion");
        assert!(result.payment_remittance.is_none());
        assert!(result.insertion_remittance.is_some());
    }

    #[test]
    fn test_validate_internalize_output_invalid_protocol() {
        let result = validate_internalize_output(
            0,
            "invalid protocol",
            None,
            None,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_originator() {
        let result = validate_originator(Some("  Example.Com  ")).unwrap();
        assert_eq!(result, Some("example.com".to_string()));
    }

    #[test]
    fn test_validate_originator_none() {
        let result = validate_originator(None).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_validate_originator_too_long_part() {
        // Create a part longer than 63 characters
        let long_part = "a".repeat(64);
        let originator = format!("{}.com", long_part);
        let result = validate_originator(Some(&originator));
        
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_certificate_fields() {
        let mut fields = std::collections::HashMap::new();
        fields.insert("email".to_string(), "user@example.com".to_string());
        fields.insert("name".to_string(), "John Doe".to_string());
        
        let result = validate_certificate_fields(&fields).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result.get("email"), Some(&"user@example.com".to_string()));
    }

    #[test]
    fn test_validate_certificate_fields_field_name_too_long() {
        let mut fields = std::collections::HashMap::new();
        let long_name = "a".repeat(51);
        fields.insert(long_name, "value".to_string());
        
        let result = validate_certificate_fields(&fields);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_create_action_input_serialization() {
        let input = ValidCreateActionInput {
            outpoint: OutPoint::new("abc", 0),
            input_description: "test".to_string(),
            sequence_number: 100,
            unlocking_script: Some("dead".to_string()),
            unlocking_script_length: 2,
        };
        
        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("\"inputDescription\":\"test\""));
        assert!(json.contains("\"sequenceNumber\":100"));
        
        let deserialized: ValidCreateActionInput = serde_json::from_str(&json).unwrap();
        assert_eq!(input, deserialized);
    }

    #[test]
    fn test_valid_create_action_output_serialization() {
        let output = ValidCreateActionOutput {
            locking_script: "beef".to_string(),
            satoshis: 1000,
            output_description: "test".to_string(),
            basket: Some("my-basket".to_string()),
            custom_instructions: None,
            tags: vec!["tag1".to_string()],
        };
        
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("\"lockingScript\":\"beef\""));
        assert!(json.contains("\"satoshis\":1000"));
        
        let deserialized: ValidCreateActionOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(output, deserialized);
    }
}
