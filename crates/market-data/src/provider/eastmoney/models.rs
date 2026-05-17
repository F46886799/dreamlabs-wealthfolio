use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct EastMoneyQuoteResponse {
    pub data: Option<EastMoneyQuoteData>,
    pub rc: i32,  // 0 is usually success
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastMoneyQuoteData {
    pub diff: Vec<EastMoneyQuoteDiff>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastMoneyQuoteDiff {
    pub f43: Option<f64>, // Close
    pub f44: Option<f64>, // High
    pub f45: Option<f64>, // Low
    pub f46: Option<f64>, // Open
    pub f47: Option<f64>, // Volume
    pub f124: Option<i64>, // Timestamp / Unix Time sometimes
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastMoneyKlineResponse {
    pub data: Option<EastMoneyKlineData>,
    pub rc: i32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastMoneyKlineData {
    pub code: String,
    pub market: i32,
    pub name: String,
    pub klines: Vec<String>, // format: "2023-01-01,10.0,11.0,9.0,10.5,100000,..."
}
