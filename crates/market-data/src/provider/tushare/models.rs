use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};


/// Request format wrapper for Tushare HTTP API
#[derive(Debug, Serialize)]
pub(crate) struct TushareRequest<'a, T> {
    pub api_name: &'a str,
    pub token: &'a str,
    pub params: T,
    pub fields: &'a str,
}

/// Params for daily quote request
#[derive(Debug, Serialize)]
pub(crate) struct DailyParams<'a> {
    pub ts_code: &'a str,
    pub start_date: Option<&'a str>,
    pub end_date: Option<&'a str>,
}

/// Params for basic stock info
#[derive(Debug, Serialize)]
pub(crate) struct BasicParams<'a> {
    pub ts_code: &'a str,
}

/// Params for query adj factor
#[derive(Debug, Serialize)]
pub(crate) struct AdjFactorParams<'a> {
    pub ts_code: &'a str,
    pub start_date: Option<&'a str>,
    pub end_date: Option<&'a str>,
}

/// A standard Tushare response structure
#[derive(Debug, Deserialize)]
pub(crate) struct TushareResponse {
    pub code: i32,
    pub msg: Option<String>,
    pub data: Option<TushareData>,
}

/// The inner tabular data from Tushare
#[derive(Debug, Deserialize)]
pub(crate) struct TushareData {
    pub fields: Vec<String>,
    pub items: Vec<Vec<serde_json::Value>>,
}

// ── Conversion functions ──────────────────────────────────────────────────

// Parse Tushare date string (YYYYMMDD) into UTC DateTime
pub(crate) fn parse_tushare_date(date_str: &str) -> Option<DateTime<Utc>> {
    NaiveDate::parse_from_str(date_str, "%Y%m%d")
        .ok()
        .map(|nd| nd.and_hms_opt(15, 0, 0).unwrap().and_utc()) // Default to 15:00 market close
}
