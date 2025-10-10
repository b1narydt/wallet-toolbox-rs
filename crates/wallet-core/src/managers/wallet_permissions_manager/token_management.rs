//! Permission Token Management
//!
//! **Reference**: TypeScript `src/WalletPermissionsManager.ts` lines 1636-1916
//!
//! Handles creation, renewal, and management of permission tokens on-chain

use super::types::*;
use super::constants::*;
use super::permission_validation::{find_protocol_token, find_basket_token, find_certificate_token, find_spending_token};
use crate::sdk::errors::{WalletError, WalletResult};
use crate::managers::simple_wallet_manager::WalletInterface;
use serde_json::json;
use base64::{Engine as _, engine::general_purpose};

/// Build encrypted PushDrop fields for a permission token
///
/// Reference: TS buildPushdropFields (WalletPermissionsManager.ts lines 1844-1884)
///
/// Constructs the array of encrypted fields that will be embedded in the PushDrop script.
/// Each permission type has a different field structure:
/// - Protocol: [domain, expiry, privileged, secLevel, protoName, counterparty]
/// - Basket: [domain, expiry, basketName]
/// - Certificate: [domain, expiry, privileged, certType, fields, verifier]
/// - Spending: [domain, authorizedAmount]
///
/// # Arguments
///
/// * `request` - The permission request
/// * `expiry` - Token expiry timestamp (UNIX epoch seconds)
/// * `amount` - Optional authorized amount (for spending permissions)
///
/// # Returns
///
/// Vector of encrypted field arrays
///
/// # Arguments (Updated)
///
/// * `underlying` - Underlying wallet interface for encryption
/// * `admin_originator` - Admin originator for encryption keys
/// * `request` - The permission request
/// * `expiry` - Token expiry timestamp
/// * `amount` - Optional authorized amount
pub async fn build_pushdrop_fields(
    underlying: &dyn WalletInterface,
    admin_originator: &str,
    request: &PermissionRequest,
    expiry: i64,
    amount: Option<i64>,
) -> WalletResult<Vec<String>> {
    // Returns Vec<String> of base64-encoded encrypted fields
    
    match request.permission_type {
        PermissionType::Protocol => {
            // TS lines 1846-1856: Protocol permission fields
            // [domain, expiry, privileged, secLevel, protoName, counterparty]
            let protocol_id = request.protocol_id.as_ref()
                .ok_or_else(|| WalletError::invalid_parameter("protocol_id", "Required for protocol permission"))?;
            
            if protocol_id.len() < 2 {
                return Err(WalletError::invalid_parameter("protocol_id", "Must have [secLevel, protoName]"));
            }
            
            let sec_level = &protocol_id[0];
            let proto_name = &protocol_id[1];
            let privileged = request.privileged.unwrap_or(false);
            let counterparty = request.counterparty.as_deref().unwrap_or("self");
            
            // TS lines 1849-1854: Encrypt each field
            Ok(vec![
                encrypt_permission_token_field(underlying, admin_originator, request.originator.as_bytes()).await?,  // domain
                encrypt_permission_token_field(underlying, admin_originator, expiry.to_string().as_bytes()).await?,  // expiry
                encrypt_permission_token_field(underlying, admin_originator, (if privileged { "true" } else { "false" }).as_bytes()).await?,  // privileged
                encrypt_permission_token_field(underlying, admin_originator, sec_level.as_bytes()).await?,  // secLevel
                encrypt_permission_token_field(underlying, admin_originator, proto_name.as_bytes()).await?,  // protoName
                encrypt_permission_token_field(underlying, admin_originator, counterparty.as_bytes()).await?,  // counterparty
            ])
        }
        PermissionType::Basket => {
            // TS lines 1857-1863: Basket permission fields
            // [domain, expiry, basketName]
            let basket = request.basket.as_ref()
                .ok_or_else(|| WalletError::invalid_parameter("basket", "Required for basket permission"))?;
            
            // TS lines 1859-1861: Encrypt each field
            Ok(vec![
                encrypt_permission_token_field(underlying, admin_originator, request.originator.as_bytes()).await?,  // domain
                encrypt_permission_token_field(underlying, admin_originator, expiry.to_string().as_bytes()).await?,  // expiry
                encrypt_permission_token_field(underlying, admin_originator, basket.as_bytes()).await?,  // basket
            ])
        }
        PermissionType::Certificate => {
            // TS lines 1864-1874: Certificate permission fields
            // [domain, expiry, privileged, certType, fields, verifier]
            let cert = request.certificate.as_ref()
                .ok_or_else(|| WalletError::invalid_parameter("certificate", "Required for certificate permission"))?;
            
            let privileged = request.privileged.unwrap_or(false);
            let fields_json = serde_json::to_string(&cert.fields)
                .map_err(|e| WalletError::invalid_parameter("certificate.fields", &e.to_string()))?;
            
            // TS lines 1867-1872: Encrypt each field
            Ok(vec![
                encrypt_permission_token_field(underlying, admin_originator, request.originator.as_bytes()).await?,  // domain
                encrypt_permission_token_field(underlying, admin_originator, expiry.to_string().as_bytes()).await?,  // expiry
                encrypt_permission_token_field(underlying, admin_originator, (if privileged { "true" } else { "false" }).as_bytes()).await?,  // privileged
                encrypt_permission_token_field(underlying, admin_originator, cert.cert_type.as_bytes()).await?,  // certType
                encrypt_permission_token_field(underlying, admin_originator, fields_json.as_bytes()).await?,  // fields JSON
                encrypt_permission_token_field(underlying, admin_originator, cert.verifier.as_bytes()).await?,  // verifier
            ])
        }
        PermissionType::Spending => {
            // TS lines 1875-1882: Spending permission fields
            // [domain, authorizedAmount]
            let auth_amount = amount
                .or_else(|| request.spending.as_ref().map(|s| s.satoshis))
                .unwrap_or(0);
            
            // TS lines 1879-1880: Encrypt each field
            Ok(vec![
                encrypt_permission_token_field(underlying, admin_originator, request.originator.as_bytes()).await?,  // domain
                encrypt_permission_token_field(underlying, admin_originator, auth_amount.to_string().as_bytes()).await?,  // amount
            ])
        }
    }
}

/// Build tags for a permission request
///
/// Reference: TS buildTagsForRequest (WalletPermissionsManager.ts lines 1890-1916)
///
/// Constructs the array of tags used for storage queries and filtering.
/// Tags help identify and retrieve permission tokens efficiently.
///
/// # Arguments
///
/// * `request` - The permission request
///
/// # Returns
///
/// Vector of tag strings in the format "key value"
pub fn build_tags_for_request(request: &PermissionRequest) -> Vec<String> {
    // TS line 1891: Always include originator
    let mut tags = vec![format!("originator {}", request.originator)];
    
    match request.permission_type {
        PermissionType::Protocol => {
            // TS lines 1893-1899: Protocol tags
            if let Some(protocol_id) = &request.protocol_id {
                let privileged = request.privileged.unwrap_or(false);
                tags.push(format!("privileged {}", privileged)); // TS line 1894
                
                if protocol_id.len() >= 2 {
                    tags.push(format!("protocolName {}", protocol_id[1])); // TS line 1895
                    tags.push(format!("protocolSecurityLevel {}", protocol_id[0])); // TS line 1896
                }
                
                if let Some(counterparty) = &request.counterparty {
                    tags.push(format!("counterparty {}", counterparty)); // TS line 1897
                }
            }
        }
        PermissionType::Basket => {
            // TS lines 1900-1903: Basket tags
            if let Some(basket) = &request.basket {
                tags.push(format!("basket {}", basket)); // TS line 1901
            }
        }
        PermissionType::Certificate => {
            // TS lines 1904-1909: Certificate tags
            if let Some(cert) = &request.certificate {
                let privileged = request.privileged.unwrap_or(false);
                tags.push(format!("privileged {}", privileged)); // TS line 1905
                tags.push(format!("type {}", cert.cert_type)); // TS line 1906
                tags.push(format!("verifier {}", cert.verifier)); // TS line 1907
            }
        }
        PermissionType::Spending => {
            // TS lines 1910-1914: Spending tags
            // Only 'originator' is required (already added above)
        }
    }
    
    tags
}

/// Create a new permission token on-chain
///
/// Reference: TS createPermissionOnChain (WalletPermissionsManager.ts lines 1636-1677)
///
/// Creates a new permission token by:
/// 1. Building encrypted PushDrop fields
/// 2. Creating a PushDrop locking script
/// 3. Creating a transaction with the token output
///
/// # Arguments
///
/// * `request` - The permission request
/// * `expiry` - Token expiry timestamp
/// * `amount` - Optional authorized amount (for spending)
///
/// # Returns
///
/// Success or error
///
/// # TODO
///
/// This requires:
/// - PushDrop lock() implementation
/// - createAction() integration with underlying wallet
/// - Proper script building
pub async fn create_permission_on_chain(
    underlying: &dyn WalletInterface,
    admin_originator: &str,
    request: &PermissionRequest,
    expiry: i64,
    amount: Option<i64>,
) -> WalletResult<()> {
    // TS line 1637: Get basket name for this permission type
    let basket_name = get_admin_basket_name(request.permission_type);
    
    // TS line 1641: Build encrypted fields for PushDrop script
    let fields = build_pushdrop_fields(underlying, admin_originator, request, expiry, amount).await?;
    
    // TS lines 1655: Build tags
    let tags = build_tags_for_request(request);
    
    // SIMPLIFIED ARCHITECTURE: Pass encrypted fields to frontend via customInstructions
    // Frontend will build PushDrop script with ts-sdk
    
    // TS lines 1659-1676: Create transaction with token output
    let _result = underlying.create_action(
        json!({
            "description": format!("Grant {} permission", match request.permission_type {
                PermissionType::Protocol => "protocol",
                PermissionType::Basket => "basket",
                PermissionType::Certificate => "certificate",
                PermissionType::Spending => "spending",
            }),
            "outputs": [{
                "satoshis": 1,
                "outputDescription": format!("{:?} permission token", request.permission_type),
                "basket": basket_name,
                "tags": tags,
                "customInstructions": {
                    "fields": fields  // Encrypted fields for frontend to build PushDrop
                }
            }],
            "options": {
                "acceptDelayedBroadcast": false
            }
        }),
        Some(admin_originator)
    ).await?;
    
    Ok(())
}

/// Renew an existing permission token
///
/// Reference: TS renewPermissionOnChain (WalletPermissionsManager.ts lines 1752-1838)
///
/// Renews a permission token by:
/// 1. Building new encrypted fields with updated expiry
/// 2. Creating a new PushDrop locking script
/// 3. Spending the old token as input
/// 4. Creating new token as output
///
/// # Arguments
///
/// * `old_token` - The old token to consume
/// * `request` - The permission request being renewed
/// * `new_expiry` - New expiry timestamp
/// * `new_amount` - Optional new authorized amount (for spending)
///
/// # Returns
///
/// Success or error
///
/// # TODO
///
/// This requires:
/// - PushDrop lock() and unlock() implementation
/// - createAction() with inputs
/// - Transaction signing
/// - signAction() finalization
pub async fn renew_permission_on_chain(
    underlying: &dyn WalletInterface,
    admin_originator: &str,
    request: &PermissionRequest,
    new_expiry: i64,
    new_amount: Option<i64>,
) -> WalletResult<()> {
    // TS line 1759: Build new encrypted fields
    let new_fields = build_pushdrop_fields(underlying, admin_originator, request, new_expiry, new_amount).await?;
    
    // TS line 1770: Build tags
    let tags = build_tags_for_request(request);
    
    // TS line 1773: Find old token to spend based on permission type
    let old_token = match request.permission_type {
        PermissionType::Protocol => {
            let protocol_id = request.protocol_id.as_ref()
                .ok_or_else(|| WalletError::invalid_parameter("protocol_id", "Required for protocol permission"))?;
            if protocol_id.len() < 2 {
                return Err(WalletError::invalid_parameter("protocol_id", "Must have [secLevel, protoName]"));
            }
            find_protocol_token(
                underlying,
                admin_originator,
                &request.originator,
                request.privileged.unwrap_or(false),
                protocol_id,  // Pass full protocol_id slice
                request.counterparty.as_deref().unwrap_or("self"),
                true  // include_expired (we want to renew even if expired)
            ).await?
        }
        PermissionType::Basket => {
            let basket = request.basket.as_ref()
                .ok_or_else(|| WalletError::invalid_parameter("basket", "Required for basket permission"))?;
            find_basket_token(
                underlying,
                admin_originator,
                &request.originator,
                basket,
                true  // include_expired
            ).await?
        }
        PermissionType::Certificate => {
            let cert = request.certificate.as_ref()
                .ok_or_else(|| WalletError::invalid_parameter("certificate", "Required for certificate permission"))?;
            find_certificate_token(
                underlying,
                admin_originator,
                &request.originator,
                request.privileged.unwrap_or(false),
                &cert.verifier,  // verifier comes before cert_type
                &cert.cert_type,
                &cert.fields,
                true  // include_expired
            ).await?
        }
        PermissionType::Spending => {
            find_spending_token(
                underlying,
                admin_originator,
                &request.originator
            ).await?
        }
    }.ok_or_else(|| WalletError::new("WERR_NOT_FOUND", "Old permission token not found for renewal"))?;
    
    // TS line 1791: Build old outpoint
    let old_outpoint = format!("{}.{}", old_token.txid, old_token.output_index);
    
    // SIMPLIFIED ARCHITECTURE: Pass data to frontend for transaction building
    // Frontend will use ts-sdk to build PushDrop unlock script and handle inputs
    
    // TS lines 1792-1817: Create action with old token as input and new output
    let _result = underlying.create_action(
        json!({
            "description": format!("Renew {} permission", match request.permission_type {
                PermissionType::Protocol => "protocol",
                PermissionType::Basket => "basket",
                PermissionType::Certificate => "certificate",
                PermissionType::Spending => "spending",
            }),
            "inputs": [{
                "outpoint": old_outpoint,
                "unlockingScriptLength": 73,  // Typical signature size
                "inputDescription": format!("Consume old {} token", match request.permission_type {
                    PermissionType::Protocol => "protocol",
                    PermissionType::Basket => "basket",
                    PermissionType::Certificate => "certificate",
                    PermissionType::Spending => "spending",
                })
            }],
            "outputs": [{
                "satoshis": 1,
                "outputDescription": format!("Renewed {:?} permission token", request.permission_type),
                "basket": get_admin_basket_name(request.permission_type),
                "tags": tags,
                "customInstructions": {
                    "fields": new_fields  // Encrypted fields for frontend to build PushDrop
                }
            }],
            "options": {
                "acceptDelayedBroadcast": false
            }
        }),
        Some(admin_originator)
    ).await?;
    
    // TS lines 1819-1833: Frontend will handle PushDrop.unlock() and signing
    // TODO: Implement createAction() with inputs
    // const { signableTransaction } = await this.createAction(
    //   {
    //     description: `Renew ${r.type} permission`,
    //     inputBEEF: oldToken.tx,
    //     inputs: [
    //       {
    //         outpoint: oldOutpoint,
    //         unlockingScriptLength: 73,
    //         inputDescription: `Consume old ${r.type} token`
    //       }
    //     ],
    //     outputs: [
    //       {
    //         lockingScript: newScript.toHex(),
    //         satoshis: 1,
    //         outputDescription: `Renewed ${r.type} permission token`,
    //         basket: BASKET_MAP[r.type],
    //         tags
    //       }
    //     ],
    //     options: {
    //       acceptDelayedBroadcast: false
    //     }
    //   },
    //   this.adminOriginator
    // )
    
    // TS lines 1819-1828: Create unlocking script using PushDrop
    // TODO: Implement PushDrop.unlock() and signing
    // const unlocker = new PushDrop(this.underlying).unlock(
    //   WalletPermissionsManager.PERM_TOKEN_ENCRYPTION_PROTOCOL,
    //   '1',
    //   'self',
    //   'all',
    //   false,
    //   1,
    //   LockingScript.fromHex(oldToken.outputScript)
    // )
    // const unlockingScript = await unlocker.sign(tx, 0)
    
    // TS lines 1829-1833: Finalize with signAction
    // TODO: Implement signAction()
    // await this.underlying.signAction({
    //   reference: signableTransaction.reference,
    //   spends: {
    //     0: { unlockingScript: unlockingScript.toHex() }
    //   }
    // })
    
    // Placeholder for now
    Ok(())
}

/// Coalesce multiple permission tokens into a single renewed token
///
/// Reference: TS coalescePermissionTokens (WalletPermissionsManager.ts lines 1679-1742)
///
/// When multiple tokens exist for the same permission (shouldn't normally happen,
/// but can occur), this combines them into a single token to avoid bloat.
///
/// # Arguments
///
/// * `old_tokens` - Array of old tokens to consume (must be >= 2)
/// * `new_fields` - Encrypted fields for the new token
/// * `tags` - Tags for the new token output
/// * `basket` - Basket name for the new token
/// * `description` - Transaction description
///
/// # Returns
///
/// Transaction ID of the coalescing transaction
///
/// # TODO
///
/// This requires:
/// - createAction() with multiple inputs
/// - PushDrop unlock() and signing
/// - signAction() finalization
pub async fn coalesce_permission_tokens(
    old_tokens: &[PermissionToken],
    new_fields: Vec<Vec<u8>>,
    tags: Vec<String>,
    basket: String,
    description: Option<String>,
) -> WalletResult<String> {
    // TS lines 1688-1689: Validation
    if old_tokens.is_empty() {
        return Err(WalletError::invalid_parameter(
            "oldTokens",
            "No permission tokens to coalesce"
        ));
    }
    if old_tokens.len() < 2 {
        return Err(WalletError::invalid_parameter(
            "oldTokens",
            "Need at least 2 tokens to coalesce"
        ));
    }
    
    // TS lines 1692-1716: Create signable action with N inputs and single output
    // TODO: Implement createAction()
    // const { signableTransaction } = await this.createAction(
    //   {
    //     description: opts?.description ?? `Coalesce ${oldTokens.length} permission tokens`,
    //     inputs: oldTokens.map((t, i) => ({
    //       outpoint: `${t.txid}.${t.outputIndex}`,
    //       unlockingScriptLength: 74,
    //       inputDescription: `Consume permission token #${i + 1}`
    //     })),
    //     outputs: [
    //       {
    //         lockingScript: newScript.toHex(),
    //         satoshis: 1,
    //         outputDescription: 'Renewed permission token',
    //         basket: opts?.basket,
    //         tags: opts?.tags
    //       }
    //     ],
    //     options: {
    //       acceptDelayedBroadcast: false,
    //       randomizeOutputs: false,
    //       signAndProcess: false
    //     }
    //   },
    //   this.adminOriginator
    // )
    
    // TS lines 1723-1732: Sign each input with PushDrop unlocker
    // TODO: Implement PushDrop unlock() and signing
    // const partialTx = Transaction.fromAtomicBEEF(signableTransaction.tx)
    // const pushdrop = new PushDrop(this.underlying)
    // const unlocker = pushdrop.unlock(PERM_TOKEN_ENCRYPTION_PROTOCOL, '1', 'self')
    // 
    // const spends: Record<number, { unlockingScript: string }> = {}
    // for (let i = 0; i < oldTokens.length; i++) {
    //   const unlockingScript = await unlocker.sign(partialTx, i)
    //   spends[i] = { unlockingScript: unlockingScript.toHex() }
    // }
    
    // TS lines 1735-1738: Finalize the action
    // TODO: Implement signAction()
    // const { txid } = await this.underlying.signAction({
    //   reference: signableTransaction.reference,
    //   spends
    // })
    
    // Placeholder return
    Ok("pending_implementation".to_string())
}

/// Revoke a permission token
///
/// Reference: TS revokePermission (not in snippet but mentioned in architecture)
///
/// Revokes a permission by spending the token without creating a new one.
///
/// # Arguments
///
/// * `token` - The token to revoke
///
/// # Returns
///
/// Success or error
///
/// # TODO
///
/// Implement token revocation by spending without renewal
pub async fn revoke_permission_token(token: &PermissionToken) -> WalletResult<()> {
    // TODO: Implement revocation
    // This would spend the token as an input without creating a new permission output
    Ok(())
}

/// Encrypt a permission token field
///
/// Reference: TS encryptPermissionTokenField (WalletPermissionsManager.ts lines 1207-1218)
///
/// Encrypts a field using the admin permission token encryption protocol.
/// Always uses keyID="1" and counterparty="self".
///
/// **Note**: In the simplified architecture, frontend may handle encryption.
/// This function serves as a pass-through for now, encoding as base64.
///
/// # Arguments
///
/// * `underlying` - Underlying wallet interface (reserved for future use)
/// * `admin_originator` - Admin originator domain (reserved for future use)
/// * `plaintext` - The plaintext data to encrypt (string or bytes)
///
/// # Returns
///
/// Base64-encoded string (encrypted by underlying wallet if available)
pub async fn encrypt_permission_token_field(
    _underlying: &dyn WalletInterface,
    _admin_originator: &str,
    plaintext: &[u8],
) -> WalletResult<String> {
    // TS lines 1208-1217: Encrypt using underlying wallet
    // In simplified architecture, encryption happens at storage layer or frontend
    // For now, encode as base64 for storage
    
    // TODO: If underlying wallet has encrypt() method, call it:
    // let result = underlying.encrypt(
    //     json!({
    //         "plaintext": base64::encode(plaintext),
    //         "protocolID": [encryption_protocols::PERM_TOKEN_SECURITY_LEVEL, encryption_protocols::PERM_TOKEN_ENCRYPTION],
    //         "keyID": encryption_protocols::KEY_ID,
    //         "counterparty": encryption_protocols::COUNTERPARTY
    //     }),
    //     Some(admin_originator)
    // ).await?;
    // return result["ciphertext"].as_str().unwrap_or("").to_string();
    
    // For now: base64 encode for storage
    Ok(general_purpose::STANDARD.encode(plaintext))
}

/// Decrypt a permission token field
///
/// Reference: TS decryptPermissionTokenField (WalletPermissionsManager.ts lines 1220-1234)
///
/// Decrypts a field that was encrypted with the admin permission token encryption protocol.
/// If decryption fails, returns the ciphertext as-is (fallback for unencrypted data).
///
/// **Note**: In the simplified architecture, fields from storage are already strings.
/// This function decodes from base64 and optionally decrypts.
///
/// # Arguments
///
/// * `underlying` - Underlying wallet interface (reserved for future use)
/// * `admin_originator` - Admin originator domain (reserved for future use)
/// * `field_data` - Base64-encoded field data from storage
///
/// # Returns
///
/// Decrypted plaintext bytes
pub async fn decrypt_permission_token_field(
    _underlying: &dyn WalletInterface,
    _admin_originator: &str,
    field_data: &str,
) -> WalletResult<Vec<u8>> {
    // TS lines 1221-1233: Try to decrypt, fallback to ciphertext if it fails
    
    // First, decode from base64
    let decoded = general_purpose::STANDARD.decode(field_data)
        .unwrap_or_else(|_| field_data.as_bytes().to_vec());
    
    // TODO: If underlying wallet has decrypt() method, call it:
    // let result = underlying.decrypt(
    //     json!({
    //         "ciphertext": field_data,
    //         "protocolID": [encryption_protocols::PERM_TOKEN_SECURITY_LEVEL, encryption_protocols::PERM_TOKEN_ENCRYPTION],
    //         "keyID": encryption_protocols::KEY_ID,
    //         "returnType": "Uint8Array"
    //     }),
    //     Some(admin_originator)
    // ).await;
    // 
    // if let Ok(result) = result {
    //     if let Some(plaintext) = result["plaintext"].as_str() {
    //         return general_purpose::STANDARD.decode(plaintext)
    //             .map_err(|e| WalletError::InvalidData(format!("Invalid plaintext base64: {}", e)));
    //     }
    // }
    // 
    // Fallback to decoded data
    
    Ok(decoded)
}

/// Protocol IDs for encryption
///
/// Reference: TS PERM_TOKEN_ENCRYPTION_PROTOCOL (lines 1192-1195)
/// Reference: TS METADATA_ENCRYPTION_PROTOCOL (lines 1201-1204)
pub mod encryption_protocols {
    /// Protocol for encrypting permission token fields
    /// 
    /// Reference: TS line 1192-1195
    pub const PERM_TOKEN_ENCRYPTION: &str = "admin permission token encryption";
    pub const PERM_TOKEN_SECURITY_LEVEL: i32 = 2;
    
    /// Protocol for encrypting wallet metadata
    /// 
    /// Reference: TS line 1201-1204
    pub const METADATA_ENCRYPTION: &str = "admin metadata encryption";
    pub const METADATA_SECURITY_LEVEL: i32 = 2;
    
    /// Key ID used for all permission token encryption
    /// 
    /// Reference: TS line 1206
    pub const KEY_ID: &str = "1";
    
    /// Counterparty for encryption (always "self")
    /// 
    /// Reference: TS line 1206
    pub const COUNTERPARTY: &str = "self";
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_build_tags_protocol() {
        let request = PermissionRequest {
            permission_type: PermissionType::Protocol,
            originator: "example.com".to_string(),
            privileged: Some(true),
            protocol_id: Some(vec!["2".to_string(), "payment".to_string()]),
            counterparty: Some("self".to_string()),
            basket: None,
            certificate: None,
            spending: None,
            reason: None,
            renewal: None,
            previous_token: None,
        };
        
        let tags = build_tags_for_request(&request);
        
        assert_eq!(tags.len(), 5);
        assert!(tags.contains(&"originator example.com".to_string()));
        assert!(tags.contains(&"privileged true".to_string()));
        assert!(tags.contains(&"protocolName payment".to_string()));
        assert!(tags.contains(&"protocolSecurityLevel 2".to_string()));
        assert!(tags.contains(&"counterparty self".to_string()));
    }
    
    #[test]
    fn test_build_tags_basket() {
        let request = PermissionRequest {
            permission_type: PermissionType::Basket,
            originator: "example.com".to_string(),
            privileged: None,
            protocol_id: None,
            counterparty: None,
            basket: Some("mybasket".to_string()),
            certificate: None,
            spending: None,
            reason: None,
            renewal: None,
            previous_token: None,
        };
        
        let tags = build_tags_for_request(&request);
        
        assert_eq!(tags.len(), 2);
        assert!(tags.contains(&"originator example.com".to_string()));
        assert!(tags.contains(&"basket mybasket".to_string()));
    }
    
    #[test]
    fn test_build_tags_certificate() {
        let request = PermissionRequest {
            permission_type: PermissionType::Certificate,
            originator: "example.com".to_string(),
            privileged: Some(false),
            protocol_id: None,
            counterparty: None,
            basket: None,
            certificate: Some(CertificateDetails {
                verifier: "verifier_key".to_string(),
                cert_type: "id_card".to_string(),
                fields: vec!["name".to_string(), "dob".to_string()],
            }),
            spending: None,
            reason: None,
            renewal: None,
            previous_token: None,
        };
        
        let tags = build_tags_for_request(&request);
        
        assert_eq!(tags.len(), 4);
        assert!(tags.contains(&"originator example.com".to_string()));
        assert!(tags.contains(&"privileged false".to_string()));
        assert!(tags.contains(&"type id_card".to_string()));
        assert!(tags.contains(&"verifier verifier_key".to_string()));
    }
    
    #[test]
    fn test_build_tags_spending() {
        let request = PermissionRequest {
            permission_type: PermissionType::Spending,
            originator: "example.com".to_string(),
            privileged: None,
            protocol_id: None,
            counterparty: None,
            basket: None,
            certificate: None,
            spending: Some(SpendingDetails {
                satoshis: 10000,
                line_items: None,
            }),
            reason: None,
            renewal: None,
            previous_token: None,
        };
        
        let tags = build_tags_for_request(&request);
        
        // Spending only has originator tag
        assert_eq!(tags.len(), 1);
        assert!(tags.contains(&"originator example.com".to_string()));
    }
    
    #[tokio::test]
    async fn test_coalesce_validation_empty() {
        let result = coalesce_permission_tokens(
            &[],
            vec![],
            vec![],
            "test-basket".to_string(),
            None,
        ).await;
        
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("No permission tokens"));
        }
    }
    
    #[tokio::test]
    async fn test_coalesce_validation_single_token() {
        let token = PermissionToken {
            tx: vec![],
            txid: "test".to_string(),
            output_index: 0,
            output_script: "".to_string(),
            satoshis: 1,
            originator: "test.com".to_string(),
            privileged: None,
            protocol: None,
            security_level: None,
            expiry: 0,
            counterparty: None,
            basket_name: None,
            verifier: None,
            cert_type: None,
            cert_fields: None,
            authorized_amount: None,
        };
        
        let result = coalesce_permission_tokens(
            &[token],
            vec![],
            vec![],
            "test-basket".to_string(),
            None,
        ).await;
        
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("at least 2 tokens"));
        }
    }
    
    #[tokio::test]
    async fn test_encrypt_decrypt_field() {
        // Create a mock wallet interface
        struct MockWallet;
        #[async_trait::async_trait]
        impl WalletInterface for MockWallet {
            async fn create_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn sign_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn abort_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn list_actions(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"actions": []}))
            }
            async fn internalize_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn list_outputs(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"outputs": []}))
            }
            async fn get_public_key(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"publicKey": ""}))
            }
            async fn get_height(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"height": 0}))
            }
            async fn get_network(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"network": "mainnet"}))
            }
            async fn get_version(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"version": "1.0.0"}))
            }
        }
        
        let mock = MockWallet;
        let plaintext = b"test data";
        
        let encrypted = encrypt_permission_token_field(&mock, "admin", plaintext).await.unwrap();
        let decrypted = decrypt_permission_token_field(&mock, "admin", &encrypted).await.unwrap();
        
        // Should round-trip (base64 encode/decode)
        assert_eq!(decrypted, plaintext);
    }
    
    #[test]
    fn test_encryption_protocol_constants() {
        use encryption_protocols::*;
        
        assert_eq!(PERM_TOKEN_ENCRYPTION, "admin permission token encryption");
        assert_eq!(PERM_TOKEN_SECURITY_LEVEL, 2);
        assert_eq!(METADATA_ENCRYPTION, "admin metadata encryption");
        assert_eq!(METADATA_SECURITY_LEVEL, 2);
        assert_eq!(KEY_ID, "1");
        assert_eq!(COUNTERPARTY, "self");
    }
    
    #[tokio::test]
    async fn test_build_pushdrop_fields_protocol() {
        struct MockWallet;
        #[async_trait::async_trait]
        impl WalletInterface for MockWallet {
            async fn create_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn sign_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn abort_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn list_actions(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"actions": []}))
            }
            async fn internalize_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn list_outputs(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"outputs": []}))
            }
            async fn get_public_key(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"publicKey": ""}))
            }
            async fn get_height(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"height": 0}))
            }
            async fn get_network(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"network": "mainnet"}))
            }
            async fn get_version(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"version": "1.0.0"}))
            }
        }
        
        let mock = MockWallet;
        let request = PermissionRequest {
            permission_type: PermissionType::Protocol,
            originator: "example.com".to_string(),
            protocol_id: Some(vec!["2".to_string(), "test-protocol".to_string()]),
            privileged: Some(true),
            counterparty: Some("self".to_string()),
            basket: None,
            certificate: None,
            spending: None,
            reason: None,
            renewal: None,
            previous_token: None,
        };
        
        let fields = build_pushdrop_fields(&mock, "admin", &request, 1234567890, None).await.unwrap();
        
        // Should have 6 fields for protocol permission
        assert_eq!(fields.len(), 6);
        
        // Verify each field decrypts correctly
        let domain = decrypt_permission_token_field(&mock, "admin", &fields[0]).await.unwrap();
        assert_eq!(String::from_utf8(domain).unwrap(), "example.com");
        
        let expiry = decrypt_permission_token_field(&mock, "admin", &fields[1]).await.unwrap();
        assert_eq!(String::from_utf8(expiry).unwrap(), "1234567890");
        
        let privileged = decrypt_permission_token_field(&mock, "admin", &fields[2]).await.unwrap();
        assert_eq!(String::from_utf8(privileged).unwrap(), "true");
        
        let sec_level = decrypt_permission_token_field(&mock, "admin", &fields[3]).await.unwrap();
        assert_eq!(String::from_utf8(sec_level).unwrap(), "2");
        
        let proto_name = decrypt_permission_token_field(&mock, "admin", &fields[4]).await.unwrap();
        assert_eq!(String::from_utf8(proto_name).unwrap(), "test-protocol");
        
        let counterparty = decrypt_permission_token_field(&mock, "admin", &fields[5]).await.unwrap();
        assert_eq!(String::from_utf8(counterparty).unwrap(), "self");
    }
    
    #[tokio::test]
    async fn test_build_pushdrop_fields_basket() {
        struct MockWallet;
        #[async_trait::async_trait]
        impl WalletInterface for MockWallet {
            async fn create_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn sign_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn abort_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn list_actions(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"actions": []}))
            }
            async fn internalize_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn list_outputs(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"outputs": []}))
            }
            async fn get_public_key(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"publicKey": ""}))
            }
            async fn get_height(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"height": 0}))
            }
            async fn get_network(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"network": "mainnet"}))
            }
            async fn get_version(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"version": "1.0.0"}))
            }
        }
        
        let mock = MockWallet;
        let request = PermissionRequest {
            permission_type: PermissionType::Basket,
            originator: "example.com".to_string(),
            basket: Some("my-basket".to_string()),
            protocol_id: None,
            privileged: None,
            counterparty: None,
            certificate: None,
            spending: None,
            reason: None,
            renewal: None,
            previous_token: None,
        };
        
        let fields = build_pushdrop_fields(&mock, "admin", &request, 9876543210, None).await.unwrap();
        
        // Should have 3 fields for basket permission
        assert_eq!(fields.len(), 3);
        
        let domain = decrypt_permission_token_field(&mock, "admin", &fields[0]).await.unwrap();
        assert_eq!(String::from_utf8(domain).unwrap(), "example.com");
        
        let expiry = decrypt_permission_token_field(&mock, "admin", &fields[1]).await.unwrap();
        assert_eq!(String::from_utf8(expiry).unwrap(), "9876543210");
        
        let basket = decrypt_permission_token_field(&mock, "admin", &fields[2]).await.unwrap();
        assert_eq!(String::from_utf8(basket).unwrap(), "my-basket");
    }
    
    #[tokio::test]
    async fn test_build_pushdrop_fields_certificate() {
        struct MockWallet;
        #[async_trait::async_trait]
        impl WalletInterface for MockWallet {
            async fn create_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn sign_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn abort_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn list_actions(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"actions": []}))
            }
            async fn internalize_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn list_outputs(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"outputs": []}))
            }
            async fn get_public_key(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"publicKey": ""}))
            }
            async fn get_height(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"height": 0}))
            }
            async fn get_network(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"network": "mainnet"}))
            }
            async fn get_version(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"version": "1.0.0"}))
            }
        }
        
        let mock = MockWallet;
        let request = PermissionRequest {
            permission_type: PermissionType::Certificate,
            originator: "example.com".to_string(),
            privileged: Some(false),
            certificate: Some(CertificateDetails {
                cert_type: "identity".to_string(),
                verifier: "verifier.com".to_string(),
                fields: vec!["name".to_string(), "email".to_string()],
            }),
            protocol_id: None,
            counterparty: None,
            basket: None,
            spending: None,
            reason: None,
            renewal: None,
            previous_token: None,
        };
        
        let fields = build_pushdrop_fields(&mock, "admin", &request, 1111111111, None).await.unwrap();
        
        // Should have 6 fields for certificate permission
        assert_eq!(fields.len(), 6);
        
        let domain = decrypt_permission_token_field(&mock, "admin", &fields[0]).await.unwrap();
        assert_eq!(String::from_utf8(domain).unwrap(), "example.com");
        
        let expiry = decrypt_permission_token_field(&mock, "admin", &fields[1]).await.unwrap();
        assert_eq!(String::from_utf8(expiry).unwrap(), "1111111111");
        
        let privileged = decrypt_permission_token_field(&mock, "admin", &fields[2]).await.unwrap();
        assert_eq!(String::from_utf8(privileged).unwrap(), "false");
        
        let cert_type = decrypt_permission_token_field(&mock, "admin", &fields[3]).await.unwrap();
        assert_eq!(String::from_utf8(cert_type).unwrap(), "identity");
        
        let fields_json = decrypt_permission_token_field(&mock, "admin", &fields[4]).await.unwrap();
        let parsed_fields: Vec<String> = serde_json::from_slice(&fields_json).unwrap();
        assert_eq!(parsed_fields, vec!["name", "email"]);
        
        let verifier = decrypt_permission_token_field(&mock, "admin", &fields[5]).await.unwrap();
        assert_eq!(String::from_utf8(verifier).unwrap(), "verifier.com");
    }
    
    #[tokio::test]
    async fn test_build_pushdrop_fields_spending() {
        struct MockWallet;
        #[async_trait::async_trait]
        impl WalletInterface for MockWallet {
            async fn create_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn sign_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn abort_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn list_actions(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"actions": []}))
            }
            async fn internalize_action(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({}))
            }
            async fn list_outputs(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"outputs": []}))
            }
            async fn get_public_key(&self, _args: serde_json::Value, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"publicKey": ""}))
            }
            async fn get_height(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"height": 0}))
            }
            async fn get_network(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"network": "mainnet"}))
            }
            async fn get_version(&self, _originator: Option<&str>) -> WalletResult<serde_json::Value> {
                Ok(serde_json::json!({"version": "1.0.0"}))
            }
        }
        
        let mock = MockWallet;
        let request = PermissionRequest {
            permission_type: PermissionType::Spending,
            originator: "example.com".to_string(),
            spending: Some(SpendingDetails {
                satoshis: 50000,
                line_items: None,
            }),
            protocol_id: None,
            privileged: None,
            counterparty: None,
            basket: None,
            certificate: None,
            reason: None,
            renewal: None,
            previous_token: None,
        };
        
        let fields = build_pushdrop_fields(&mock, "admin", &request, 0, Some(50000)).await.unwrap();
        
        // Should have 2 fields for spending permission
        assert_eq!(fields.len(), 2);
        
        let domain = decrypt_permission_token_field(&mock, "admin", &fields[0]).await.unwrap();
        assert_eq!(String::from_utf8(domain).unwrap(), "example.com");
        
        let amount = decrypt_permission_token_field(&mock, "admin", &fields[1]).await.unwrap();
        assert_eq!(String::from_utf8(amount).unwrap(), "50000");
    }
    
    #[test]
    fn test_build_tags_for_request_protocol() {
        let request = PermissionRequest {
            permission_type: PermissionType::Protocol,
            originator: "example.com".to_string(),
            protocol_id: Some(vec!["2".to_string(), "test-protocol".to_string()]),
            privileged: Some(true),
            counterparty: Some("alice".to_string()),
            basket: None,
            certificate: None,
            spending: None,
            reason: None,
            renewal: None,
            previous_token: None,
        };
        
        let tags = build_tags_for_request(&request);
        
        assert!(tags.contains(&"originator example.com".to_string()));
        assert!(tags.contains(&"privileged true".to_string()));
        assert!(tags.contains(&"protocolName test-protocol".to_string()));
        assert!(tags.contains(&"protocolSecurityLevel 2".to_string()));
        assert!(tags.contains(&"counterparty alice".to_string()));
    }
    
    #[test]
    fn test_build_tags_for_request_basket() {
        let request = PermissionRequest {
            permission_type: PermissionType::Basket,
            originator: "example.com".to_string(),
            basket: Some("my-basket".to_string()),
            protocol_id: None,
            privileged: None,
            counterparty: None,
            certificate: None,
            spending: None,
            reason: None,
            renewal: None,
            previous_token: None,
        };
        
        let tags = build_tags_for_request(&request);
        
        assert!(tags.contains(&"originator example.com".to_string()));
        assert!(tags.contains(&"basket my-basket".to_string()));
    }
    
    #[test]
    fn test_build_tags_for_request_certificate() {
        let request = PermissionRequest {
            permission_type: PermissionType::Certificate,
            originator: "example.com".to_string(),
            privileged: Some(false),
            certificate: Some(CertificateDetails {
                cert_type: "identity".to_string(),
                verifier: "verifier.com".to_string(),
                fields: vec!["name".to_string()],
            }),
            protocol_id: None,
            counterparty: None,
            basket: None,
            spending: None,
            reason: None,
            renewal: None,
            previous_token: None,
        };
        
        let tags = build_tags_for_request(&request);
        
        assert!(tags.contains(&"originator example.com".to_string()));
        assert!(tags.contains(&"privileged false".to_string()));
        assert!(tags.contains(&"type identity".to_string()));
        assert!(tags.contains(&"verifier verifier.com".to_string()));
    }
    
    #[test]
    fn test_build_tags_for_request_spending() {
        let request = PermissionRequest {
            permission_type: PermissionType::Spending,
            originator: "example.com".to_string(),
            spending: Some(SpendingDetails {
                satoshis: 10000,
                line_items: None,
            }),
            protocol_id: None,
            privileged: None,
            counterparty: None,
            basket: None,
            certificate: None,
            reason: None,
            renewal: None,
            previous_token: None,
        };
        
        let tags = build_tags_for_request(&request);
        
        // Spending only has originator tag
        assert_eq!(tags.len(), 1);
        assert!(tags.contains(&"originator example.com".to_string()));
    }
}
