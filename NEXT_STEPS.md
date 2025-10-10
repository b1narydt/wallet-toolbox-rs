# Next Steps - Immediate Actions

**Last Updated**: January 8, 2025 - 18:19 CST  
**Current Phase**: 5.3 - Transaction/PushDrop Integration  
**Time to Next Milestone**: 4-5 hours (Transaction + PushDrop MVP)

**See Also**: [`INTEGRATION_PLAN.md`](INTEGRATION_PLAN.md) for detailed wiring instructions

---

## 🎯 Immediate Task (Next 3-4 Hours)

### Complete WalletPermissionsManager Integration

**Goal**: Wire up BEEF/PushDrop integration to make all token operations functional

**Current Status**: 
- ✅ Structure complete (4,116 lines, 8 modules)
- ✅ All method signatures defined
- ✅ Complete logic flow documented
- ⏸️ Integration pending

**What Needs Doing**:

#### 1. BEEF Parsing Integration (1 hour)
**File**: `crates/wallet-core/src/managers/wallet_permissions_manager/permission_validation.rs`

**Task**: Wire up BEEF parsing in all `find_*_token()` methods

**Steps**:
```rust
// Current (lines 130-159 in find_protocol_token):
// TODO: Call underlying.listOutputs({...})
// TODO: Parse BEEF, decode PushDrop, decrypt fields

// Need to add:
let outputs = self.underlying.list_outputs(
    ListOutputsArgs {
        basket: Some(BASKET_MAP_PROTOCOL.to_string()),
        tags: Some(tags),
        tag_query_mode: Some("all".to_string()),
        include: Some("entire transactions".to_string()),
        ..Default::default()
    },
    &self.admin_originator
).await?;

for output in outputs.outputs {
    let (txid, output_index_str) = output.outpoint.split_once('.').unwrap();
    let tx = Transaction::from_beef(&outputs.beef.unwrap(), txid)?;
    let locking_script = &tx.outputs[output_index_str.parse::<usize>()?].locking_script;
    let decoded = PushDrop::decode(locking_script)?;
    
    if decoded.fields.len() < 6 { continue; }
    
    // Decrypt and validate fields...
}
```

**Apply to**:
- `find_protocol_token()` (6 fields)
- `find_basket_token()` (3 fields)
- `find_certificate_token()` (6 fields + JSON)
- `find_spending_token()` (2 fields)

**Reference**: TS lines 1247-1595 in `WalletPermissionsManager.ts`

#### 2. PushDrop Integration (1 hour)
**File**: `crates/wallet-core/src/managers/wallet_permissions_manager/token_management.rs`

**Task**: Wire up PushDrop encode/decode in token creation/renewal

**Steps**:
```rust
// In create_permission_on_chain (line 418):
// TODO: PushDrop.lock(...) 

let pushdrop_script = PushDrop::lock(
    &fields,
    &lockingPublicKey
)?;

// In renew_permission_on_chain (line 544):
// TODO: PushDrop.unlock(...)

let unlock_script = PushDrop::unlock(
    &oldToken.lockingScript,
    &signature
)?;
```

**Reference**: TS lines 1636-1678 in `WalletPermissionsManager.ts`

#### 3. createAction/signAction Integration (1 hour)
**File**: `crates/wallet-core/src/managers/wallet_permissions_manager/token_management.rs`

**Task**: Wire up action creation and signing

**Steps**:
```rust
// In create_permission_on_chain (line 440):
// TODO: createAction() with single output

let action = self.underlying.create_action(
    CreateActionArgs {
        description: Some(description),
        outputs: vec![CreateActionOutput {
            satoshis: 1,
            script: pushdrop_script.to_hex(),
            basket: Some(basket_name),
            tags: Some(tags),
            ..Default::default()
        }],
        ..Default::default()
    },
    &self.admin_originator
).await?;

// Sign it
let signed = self.underlying.sign_action(
    SignActionArgs {
        reference: action.reference,
        ..Default::default()
    },
    &self.admin_originator
).await?;
```

**Reference**: TS lines 1640-1650 in `WalletPermissionsManager.ts`

#### 4. Field Encryption/Decryption (30 min)
**File**: `crates/wallet-core/src/managers/wallet_permissions_manager/token_management.rs`

**Task**: Wire up encrypt/decrypt methods

**Steps**:
```rust
// In encrypt_permission_token_field (line 640):
// TODO: this.underlying.encrypt(...)

self.underlying.encrypt(
    WalletEncryptArgs {
        plaintext: field.to_vec(),
        protocol_id: vec!["2".to_string(), PERM_TOKEN_ENCRYPTION_PROTOCOL.to_string()],
        key_id: "1".to_string(),
        counterparty: Some(self.admin_originator.clone()),
        ..Default::default()
    },
    &self.admin_originator
).await

// In decrypt_permission_token_field (line 666):
// TODO: this.underlying.decrypt(...)

self.underlying.decrypt(
    WalletDecryptArgs {
        ciphertext: field.to_vec(),
        protocol_id: vec!["2".to_string(), PERM_TOKEN_ENCRYPTION_PROTOCOL.to_string()],
        key_id: "1".to_string(),
        counterparty: Some(self.admin_originator.clone()),
        ..Default::default()
    },
    &self.admin_originator
).await
```

**Reference**: TS lines 1207-1234 in `WalletPermissionsManager.ts`

#### 5. Integration Tests (30 min)
**File**: `crates/wallet-core/src/managers/wallet_permissions_manager/mod.rs` (bottom)

**Task**: Add integration tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_protocol_permission_flow() {
        // Create mock wallet
        // Call ensureProtocolPermission
        // Verify token created
        // Verify token can be found
    }
    
    #[tokio::test]
    async fn test_token_renewal() {
        // Create token
        // Renew it
        // Verify old token spent
        // Verify new token created
    }
}
```

**Total Estimated Time**: 3-4 hours

---

## 📋 After WalletPermissionsManager (Next 4 Hours)

### 1. Complete SimpleWalletManager (1 hour)
**File**: `crates/wallet-core/src/managers/simple_wallet_manager.rs`

**Task**: Fill remaining TODOs

**Search for**: `TODO` comments in the file

**Estimated**: ~40 lines of code

### 2. Complete WalletSigner (1 hour)
**File**: `crates/wallet-core/src/signer/`

**Task**: Fill helper method TODOs

**Search for**: `TODO` comments

**Estimated**: ~30 lines of code

### 3. Implement Main Wallet Struct (2 hours)
**File**: `crates/wallet-core/src/wallet/mod.rs`

**Task**: Implement the primary Wallet orchestrator

**Reference**: `wallet-toolbox/src/Wallet.ts` lines 136-1135

**Structure**:
```rust
pub struct Wallet {
    pub chain: Chain,
    pub key_deriver: Arc<dyn KeyDeriver>,
    pub storage: Arc<WalletStorageManager>,
    pub settings_manager: WalletSettingsManager,
    // ... ~15 more fields
}

impl Wallet {
    pub fn new(args: WalletArgs) -> WalletResult<Self> {
        // Constructor logic
    }
}

#[async_trait]
impl WalletInterface for Wallet {
    // Implement all ~30 methods
    // Most delegate to storage/signer/managers
}
```

**Estimated**: ~500 lines

### 4. Add Top-Level Exports (30 min)
**File**: `crates/wallet-core/src/lib.rs`

**Task**: Re-export main types at top level

```rust
// Add at top of lib.rs
pub use wallet::Wallet;
pub use managers::{
    SimpleWalletManager,
    WalletPermissionsManager,
    WalletAuthenticationManager,
    WalletSettingsManager,
};
pub use sdk::{WalletInterface, WalletError, WalletResult, /*...*/};
```

**Result**: Users can `use wallet_toolbox::Wallet;`

---

## 🎯 Milestones

### Milestone 1: WalletPermissionsManager Complete ✨
**Time**: 3-4 hours from now  
**Result**: All permission token operations functional

**Deliverables**:
- ✅ All find_*_token() methods working
- ✅ All token creation/renewal working
- ✅ All encryption working
- ✅ Integration tests passing

### Milestone 2: Phase 5 Complete ✨
**Time**: 8 hours from now  
**Result**: Full integration layer functional

**Deliverables**:
- ✅ All managers 100% complete
- ✅ Main Wallet orchestrator working
- ✅ API compatibility achieved
- ✅ Can use: `use wallet_toolbox::Wallet;`

### Milestone 3: Production Ready ✨
**Time**: 2 weeks from now  
**Result**: Full feature parity with TypeScript

**Deliverables**:
- ✅ All components 100% complete
- ✅ Comprehensive test suite
- ✅ Documentation complete
- ✅ Ready for production use

---

## 🔧 How to Continue

### For Current Session:
```bash
# 1. Focus on permission_validation.rs
cd crates/wallet-core/src/managers/wallet_permissions_manager/

# 2. Edit permission_validation.rs
# Add BEEF parsing to find_protocol_token() first

# 3. Test compilation
cargo build -p wallet-core

# 4. Run tests
cargo test -p wallet-core wallet_permissions_manager
```

### For Next Session:
1. Read `PROJECT_STATUS.md` - Current state
2. Read `PHASES.md` - Overall structure  
3. Read this file - Next tasks
4. Continue where TODOs are marked

---

## 📚 Reference Files

**TypeScript Source**:
- `wallet-toolbox/src/WalletPermissionsManager.ts` (3,111 lines)
- Lines 1247-1595: Token finding methods
- Lines 1636-1742: Token creation/renewal
- Lines 1207-1234: Encryption/decryption

**Rust Implementation**:
- `crates/wallet-core/src/managers/wallet_permissions_manager/`
- 8 modules, 4,116 lines total
- Focus on: `permission_validation.rs` and `token_management.rs`

**Tests**:
- 46 unit tests currently
- Need: 10+ integration tests

---

## ✅ Success Criteria

**WalletPermissionsManager Integration Complete When**:
- [ ] All find_*_token() methods parse BEEF
- [ ] All find_*_token() methods decode PushDrop
- [ ] All find_*_token() methods decrypt fields
- [ ] create_permission_on_chain() creates transactions
- [ ] renew_permission_on_chain() spends and creates
- [ ] encrypt/decrypt methods work
- [ ] Integration tests pass
- [ ] Can grant and verify permissions end-to-end

**Phase 5 Complete When**:
- [ ] WalletPermissionsManager 100%
- [ ] SimpleWalletManager 100%
- [ ] WalletSigner 100%
- [ ] Main Wallet struct implemented
- [ ] Top-level exports added
- [ ] API compatibility test passes

---

**Current Focus**: WalletPermissionsManager integration (3-4 hours)  
**Next Focus**: Complete remaining Phase 5 components (4 hours)  
**End Goal**: Seamless TypeScript replacement
