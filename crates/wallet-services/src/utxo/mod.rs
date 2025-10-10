//! UTXO Service Module
//!
//! **Reference**: TypeScript `src/services/providers/WhatsOnChain.ts`
//!
//! Provides UTXO status checking and script hash history

pub mod whatsonchain;
pub mod types;
pub mod script_hash;

pub use whatsonchain::WhatsOnChainClient;
pub use types::*;
pub use script_hash::validate_script_hash;
