//! Exchange rate types
//!
//! **Reference**: TypeScript `src/sdk/WalletServices.interfaces.ts` (lines 183-193)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// BSV exchange rate
///
/// Reference: TS BsvExchangeRate interface (WalletServices.interfaces.ts lines 183-187)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BsvExchangeRate {
    /// Timestamp of rate
    pub timestamp: DateTime<Utc>,
    
    /// Base currency (always USD)
    pub base: String,
    
    /// Exchange rate (USD per BSV)
    pub rate: f64,
}

/// Fiat exchange rates
///
/// Reference: TS FiatExchangeRates interface (WalletServices.interfaces.ts lines 189-193)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatExchangeRates {
    /// Timestamp of rates
    pub timestamp: DateTime<Utc>,
    
    /// Base currency (always USD)
    pub base: String,
    
    /// Exchange rates map (currency code -> rate)
    pub rates: HashMap<String, f64>,
}

/// WhatsOnChain exchange rate response
///
/// Reference: TS updateBsvExchangeRate response (WhatsOnChain.ts lines 322-326)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsOnChainExchangeRateResponse {
    /// Exchange rate value
    pub rate: f64,
    
    /// Timestamp (unix timestamp)
    pub time: u64,
    
    /// Currency code
    pub currency: String,
}

/// ExchangeRatesAPI response
///
/// Reference: TS ExchangeRatesIoApi interface (exchangeRates.ts lines 63-69)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRatesApiResponse {
    /// Success flag
    pub success: bool,
    
    /// Unix timestamp
    pub timestamp: u64,
    
    /// Base currency
    pub base: String,
    
    /// Date string
    pub date: String,
    
    /// Rates map
    pub rates: HashMap<String, f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bsv_exchange_rate() {
        let rate = BsvExchangeRate {
            timestamp: Utc::now(),
            base: "USD".to_string(),
            rate: 50.0,
        };
        
        assert_eq!(rate.base, "USD");
        assert_eq!(rate.rate, 50.0);
    }
    
    #[test]
    fn test_fiat_exchange_rates() {
        let mut rates = HashMap::new();
        rates.insert("EUR".to_string(), 0.85);
        rates.insert("GBP".to_string(), 0.73);
        
        let fiat_rates = FiatExchangeRates {
            timestamp: Utc::now(),
            base: "USD".to_string(),
            rates,
        };
        
        assert_eq!(fiat_rates.rates.len(), 2);
        assert_eq!(fiat_rates.rates.get("EUR"), Some(&0.85));
    }
}
