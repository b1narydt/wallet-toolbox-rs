//! Wallet Services
//!
//! External service integrations for wallet-toolbox-rs
//!
//! This crate provides interfaces to blockchain services:
//! - ChainTracker: Blockchain state tracking
//! - Broadcaster: Transaction broadcasting
//! - UTXO Status: Output spendability checking
//! - Exchange Rates: Fiat currency conversion
//!
//! **Reference**: TypeScript `src/sdk/WalletServices.interfaces.ts`

pub mod error;
pub mod types;
pub mod traits;
pub mod chaintracker;
pub mod broadcaster;
pub mod utxo;
pub mod exchange;
pub mod collection;

// Re-exports
pub use error::{ServiceError, ServiceResult};
pub use types::*;
pub use traits::*;
pub use chaintracker::{ChaintracksClient, BlockHeader, ChaintracksInfo};
pub use broadcaster::{ArcBroadcaster, ArcConfig};
pub use utxo::{WhatsOnChainClient, UtxoDetail, validate_script_hash};
pub use exchange::{BsvExchangeRate, FiatExchangeRates, WhatsOnChainExchangeRate, ExchangeRatesApiClient};
pub use collection::{ServiceCollection, ServiceConfig};
