# wallet-toolbox-rs

Rust translation of the @bsv/wallet-toolbox codebase.

Workspace crates:
- wallet-core
- wallet-storage
- wallet-storage-sqlite
- wallet-storage-mysql
- wallet-storage-indexeddb
- wallet-monitor
- wallet-wab-client
- wallet-web
- wallet-client
- wallet-mobile

Build:
- Native: `cargo build --workspace`
- WASM (client/mobile crates will be wasm-ready later): TBD
