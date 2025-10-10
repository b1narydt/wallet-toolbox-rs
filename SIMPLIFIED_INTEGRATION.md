# Simplified Integration Guide

**Last Updated**: January 8, 2025 - 18:50 CST  
**Architecture**: Frontend parses, Backend validates  
**Time to Complete**: 2-3 hours

---

## 🎯 Core Principle

**Frontend (ts-sdk) parses everything** → **Backend (Rust) validates and stores**

This means:
- ❌ No BEEF parsing in Rust
- ❌ No PushDrop parsing in Rust
- ❌ No Transaction parsing in Rust
- ✅ Just work with data already in storage!

---

## 📋 What Needs to be Done

### **Step 1: Update Storage Schema** (Already Done! ✅)

The `list_outputs()` result already includes custom metadata:

```rust
// What list_outputs() returns:
{
    "outputs": [{
        "outpoint": "txid.vout",
        "satoshis": 1,
        "lockingScript": "hex...",
        "basket": "admin protocol-permission",
        "tags": ["originator domain.com", "privileged true", ...],
        "customInstructions": {
            "fields": [
                "base64_encrypted_field_1",
                "base64_encrypted_field_2",
                // ...
            ]
        }
    }]
}
```

**Key Insight**: Fields are **already extracted and stored** by frontend!

---

### **Step 2: Implement Field Decryption** (2 hours)

**Location**: `token_management.rs` - `decrypt_permission_token_field()`

**Current State**: Stub function exists

```rust
/// Decrypt a permission token field
///
/// Reference: TS decryptPermissionTokenField (WalletPermissionsManager.ts lines 1223-1234)
pub async fn decrypt_permission_token_field(
    underlying: &dyn WalletInterface,
    admin_originator: &str,
    field_data: &[u8],
) -> WalletResult<Vec<u8>> {
    // TODO: Implement actual decryption
    // For now, return the field as-is
    Ok(field_data.to_vec())
}
```

**Needed Implementation**:

```rust
pub async fn decrypt_permission_token_field(
    underlying: &dyn WalletInterface,
    admin_originator: &str,
    field_data: &[u8],  // Base64 encoded encrypted data
) -> WalletResult<Vec<u8>> {
    // Decode base64
    let encrypted_bytes = base64::decode(field_data)
        .map_err(|e| WalletError::InvalidData(format!("Invalid base64: {}", e)))?;
    
    // Call underlying.decrypt() - this should exist in WalletInterface
    let result = underlying.decrypt(
        json!({
            "ciphertext": encrypted_bytes,
            "protocolID": [2, "wallet admin"],
            "keyID": admin_originator,
            "returnType": "Uint8Array"
        }),
        Some(admin_originator)
    ).await?;
    
    // Extract decrypted data
    let plaintext = result["plaintext"].as_str()
        .ok_or(WalletError::InvalidData("Missing plaintext".into()))?;
    
    // Decode from base64 or hex
    let decrypted = base64::decode(plaintext)
        .map_err(|e| WalletError::InvalidData(format!("Invalid plaintext: {}", e)))?;
    
    Ok(decrypted)
}
```

**Alternative**: If fields aren't encrypted (depends on your implementation):

```rust
pub async fn decrypt_permission_token_field(
    _underlying: &dyn WalletInterface,
    _admin_originator: &str,
    field_data: &[u8],
) -> WalletResult<Vec<u8>> {
    // If fields are stored unencrypted, just decode base64
    let decoded = base64::decode(field_data)
        .map_err(|e| WalletError::InvalidData(format!("Invalid base64: {}", e)))?;
    Ok(decoded)
}
```

---

### **Step 3: Update Token Finding Functions** (30 minutes)

**Location**: `permission_validation.rs`

**Replace all TODO blocks** with simple field extraction:

#### **find_protocol_token()** (lines 134-243)

**Replace**:
```rust
// TS line 1313: Parse transaction from BEEF
// TODO: Implement Transaction::from_beef()
// let tx = Transaction::from_beef(&result["BEEF"], txid)?;
// ... 100 lines of TODO ...
```

**With**:
```rust
// Extract fields from storage metadata (already parsed by frontend)
let custom = output["customInstructions"].as_object()
    .ok_or(WalletError::InvalidData("Missing customInstructions".into()))?;
let fields = custom["fields"].as_array()
    .ok_or(WalletError::InvalidData("Missing fields".into()))?;

// Need 6 fields for protocol token
if fields.len() < 6 {
    continue;
}

// Decrypt all 6 fields (TS lines 1323-1331)
let domain_bytes = decrypt_permission_token_field(
    underlying, 
    admin_originator, 
    fields[0].as_str().unwrap_or("").as_bytes()
).await?;
let domain_decoded = String::from_utf8(domain_bytes)?;

let expiry_bytes = decrypt_permission_token_field(
    underlying, 
    admin_originator, 
    fields[1].as_str().unwrap_or("").as_bytes()
).await?;
let expiry_decoded: i64 = String::from_utf8(expiry_bytes)?.parse().unwrap_or(0);

let priv_bytes = decrypt_permission_token_field(
    underlying, 
    admin_originator, 
    fields[2].as_str().unwrap_or("").as_bytes()
).await?;
let privileged_decoded = String::from_utf8(priv_bytes)? == "true";

let sec_level_bytes = decrypt_permission_token_field(
    underlying, 
    admin_originator, 
    fields[3].as_str().unwrap_or("").as_bytes()
).await?;
let sec_level_decoded = String::from_utf8(sec_level_bytes)?;

let proto_name_bytes = decrypt_permission_token_field(
    underlying, 
    admin_originator, 
    fields[4].as_str().unwrap_or("").as_bytes()
).await?;
let proto_name_decoded = String::from_utf8(proto_name_bytes)?;

let counterparty_bytes = decrypt_permission_token_field(
    underlying, 
    admin_originator, 
    fields[5].as_str().unwrap_or("").as_bytes()
).await?;
let counterparty_decoded = String::from_utf8(counterparty_bytes)?;

// Validate matches (TS lines 1333-1341)
if domain_decoded != originator
    || privileged_decoded != privileged
    || sec_level_decoded != sec_level
    || proto_name_decoded != proto_name
{
    continue;
}

// For security level 2, validate counterparty (TS line 1338)
if sec_level == "2" && counterparty_decoded != counterparty {
    continue;
}

// Check expiry (TS lines 1342-1344)
if !include_expired && is_token_expired_internal(expiry_decoded) {
    continue;
}

// Return the found token (TS lines 1345-1357)
return Ok(Some(PermissionToken {
    tx: vec![],  // Not needed, frontend has it
    txid: txid.to_string(),
    output_index,
    output_script: output["lockingScript"].as_str().unwrap_or("").to_string(),
    satoshis: output["satoshis"].as_i64().unwrap_or(0),
    originator: originator.to_string(),
    privileged: Some(privileged_decoded),
    protocol: Some(proto_name_decoded),
    security_level: Some(sec_level_decoded.parse().unwrap_or(0)),
    expiry: expiry_decoded,
    counterparty: Some(counterparty_decoded),
    basket_name: None,
    verifier: None,
    cert_type: None,
    cert_fields: None,
    authorized_amount: None,
}));
```

#### **Apply Same Pattern to Other 3 Functions**:

- **find_basket_token()**: 3 fields (domain, expiry, basketName)
- **find_certificate_token()**: 6 fields (domain, expiry, privileged, type, fields_json, verifier)
- **find_spending_token()**: 2 fields (domain, authorizedAmount)

**Pattern is identical**, just different field counts and validation logic.

---

### **Step 4: Update Token Creation** (1 hour)

**Location**: `token_management.rs` - `create_permission_on_chain()`

**Current TODO**:
```rust
// TODO: Implement PushDrop.lock() and createAction()
```

**Replace With**:
```rust
// Fields are built above, now create the action
// Frontend will handle PushDrop encoding when it receives this

// Encrypt fields before storing
let mut encrypted_fields = Vec::new();
for field in &fields {
    let encrypted = encrypt_permission_token_field(
        underlying,
        admin_originator,
        field.as_bytes()
    ).await?;
    encrypted_fields.push(base64::encode(&encrypted));
}

// Create action (frontend will build PushDrop from fields)
let result = underlying.create_action(
    json!({
        "outputs": [{
            "satoshis": 1,
            "basket": admin_basket,
            "tags": tags,
            "customInstructions": {
                "fields": encrypted_fields,
                "lockingPublicKey": locking_public_key
            }
        }],
        "description": format!("Create {} permission token", permission_type),
    }),
    Some(admin_originator)
).await?;

Ok(result)
```

**Note**: Frontend intercepts `create_action()` and:
1. Builds PushDrop script from `customInstructions.fields`
2. Creates transaction with that script
3. Calls backend to sign
4. Stores output with metadata

---

## ✅ Summary of Changes

### **Files to Modify**:

1. **permission_validation.rs** (4 functions × ~30 lines each = 120 lines changed)
   - Replace TODO blocks with field extraction from storage
   - Add decryption calls
   - Keep validation logic (already perfect)

2. **token_management.rs** (3 functions × ~20 lines each = 60 lines changed)
   - Remove PushDrop encoding TODOs
   - Add field encryption
   - Pass encrypted fields in customInstructions

3. **Add encryption function** (~50 lines new)
   ```rust
   pub async fn encrypt_permission_token_field(
       underlying: &dyn WalletInterface,
       admin_originator: &str,
       plaintext: &[u8],
   ) -> WalletResult<Vec<u8>>
   ```

**Total Changes**: ~230 lines (vs 1500+ lines for full BEEF/PushDrop implementation)

---

## 🧪 Testing Strategy

### **Unit Tests** (with mocks):

```rust
#[tokio::test]
async fn test_find_protocol_token() {
    let mock = MockWalletInterface {
        list_outputs_response: json!({
            "outputs": [{
                "outpoint": "abc123.0",
                "satoshis": 1,
                "customInstructions": {
                    "fields": [
                        "domain.com",  // domain
                        "1234567890",  // expiry
                        "true",        // privileged
                        "2",           // secLevel
                        "myProtocol",  // protocolName
                        "anyone"       // counterparty
                    ]
                }
            }]
        })
    };
    
    let result = find_protocol_token(
        &mock, 
        "admin", 
        "domain.com",
        true,
        "myProtocol",
        "2",
        "anyone",
        false
    ).await.unwrap();
    
    assert!(result.is_some());
    assert_eq!(result.unwrap().protocol, Some("myProtocol".into()));
}
```

### **Integration Tests** (with real frontend):

1. Frontend creates permission token
2. Backend queries and validates
3. Verify decryption works
4. Verify expiry checking works

---

## ⏱️ Implementation Timeline

| Task | Time | Status |
|------|------|--------|
| Implement decrypt_permission_token_field() | 1h | ⏸️ |
| Implement encrypt_permission_token_field() | 30min | ⏸️ |
| Update find_protocol_token() | 30min | ⏸️ |
| Update find_basket_token() | 15min | ⏸️ |
| Update find_certificate_token() | 15min | ⏸️ |
| Update find_spending_token() | 15min | ⏸️ |
| Update token creation functions | 30min | ⏸️ |
| Write unit tests | 30min | ⏸️ |
| **Total** | **3.5h** | **Ready** |

---

## 🚀 Next Immediate Action

**Start with**: Implement `decrypt_permission_token_field()`

This unblocks all 4 find functions immediately.

**After that**: Update the 4 find functions (copy-paste pattern 4x)

**Then**: Token creation can wait until testing phase

---

**Ready to start?** I can implement the decryption function first!
