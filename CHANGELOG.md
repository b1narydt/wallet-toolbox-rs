# Changelog

All notable changes to the wallet-toolbox Rust translation will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [1.3.0] - 2025-01-06

### Added
- **TableOutputTagMap** - Composite key table for output-tag many-to-many relationships
- **TableTxLabelMap** - Composite key table for transaction-label many-to-many relationships
- **EntityOutputTagMap** - Entity wrapper with 10 tests, composite key (no primary id)
- **EntityTxLabelMap** - Entity wrapper with 10 tests, composite key (no primary id)
- **MergeEntity** helper - Generic merge coordinator with 9 tests
- `max_date` utility function for sync operations

### Changed
- **Target Version**: Updated from TypeScript v1.6.8 to v1.6.25
- **SyncMap**: Added `tx_label_map` and `output_tag_map` fields
- **Total Tests**: Increased from 321 to 350 tests
- **Phase 1**: Now 100% complete (was 95%)

### Fixed
- Composite key entities now properly panic on `id()` calls (matching TypeScript behavior)
- SyncMap serialization includes all 12 entity types with proper camelCase field names

## [1.2.0] - 2025-01-06

### Added
- **All 12 Core Entities** with full functional parity
  - EntityUser, EntityTransaction, EntityOutput
  - EntityProvenTx, EntityProvenTxReq
  - EntityCertificate, EntityCertificateField
  - EntityOutputBasket, EntityOutputTag, EntityTxLabel
  - EntityCommission, EntitySyncState
- **EntityBase trait** with equals/merge interface
- **SyncMap and EntitySyncMap** types with PartialEq, Eq
- **SyncError** structured error type

### Changed
- Entity system complete with 122 tests
- Total tests: 321 passing

## [1.1.0] - 2025-01-05

### Added
- **All 11 Core Tables** with full serde serialization
  - TableUser, TableTransaction, TableOutput
  - TableOutputBasket, TableOutputTag, TableTxLabel
  - TableCertificate, TableCertificateField
  - TableProvenTx, TableProvenTxReq
  - TableCommission, TableSyncState
- **Validation Helpers** (30 tests)
- **Storage Traits** (4 tests)

### Changed
- Storage schema 85% complete

## [1.0.0] - 2024-12-20

### Added
- **Initial Workspace Structure** - 10 crates configured
- **Build System** - Cargo workspace setup
- **Error System** - WERR error codes with 23 tests
- **Documentation** - mapping.md TypeScript→Rust guide
- **Basic Scaffolding** for all crates

### Infrastructure
- wallet-core: Error types and placeholder traits
- wallet-storage: Schema infrastructure
- wallet-client, wallet-mobile, wallet-web: Re-export stubs
- wallet-storage-{sqlite,mysql,indexeddb}: Backend stubs
- wallet-monitor, wallet-wab-client: Service stubs
