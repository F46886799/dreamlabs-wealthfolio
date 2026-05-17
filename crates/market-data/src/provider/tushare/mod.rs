use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::time::Duration;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::errors::MarketDataError;
use crate::models::{
    ProviderInstrument, Quote, QuoteContext,
};
use crate::provider::{MarketDataProvider, ProviderCapabilities, RateLimit};

mod models;
use models::{DailyParams, TushareRequest, TushareResponse, parse_tushare_date};

const TUSHARE_API_URL: &str = "http://api.tushare.pro";

/// Tushare provider for Chinese A-shares
pub struct TushareProvider {
    client: reqwest::Client,
    api_token: String,
}

impl TushareProvider {
    /// Create a new Tushare provider
    pub fn new(api_token: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .unwrap_or_default();

        Self { client, api_token }
    }

    /// Extract Tushare code strictly
    fn extract_symbol(&self, instrument: &ProviderInstrument) -> Result<String, MarketDataError> {
        match instrument {
            ProviderInstrument::EquitySymbol { symbol } => Ok(symbol.to_string()),
            _ => Err(MarketDataError::ResolutionFailed {
                provider: self.id().to_string(),
            }),
        }
    }

    /// Execute a Tushare Request
    async fn fetch_records<'a, T: serde::Serialize>(
        &self,
        api_name: &'a str,
        params: T,
        fields: &'a str,
    ) -> Result<TushareResponse, MarketDataError> {
        let req = TushareRequest {
            api_name,
            token: &self.api_token,
            params,
            fields,
        };

        if self.api_token.is_empty() {
             return Err(MarketDataError::ProviderError {
                 provider: self.id().to_string(),
                 message: "Tushare API token is missing".into(),
             });
        }

        let resp = self
            .client
            .post(TUSHARE_API_URL)
            .json(&req)
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

        let status = resp.status();
        if !status.is_success() {
             return Err(MarketDataError::ProviderError {
                 provider: self.id().to_string(),
                 message: format!("HTTP {}", status),
             });
        }

        let body: TushareResponse = resp.json().await.map_err(|e| {
             MarketDataError::ProviderError {
                 provider: self.id().to_string(),
                 message: format!("Failed to parse Tushare JSON: {}", e),
             }
        })?;

        if body.code != 0 {
             return Err(MarketDataError::ProviderError {
                 provider: self.id().to_string(),
                 message: format!("Tushare Error {}: {}", body.code, body.msg.unwrap_or_default()),
             });
        }

        Ok(body)
    }

    /// Convert Tushare fields and item array into a unified Quote
    fn parse_quote_item(fields: &[String], item: &[serde_json::Value], currency: &str) -> Option<Quote> {
        let mut trade_date = None;
        let mut close = None;
        let mut open = None;
        let mut high = None;
        let mut low = None;
        let mut vol = None;

        for (i, field) in fields.iter().enumerate() {
            if i >= item.len() {
                continue;
            }
            let val = &item[i];

            match field.as_str() {
                "trade_date" => {
                    if let Some(ds) = val.as_str() {
                        trade_date = parse_tushare_date(ds);
                    }
                }
                "open" => open = val.as_f64(),
                "high" => high = val.as_f64(),
                "low" => low = val.as_f64(),
                "close" => close = val.as_f64(),
                "vol" => vol = val.as_f64(),
                _ => {}
            }
        }

        Some(Quote {
            timestamp: trade_date?,
            close: close.and_then(Decimal::from_f64).unwrap_or(Decimal::ZERO),
            open: open.and_then(Decimal::from_f64),
            high: high.and_then(Decimal::from_f64),
            low: low.and_then(Decimal::from_f64),
            volume: vol.and_then(|v| Decimal::from_f64(v * 100.0)), // Tushare volume is in lots (100 shares)
            currency: currency.to_string(),
            source: "TUSHARE".to_string(),
        })
    }
}

#[async_trait]
impl MarketDataProvider for TushareProvider {
    fn id(&self) -> &'static str {
        "TUSHARE"
    }

    fn priority(&self) -> u8 {
        // High priority if configured, excellent for A-shares
        5
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
            requests_per_minute: 200, // Typically generous depending on Tushare points
            max_concurrency: 5,
            min_delay: Duration::from_millis(100),
        }
    }

    async fn get_latest_quote(
        &self,
        context: &QuoteContext,
        instrument: ProviderInstrument,
    ) -> Result<Quote, MarketDataError> {
        let ts_code = self.extract_symbol(&instrument)?;

        let params = DailyParams {
            ts_code: &ts_code,
            start_date: None,
            end_date: None,
        };

        let response = self.fetch_records("daily", params, "trade_date,open,high,low,close,vol").await?;

        let data = response.data.ok_or_else(|| MarketDataError::NoDataForRange)?;

        if data.items.is_empty() {
             return Err(MarketDataError::NoDataForRange);
        }

        let currency = context.currency_hint.as_deref().unwrap_or("CNY");

        // The first item is typically the most recent in Tushare responses
        Self::parse_quote_item(&data.fields, &data.items[0], currency).ok_or_else(|| {
             MarketDataError::ProviderError {
                 provider: self.id().to_string(),
                 message: "Failed to map Tushare row to Quote".into(),
             }
        })
    }

    async fn get_historical_quotes(
        &self,
        context: &QuoteContext,
        instrument: ProviderInstrument,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Quote>, MarketDataError> {
        let ts_code = self.extract_symbol(&instrument)?;

        let start_date = start.format("%Y%m%d").to_string();
        let end_date = end.format("%Y%m%d").to_string();

        let params = DailyParams {
            ts_code: &ts_code,
            start_date: Some(&start_date),
            end_date: Some(&end_date),
        };

        let response = self.fetch_records("daily", params, "trade_date,open,high,low,close,vol").await?;

        let data = response.data.ok_or_else(|| MarketDataError::NoDataForRange)?;

        let currency = context.currency_hint.as_deref().unwrap_or("CNY");

        let mut quotes = Vec::new();
        for item in data.items {
            if let Some(quote) = Self::parse_quote_item(&data.fields, &item, currency) {
                quotes.push(quote);
            }
        }

        // Tushare returns DESC (newest first). Let's sort ASC (oldest first).
        quotes.sort_by_key(|q| q.timestamp);

        Ok(quotes)
    }
}