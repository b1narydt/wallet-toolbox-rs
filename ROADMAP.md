# Wallet-Toolbox-RS Development Roadmap

## Quick Start Guide

### Current Status (Week 0)
✅ Workspace structure established  
✅ All 10 crates scaffold complete  
✅ Builds successfully with `cargo check`  
✅ Comprehensive mapping documented  
✅ Translation plan created  
✅ API specifications defined  

### Immediate Next Steps (Week 1)

**Day 1-2: Core Dependencies**
```bash
cd wallet-toolbox-rs
# Add BSV SDK dependency (when available) or create SDK type stubs
# Update wallet-core/Cargo.toml with dependencies
```

**Day 3-4: Error System**
- Complete `wallet-core/src/sdk/errors/werr.rs` (106 error codes)
- Complete `wallet-core/src/sdk/errors/wallet_error.rs`
- Enhance `wallet-storage/src/error.rs`

**Day 5-7: Storage Schema**
- Translate all table definitions from TypeScript
- Implement serialization/deserialization
- Add schema migration support

### Week-by-Week Milestones

**Week 1-2: Foundation** 🎯
- [ ] Integrate or stub BSV SDK types
- [ ] Complete error type hierarchy
- [ ] Translate all storage schemas
- [ ] Set up async runtime (tokio)

**Week 3-4: Storage Layer** 💾
- [ ] Complete storage traits
- [ ] Implement SQLite backend
- [ ] Implement storage methods
- [ ] Add basic tests

**Week 5-7: Core Wallet** 🔐
- [ ] Implement SDK module interfaces
- [ ] Implement WalletSigner
- [ ] Implement main Wallet struct
- [ ] Implement wallet managers

**Week 8-9: Services** 🌐
- [ ] Service collection & providers
- [ ] ChainTracker integration
- [ ] External API clients (ARC, WhatsOnChain)

**Week 10: Monitor** ⏰
- [ ] Monitor daemon
- [ ] Background tasks
- [ ] Task scheduler

**Week 11: WAB Client** 🔌
- [ ] WAB client implementation
- [ ] Auth method interactors

**Week 12: Utilities** 🛠️
- [ ] Format utilities
- [ ] BRC-29 script templates
- [ ] Identity utilities
- [ ] Helper functions

**Week 13-14: FFI/WASM** 🌍
- [ ] C API (FFI) for native
- [ ] WASM bindings for web
- [ ] TypeScript definitions
- [ ] Mobile bindings (UniFFI)

**Week 15-16: Testing** ✅
- [ ] Port Jest tests to Rust
- [ ] Integration tests
- [ ] Performance benchmarks
- [ ] Memory safety audits

**Week 17: Documentation** 📚
- [ ] Complete Rustdoc
- [ ] Usage examples
- [ ] Migration guide
- [ ] API reference

## Critical Path

```
Dependencies → Errors → Schema → Storage → Wallet → FFI/WASM → Tests
     ↓           ↓         ↓         ↓        ↓         ↓         ↓
   Week 1      Week 1    Week 2   Week 3-4  Week 5-7  Week 13-14 Week 15-16
```

## Parallel Work Opportunities

These can be developed simultaneously:
- **Services** (Week 8-9) - Independent of wallet core
- **Monitor** (Week 10) - Uses storage interface
- **WAB Client** (Week 11) - Uses wallet interface
- **Utilities** (Week 12) - Mostly independent

## Key Decision Points

### Week 2: BSV SDK Integration
**Decision:** Use existing Rust SDK vs. create minimal SDK stubs  
**Impact:** Affects type system and API compatibility  
**Recommendation:** Use `rs-sdk` from workspace if complete enough

### Week 4: Storage Backend Priority
**Decision:** SQLite first vs. MySQL first vs. IndexedDB first  
**Impact:** Testing and deployment strategy  
**Recommendation:** SQLite → MySQL → IndexedDB (defer WASM)

### Week 7: Manager Implementation Strategy
**Decision:** Full feature parity vs. minimal viable managers  
**Impact:** Testing complexity, timeline  
**Recommendation:** Start with SimpleWalletManager, defer CWIStyleWalletManager complex features

### Week 13: FFI vs. WASM Priority
**Decision:** Native FFI first vs. WASM first  
**Impact:** Which client gets faster access  
**Recommendation:** FFI first for metanet client, WASM in parallel if resources allow

## Success Metrics

### Phase 1 (Foundation)
- ✅ All dependencies compile
- ✅ Error types match TS WERR codes
- ✅ Schema types serialize correctly

### Phase 2 (Storage)
- ✅ Can create/read/update/delete in SQLite
- ✅ All storage methods implemented
- ✅ 50+ storage unit tests passing

### Phase 3 (Core Wallet)
- ✅ Can create and sign transactions
- ✅ Can list actions/outputs/certificates
- ✅ Key derivation works correctly

### Phase 4-7 (Services/Monitor/WAB/Utilities)
- ✅ Can connect to external services
- ✅ Monitor daemon runs without crashes
- ✅ 100+ unit tests passing

### Phase 8 (FFI/WASM)
- ✅ C example compiles and runs
- ✅ WASM bundle loads in browser
- ✅ TypeScript definitions generated

### Phase 9 (Testing)
- ✅ 80%+ code coverage
- ✅ All critical paths tested
- ✅ Performance within 2x of TS version

### Phase 10 (Documentation)
- ✅ All public APIs documented
- ✅ 5+ complete examples
- ✅ Migration guide published

## Risk Management

### High Priority Risks

**1. BSV SDK Dependency**
- **Risk:** Rust SDK may be incomplete or incompatible
- **Mitigation:** Create adapter layer, contribute to SDK if needed
- **Fallback:** Implement minimal SDK types in-crate

**2. Async Complexity**
- **Risk:** TypeScript Promises vs. Rust async may have subtle differences
- **Mitigation:** Extensive integration testing, use tokio best practices
- **Fallback:** Blocking APIs with async wrappers

**3. WASM Size**
- **Risk:** Bundle too large for web deployment
- **Mitigation:** Use wasm-opt, split features, lazy loading
- **Fallback:** Server-side wallet with thin client

**4. FFI Safety**
- **Risk:** Memory leaks or crashes in C API
- **Mitigation:** Extensive valgrind testing, careful ownership design
- **Fallback:** Higher-level IPC API (gRPC/REST)

### Medium Priority Risks

**5. Test Porting Effort**
- **Risk:** 1000+ tests may take longer than estimated
- **Mitigation:** Focus on integration tests, automate where possible
- **Fallback:** Prioritize critical path tests

**6. Performance**
- **Risk:** Rust version slower than expected
- **Mitigation:** Profile early, optimize hot paths
- **Fallback:** Acceptable if within 2x of TS

## Resource Requirements

### Development
- 1 Senior Rust developer (full-time, 17 weeks)
- OR 2 Mid-level Rust developers (full-time, 12 weeks)

### Infrastructure
- CI/CD pipeline for multi-target builds
- Test environments (SQLite, MySQL, browser)
- Mobile build toolchains (iOS, Android)

### External Dependencies
- BSV SDK Rust version
- Storage backend libraries (sqlx, wasm-bindgen)
- Build tools (cargo, wasm-pack, cbindgen, uniffi)

## Deliverables

### Code Artifacts
1. `wallet-toolbox-rs` workspace (10 crates)
2. Native library (`.a`, `.so`, `.dylib`)
3. WASM package (`npm` package)
4. Mobile frameworks (`.xcframework`, `.aar`)

### Documentation
1. API reference (Rustdoc)
2. TypeScript definitions (`.d.ts`)
3. C header files (`.h`)
4. Usage examples (5+ projects)
5. Migration guide (TS → Rust)

### Testing
1. Unit test suite (500+ tests)
2. Integration test suite (100+ tests)
3. Performance benchmarks
4. Example applications

## Getting Started

### For Contributors

```bash
# Clone and build
git clone <repo>
cd wallet-toolbox-rs
cargo build --workspace

# Run tests
cargo test --workspace

# Check specific crate
cd crates/wallet-core
cargo check
cargo test
```

### For API Users

**Native (C/C++):**
```bash
cargo build --release -p wallet-client
# Use target/release/libwallet_client.a
```

**Web (WASM):**
```bash
cd crates/wallet-web
wasm-pack build --target web
# Use pkg/ directory in your web project
```

**Mobile:**
```bash
# iOS
cargo build --release --target aarch64-apple-ios -p wallet-mobile

# Android  
cargo build --release --target aarch64-linux-android -p wallet-mobile
```

## Communication

### Weekly Updates
- Progress report on completed milestones
- Blockers and risks
- Next week goals

### Decision Log
- Document all major technical decisions
- Rationale and alternatives considered
- Impact assessment

### Code Reviews
- All PRs require review
- Focus on safety, correctness, API design
- Performance considerations

## Next Action

**Start Phase 1:** Begin with `wallet-core/Cargo.toml` dependencies and error type implementation.

See `TRANSLATION_PLAN.md` for detailed task breakdown.
