# WalletPermissionsManager Implementation Plan

**Date**: January 8, 2025 (15:00 CST)  
**Component**: WalletPermissionsManager  
**TypeScript Source**: `src/WalletPermissionsManager.ts` (3,111 lines)  
**Estimated Rust**: ~3,500 lines + tests  
**Complexity**: ⭐⭐⭐⭐⭐ (VERY HIGH - Largest component)

---

## 📋 Overview

WalletPermissionsManager is the **largest and most complex component** in the entire wallet-toolbox codebase. It implements BRC-73 grouped permissions and manages four types of permission tokens:

1. **DPACP** - Domain Protocol Access Control Protocol
2. **DBAP** - Domain Basket Access Protocol
3. **DCAP** - Domain Certificate Access Protocol
4. **DSAP** - Domain Spending Authorization Protocol

---

## 🎯 Core Responsibilities

### Permission Management
- Request permissions from users (with UI callbacks)
- Create on-chain permission tokens (PushDrop outputs)
- Validate permissions before operations
- Track token expiration
- Handle token renewal
- Support ephemeral (one-time) grants

### Token Storage
- Store tokens as UTXOs in admin baskets
- Four specialized admin baskets (one per permission type)
- Track spending authorization amounts
- Manage token lifecycle

### Security
- Validate originator domains
- Check privileged vs non-privileged operations
- Enforce spending limits
- Verify certificate access
- Control basket access
- Manage protocol permissions

---

## 📊 TypeScript Structure Analysis

### File Breakdown (3,111 lines)

```typescript
Lines 1-45:    Imports & utility functions (deepEqual, isObject)
Lines 47-102:  GroupedPermissions interface (BRC-73)
Lines 103-132: PermissionRequest interface
Lines 134-198: PermissionToken interface
Lines 200-210: BASKET_MAP constants
Lines 212-222: Callbacks interface
Lines 224-269: PermissionsManagerConfig interface
Lines 271-400: WalletPermissionsManager class structure
Lines 401-800: Permission request methods
Lines 801-1200: Permission validation methods
Lines 1201-1600: Token creation methods
Lines 1601-2000: Token management methods
Lines 2001-2400: Spending authorization logic
Lines 2401-2800: Protocol permission logic
Lines 2801-3000: Certificate permission logic
Lines 3001-3111: Basket permission logic
```

### Key Methods (~60 total)

**Permission Requests (8 methods)**
- `requestProtocolPermission()`
- `requestBasketAccess()`
- `requestCertificateAccess()`
- `requestSpendingAuthorization()`
- `requestGroupedPermissions()` (BRC-73)
- `grantPermission()`
- `denyPermission()`
- `revokePermission()`

**Permission Validation (12 methods)**
- `checkProtocolPermission()`
- `checkBasketPermission()`
- `checkCertificatePermission()`
- `checkSpendingAuthorization()`
- `validateOriginator()`
- `isPrivilegedOperation()`
- `hasValidToken()`
- `isTokenExpired()`
- `findMatchingToken()`
- `calculateRemainingAmount()`
- `validateCounterparty()`
- `validateSecurityLevel()`

**Token Management (10 methods)**
- `createPermissionToken()`
- `updatePermissionToken()`
- `deletePermissionToken()`
- `renewPermissionToken()`
- `listPermissionTokens()`
- `getPermissionToken()`
- `serializeToken()`
- `deserializeToken()`
- `buildTokenScript()`
- `parseTokenScript()`

**Integration Methods (8 methods)**
- `wrapCreateAction()`
- `wrapSignAction()`
- `wrapInternalizeAction()`
- `wrapListOutputs()`
- `wrapRelinquishOutput()`
- `wrapAcquireCertificate()`
- `wrapProveCertificate()`
- `wrapRelinquishCertificate()`

**Spending Authorization (8 methods)**
- `trackSpending()`
- `getMonthlySpending()`
- `resetMonthlySpending()`
- `calculateSpendingAmount()`
- `validateSpendingLimit()`
- `getSpendingBreakdown()`
- `createSpendingLineItems()`
- `validateLineItems()`

**Callback Management (6 methods)**
- `registerProtocolCallback()`
- `registerBasketCallback()`
- `registerCertificateCallback()`
- `registerSpendingCallback()`
- `registerGroupedCallback()`
- `emitPermissionRequest()`

**Utility Methods (8 methods)**
- `getAdminBasketName()`
- `parsePermissionType()`
- `createRequestID()`
- `validateExpiry()`
- `getCurrentMonth()`
- `deepCompare()`
- `sanitizeOriginator()`
- `validateConfig()`

---

## 🏗️ Rust Module Structure

Given the size and complexity, break into sub-modules:

```
wallet-core/src/managers/wallet_permissions_manager/
├── mod.rs                          (Main manager struct, ~200 lines)
├── types.rs                        (All interfaces/types, ~400 lines)
├── constants.rs                    (BASKET_MAP, protocols, ~50 lines)
├── permission_request.rs           (Request handling, ~500 lines)
├── permission_validation.rs        (Validation logic, ~600 lines)
├── token_management.rs             (Token CRUD ops, ~500 lines)
├── spending_authorization.rs       (DSAP logic, ~400 lines)
├── protocol_permission.rs          (DPACP logic, ~400 lines)
├── certificate_permission.rs       (DCAP logic, ~400 lines)
├── basket_permission.rs            (DBAP logic, ~300 lines)
├── integration.rs                  (Wallet method wrappers, ~400 lines)
├── callbacks.rs                    (Event handlers, ~200 lines)
├── utils.rs                        (Utilities, ~150 lines)
└── tests.rs                        (Comprehensive tests, ~600 lines)

Total: ~4,900 lines (including tests)
```

---

## 📝 Implementation Plan

### Phase 1: Foundation (Day 1-2, ~10 hours)

**1.1 Types & Interfaces**
- [ ] Create `types.rs` with all TypeScript interfaces:
  - `GroupedPermissions`
  - `GroupedPermissionRequest`
  - `PermissionRequest`
  - `PermissionToken`
  - `PermissionsManagerConfig`
  - `WalletPermissionsManagerCallbacks`
- [ ] Add serde derives for serialization
- [ ] Reference: TS lines 47-269

**1.2 Constants & Utilities**
- [ ] Create `constants.rs` with BASKET_MAP
- [ ] Create `utils.rs` with:
  - `deep_equal()` function
  - `is_object()` helper
  - `create_request_id()`
  - `sanitize_originator()`
- [ ] Reference: TS lines 1-45, 200-210

**1.3 Main Module Structure**
- [ ] Create `mod.rs` with `WalletPermissionsManager` struct
- [ ] Define constructor
- [ ] Add config fields
- [ ] Add callback registry
- [ ] Add wallet reference
- [ ] Reference: TS lines 271-400

### Phase 2: Permission Requests (Day 3-4, ~12 hours)

**2.1 Request Methods**
- [ ] Implement `requestProtocolPermission()`
- [ ] Implement `requestBasketAccess()`
- [ ] Implement `requestCertificateAccess()`
- [ ] Implement `requestSpendingAuthorization()`
- [ ] Implement `requestGroupedPermissions()` (BRC-73)
- [ ] Reference: TS lines 401-800

**2.2 Grant/Deny Logic**
- [ ] Implement `grantPermission()`
- [ ] Implement `denyPermission()`
- [ ] Implement `revokePermission()`
- [ ] Add request ID tracking
- [ ] Reference: TS lines 801-1000

**2.3 Callback System**
- [ ] Create `callbacks.rs`
- [ ] Implement callback registration
- [ ] Implement event emission
- [ ] Add async callback support
- [ ] Reference: TS lines 2000-2100

### Phase 3: Token Management (Day 5-6, ~12 hours)

**3.1 Token Creation**
- [ ] Implement `createPermissionToken()`
- [ ] Implement `buildTokenScript()` (PushDrop)
- [ ] Implement `serializeToken()`
- [ ] Add UTXO creation logic
- [ ] Reference: TS lines 1201-1600

**3.2 Token Operations**
- [ ] Implement `updatePermissionToken()`
- [ ] Implement `deletePermissionToken()`
- [ ] Implement `renewPermissionToken()`
- [ ] Implement `listPermissionTokens()`
- [ ] Implement `getPermissionToken()`
- [ ] Reference: TS lines 1601-2000

**3.3 Token Parsing**
- [ ] Implement `parseTokenScript()`
- [ ] Implement `deserializeToken()`
- [ ] Add validation for token fields
- [ ] Reference: TS lines 1400-1600

### Phase 4: Permission Validation (Day 7-8, ~12 hours)

**4.1 Validation Core**
- [ ] Implement `checkProtocolPermission()`
- [ ] Implement `checkBasketPermission()`
- [ ] Implement `checkCertificatePermission()`
- [ ] Implement `checkSpendingAuthorization()`
- [ ] Reference: TS lines 801-1200

**4.2 Token Validation**
- [ ] Implement `hasValidToken()`
- [ ] Implement `isTokenExpired()`
- [ ] Implement `findMatchingToken()`
- [ ] Implement `validateOriginator()`
- [ ] Reference: TS lines 1000-1200

**4.3 Advanced Validation**
- [ ] Implement `isPrivilegedOperation()`
- [ ] Implement `validateCounterparty()`
- [ ] Implement `validateSecurityLevel()`
- [ ] Reference: TS lines 1100-1300

### Phase 5: Specialized Permissions (Day 9-11, ~18 hours)

**5.1 Spending Authorization (DSAP)**
- [ ] Create `spending_authorization.rs`
- [ ] Implement `trackSpending()`
- [ ] Implement `getMonthlySpending()`
- [ ] Implement `resetMonthlySpending()`
- [ ] Implement `calculateSpendingAmount()`
- [ ] Implement `validateSpendingLimit()`
- [ ] Implement `getSpendingBreakdown()`
- [ ] Implement `createSpendingLineItems()`
- [ ] Implement `validateLineItems()`
- [ ] Reference: TS lines 2001-2400

**5.2 Protocol Permission (DPACP)**
- [ ] Create `protocol_permission.rs`
- [ ] Implement protocol validation logic
- [ ] Implement counterparty checking
- [ ] Implement security level validation
- [ ] Implement privileged operation checks
- [ ] Reference: TS lines 2401-2800

**5.3 Certificate Permission (DCAP)**
- [ ] Create `certificate_permission.rs`
- [ ] Implement certificate access validation
- [ ] Implement field validation
- [ ] Implement verifier validation
- [ ] Reference: TS lines 2801-3000

**5.4 Basket Permission (DBAP)**
- [ ] Create `basket_permission.rs`
- [ ] Implement basket access validation
- [ ] Implement basket name validation
- [ ] Reference: TS lines 3001-3111

### Phase 6: Wallet Integration (Day 12-13, ~10 hours)

**6.1 Wrapper Methods**
- [ ] Create `integration.rs`
- [ ] Implement `wrapCreateAction()`
- [ ] Implement `wrapSignAction()`
- [ ] Implement `wrapInternalizeAction()`
- [ ] Implement `wrapListOutputs()`
- [ ] Implement `wrapRelinquishOutput()`
- [ ] Implement `wrapAcquireCertificate()`
- [ ] Implement `wrapProveCertificate()`
- [ ] Implement `wrapRelinquishCertificate()`
- [ ] Reference: TS lines 400-800

### Phase 7: Testing (Day 14-15, ~12 hours)

**7.1 Unit Tests**
- [ ] Test permission request flow
- [ ] Test token creation/parsing
- [ ] Test validation logic
- [ ] Test spending authorization
- [ ] Test expiration handling
- [ ] Reference: Add comprehensive tests

**7.2 Integration Tests**
- [ ] Test end-to-end permission flow
- [ ] Test grouped permissions (BRC-73)
- [ ] Test token renewal
- [ ] Test spending limits
- [ ] Test all four permission types

---

## 🎯 Implementation Order (Priority)

### Week 1: Foundation & Core
1. **Day 1-2**: Types, constants, main structure
2. **Day 3-4**: Permission request methods
3. **Day 5-6**: Token management

### Week 2: Validation & Specialization
4. **Day 7-8**: Permission validation
5. **Day 9-11**: Specialized permissions (DSAP, DPACP, DCAP, DBAP)

### Week 3: Integration & Testing
6. **Day 12-13**: Wallet integration wrappers
7. **Day 14-15**: Comprehensive testing

**Total Estimate**: 15 days (~90 hours)

---

## 🔑 Critical Features

### Must-Have (Core Functionality)
1. ✅ All four permission types (DPACP, DBAP, DCAP, DSAP)
2. ✅ Token creation with PushDrop scripts
3. ✅ Permission validation before operations
4. ✅ Expiration checking
5. ✅ Originator validation
6. ✅ Callback system for UI integration

### Should-Have (Enhanced Features)
7. ✅ Grouped permissions (BRC-73)
8. ✅ Token renewal
9. ✅ Spending limit tracking
10. ✅ Ephemeral grants
11. ✅ Privileged operation checks

### Nice-to-Have (Advanced Features)
12. ⏭️ Counterparty validation
13. ⏭️ Security level enforcement
14. ⏭️ Monthly spending reset
15. ⏭️ Line item breakdown

---

## ⚠️ Security Considerations

### Critical Security Notes (from TS file)
```typescript
////// TODO: ADD SUPPORT FOR ADMIN COUNTERPARTIES BASED ON WALLET STORAGE
//////       PROHIBITION OF SPECIAL OPERATIONS IS ALSO CRITICAL.
////// !!!!!!!! SECURITY-CRITICAL ADDITION — DO NOT USE UNTIL IMPLEMENTED.
```

**Action Items**:
1. Document security warnings prominently
2. Implement admin counterparty checks
3. Prohibit special operations without proper permissions
4. Add security audit notes

### Validation Requirements
- **Originator**: Must be valid FQDN or domain
- **Expiry**: Must check token expiration
- **Spending**: Must enforce monthly limits
- **Privileged**: Must validate privileged operations
- **Counterparty**: Must verify counterparty matches

---

## 📊 TypeScript Parity Checklist

### Interfaces (7 types)
- [ ] `GroupedPermissions`
- [ ] `GroupedPermissionRequest`
- [ ] `GroupedPermissionEventHandler`
- [ ] `PermissionRequest`
- [ ] `PermissionEventHandler`
- [ ] `PermissionToken`
- [ ] `WalletPermissionsManagerCallbacks`
- [ ] `PermissionsManagerConfig`

### Methods (~60 total)
#### Permission Requests (8)
- [ ] `requestProtocolPermission()`
- [ ] `requestBasketAccess()`
- [ ] `requestCertificateAccess()`
- [ ] `requestSpendingAuthorization()`
- [ ] `requestGroupedPermissions()`
- [ ] `grantPermission()`
- [ ] `denyPermission()`
- [ ] `revokePermission()`

#### Validation (12)
- [ ] `checkProtocolPermission()`
- [ ] `checkBasketPermission()`
- [ ] `checkCertificatePermission()`
- [ ] `checkSpendingAuthorization()`
- [ ] `validateOriginator()`
- [ ] `isPrivilegedOperation()`
- [ ] `hasValidToken()`
- [ ] `isTokenExpired()`
- [ ] `findMatchingToken()`
- [ ] `calculateRemainingAmount()`
- [ ] `validateCounterparty()`
- [ ] `validateSecurityLevel()`

#### Token Management (10)
- [ ] `createPermissionToken()`
- [ ] `updatePermissionToken()`
- [ ] `deletePermissionToken()`
- [ ] `renewPermissionToken()`
- [ ] `listPermissionTokens()`
- [ ] `getPermissionToken()`
- [ ] `serializeToken()`
- [ ] `deserializeToken()`
- [ ] `buildTokenScript()`
- [ ] `parseTokenScript()`

#### Integration (8)
- [ ] `wrapCreateAction()`
- [ ] `wrapSignAction()`
- [ ] `wrapInternalizeAction()`
- [ ] `wrapListOutputs()`
- [ ] `wrapRelinquishOutput()`
- [ ] `wrapAcquireCertificate()`
- [ ] `wrapProveCertificate()`
- [ ] `wrapRelinquishCertificate()`

#### Spending (8)
- [ ] `trackSpending()`
- [ ] `getMonthlySpending()`
- [ ] `resetMonthlySpending()`
- [ ] `calculateSpendingAmount()`
- [ ] `validateSpendingLimit()`
- [ ] `getSpendingBreakdown()`
- [ ] `createSpendingLineItems()`
- [ ] `validateLineItems()`

#### Callbacks (6)
- [ ] `registerProtocolCallback()`
- [ ] `registerBasketCallback()`
- [ ] `registerCertificateCallback()`
- [ ] `registerSpendingCallback()`
- [ ] `registerGroupedCallback()`
- [ ] `emitPermissionRequest()`

#### Utilities (8)
- [ ] `getAdminBasketName()`
- [ ] `parsePermissionType()`
- [ ] `createRequestID()`
- [ ] `validateExpiry()`
- [ ] `getCurrentMonth()`
- [ ] `deepCompare()`
- [ ] `sanitizeOriginator()`
- [ ] `validateConfig()`

---

## 📈 Success Metrics

### Code Quality
- ✅ 100% TypeScript parity (all methods implemented)
- ✅ Zero unsafe code
- ✅ Comprehensive error handling
- ✅ Full documentation with TS line references
- ✅ 40+ tests covering all scenarios

### Functionality
- ✅ All four permission types work
- ✅ Token creation/validation works
- ✅ Callback system functional
- ✅ Integration with wallet methods
- ✅ Spending limits enforced

### Performance
- ✅ Token lookups O(log n) with indexing
- ✅ Validation checks optimized
- ✅ Minimal allocations
- ✅ Async where appropriate

---

## 🎓 Learning Points

### Key Concepts
1. **BRC-73**: Grouped permissions standard
2. **PushDrop**: On-chain token format
3. **UTXO-based permissions**: Tokens as outputs
4. **Four permission types**: DPACP, DBAP, DCAP, DSAP
5. **Expiration handling**: Time-based validation
6. **Spending limits**: Monthly authorization amounts
7. **Callback architecture**: Event-driven UI integration

### Rust Patterns
1. **Sub-modules**: Breaking large components down
2. **Trait-based callbacks**: Type-safe event handlers
3. **Builder pattern**: For complex token creation
4. **State machines**: For permission request flow
5. **Async traits**: For callback handlers

---

## 📝 Notes

### Complexity Factors
- **Largest component**: 3,111 TypeScript lines
- **Four permission types**: Each with unique logic
- **Token management**: Complex UTXO lifecycle
- **Spending tracking**: Monthly limits with rollover
- **Security critical**: Requires careful validation
- **Callback system**: Event-driven architecture

### Dependencies
- Requires `WalletInterface` (from SimpleWalletManager)
- Requires PushDrop script building
- Requires UTXO management
- Requires transaction creation
- Requires storage integration

---

**Est. Completion**: 15 days (~90 hours)  
**Priority**: HIGH (blocks CWIStyleWalletManager)  
**Status**: 📋 Planning Complete - Ready to Begin

**Next Step**: Start with Phase 1.1 - Create types.rs

