//! ExchangeRatesAPI Provider
//!
//! **Reference**: TypeScript `src/services/providers/exchangeRates.ts` (updateExchangeratesapi, lines 26-61)

use async_trait::async_trait;
use reqwest::Client;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use crate::error::{ServiceError, ServiceResult};
use crate::traits::{ExchangeRateProvider, FiatCurrency};
use super::types::{FiatExchangeRates, ExchangeRatesApiResponse};

/// ExchangeRatesAPI client
///
/// Reference: TS updateExchangeratesapi and getExchangeRatesIo
pub struct ExchangeRatesApiClient {
    /// API key
    api_key: String,
    
    /// HTTP client
    client: Client,
    
    /// Cached rates
    cached_rates: Option<FiatExchangeRates>,
    
    /// Update interval (milliseconds)
    update_msecs: u64,
}

impl ExchangeRatesApiClient {
    /// Create new ExchangeRatesAPI client
    ///
    /// Reference: TS getExchangeRatesIo (exchangeRates.ts lines 71-85)
    ///
    /// # Arguments
    /// * `api_key` - API key for exchangeratesapi.io
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            cached_rates: None,
            update_msecs: 1000 * 60 * 60 * 24, // 24 hours
        }
    }
    
    /// Get exchange rates from ExchangeRatesAPI
    ///
    /// Reference: TS getExchangeRatesIo (exchangeRates.ts lines 71-85)
    async fn get_exchange_rates_io(&self) -> ServiceResult<ExchangeRatesApiResponse> {
        // Build URL (TS line 72)
        let url = format!(
            "http://api.exchangeratesapi.io/v1/latest?access_key={}",
            self.api_key
        );
        
        // Make request (TS lines 74-76)
        let response = self.client.get(&url).send().await
            .map_err(ServiceError::Http)?;
        
        // Check status (TS lines 78-80)
        if !response.status().is_success() {
            return Err(ServiceError::ServiceFailed {
                service: "ExchangeRatesAPI".to_string(),
                message: format!("HTTP {}", response.status()),
            });
        }
        
        // Parse response (TS lines 82-84)
        let rates: ExchangeRatesApiResponse = response.json().await
            .map_err(ServiceError::Http)?;
        
        Ok(rates)
    }
    
    /// Update fiat exchange rates for target currencies
    ///
    /// Reference: TS updateExchangeratesapi (exchangeRates.ts lines 26-61)
    ///
    /// # Arguments
    /// * `target_currencies` - List of currency codes to fetch
    pub async fn update_fiat_exchange_rates(
        &mut self,
        target_currencies: &[String],
    ) -> ServiceResult<FiatExchangeRates> {
        // Check if cached rates are still fresh
        if let Some(ref rates) = self.cached_rates {
            let age = Utc::now()
                .signed_duration_since(rates.timestamp)
                .num_milliseconds() as u64;
            
            if age < self.update_msecs {
                return Ok(rates.clone());
            }
        }
        
        // Fetch rates (TS line 32)
        let io_rates = self.get_exchange_rates_io().await?;
        
        // Check success (TS lines 34-35)
        if !io_rates.success {
            return Err(ServiceError::ServiceFailed {
                service: "ExchangeRatesAPI".to_string(),
                message: "API returned success=false".to_string(),
            });
        }
        
        // Validate required rates (TS lines 36-37)
        if !io_rates.rates.contains_key("USD") || !io_rates.rates.contains_key(&io_rates.base) {
            return Err(ServiceError::InvalidResponse(
                "Missing rates for USD or base currency".to_string()
            ));
        }
        
        // Calculate base per USD (TS line 45)
        let base_per_usd = io_rates.rates[&io_rates.base] / io_rates.rates["USD"];
        
        // Convert rates to USD base (TS lines 47-53)
        let mut usd_based_rates = HashMap::new();
        let mut updates = 0;
        
        for currency in target_currencies {
            if let Some(&rate) = io_rates.rates.get(currency) {
                usd_based_rates.insert(currency.clone(), rate * base_per_usd);
                updates += 1;
            }
        }
        
        // Ensure all target currencies were found (TS lines 55-56)
        if updates != target_currencies.len() {
            return Err(ServiceError::InvalidResponse(
                "Failed to update all target currencies".to_string()
            ));
        }
        
        // Build result (TS lines 39-43)
        let result = FiatExchangeRates {
            timestamp: DateTime::from_timestamp(io_rates.timestamp as i64, 0)
                .unwrap_or_else(Utc::now),
            base: "USD".to_string(),
            rates: usd_based_rates,
        };
        
        // Cache the result
        self.cached_rates = Some(result.clone());
        
        Ok(result)
    }
}

#[async_trait]
impl ExchangeRateProvider for ExchangeRatesApiClient {
    /// Get BSV/USD exchange rate
    async fn get_bsv_rate(&self) -> ServiceResult<f64> {
        // ExchangeRatesAPI doesn't provide BSV rates
        Err(ServiceError::InvalidParams(
            "ExchangeRatesAPI does not support BSV exchange rates".to_string()
        ))
    }
    
    /// Get fiat exchange rate
    async fn get_fiat_rate(
        &self,
        currency: FiatCurrency,
        base: Option<FiatCurrency>,
    ) -> ServiceResult<f64> {
        let base = base.unwrap_or(FiatCurrency::USD);
        
        if base != FiatCurrency::USD {
            return Err(ServiceError::InvalidParams(
                "Only USD base is supported".to_string()
            ));
        }
        
        // Clone to allow mutation
        let mut provider = ExchangeRatesApiClient::new(self.api_key.clone());
        provider.cached_rates = self.cached_rates.clone();
        provider.update_msecs = self.update_msecs;
        
        let currencies = vec![currency.as_str().to_string()];
        let rates = provider.update_fiat_exchange_rates(&currencies).await?;
        
        rates.rates.get(currency.as_str())
            .copied()
            .ok_or_else(|| ServiceError::InvalidResponse(
                "Currency not found in response".to_string()
            ))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_exchangeratesapi_client_creation() {
        let client = ExchangeRatesApiClient::new("test-key".to_string());
        assert_eq!(client.api_key, "test-key");
        assert_eq!(client.update_msecs, 1000 * 60 * 60 * 24);
    }
    
    #[test]
    fn test_cache_initialization() {
        let client = ExchangeRatesApiClient::new("test-key".to_string());
        assert!(client.cached_rates.is_none());
    }
}
