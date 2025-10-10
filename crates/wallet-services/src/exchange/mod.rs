//! Exchange Rate Service Module
//!
//! **Reference**: TypeScript `src/services/providers/exchangeRates.ts`
//! **Reference**: TypeScript `src/services/providers/WhatsOnChain.ts` (updateBsvExchangeRate)
//!
//! Provides BSV and fiat exchange rate fetching

pub mod types;
pub mod whatsonchain;
pub mod exchangeratesapi;

pub use types::*;
pub use whatsonchain::WhatsOnChainExchangeRate;
pub use exchangeratesapi::ExchangeRatesApiClient;
