//! WhatsOnChain Exchange Rate Provider
//!
//! **Reference**: TypeScript `src/services/providers/WhatsOnChain.ts` (updateBsvExchangeRate, lines 309-348)

use async_trait::async_trait;
use reqwest::Client;
use chrono::Utc;
use crate::error::{ServiceError, ServiceResult};
use crate::traits::ExchangeRateProvider;
use crate::traits::FiatCurrency;
use crate::types::Chain;
use super::types::{BsvExchangeRate, WhatsOnChainExchangeRateResponse};

/// WhatsOnChain exchange rate provider
///
/// Reference: TS WhatsOnChain.updateBsvExchangeRate
pub struct WhatsOnChainExchangeRate {
    /// Chain (main or test)
    chain: Chain,
    
    /// Base URL
    url: String,
    
    /// HTTP client
    client: Client,
    
    /// Cached rate
    cached_rate: Option<BsvExchangeRate>,
    
    /// Update interval (milliseconds)
    update_msecs: u64,
}

impl WhatsOnChainExchangeRate {
    /// Create new WhatsOnChain exchange rate provider
    ///
    /// Reference: TS WhatsOnChain constructor + updateBsvExchangeRate
    ///
    /// # Arguments
    /// * `chain` - Chain to query
    pub fn new(chain: Chain) -> Self {
        let url = match chain {
            Chain::Main => "https://api.whatsonchain.com/v1/bsv/main",
            Chain::Test => "https://api.whatsonchain.com/v1/bsv/test",
        };
        
        Self {
            chain,
            url: url.to_string(),
            client: Client::new(),
            cached_rate: None,
            update_msecs: 1000 * 60 * 15, // 15 minutes (TS line 312)
        }
    }
    
    /// Update BSV exchange rate
    ///
    /// Reference: TS WhatsOnChain.updateBsvExchangeRate (lines 309-348)
    pub async fn update_bsv_exchange_rate(&mut self) -> ServiceResult<BsvExchangeRate> {
        // Check if cached rate is still fresh (TS lines 310-314)
        if let Some(ref rate) = self.cached_rate {
            let age = Utc::now()
                .signed_duration_since(rate.timestamp)
                .num_milliseconds() as u64;
            
            if age < self.update_msecs {
                return Ok(rate.clone());
            }
        }
        
        // Fetch new rate with retry (TS lines 321-346)
        for retry in 0..2 {
            let url = format!("{}/exchangerate", self.url);
            
            match self.client.get(&url).send().await {
                Ok(response) => {
                    // Handle rate limit (TS lines 327-330)
                    if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS && retry < 2 {
                        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
                        continue;
                    }
                    
                    // Check success (TS lines 333-334)
                    if !response.status().is_success() {
                        return Err(ServiceError::ServiceFailed {
                            service: "WhatsOnChain".to_string(),
                            message: format!("HTTP {}", response.status()),
                        });
                    }
                    
                    // Parse response (TS lines 336-343)
                    let woc_rate: WhatsOnChainExchangeRateResponse = response.json().await
                        .map_err(ServiceError::Http)?;
                    
                    // Validate currency (TS lines 337-338)
                    if woc_rate.currency != "USD" {
                        return Err(ServiceError::InvalidResponse(
                            "Currency is not USD".to_string()
                        ));
                    }
                    
                    // Build result (TS lines 339-343)
                    let new_rate = BsvExchangeRate {
                        timestamp: Utc::now(),
                        base: "USD".to_string(),
                        rate: woc_rate.rate,
                    };
                    
                    // Cache the rate
                    self.cached_rate = Some(new_rate.clone());
                    
                    return Ok(new_rate);
                }
                Err(e) => {
                    if retry >= 1 {
                        return Err(ServiceError::Http(e));
                    }
                }
            }
        }
        
        // All retries exhausted (TS line 347)
        Err(ServiceError::ServiceFailed {
            service: "WhatsOnChain".to_string(),
            message: "All retries exhausted".to_string(),
        })
    }
}

#[async_trait]
impl ExchangeRateProvider for WhatsOnChainExchangeRate {
    /// Get BSV/USD exchange rate
    async fn get_bsv_rate(&self) -> ServiceResult<f64> {
        // Clone to allow mutation
        let mut provider = WhatsOnChainExchangeRate::new(self.chain);
        provider.cached_rate = self.cached_rate.clone();
        provider.update_msecs = self.update_msecs;
        
        let rate = provider.update_bsv_exchange_rate().await?;
        Ok(rate.rate)
    }
    
    /// Get fiat exchange rate
    async fn get_fiat_rate(
        &self,
        _currency: FiatCurrency,
        _base: Option<FiatCurrency>,
    ) -> ServiceResult<f64> {
        // WhatsOnChain doesn't provide fiat rates
        Err(ServiceError::InvalidParams(
            "WhatsOnChain does not support fiat exchange rates".to_string()
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
    fn test_whatsonchain_exchange_rate_creation() {
        let provider = WhatsOnChainExchangeRate::new(Chain::Main);
        assert_eq!(provider.chain, Chain::Main);
        assert_eq!(provider.update_msecs, 1000 * 60 * 15);
    }
    
    #[test]
    fn test_cache_freshness() {
        let mut provider = WhatsOnChainExchangeRate::new(Chain::Main);
        
        // Set a recent cached rate
        provider.cached_rate = Some(BsvExchangeRate {
            timestamp: Utc::now(),
            base: "USD".to_string(),
            rate: 50.0,
        });
        
        // Check that cached rate exists
        assert!(provider.cached_rate.is_some());
    }
}
