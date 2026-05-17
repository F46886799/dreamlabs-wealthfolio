use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use std::time::Duration;

use crate::errors::MarketDataError;
use crate::models::{
    ProviderInstrument, Quote, QuoteContext,
};
use crate::provider::{MarketDataProvider, ProviderCapabilities, RateLimit};

mod models;
use models::{EastMoneyKlineResponse, EastMoneyQuoteResponse};

const EASTMONEY_QUOTE_URL: &str = "https://push2.eastmoney.com/api/qt/ulist.np/get";
const EASTMONEY_KLINE_URL: &str = "https://push2his.eastmoney.com/api/qt/stock/kline/get";

/// EastMoney (东方财富) provider for Chinese A-shares
pub struct EastMoneyProvider {
    client: reqwest::Client,
}

impl EastMoneyProvider {
    /// Create a new EastMoney provider
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        Self { client }
    }

    /// Extract EastMoney symbol (e.g. "1.600000" for SH, "0.000001" for SZ)
    fn extract_symbol(&self, instrument: &ProviderInstrument) -> Result<String, MarketDataError> {
        match instrument {
            ProviderInstrument::EquitySymbol { symbol } => Ok(symbol.to_string()),
            _ => Err(MarketDataError::ResolutionFailed {
                provider: self.id().to_string(),
            }),
        }
    }
}

impl Default for EastMoneyProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MarketDataProvider for EastMoneyProvider {
    fn id(&self) -> &'static str {
        "EASTMONEY"
    }

    fn priority(&self) -> u8 {
        // Fallback for Chinese markets
        10
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            instrument_kinds: &[crate::models::InstrumentKind::Equity],
            coverage: crate::models::Coverage::global_best_effort(),
            supports_latest: true,
            supports_historical: true,
            supports_search: false,
            supports_profile: false,
        }
    }

    fn rate_limit(&self) -> RateLimit {
        RateLimit {
            requests_per_minute: 100, // Be kind to public undocumented APIs
            max_concurrency: 3,
            min_delay: Duration::from_millis(200),
        }
    }

    async fn get_latest_quote(
        &self,
        context: &QuoteContext,
        instrument: ProviderInstrument,
    ) -> Result<Quote, MarketDataError> {
        let em_code = self.extract_symbol(&instrument)?;

        let query = vec![
            ("fltt", "2"),
            ("fields", "f43,f44,f45,f46,f47,f124"),
            ("secids", &em_code),
        ];

        let resp = self
            .client
            .get(EASTMONEY_QUOTE_URL)
            .query(&query)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    MarketDataError::Timeout {
                        provider: self.id().to_string(),
                    }
                } else {
                    MarketDataError::ProviderError {
                        provider: self.id().to_string(),
                        message: e.to_string(),
                    }
                }
            })?;

        if !resp.status().is_success() {
             return Err(MarketDataError::ProviderError {
                 provider: self.id().to_string(),
                 message: format!("HTTP {}", resp.status()),
             });
        }

        let body: EastMoneyQuoteResponse = resp.json().await.map_err(|e| {
             MarketDataError::ProviderError {
                 provider: self.id().to_string(),
                 message: format!("Failed to parse EastMoney JSON: {}", e),
             }
        })?;

        if body.rc != 0 {
             return Err(MarketDataError::ProviderError {
                 provider: self.id().to_string(),
                 message: format!("EastMoney Error code: {}", body.rc),
             });
        }

        let data = body.data.ok_or_else(|| MarketDataError::NoDataForRange)?;

        let diff = data.diff.first().ok_or_else(|| MarketDataError::NoDataForRange)?;

        // Fallback to now if f124 lacks a valid timestamp
        let timestamp = diff.f124.and_then(|ts| Utc.timestamp_opt(ts, 0).single()).unwrap_or_else(Utc::now);

        let currency = context.currency_hint.as_deref().unwrap_or("CNY").to_string();

        Ok(Quote {
            timestamp,
            open: diff.f46.and_then(Decimal::from_f64),
            high: diff.f44.and_then(Decimal::from_f64),
            low: diff.f45.and_then(Decimal::from_f64),
            close: diff.f43.and_then(Decimal::from_f64).unwrap_or(Decimal::ZERO), // fallback if not available
            volume: diff.f47.and_then(Decimal::from_f64),
            currency,
            source: self.id().to_string(),
        })
    }

    async fn get_historical_quotes(
        &self,
        context: &QuoteContext,
        instrument: ProviderInstrument,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Quote>, MarketDataError> {
        let em_code = self.extract_symbol(&instrument)?;

        let beg = start.format("%Y%m%d").to_string();
        let end_date = end.format("%Y%m%d").to_string();

        let query = vec![
            ("secid", em_code.as_str()),
            ("klt", "101"), // Daily k-line
            ("fqt", "1"),   // Forward adj
            ("beg", &beg),
            ("end", &end_date),
        ];

        let resp = self
            .client
            .get(EASTMONEY_KLINE_URL)
            .query(&query)
            .send()
            .await
            .map_err(|e| MarketDataError::ProviderError {
                provider: self.id().to_string(),
                message: e.to_string(),
            })?;

        let body: EastMoneyKlineResponse = resp.json().await.map_err(|e| {
             MarketDataError::ProviderError {
                 provider: self.id().to_string(),
                 message: format!("Failed to parse EastMoney Kline JSON: {}", e),
             }
        })?;

        let data = body.data.ok_or_else(|| MarketDataError::NoDataForRange)?;

        let currency = context.currency_hint.as_deref().unwrap_or("CNY").to_string();

        let mut quotes = Vec::new();
        for k_str in data.klines {
            let parts: Vec<&str> = k_str.split(',').collect();
            if parts.len() >= 6 {
                // Formatting: Date, Open, Close, High, Low, Volume
                if let Ok(date) = NaiveDate::parse_from_str(parts[0], "%Y-%m-%d") {
                    let ts = date.and_hms_opt(15, 0, 0).unwrap().and_utc();
                    let open = parts[1].parse::<f64>().ok().and_then(Decimal::from_f64);
                    let close = parts[2].parse::<f64>().unwrap_or(0.0);
                    let high = parts[3].parse::<f64>().ok().and_then(Decimal::from_f64);
                    let low = parts[4].parse::<f64>().ok().and_then(Decimal::from_f64);
                    let volume = parts[5].parse::<f64>().ok().and_then(Decimal::from_f64);

                    quotes.push(Quote {
                        timestamp: ts,
                        open,
                        high,
                        low,
                        close: Decimal::from_f64(close).unwrap_or(Decimal::ZERO),
                        volume,
                        currency: currency.clone(),
                        source: self.id().to_string(),
                    });
                }
            }
        }

        quotes.sort_by_key(|q| q.timestamp);
        Ok(quotes)
    }
}