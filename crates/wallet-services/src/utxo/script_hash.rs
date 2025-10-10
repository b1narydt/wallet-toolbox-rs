//! Script hash validation
//!
//! **Reference**: TypeScript `src/services/Services.ts` (lines 546-565)

use sha2::{Sha256, Digest};
use crate::error::{ServiceError, ServiceResult};
use crate::types::GetUtxoStatusOutputFormat;

/// Validate and convert output to script hash
///
/// Reference: TS validateScriptHash (Services.ts lines 546-565)
///
/// # Arguments
/// * `output` - Output script or hash
/// * `output_format` - Format of the output
///
/// # Returns
/// Script hash in big-endian hex format
pub fn validate_script_hash(
    output: &str,
    output_format: Option<GetUtxoStatusOutputFormat>,
) -> ServiceResult<String> {
    // Decode hex input (TS line 547)
    let mut bytes = hex::decode(output)
        .map_err(|_| ServiceError::InvalidParams("Invalid hex string".to_string()))?;
    
    // Determine format if not specified (TS lines 548-551)
    let format = match output_format {
        Some(f) => f,
        None => {
            if bytes.len() == 32 {
                GetUtxoStatusOutputFormat::HashLE
            } else {
                GetUtxoStatusOutputFormat::Script
            }
        }
    };
    
    // Process based on format (TS lines 552-563)
    match format {
        // hashBE - no processing needed (TS line 554)
        GetUtxoStatusOutputFormat::HashBE => {
            // Already in correct format
        }
        // hashLE - reverse to get BE (TS lines 555-557)
        GetUtxoStatusOutputFormat::HashLE => {
            bytes.reverse();
        }
        // script - hash it and reverse (TS lines 558-560)
        GetUtxoStatusOutputFormat::Script => {
            let hash = Sha256::digest(&bytes);
            bytes = hash.to_vec();
            bytes.reverse();
        }
    }
    
    Ok(hex::encode(&bytes))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_script_hash_be() {
        let hash = "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";
        let result = validate_script_hash(hash, Some(GetUtxoStatusOutputFormat::HashBE)).unwrap();
        assert_eq!(result, hash);
    }
    
    #[test]
    fn test_validate_script_hash_le() {
        let hash_le = "90785634ef12cdab90785634ef12cdab90785634ef12cdab90785634ef12cdab";
        let result = validate_script_hash(hash_le, Some(GetUtxoStatusOutputFormat::HashLE)).unwrap();
        // Should be reversed
        assert_eq!(result, "abcd12ef34567890abcd12ef34567890abcd12ef34567890abcd12ef34567890");
    }
    
    #[test]
    fn test_validate_script_hash_auto_detect_32bytes() {
        let hash = "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";
        // Auto-detect should treat 32 bytes as HashLE
        let result = validate_script_hash(hash, None).unwrap();
        assert_eq!(result.len(), 64); // 32 bytes = 64 hex chars
    }
    
    #[test]
    fn test_validate_script_hash_from_script() {
        let script = "76a914";
        let result = validate_script_hash(script, Some(GetUtxoStatusOutputFormat::Script)).unwrap();
        // Should be SHA256 hash, reversed
        assert_eq!(result.len(), 64);
    }
}
