# Phase 5 Implementation Guide - Next Steps

**Current Status**: Phases 1-4 Complete (67%)  
**Next Phase**: Phase 5 - Integration & Orchestration  
**Remaining**: ~15,000 lines, ~193 tests

---

## 🎯 Quick Summary

### ✅ What We Have (Phases 1-4)
```
✅ Storage Layer (wallet-storage, wallet-storage-sqlite)
   - All tables, entities, CRUD operations
   - 250 tests passing

✅ Core Wallet Methods (wallet-core)
   - createAction, signAction, listOutputs, listActions
   - internalizeAction, processAction
   - Transaction building, BEEF support
   - BRC-42/43 key derivation
   - 211 tests passing

✅ Services Layer (wallet-services)
   - ChainTracker, Broadcaster, UTXO, Exchange Rates
   - All external service integrations
   - 39 tests passing
```

### ⏸️ What We Need (Phase 5)

**Empty/Stub Modules in wallet-core**:
- `managers.rs` - 5 manager stubs (only type definitions)
- `signer/methods/` - 4 method stubs (placeholders only)
- `monitor/` - Monitor & daemon stubs
- `wallet.rs` - Main wallet stub
- `wab_client/` - WAB client stubs

**These are NOT implemented** - they're just type placeholders to allow compilation.

---

## 🚀 Phase 5 Roadmap

### Priority 1: Signer Methods (Week 1-2)
**Why First**: Completes the transaction lifecycle

**Files** (all in `wallet-core/src/signer/methods/`):

1. **buildSignableTransaction.rs** ⭐ CRITICAL
   - Reference: TS `buildSignableTransaction.ts` (680 lines)
   - Build transaction ready for signing
   - Input selection and validation
   - Output construction
   - **Estimated**: 800 lines, 8 tests
   - **Complexity**: ⭐⭐⭐⭐ High

2. **completeSignedTransaction.rs** ⭐ CRITICAL
   - Reference: TS `completeSignedTransaction.ts` (470 lines)
   - Add signatures to transaction
   - Finalize transaction
   - Validate completeness
   - **Estimated**: 550 lines, 4 tests
   - **Complexity**: ⭐⭐⭐ Medium-High

3. **acquireDirectCertificate.rs**
   - Reference: TS `acquireDirectCertificate.ts` (170 lines)
   - Certificate acquisition flow
   - Keyring integration
   - **Estimated**: 200 lines, 2 tests
   - **Complexity**: ⭐⭐ Medium

4. **proveCertificate.rs**
   - Reference: TS `proveCertificate.ts` (140 lines)
   - Certificate proving logic
   - **Estimated**: 180 lines, 1 test
   - **Complexity**: ⭐⭐ Medium

**Total**: ~1,730 lines, 15 tests

---

### Priority 2: SimpleWalletManager (Week 2-3)
**Why Second**: Provides basic wallet functionality

**File**: `wallet-core/src/managers/simple_wallet_manager.rs`

- Reference: TS `SimpleWalletManager.ts` (500 lines)
- Basic wallet operations
- Action creation/signing
- Output management
- Simplified API surface
- **Estimated**: 600 lines, 15 tests
- **Complexity**: ⭐⭐⭐ Medium-High

**Dependencies**: 
- ✅ Core methods (complete)
- ✅ Storage (complete)
- ✅ Services (complete)
- ⚠️ Needs signer methods (Priority 1)

---

### Priority 3: WalletSettingsManager (Week 3)
**Why Third**: Simple, enables configuration

**File**: `wallet-core/src/managers/wallet_settings_manager.rs`

- Reference: TS `WalletSettingsManager.ts` (100 lines)
- Wallet configuration
- Settings persistence
- User preferences
- **Estimated**: 150 lines, 5 tests
- **Complexity**: ⭐ Low

---

### Priority 4: WalletAuthenticationManager (Week 3-4)
**Why Fourth**: Needed for auth features

**File**: `wallet-core/src/managers/wallet_auth_manager.rs`

- Reference: TS `WalletAuthenticationManager.ts` (200 lines)
- User authentication
- Identity management
- Session handling
- **Estimated**: 250 lines, 8 tests
- **Complexity**: ⭐⭐ Medium

---

### Priority 5: WalletPermissionsManager (Week 4-6) ⚠️ LARGE
**Why Later**: Most complex, needs other managers first

**File**: `wallet-core/src/managers/wallet_permissions_manager.rs`

- Reference: TS `WalletPermissionsManager.ts` (3,000+ lines!)
- Permission validation
- Action authorization
- Certificate permissions
- App permissions
- **Estimated**: 3,500 lines, 40 tests
- **Complexity**: ⭐⭐⭐⭐⭐ Very High

**Note**: This is the LARGEST single file. Take time.

---

### Priority 6: CWIStyleWalletManager (Week 7-9)
**Why Later**: Full-featured, needs all managers

**File**: `wallet-core/src/managers/cwi_wallet_manager.rs`

- Reference: TS `CWIStyleWalletManager.ts` (2,000 lines)
- Complete wallet manager
- All features integrated
- Full API surface
- **Estimated**: 2,500 lines, 35 tests
- **Complexity**: ⭐⭐⭐⭐⭐ Very High

---

### Priority 7: Main Wallet Orchestration (Week 9-10)
**Why Later**: Coordinates everything else

**File**: `wallet-core/src/wallet.rs` (replace stub)

- Reference: TS `Wallet.ts` (1,100 lines)
- Main wallet interface
- Coordinates all managers
- Action lifecycle
- Transaction orchestration
- **Estimated**: 1,500 lines, 25 tests
- **Complexity**: ⭐⭐⭐⭐⭐ Very High

---

### Priority 8: Monitor & Daemon (Week 10)
**Why Later**: Background functionality

**Files**: 
- `wallet-core/src/monitor/monitor.rs`
- `wallet-core/src/monitor/daemon.rs`

- Reference: TS `monitor/` directory
- Transaction monitoring
- Status updates
- Proof tracking
- Background processing
- **Estimated**: 1,300 lines, 15 tests
- **Complexity**: ⭐⭐⭐⭐ High

---

### Priority 9: Setup & Initialization (Week 10-11)
**Why Last**: Initialization ties everything together

**Files**:
- `wallet-core/src/setup.rs` (expand)
- `wallet-core/src/setup_wallet.rs` (new)
- `wallet-core/src/setup_client.rs` (new)

- Reference: TS `Setup.ts`, `SetupWallet.ts`, `SetupClient.ts`
- Environment configuration
- Service initialization
- Wallet configuration
- Client setup
- **Estimated**: 3,120 lines, 20 tests
- **Complexity**: ⭐⭐⭐⭐ High

---

## 📋 Recommended Session Plan

### Session 1: Start Signer Methods
**Goal**: Complete critical signing infrastructure

**Tasks**:
1. Read TS `buildSignableTransaction.ts` thoroughly
2. Implement `buildSignableTransaction.rs`
3. Write tests
4. Verify integration with createAction

**Output**: 800 lines, 8 tests

---

### Session 2: Complete Signer Methods
**Goal**: Finish transaction signing pipeline

**Tasks**:
1. Implement `completeSignedTransaction.rs`
2. Implement `acquireDirectCertificate.rs`
3. Implement `proveCertificate.rs`
4. Integration tests

**Output**: 930 lines, 7 tests

---

### Session 3: SimpleWalletManager
**Goal**: Basic wallet operations working

**Tasks**:
1. Implement SimpleWalletManager
2. Test basic workflows
3. Verify end-to-end flows

**Output**: 600 lines, 15 tests

---

### Sessions 4-5: Settings & Auth
**Goal**: Configuration and authentication

**Tasks**:
1. Implement WalletSettingsManager
2. Implement WalletAuthenticationManager
3. Integration tests

**Output**: 400 lines, 13 tests

---

### Sessions 6-10: Large Components
**Goal**: Complete permission system and full managers

**Tasks**:
1. WalletPermissionsManager (2-3 sessions)
2. CWIStyleWalletManager (2 sessions)
3. Main Wallet orchestration (1 session)

**Output**: 7,500 lines, 100 tests

---

### Sessions 11-12: Monitor & Setup
**Goal**: Background processing and initialization

**Tasks**:
1. Monitor implementation
2. MonitorDaemon implementation
3. Setup & initialization

**Output**: 4,420 lines, 35 tests

---

## 🎯 Success Criteria Per Component

### Signer Methods ✅ When:
- [ ] buildSignableTransaction creates valid signable TX
- [ ] completeSignedTransaction adds signatures correctly
- [ ] acquireDirectCertificate integrates with keyring
- [ ] proveCertificate generates valid proofs
- [ ] All 15 tests passing

### SimpleWalletManager ✅ When:
- [ ] Can create actions
- [ ] Can sign actions
- [ ] Can list outputs/actions
- [ ] Can process actions
- [ ] All 15 tests passing

### WalletPermissionsManager ✅ When:
- [ ] Validates all permission types
- [ ] Authorizes actions correctly
- [ ] Handles certificate permissions
- [ ] Manages app permissions
- [ ] All 40 tests passing

### CWIStyleWalletManager ✅ When:
- [ ] All wallet operations work
- [ ] Complete API implemented
- [ ] Integrates all managers
- [ ] All 35 tests passing

### Main Wallet ✅ When:
- [ ] Orchestrates all managers
- [ ] Handles complete action lifecycle
- [ ] Manages transactions end-to-end
- [ ] All 25 tests passing

### Monitor ✅ When:
- [ ] Tracks transactions
- [ ] Updates status
- [ ] Processes proofs
- [ ] All 15 tests passing

### Setup ✅ When:
- [ ] Initializes environment
- [ ] Configures services
- [ ] Sets up wallet
- [ ] All 20 tests passing

---

## 💡 Key Implementation Notes

### For Signer Methods:
- Use existing `createAction` as foundation
- Leverage `Transaction` module from wallet-core
- Reference BRC-42/43 implementation
- Follow TypeScript line-by-line for complex logic

### For Managers:
- Start with trait definitions
- Implement simple methods first
- Build up to complex permission logic
- Each manager is semi-independent

### For WalletPermissionsManager:
- Break into sub-modules (app perms, cert perms, action perms)
- This is ~3,500 lines - don't try in one session
- Test each permission type independently
- Most complex component in Phase 5

### For Main Wallet:
- Delegates to managers
- Thin orchestration layer
- Error handling is critical
- State management is key

---

## 📊 Estimated Timeline

```
Week 1-2:   Signer Methods (1,730 lines, 15 tests)
Week 2-3:   SimpleWalletManager (600 lines, 15 tests)
Week 3:     Settings (150 lines, 5 tests)
Week 3-4:   Authentication (250 lines, 8 tests)
Week 4-6:   Permissions (3,500 lines, 40 tests) ⚠️ LARGE
Week 7-9:   CWIStyleManager (2,500 lines, 35 tests)
Week 9-10:  Main Wallet (1,500 lines, 25 tests)
Week 10:    Monitor (1,300 lines, 15 tests)
Week 10-11: Setup (3,120 lines, 20 tests)

Total: ~11 weeks for Phase 5
```

---

## 🔥 Start Here

**Next Session**: Begin with `buildSignableTransaction.rs`

**Steps**:
1. Read `TRANSLATION_PLAN_UPDATED.md` for full context
2. Read TypeScript `src/signer/methods/buildSignableTransaction.ts`
3. Create `wallet-core/src/signer/methods/build_signable_transaction.rs`
4. Implement following TS logic exactly
5. Write tests
6. Verify integration

**Expected Output**: Core signing infrastructure working

---

**Created**: January 7, 2025  
**Status**: Ready to begin Phase 5  
**Next**: Signer Methods Implementation

