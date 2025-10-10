# wallet-toolbox-rs

Rust implementation of BSV `@bsv/wallet-toolbox` with **perfect functional parity**.

## 🎯 Goal

Seamless drop-in replacement for TypeScript wallet-toolbox - existing wallets should work with minimal changes.

## 📊 Status

**Progress**: 95% Complete  
**Build**: ✅ GREEN (compiles successfully)  
**Current Phase**: Core functionality restored and compiling

## 📚 Quick Navigation

**For Current Work**:
- [`RESTORATION_SUCCESS.md`](RESTORATION_SUCCESS.md) - Latest restoration status
- [`PROJECT_STATUS.md`](PROJECT_STATUS.md) - Current progress and what's done
- [`PHASES.md`](PHASES.md) - Overall phase structure

**For Understanding**:
- [`ROADMAP.md`](ROADMAP.md) - Long-term plan
- [`SDK_INTERFACES_SUMMARY.md`](SDK_INTERFACES_SUMMARY.md) - API overview

## 🏗️ Structure

```
crates/
├── wallet-core/        Core wallet logic (8,500+ lines)
├── wallet-storage/     Storage layer (2,000+ lines)  
├── wallet-services/    External services (1,200+ lines)
└── wallet-cli/         Examples (150 lines)
```

## 🚀 Build

```bash
cargo build
cargo test
cargo run -p wallet-cli
```

## 🎯 Next Milestone

**Complete remaining implementation details**

See [`RESTORATION_SUCCESS.md`](RESTORATION_SUCCESS.md) for detailed status.
