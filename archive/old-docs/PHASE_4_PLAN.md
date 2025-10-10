# Phase 4: Services Layer - Implementation Plan

**Status**: 🚀 STARTING  
**Duration**: Week 8-10 (Estimated 3 weeks)  
**Complexity**: ⭐⭐⭐ Medium (easier than Phase 3!)  
**Dependencies**: Phase 3 (100% complete ✅)

---

## 🎯 Overview

Phase 4 implements the **service layer** that connects the wallet to external blockchain services. This layer handles:
- Blockchain state tracking (ChainTracker)
- Transaction broadcasting
- UTXO status checking
- Exchange rates
- Raw transaction fetching
- Merkle proof retrieval

**Reference**: TypeScript `src/sdk/WalletServices.interfaces.ts` (690 lines)

---

## 📋 Phase 4 Scope

### 4.1 Service Interfaces (Week 8, Day 1-2)
**Goal**: Define all service trait interfaces

**TypeScript Reference**: `src/sdk/WalletServices.interfaces.ts`

#### Core Interfaces
1. **WalletServices** trait (main interface)
   - Chain configuration
   - Service provider management
   - Result types for all operations

2. **Service Result Types** (15+ types)
   - `GetRawTxResult`
   - `GetMerklePathResult`
   - `PostBeefResult`
   - `GetUtxoStatusResult`
   - `GetStatusForTxidsResult`
   - `GetScriptHashHistoryResult`
   - `PostRawTxResult`
   - `GetBlockHeaderResult`
   - `GetMerkleProofResult`
   - `PostRawTxsResult`
   - `GetBalanceResult`

3. **Service Provider** trait
   - Common interface for all providers
   - Error handling
   - Rate limiting support

**Deliverables**:
- `crates/wallet-services/src/traits.rs` - Service traits
- `crates/wallet-services/src/types.rs` - Result types
- 15+ type definitions
- 5+ tests

---

### 4.2 ChainTracker Service (Week 8, Day 3-5)
**Goal**: Implement blockchain state tracking

**TypeScript Reference**: 
- `src/services/chaintracker/BHServiceClient.ts`
- `src/services/chaintracker/chaintracks/ChaintracksService.ts`

#### Components

1. **ChainTracker Trait**
   ```rust
   pub trait ChainTracker: Send + Sync {
       async fn is_valid_root_for_height(&self, root: &str, height: u32) -> Result<bool>;
       async fn get_header_for_height(&self, height: u32) -> Result<Vec<u8>>;
       async fn get_height(&self) -> Result<u32>;
       async fn get_merkle_path(&self, txid: &str) -> Result<MerklePath>;
   }
   ```

2. **ChaintracksClient** (Primary Implementation)
   - HTTP client for Chaintracks service
   - Merkle proof verification
   - Block header caching
   - Height tracking

3. **BlockHeaders Service**
   - Header validation
   - Chain reorganization detection
   - SPV verification support

**Deliverables**:
- `crates/wallet-services/src/chaintracker/mod.rs`
- `crates/wallet-services/src/chaintracker/chaintracks.rs`
- `crates/wallet-services/src/chaintracker/types.rs`
- 10+ tests

---

### 4.3 Broadcaster Service (Week 9, Day 1-3)
**Goal**: Implement transaction broadcasting

**TypeScript Reference**: 
- `src/services/ServiceCollection.ts` (postRawTx, postBeef)

#### Components

1. **Broadcaster Trait**
   ```rust
   pub trait Broadcaster: Send + Sync {
       async fn post_raw_tx(&self, raw_tx: &[u8]) -> Result<PostRawTxResult>;
       async fn post_beef(&self, beef: &[u8], txids: &[String]) -> Result<Vec<PostBeefResult>>;
       async fn get_status_for_txids(&self, txids: &[String]) -> Result<GetStatusForTxidsResult>;
   }
   ```

2. **ARC Broadcaster** (Primary Implementation)
   - ARC (BEEF) transaction submission
   - Status polling
   - Double-spend detection
   - MAPI compatibility

3. **Fallback Logic**
   - Multiple service providers
   - Automatic failover
   - Service health tracking

**Deliverables**:
- `crates/wallet-services/src/broadcaster/mod.rs`
- `crates/wallet-services/src/broadcaster/arc.rs`
- `crates/wallet-services/src/broadcaster/fallback.rs`
- 8+ tests

---

### 4.4 UTXO Services (Week 9, Day 4-5)
**Goal**: Implement UTXO status checking

**TypeScript Reference**: 
- `WalletServices.getUtxoStatus`
- `WalletServices.isUtxo`
- `WalletServices.getScriptHashHistory`

#### Components

1. **UTXO Status Checker**
   - Script hash computation
   - UTXO status queries
   - Balance checking
   - History retrieval

2. **WhatsOnChain Provider**
   - HTTP client implementation
   - Rate limiting
   - Error handling

**Deliverables**:
- `crates/wallet-services/src/utxo/mod.rs`
- `crates/wallet-services/src/utxo/whatsonchain.rs`
- 6+ tests

---

### 4.5 Exchange Rate Service (Week 10, Day 1-2)
**Goal**: Implement fiat exchange rate fetching

**TypeScript Reference**: 
- `WalletServices.getBsvExchangeRate`
- `WalletServices.getFiatExchangeRate`

#### Components

1. **Exchange Rate Provider**
   - BSV/USD rate
   - Multi-currency support (USD, GBP, EUR)
   - Rate caching
   - Fallback providers

2. **CoinGecko Integration**
   - HTTP client
   - Response parsing
   - Error handling

**Deliverables**:
- `crates/wallet-services/src/exchange/mod.rs`
- `crates/wallet-services/src/exchange/coingecko.rs`
- 4+ tests

---

### 4.6 Service Collection (Week 10, Day 3-5)
**Goal**: Implement main WalletServices

**TypeScript Reference**: 
- `src/services/ServiceCollection.ts`
- `src/services/Services.ts`

#### Components

1. **ServiceCollection Struct**
   - Aggregates all services
   - Configuration management
   - Service lifecycle
   - Chain selection (mainnet/testnet)

2. **Service Registry**
   - Multiple provider support
   - Priority-based fallback
   - Health monitoring

3. **Integration**
   - Complete WalletServices implementation
   - All methods implemented
   - Error aggregation

**Deliverables**:
- `crates/wallet-services/src/collection.rs`
- `crates/wallet-services/src/config.rs`
- `crates/wallet-services/src/lib.rs`
- 10+ integration tests

---

## 📊 Success Criteria

### Code Metrics
- **Production Code**: 2,500+ lines
- **Test Code**: 500+ lines
- **Tests**: 50+ passing (100%)
- **Modules**: 6 complete
- **TypeScript Parity**: 100%

### Functional Requirements
- ✅ All WalletServices methods implemented
- ✅ ChainTracker fully functional
- ✅ Transaction broadcasting working
- ✅ UTXO status checking accurate
- ✅ Exchange rates retrievable
- ✅ Fallback/retry logic robust
- ✅ Error handling comprehensive

### Quality Requirements
- ✅ Zero compilation errors
- ✅ All tests passing
- ✅ TypeScript references throughout
- ✅ Comprehensive documentation
- ✅ Async/await properly used
- ✅ HTTP clients with timeouts
- ✅ Rate limiting implemented

---

## 🗂️ File Structure

```
wallet-toolbox-rs/
├── crates/
│   └── wallet-services/              # NEW CRATE
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs                 # Re-exports
│           ├── traits.rs              # Service traits (200 lines)
│           ├── types.rs               # Result types (300 lines)
│           ├── config.rs              # Configuration (150 lines)
│           ├── collection.rs          # ServiceCollection (400 lines)
│           │
│           ├── chaintracker/
│           │   ├── mod.rs
│           │   ├── types.rs           # MerklePath, etc
│           │   ├── chaintracks.rs     # Chaintracks client (300 lines)
│           │   └── headers.rs         # Header validation (200 lines)
│           │
│           ├── broadcaster/
│           │   ├── mod.rs
│           │   ├── arc.rs             # ARC broadcaster (250 lines)
│           │   └── fallback.rs        # Multi-provider (150 lines)
│           │
│           ├── utxo/
│           │   ├── mod.rs
│           │   ├── whatsonchain.rs    # WoC client (200 lines)
│           │   └── status.rs          # Status checker (150 lines)
│           │
│           └── exchange/
│               ├── mod.rs
│               ├── coingecko.rs       # CoinGecko client (150 lines)
│               └── cache.rs           # Rate cache (100 lines)
```

---

## 📦 Dependencies

### New Cargo Dependencies
```toml
[dependencies]
# Existing
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
async-trait = "0.1"

# HTTP clients
reqwest = { version = "0.11", features = ["json"] }
hyper = "0.14"

# Utilities
url = "2.5"
base64 = "0.21"
hex = "0.4"

# Caching
moka = { version = "0.12", features = ["future"] }

# Time
chrono = "0.4"
```

---

## 🎯 Weekly Breakdown

### Week 8: Core Service Infrastructure
**Days 1-2**: Service interfaces & types
**Days 3-5**: ChainTracker implementation

**Deliverables**:
- Service traits complete
- ChainTracker working
- 20+ tests passing

### Week 9: Broadcasting & UTXO
**Days 1-3**: Broadcaster service
**Days 4-5**: UTXO status checking

**Deliverables**:
- Broadcasting functional
- UTXO checking working
- 15+ tests passing

### Week 10: Integration & Polish
**Days 1-2**: Exchange rate service
**Days 3-5**: ServiceCollection & integration

**Deliverables**:
- All services integrated
- 50+ tests passing
- Phase 4 complete!

---

## 🧪 Testing Strategy

### Unit Tests (30+)
- Service trait implementations
- HTTP client responses
- Error handling
- Rate limiting
- Caching behavior

### Integration Tests (20+)
- Real service calls (with mocks)
- Fallback logic
- Multi-provider scenarios
- End-to-end workflows

### Mock Services
- TestChainTracker
- TestBroadcaster
- TestUtxoService
- Deterministic responses

---

## 🚀 Quick Start Plan

### Step 1: Create New Crate
```bash
cd crates
cargo new --lib wallet-services
```

### Step 2: Set Up Traits
- Define WalletServices trait
- Define service result types
- Add basic tests

### Step 3: Implement ChainTracker
- Chaintracks HTTP client
- Merkle path verification
- Integration with Phase 3

### Step 4: Build Out Services
- Broadcaster
- UTXO checker
- Exchange rates

### Step 5: Service Collection
- Aggregate all services
- Configuration
- Integration tests

---

## 💡 Key Insights

### Why Phase 4 Is Easier Than Phase 3
1. **Less Complex Logic**: Mostly HTTP clients & data transformation
2. **Well-Defined Interfaces**: TypeScript provides clear contracts
3. **Independent Components**: Services don't depend on each other much
4. **Proven Patterns**: Standard REST/HTTP client patterns
5. **No Cryptography**: No ECDSA, no key derivation complexity

### Challenges to Watch
1. **HTTP Error Handling**: Network failures, timeouts
2. **Rate Limiting**: API quotas and throttling
3. **Caching Strategy**: When to cache, TTL management
4. **Fallback Logic**: When to retry, when to fail
5. **Testing**: Mocking external services

---

## 📈 Progress Tracking

```
Phase 4 Checklist:
[ ] 4.1 Service Interfaces (2 days)
[ ] 4.2 ChainTracker (3 days)
[ ] 4.3 Broadcaster (3 days)
[ ] 4.4 UTXO Services (2 days)
[ ] 4.5 Exchange Rates (2 days)
[ ] 4.6 Service Collection (3 days)
Total: ~15 days = 3 weeks
```

---

## 🎉 Phase 4 Completion Criteria

Phase 4 is complete when:
- ✅ All WalletServices methods implemented
- ✅ 50+ tests passing (100%)
- ✅ Zero compilation errors
- ✅ ChainTracker functional
- ✅ Broadcasting functional
- ✅ UTXO checking functional
- ✅ Exchange rates working
- ✅ Fallback logic robust
- ✅ Documentation complete
- ✅ Integration with Phase 3 tested

**Then we move to Phase 5: Integration (Wallet Manager, Auth, Sync)!**

---

**Let's build the service layer!** 🚀
