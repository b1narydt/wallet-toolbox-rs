//! Permission Token Validation and Finding
//!
//! **Reference**: TypeScript `src/WalletPermissionsManager.ts` lines 1236-1621
//!
//! Functions for finding, decoding, and validating permission tokens in the wallet storage.

use super::types::*;
use super::constants::*;
use super::token_management::{decrypt_permission_token_field};
use crate::sdk::errors::{WalletError, WalletResult};
use crate::managers::simple_wallet_manager::WalletInterface;
use serde_json::json;

/// Check if a token is expired (internal helper)
///
/// Reference: TS isTokenExpired (WalletPermissionsManager.ts lines 1236-1246)
///
/// Checks if a permission token has expired based on its expiry timestamp.
/// Uses current UNIX epoch time for comparison.
///
/// # Arguments
///
/// * `expiry` - Token expiry timestamp (UNIX epoch seconds)
///
/// # Returns
///
/// `true` if expired, `false` otherwise
pub fn is_token_expired_internal(expiry: i64) -> bool {
    // TS lines 1237-1245: Compare with current time
    // if (expiry === 0) return false // never expires
    // const now = Math.floor(Date.now() / 1000)
    // return now > expiry
    
    if expiry == 0 {
        return false; // TS line 1237: Never expires
    }
    
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    
    now > expiry // TS line 1245
}

/// Get current month in YYYY-MM format (UTC)
///
/// Reference: TS getCurrentMonthYearUTC (WalletPermissionsManager.ts lines 1602-1607)
///
/// Returns the current month and year in UTC as a string for spending tracking.
///
/// # Returns
///
/// String in format "YYYY-MM"
pub fn get_current_month_utc() -> String {
    // TS lines 1603-1606
    // const now = new Date()
    // const year = now.getUTCFullYear()
    // const month = (now.getUTCMonth() + 1).toString().padStart(2, '0')
    // return `${year}-${month}`
    
    use chrono::{Datelike, Utc};
    let now = Utc::now();
    format!("{:04}-{:02}", now.year(), now.month())
}

/// Find a protocol permission token (DPACP)
///
/// Reference: TS findProtocolToken (WalletPermissionsManager.ts lines 1247-1323)
///
/// Looks for a DPACP permission token matching originator, privileged flag, protocol, and counterparty.
///
///
/// * `originator` - Domain or FQDN
/// * `privileged` - Whether this is a privileged operation
/// * `protocol_id` - [securityLevel, protocolName]
/// * `counterparty` - Counterparty identifier
/// * `include_expired` - Whether to include expired tokens in the search
///
/// # Returns
///
/// Optional permission token if found
///
/// # TODO
///
/// This requires integration with:
/// - underlying.listOutputs() to query tokens by basket and tags
/// - Transaction.fromBEEF() to parse BEEF transactions
/// - PushDrop.decode() to decode locking scripts
/// - decryptPermissionTokenField() to decrypt each field
pub async fn find_protocol_token(
    underlying: &dyn WalletInterface,
    admin_originator: &str,
    originator: &str,
    privileged: bool,
    protocol_id: &[String],
    counterparty: &str,
    include_expired: bool,
) -> WalletResult<Option<PermissionToken>> {
    // TS lines 1248-1260: Build tags for query
    // const [secLevel, protoName] = protocolID
    // const tags = [
    //   `originator ${originator}`,
    //   `privileged ${!!privileged}`,
    //   `protocolName ${protoName}`,
    //   `protocolSecurityLevel ${secLevel}`
    // ]
    // if (secLevel === 2) {
    //   tags.push(`counterparty ${counterparty}`)
    // }
    
    if protocol_id.len() < 2 {
        return Err(WalletError::invalid_parameter(
            "protocol_id",
            "Must have [securityLevel, protocolName]"
        ));
    }
    
    let sec_level = &protocol_id[0];
    let proto_name = &protocol_id[1];
    
    // Build tags for query
    let mut _tags = vec![
        format!("originator {}", originator),
        format!("privileged {}", privileged),
        format!("protocolName {}", proto_name),
        format!("protocolSecurityLevel {}", sec_level),
    ];
    
    if sec_level == "2" {
        _tags.push(format!("counterparty {}", counterparty));
    }
    
    // TS lines 1301-1309: Query outputs from storage
    let basket_name = get_admin_basket_name(PermissionType::Protocol);
    let result = underlying.list_outputs(
        json!({
            "basket": basket_name,
            "tags": _tags,
            "tagQueryMode": "all",
            "include": "entire transactions"
        }),
        Some(admin_originator)
    ).await?;
    
    // TS lines 1311-1359: Loop through results, decode PushDrop, decrypt fields
    let empty_vec = vec![];
    let outputs = result["outputs"].as_array().unwrap_or(&empty_vec);
    
    for output in outputs {
        // TS line 1312: Split outpoint into txid and outputIndex
        let outpoint = output["outpoint"].as_str().unwrap_or("");
        let parts: Vec<&str> = outpoint.split('.').collect();
        if parts.len() != 2 {
            continue;
        }
        
        let txid = parts[0];
        let output_index_str = parts[1];
        let output_index: usize = match output_index_str.parse() {
            Ok(idx) => idx,
            Err(_) => continue,
        };
        
        // SIMPLIFIED ARCHITECTURE: Extract fields from storage metadata
        // Frontend (ts-sdk) already parsed PushDrop and stored fields in customInstructions
        
        // Extract custom instructions (TS line 1313-1321 equivalent)
        let custom = match output["customInstructions"].as_object() {
            Some(c) => c,
            None => continue, // No custom instructions, skip
        };
        
        let fields = match custom["fields"].as_array() {
            Some(f) => f,
            None => continue, // No fields, skip
        };
        
        // Need 6 fields for protocol token (TS line 1314-1315)
        if fields.len() < 6 {
            continue;
        }
        
        // TS lines 1323-1331: Decrypt all 6 fields
        let domain_decoded = String::from_utf8(
            decrypt_permission_token_field(
                underlying,
                admin_originator,
                fields[0].as_str().unwrap_or("")
            ).await?
        ).map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in domain: {}", e)))?;
        
        let expiry_str = String::from_utf8(
            decrypt_permission_token_field(
                underlying,
                admin_originator,
                fields[1].as_str().unwrap_or("")
            ).await?
        ).map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in expiry: {}", e)))?;
        let expiry_decoded: i64 = expiry_str.parse().unwrap_or(0);
        
        let priv_str = String::from_utf8(
            decrypt_permission_token_field(
                underlying,
                admin_originator,
                fields[2].as_str().unwrap_or("")
            ).await?
        ).map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in privileged: {}", e)))?;
        let priv_decoded = priv_str == "true";
        
        let sec_level_str = String::from_utf8(
            decrypt_permission_token_field(
                underlying,
                admin_originator,
                fields[3].as_str().unwrap_or("")
            ).await?
        ).map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in security level: {}", e)))?;
        let sec_level_decoded = sec_level_str.as_str();
        
        let proto_name_decoded = String::from_utf8(
            decrypt_permission_token_field(
                underlying,
                admin_originator,
                fields[4].as_str().unwrap_or("")
            ).await?
        ).map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in protocol name: {}", e)))?;
        
        let cpty_decoded = String::from_utf8(
            decrypt_permission_token_field(
                underlying,
                admin_originator,
                fields[5].as_str().unwrap_or("")
            ).await?
        ).map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in counterparty: {}", e)))?;
        
        // TS lines 1333-1341: Validate all fields match
        if domain_decoded != originator
            || priv_decoded != privileged
            || sec_level_decoded != sec_level
            || proto_name_decoded != *proto_name
        {
            continue;
        }
        
        // For security level 2, validate counterparty (TS line 1338)
        if sec_level == "2" && cpty_decoded != counterparty {
            continue;
        }
        
        // TS lines 1342-1344: Check expiry if needed
        if !include_expired && is_token_expired_internal(expiry_decoded) {
            continue;
        }
        
        // TS lines 1345-1357: Return the found token
        // Convert security level string to SecurityLevel enum
        let sec_level_enum = match sec_level.as_str() {
            "0" => SecurityLevel::Public,
            "1" => SecurityLevel::Shared,
            "2" => SecurityLevel::Private,
            _ => SecurityLevel::Public, // Default to public if invalid
        };
        
        return Ok(Some(PermissionToken {
            tx: vec![], // Frontend has BEEF, not needed here
            txid: txid.to_string(),
            output_index: output_index as u32,
            output_script: output["lockingScript"].as_str().unwrap_or("").to_string(),
            satoshis: output["satoshis"].as_i64().unwrap_or(0),
            originator: originator.to_string(),
            privileged: Some(priv_decoded),
            protocol: Some(proto_name_decoded),
            security_level: Some(sec_level_enum),
            expiry: expiry_decoded,
            counterparty: Some(cpty_decoded),
            basket_name: None,
            verifier: None,
            cert_type: None,
            cert_fields: None,
            authorized_amount: None,
        }));
    }
    
    // TS line 1359: No token found
    Ok(None)
}

/// Find a basket access token (DBAP)
///
/// Reference: TS findBasketToken (WalletPermissionsManager.ts lines 1445-1488)
///
/// Looks for a DBAP permission token matching originator and basket name.
///
/// # Arguments
///
/// * `originator` - Domain or FQDN
/// * `basket` - Basket name
/// * `include_expired` - Whether to include expired tokens
///
/// # Returns
///
/// Optional permission token if found
///
/// # TODO
///
/// This requires integration with:
/// - underlying.listOutputs() to query tokens
/// - Transaction.fromBEEF() to parse transactions
/// - PushDrop.decode() to decode locking scripts
/// - decryptPermissionTokenField() to decrypt fields
pub async fn find_basket_token(
    underlying: &dyn WalletInterface,
    admin_originator: &str,
    originator: &str,
    basket: &str,
    include_expired: bool,
) -> WalletResult<Option<PermissionToken>> {
    // TS lines 1451-1459: Query outputs with 2 tags
    // const result = await this.underlying.listOutputs(
    //   {
    //     basket: BASKET_MAP.basket,
    //     tags: [`originator ${originator}`, `basket ${basket}`],
    //     tagQueryMode: 'all',
    //     include: 'entire transactions'
    //   },
    //   this.adminOriginator
    // )
    
    let tags = vec![
        format!("originator {}", originator),
        format!("basket {}", basket),
    ];
    
    // TS lines 1451-1459: Query outputs from storage
    let basket_name = get_admin_basket_name(PermissionType::Basket);
    let result = underlying.list_outputs(
        json!({
            "basket": basket_name,
            "tags": tags,
            "tagQueryMode": "all",
            "include": "entire transactions"
        }),
        Some(admin_originator)
    ).await?;
    
    // TS lines 1461-1488: Loop through results, decode PushDrop, decrypt fields
    let empty_vec = vec![];
    let outputs = result["outputs"].as_array().unwrap_or(&empty_vec);
    
    for output in outputs {
        // TS line 1462: Split outpoint into txid and outputIndex
        let outpoint = output["outpoint"].as_str().unwrap_or("");
        let parts: Vec<&str> = outpoint.split('.').collect();
        if parts.len() != 2 {
            continue;
        }
        
        let txid = parts[0];
        let output_index_str = parts[1];
        let output_index: usize = match output_index_str.parse() {
            Ok(idx) => idx,
            Err(_) => continue,
        };
        
        // SIMPLIFIED ARCHITECTURE: Extract fields from storage metadata
        // Frontend (ts-sdk) already parsed PushDrop and stored fields in customInstructions
        
        // Extract custom instructions (TS line 1463-1468 equivalent)
        let custom = match output["customInstructions"].as_object() {
            Some(c) => c,
            None => continue, // No custom instructions, skip
        };
        
        let fields = match custom["fields"].as_array() {
            Some(f) => f,
            None => continue, // No fields, skip
        };
        
        // Need 3 fields for basket token (TS line 1464-1465)
        if fields.len() < 3 {
            continue;
        }
        
        // TS lines 1470-1472: Decrypt all 3 fields
        let domain_decoded = String::from_utf8(
            decrypt_permission_token_field(
                underlying,
                admin_originator,
                fields[0].as_str().unwrap_or("")
            ).await?
        ).map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in domain: {}", e)))?;
        
        let expiry_str = String::from_utf8(
            decrypt_permission_token_field(
                underlying,
                admin_originator,
                fields[1].as_str().unwrap_or("")
            ).await?
        ).map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in expiry: {}", e)))?;
        let expiry_decoded: i64 = expiry_str.parse().unwrap_or(0);
        
        let basket_decoded = String::from_utf8(
            decrypt_permission_token_field(
                underlying,
                admin_originator,
                fields[2].as_str().unwrap_or("")
            ).await?
        ).map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in basket: {}", e)))?;
        
        // TS lines 1473-1474: Validate matches and check expiry
        if domain_decoded != originator || basket_decoded != basket {
            continue;
        }
        
        if !include_expired && is_token_expired_internal(expiry_decoded) {
            continue;
        }
        
        // TS lines 1476-1485: Return the found token
        return Ok(Some(PermissionToken {
            tx: vec![], // Frontend has BEEF, not needed here
            txid: txid.to_string(),
            output_index: output_index as u32,
            output_script: output["lockingScript"].as_str().unwrap_or("").to_string(),
            satoshis: output["satoshis"].as_i64().unwrap_or(0),
            originator: originator.to_string(),
            privileged: None,
            protocol: None,
            security_level: None,
            expiry: expiry_decoded,
            counterparty: None,
            basket_name: Some(basket_decoded),
            verifier: None,
            cert_type: None,
            cert_fields: None,
            authorized_amount: None,
        }));
    }
    
    // TS line 1487: No token found
    Ok(None)
}

/// Find a certificate access token (DCAP)
///
/// Reference: TS findCertificateToken (WalletPermissionsManager.ts lines 1490-1556)
///
/// Looks for a DCAP permission token matching originator, privileged flag, verifier, cert type,
/// and checking that the token's fields are a superset of the requested fields.
///
/// # Arguments
///
/// * `originator` - Domain or FQDN
/// * `privileged` - Whether this is a privileged operation
/// * `verifier` - Verifier public key
/// * `cert_type` - Certificate type identifier
/// * `fields` - Requested certificate fields (must be subset of token fields)
/// * `include_expired` - Whether to include expired tokens
///
/// # Returns
///
/// Optional permission token if found
///
/// # TODO
///
/// This requires integration with:
/// - underlying.listOutputs() to query tokens
/// - Transaction.fromBEEF() to parse transactions
/// - PushDrop.decode() to decode locking scripts
/// - decryptPermissionTokenField() to decrypt fields
/// - JSON parsing for fields array
pub async fn find_certificate_token(
    underlying: &dyn WalletInterface,
    admin_originator: &str,
    originator: &str,
    privileged: bool,
    verifier: &str,
    cert_type: &str,
    fields: &[String],
    include_expired: bool,
) -> WalletResult<Option<PermissionToken>> {
    // TS lines 1499-1507: Query outputs with 4 tags
    // const result = await this.underlying.listOutputs(
    //   {
    //     basket: BASKET_MAP.certificate,
    //     tags: [
    //       `originator ${originator}`,
    //       `privileged ${!!privileged}`,
    //       `type ${certType}`,
    //       `verifier ${verifier}`
    //     ],
    //     tagQueryMode: 'all',
    //     include: 'entire transactions'
    //   },
    //   this.adminOriginator
    // )
    
    let tags = vec![
        format!("originator {}", originator),
        format!("privileged {}", privileged),
        format!("type {}", cert_type),
        format!("verifier {}", verifier),
    ];
    
    // TS lines 1499-1507: Query outputs from storage
    let basket_name = get_admin_basket_name(PermissionType::Certificate);
    let result = underlying.list_outputs(
        json!({
            "basket": basket_name,
            "tags": tags,
            "tagQueryMode": "all",
            "include": "entire transactions"
        }),
        Some(admin_originator)
    ).await?;
    
    // TS lines 1509-1556: Loop through results, decode PushDrop, decrypt fields
    let empty_vec = vec![];
    let outputs = result["outputs"].as_array().unwrap_or(&empty_vec);
    
    for output in outputs {
        // TS line 1510: Split outpoint into txid and outputIndex
        let outpoint = output["outpoint"].as_str().unwrap_or("");
        let parts: Vec<&str> = outpoint.split('.').collect();
        if parts.len() != 2 {
            continue;
        }
        
        let txid = parts[0];
        let output_index_str = parts[1];
        let output_index: usize = match output_index_str.parse() {
            Ok(idx) => idx,
            Err(_) => continue,
        };
        
        // SIMPLIFIED ARCHITECTURE: Extract fields from storage metadata
        // Frontend (ts-sdk) already parsed PushDrop and stored fields in customInstructions
        
        // Extract custom instructions (TS line 1511-1514 equivalent)
        let custom = match output["customInstructions"].as_object() {
            Some(c) => c,
            None => continue, // No custom instructions, skip
        };
        
        let token_fields = match custom["fields"].as_array() {
            Some(f) => f,
            None => continue, // No fields, skip
        };
        
        // Need 6 fields for certificate token (TS line 1512-1513)
        if token_fields.len() < 6 {
            continue;
        }
        
        // TS lines 1516-1520: Decrypt 5 string fields
        let domain_decoded = String::from_utf8(
            decrypt_permission_token_field(
                underlying,
                admin_originator,
                token_fields[0].as_str().unwrap_or("")
            ).await?
        ).map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in domain: {}", e)))?;
        
        let expiry_str = String::from_utf8(
            decrypt_permission_token_field(
                underlying,
                admin_originator,
                token_fields[1].as_str().unwrap_or("")
            ).await?
        ).map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in expiry: {}", e)))?;
        let expiry_decoded: i64 = expiry_str.parse().unwrap_or(0);
        
        let priv_str = String::from_utf8(
            decrypt_permission_token_field(
                underlying,
                admin_originator,
                token_fields[2].as_str().unwrap_or("")
            ).await?
        ).map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in privileged: {}", e)))?;
        let priv_decoded = priv_str == "true";
        
        let type_decoded = String::from_utf8(
            decrypt_permission_token_field(
                underlying,
                admin_originator,
                token_fields[3].as_str().unwrap_or("")
            ).await?
        ).map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in type: {}", e)))?;
        
        let verifier_decoded = String::from_utf8(
            decrypt_permission_token_field(
                underlying,
                admin_originator,
                token_fields[5].as_str().unwrap_or("")
            ).await?
        ).map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in verifier: {}", e)))?;
        
        // TS lines 1522-1523: Decrypt and parse fields JSON array
        let fields_json_bytes = decrypt_permission_token_field(
            underlying,
            admin_originator,
            token_fields[4].as_str().unwrap_or("")
        ).await?;
        let fields_json_str = String::from_utf8(fields_json_bytes)
            .map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in fields JSON: {}", e)))?;
        let all_fields: Vec<String> = serde_json::from_str(&fields_json_str)
            .map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid JSON in fields: {}", e)))?;
        
        // TS lines 1525-1532: Validate all fields match
        if domain_decoded != originator
            || priv_decoded != privileged
            || type_decoded != cert_type
            || verifier_decoded != verifier
        {
            continue;
        }
        
        // TS lines 1533-1537: Check if 'fields' is a subset of 'allFields'
        let all_fields_set: std::collections::HashSet<_> = all_fields.iter().map(|s| s.as_str()).collect();
        if !fields.iter().all(|f| all_fields_set.contains(f.as_str())) {
            continue;
        }
        
        // TS lines 1538-1540: Check expiry
        if !include_expired && is_token_expired_internal(expiry_decoded) {
            continue;
        }
        
        // TS lines 1541-1553: Return the found token
        return Ok(Some(PermissionToken {
            tx: vec![], // Frontend has BEEF, not needed here
            txid: txid.to_string(),
            output_index: output_index as u32,
            output_script: output["lockingScript"].as_str().unwrap_or("").to_string(),
            satoshis: output["satoshis"].as_i64().unwrap_or(0),
            originator: originator.to_string(),
            privileged: Some(priv_decoded),
            protocol: None,
            security_level: None,
            expiry: expiry_decoded,
            counterparty: None,
            basket_name: None,
            verifier: Some(verifier_decoded),
            cert_type: Some(type_decoded),
            cert_fields: Some(all_fields),
            authorized_amount: None,
        }));
    }
    
    // TS line 1555: No token found
    Ok(None)
}

/// Find a spending authorization token (DSAP)
///
/// Reference: TS findSpendingToken (WalletPermissionsManager.ts lines 1558-1595)
///
/// Looks for a DSAP permission token matching the originator.
/// Returns the first matching token found. DSAP tokens don't have expiry (monthly authorization).
///
/// # Arguments
///
/// * `originator` - Domain or FQDN
///
/// # Returns
///
/// Optional permission token if found
///
/// # TODO
///
/// This requires integration with:
/// - underlying.listOutputs() to query tokens
/// - Transaction.fromBEEF() to parse transactions
/// - PushDrop.decode() to decode locking scripts
/// - decryptPermissionTokenField() to decrypt fields
pub async fn find_spending_token(
    underlying: &dyn WalletInterface,
    admin_originator: &str,
    originator: &str,
) -> WalletResult<Option<PermissionToken>> {
    // TS lines 1560-1568: Query outputs with 1 tag
    // const result = await this.underlying.listOutputs(
    //   {
    //     basket: BASKET_MAP.spending,
    //     tags: [`originator ${originator}`],
    //     tagQueryMode: 'all',
    //     include: 'entire transactions'
    //   },
    //   this.adminOriginator
    // )
    
    let tags = vec![format!("originator {}", originator)];
    
    // TS lines 1560-1568: Query outputs from storage
    let basket_name = get_admin_basket_name(PermissionType::Spending);
    let result = underlying.list_outputs(
        json!({
            "basket": basket_name,
            "tags": tags,
            "tagQueryMode": "all",
            "include": "entire transactions"
        }),
        Some(admin_originator)
    ).await?;
    
    // TS lines 1570-1594: Loop through results, decode PushDrop, decrypt fields
    let empty_vec = vec![];
    let outputs = result["outputs"].as_array().unwrap_or(&empty_vec);
    
    for output in outputs {
        // TS line 1571: Split outpoint into txid and outputIndex
        let outpoint = output["outpoint"].as_str().unwrap_or("");
        let parts: Vec<&str> = outpoint.split('.').collect();
        if parts.len() != 2 {
            continue;
        }
        
        let txid = parts[0];
        let output_index_str = parts[1];
        let output_index: usize = match output_index_str.parse() {
            Ok(idx) => idx,
            Err(_) => continue,
        };
        
        // SIMPLIFIED ARCHITECTURE: Extract fields from storage metadata
        // Frontend (ts-sdk) already parsed PushDrop and stored fields in customInstructions
        
        // Extract custom instructions (TS line 1572-1576 equivalent)
        let custom = match output["customInstructions"].as_object() {
            Some(c) => c,
            None => continue, // No custom instructions, skip
        };
        
        let fields = match custom["fields"].as_array() {
            Some(f) => f,
            None => continue, // No fields, skip
        };
        
        // Need 2 fields for spending token (TS line 1573-1574)
        if fields.len() < 2 {
            continue;
        }
        
        // TS lines 1578-1579: Decrypt domain and validate
        let domain_decoded = String::from_utf8(
            decrypt_permission_token_field(
                underlying,
                admin_originator,
                fields[0].as_str().unwrap_or("")
            ).await?
        ).map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in domain: {}", e)))?;
        
        if domain_decoded != originator {
            continue;
        }
        
        // TS lines 1580-1581: Decrypt and parse authorized amount
        let amt_str = String::from_utf8(
            decrypt_permission_token_field(
                underlying,
                admin_originator,
                fields[1].as_str().unwrap_or("")
            ).await?
        ).map_err(|e| WalletError::new("WERR_INVALID_DATA", format!("Invalid UTF-8 in amount: {}", e)))?;
        let authorized_amount: i64 = amt_str.parse().unwrap_or(0);
        
        // TS lines 1583-1592: Return the found token
        return Ok(Some(PermissionToken {
            tx: vec![], // Frontend has BEEF, not needed here
            txid: txid.to_string(),
            output_index: output_index as u32,
            output_script: output["lockingScript"].as_str().unwrap_or("").to_string(),
            satoshis: output["satoshis"].as_i64().unwrap_or(0),
            originator: originator.to_string(),
            privileged: None,
            protocol: None,
            security_level: None,
            expiry: 0,  // TS line 1591: Not time-limited, monthly authorization
            counterparty: None,
            basket_name: None,
            verifier: None,
            cert_type: None,
            cert_fields: None,
            authorized_amount: Some(authorized_amount),
        }));
    }
    
    // TS line 1594: No token found
    Ok(None)
}

/// Query how much has been spent this month for a spending token
///
/// Reference: TS querySpentSince (WalletPermissionsManager.ts lines 1609-1621)
///
/// Returns the total spending for an originator in the current calendar month (UTC).
/// This is used to enforce monthly spending limits.
///
/// # Arguments
///
/// * `token` - The spending permission token
///
/// # Returns
///
/// Total satoshis spent this month
///
/// # TODO
///
/// This requires integration with:
/// - underlying.listActions() to query spending actions
/// - Label filtering with 'admin originator {originator}' and 'admin month {YYYY-MM}'
pub async fn query_spent_since(
    underlying: &dyn WalletInterface,
    admin_originator: &str,
    token: &PermissionToken,
) -> WalletResult<i64> {
    // TS lines 1613-1620: Query actions with labels
    // const { actions } = await this.underlying.listActions(
    //   {
    //     labels: [
    //       `admin originator ${token.originator}`,
    //       `admin month ${this.getCurrentMonthYearUTC()}`
    //     ],
    //     labelQueryMode: 'all'
    //   },
    //   this.adminOriginator
    // )
    // return actions.reduce((a, e) => a + e.satoshis, 0)
    
    let current_month = get_current_month_utc();
    let labels = vec![
        format!("admin originator {}", token.originator),
        format!("admin month {}", current_month),
    ];
    
    // TS lines 1613-1620: Query actions with labels
    let result = underlying.list_actions(
        json!({
            "labels": labels,
            "labelQueryMode": "all"
        }),
        Some(admin_originator)
    ).await?;
    
    // TS line 1620: Sum satoshis from all actions
    // return actions.reduce((a, e) => a + e.satoshis, 0)
    let empty_vec = vec![];
    let actions = result["actions"].as_array().unwrap_or(&empty_vec);
    
    let total = actions.iter()
        .filter_map(|action| action["satoshis"].as_i64())
        .sum();
    
    Ok(total)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_token_expired() {
        // Token that never expires (expiry = 0)
        assert!(!is_token_expired_internal(0));
        
        // Token far in the future
        let future = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64) + 86400; // 1 day from now
        assert!(!is_token_expired_internal(future));
        
        // Token in the past
        assert!(is_token_expired_internal(1000000000)); // Way in the past
    }
    
    #[test]
    fn test_get_current_month_utc() {
        let month = get_current_month_utc();
        
        // Should be in format YYYY-MM
        assert_eq!(month.len(), 7);
        assert_eq!(&month[4..5], "-");
        
        // Year should be 4 digits
        let year_part = &month[0..4];
        assert!(year_part.parse::<i32>().is_ok());
        
        // Month should be 2 digits between 01-12
        let month_part = &month[5..7];
        let month_num = month_part.parse::<u32>().unwrap();
        assert!(month_num >= 1 && month_num <= 12);
    }
    
    // TODO: Implement proper mock WalletInterface for testing
    // These tests are commented out until we have a mock
    
    // #[tokio::test]
    // async fn test_find_protocol_token_validation() {
    //     // Need mock WalletInterface
    // }
    
    // #[tokio::test]
    // async fn test_find_basket_token_structure() {
    //     // Need mock WalletInterface
    // }
    
    // #[tokio::test]
    // async fn test_find_certificate_token_structure() {
    //     // Need mock WalletInterface  
    // }
    
    // #[tokio::test]
    // async fn test_find_spending_token_structure() {
    //     // Need mock WalletInterface
    // }
    
    // #[tokio::test]
    // async fn test_query_spent_since_structure() {
    //     // Need mock WalletInterface
    // }
}
