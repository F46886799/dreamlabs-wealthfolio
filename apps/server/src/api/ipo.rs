use axum::{routing::get, Json, Router};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, REFERER, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::main_lib::AppState;

// ── Simple TTL cache ──────────────────────────────────────────────────────────
const CACHE_TTL: Duration = Duration::from_secs(300); // 5 minutes

struct TtlCache<T> {
    lock: OnceLock<RwLock<Option<(Instant, T)>>>,
}

impl<T: Clone + Send + Sync + 'static> TtlCache<T> {
    const fn new() -> Self {
        Self { lock: OnceLock::new() }
    }

    fn inner(&self) -> &RwLock<Option<(Instant, T)>> {
        self.lock.get_or_init(|| RwLock::new(None))
    }

    async fn get(&self) -> Option<T> {
        let guard = self.inner().read().await;
        guard.as_ref().and_then(|(ts, data)| {
            if ts.elapsed() < CACHE_TTL { Some(data.clone()) } else { None }
        })
    }

    async fn get_stale(&self) -> Option<T> {
        let guard = self.inner().read().await;
        guard.as_ref().map(|(_, data)| data.clone())
    }

    async fn set(&self, value: T) {
        let mut guard = self.inner().write().await;
        *guard = Some((Instant::now(), value));
    }
}

const JISILU_HKIPO_URL: &str = "https://www.jisilu.cn/data/new_stock/hkipo/";
const JISILU_REFERER: &str = "https://www.jisilu.cn/data/new_stock/";
const JISILU_UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

fn jisilu_headers(referer: &'static str) -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(REFERER, HeaderValue::from_static(referer));
    h.insert(USER_AGENT, HeaderValue::from_static(JISILU_UA));
    h
}

// ── Jisilu raw response types ─────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct JiSiLuResponse {
    rows: Vec<JiSiLuRow>,
}

#[derive(Debug, Deserialize)]
struct JiSiLuRow {
    id: String,
    cell: JiSiLuCell,
}

#[derive(Debug, Deserialize)]
struct JiSiLuCell {
    stock_cd: String,
    stock_nm: String,
    #[serde(default)]
    market: Option<String>,
    #[serde(default)]
    apply_dt: Option<String>,
    #[serde(default)]
    apply_end_dt: Option<String>,
    #[serde(default)]
    list_dt: Option<String>,
    #[serde(default)]
    price_range: Option<String>,
    #[serde(default)]
    issue_price: Option<String>,
    #[serde(default)]
    single_draw_money: Option<String>,
    #[serde(default)]
    lucky_draw_rt: Option<serde_json::Value>,
    #[serde(default)]
    raise_money: Option<String>,
    #[serde(default)]
    first_incr_rt: Option<serde_json::Value>,
    #[serde(default)]
    underwriter: Option<String>,
    #[serde(default)]
    prospectus: Option<String>,
    #[serde(default)]
    apply_flg: Option<i32>,
    #[serde(default)]
    list_flg: Option<i32>,
}

// ── Our response type ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HkIpoRecord {
    pub id: String,
    pub code: String,
    pub name: String,
    pub board: String,
    pub subscription_start: String,
    pub subscription_end: String,
    pub listing_date: String,
    /// Price range or final issue price (HKD)
    pub issue_price: String,
    /// Total funds raised (亿 HKD)
    pub issue_size: String,
    /// Minimum subscription amount per lot (HKD)
    pub lot_size: String,
    /// Win rate (中签率), empty string if not yet available
    pub win_rate: String,
    /// First day change, empty if not yet listed
    pub first_day_change: String,
    pub underwriter: String,
    pub prospectus_url: Option<String>,
    /// "upcoming" | "open" | "closed" | "listed"
    pub status: String,
}

fn to_display(v: &Option<serde_json::Value>) -> String {
    match v {
        Some(serde_json::Value::String(s)) => {
            // Jisilu wraps member-only fields in HTML; show "-" instead
            if s.contains("<a href") {
                "-".to_string()
            } else {
                s.clone()
            }
        }
        Some(serde_json::Value::Number(n)) => n.to_string(),
        _ => "-".to_string(),
    }
}

fn derive_status(apply_flg: Option<i32>, list_flg: Option<i32>) -> String {
    match (list_flg.unwrap_or(0), apply_flg.unwrap_or(0)) {
        (1, _) => "listed".to_string(),
        (_, 1) => "open".to_string(),
        _ => "upcoming".to_string(),
    }
}

fn map_row(row: JiSiLuRow) -> HkIpoRecord {
    let c = row.cell;
    let issue_price = c
        .issue_price
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| c.price_range.clone().unwrap_or_default());

    HkIpoRecord {
        id: row.id,
        code: c.stock_cd,
        name: c.stock_nm,
        board: c.market.unwrap_or_default(),
        subscription_start: c.apply_dt.unwrap_or_default(),
        subscription_end: c.apply_end_dt.unwrap_or_default(),
        listing_date: c.list_dt.unwrap_or_default(),
        issue_price,
        issue_size: c.raise_money.unwrap_or_default(),
        lot_size: c
            .single_draw_money
            .map(|s| format!("HK${s}"))
            .unwrap_or_default(),
        win_rate: to_display(&c.lucky_draw_rt.map(|v| v)),
        first_day_change: to_display(&c.first_incr_rt.map(|v| v)),
        underwriter: c.underwriter.unwrap_or_default(),
        prospectus_url: c.prospectus,
        status: derive_status(c.apply_flg, c.list_flg),
    }
}

// ── Handler ───────────────────────────────────────────────────────────────────

static HK_IPO_CACHE: OnceLock<TtlCache<Vec<HkIpoRecord>>> = OnceLock::new();
fn hk_cache() -> &'static TtlCache<Vec<HkIpoRecord>> {
    HK_IPO_CACHE.get_or_init(TtlCache::new)
}

async fn list_hk_ipos(_state: axum::extract::State<Arc<AppState>>) -> Json<Vec<HkIpoRecord>> {
    if let Some(cached) = hk_cache().get().await {
        return Json(cached);
    }
    match fetch_hk_ipos().await {
        Ok(records) if !records.is_empty() => {
            hk_cache().set(records.clone()).await;
            Json(records)
        }
        Ok(_empty) => Json(hk_cache().get_stale().await.unwrap_or_default()),
        Err(_) => Json(hk_cache().get_stale().await.unwrap_or_default()),
    }
}

async fn fetch_hk_ipos() -> anyhow::Result<Vec<HkIpoRecord>> {
    let client = reqwest::Client::new();
    let resp = client
        .get(JISILU_HKIPO_URL)
        .headers(jisilu_headers(JISILU_REFERER))
        .send()
        .await?;

    let raw: JiSiLuResponse = resp.json().await?;
    Ok(raw.rows.into_iter().map(map_row).collect())
}

// ── CN IPO ────────────────────────────────────────────────────────────────────

const JISILU_CNIPO_URL: &str = "https://www.jisilu.cn/data/new_stock/apply/";

#[derive(Debug, Deserialize)]
struct JiSiLuCnResponse {
    rows: Vec<JiSiLuCnRow>,
}

#[derive(Debug, Deserialize)]
struct JiSiLuCnRow {
    id: String,
    cell: JiSiLuCnCell,
}

#[derive(Debug, Deserialize)]
struct JiSiLuCnCell {
    stock_cd: String,
    stock_nm: String,
    #[serde(default)]
    market_cd: Option<String>,
    #[serde(default)]
    apply_dt: Option<String>,
    #[serde(default)]
    apply_dt2: Option<String>,
    #[serde(default)]
    apply_cd: Option<String>,
    #[serde(default)]
    need_market_value: Option<String>,
    #[serde(default)]
    issue_price: Option<String>,
    #[serde(default)]
    individual_limit: Option<String>,
    #[serde(default)]
    money_out_dt: Option<String>,
    #[serde(default)]
    list_dt2: Option<String>,
    #[serde(default)]
    lucky_draw_rt: Option<serde_json::Value>,
    #[serde(default)]
    after_issue_show: Option<String>,
    #[serde(default)]
    issue_show: Option<String>,
    #[serde(default)]
    pub_pe: Option<String>,
    #[serde(default)]
    avg_pe: Option<String>,
    #[serde(default)]
    profit_rt: Option<serde_json::Value>,
    #[serde(default)]
    underwriter: Option<String>,
    #[serde(default)]
    prospectus: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CnIpoRecord {
    pub id: String,
    pub code: String,
    pub name: String,
    pub board: String,
    pub subscription_date: String,
    pub issue_price: String,
    pub subscription_code: String,
    pub pe: String,
    pub max_subscriptions: String,
    pub listing_date: String,
    pub online_rate: String,
    pub top_market_value: String,
    pub issue_size: String,
    pub industry_pe: String,
    pub online_issue: String,
    pub sponsor: String,
    pub status: String,
    pub prospectus_url: Option<String>,
}

fn strip_html(s: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }
    result.trim().to_string()
}

fn derive_board(market_cd: &str) -> &'static str {
    match market_cd {
        "shkc" => "科创板",
        "szcy" | "szgem" => "创业板",
        "bj" => "北交所",
        _ => "主板",
    }
}

fn derive_cn_status(apply_dt2: &str, list_dt2: &str, profit_rt: &serde_json::Value) -> String {
    // If first-day change data exists, it's listed
    if let serde_json::Value::String(s) = profit_rt {
        if s != "-" && !s.is_empty() && !s.contains("<a href") {
            return "listed".to_string();
        }
    }
    // Compare dates (ISO format YYYY-MM-DD allows string comparison)
    let today = chrono::Utc::now().date_naive().to_string();
    let today = today.as_str();
    if !list_dt2.is_empty() && list_dt2 != "-" && list_dt2 <= today {
        return "listed".to_string();
    }
    if apply_dt2.is_empty() || apply_dt2 == "-" {
        return "upcoming".to_string();
    }
    if apply_dt2 == today {
        "open".to_string()
    } else if apply_dt2 > today {
        "upcoming".to_string()
    } else {
        "closed".to_string()
    }
}

fn map_cn_row(row: JiSiLuCnRow) -> CnIpoRecord {
    let c = row.cell;
    let market_cd = c.market_cd.as_deref().unwrap_or("");
    let board = derive_board(market_cd).to_string();
    let is_bj = market_cd == "bj";

    let apply_dt2 = c.apply_dt2.as_deref().unwrap_or("");
    let list_dt2 = c.list_dt2.as_deref().unwrap_or("");
    let profit_rt_val = c.profit_rt.unwrap_or(serde_json::Value::String("-".to_string()));
    let status = derive_cn_status(apply_dt2, list_dt2, &profit_rt_val);

    CnIpoRecord {
        id: row.id,
        code: c.stock_cd,
        name: c.stock_nm,
        board,
        subscription_date: c.apply_dt.as_deref().map(strip_html).unwrap_or_default(),
        issue_price: if is_bj {
            "-".to_string()
        } else {
            c.need_market_value.unwrap_or_else(|| "-".to_string())
        },
        subscription_code: c.apply_cd.as_deref().map(strip_html).unwrap_or_default(),
        pe: c.issue_price.unwrap_or_else(|| "-".to_string()),
        max_subscriptions: c
            .individual_limit
            .as_deref()
            .map(|s| s.trim_end_matches('0').trim_end_matches('.').to_string())
            .unwrap_or_else(|| "-".to_string()),
        listing_date: c.money_out_dt.unwrap_or_else(|| "-".to_string()),
        online_rate: to_display(&Some(
            c.lucky_draw_rt.unwrap_or(serde_json::Value::String("-".to_string())),
        )),
        top_market_value: c.after_issue_show.unwrap_or_else(|| "-".to_string()),
        issue_size: c.issue_show.unwrap_or_else(|| "-".to_string()),
        industry_pe: c.pub_pe.unwrap_or_else(|| "-".to_string()),
        online_issue: c.avg_pe.unwrap_or_else(|| "-".to_string()),
        sponsor: c.underwriter.unwrap_or_else(|| "-".to_string()),
        status,
        prospectus_url: c.prospectus.filter(|s| s != "-" && !s.is_empty()),
    }
}

static CN_IPO_CACHE: OnceLock<TtlCache<Vec<CnIpoRecord>>> = OnceLock::new();
fn cn_cache() -> &'static TtlCache<Vec<CnIpoRecord>> {
    CN_IPO_CACHE.get_or_init(TtlCache::new)
}

async fn list_cn_ipos(_state: axum::extract::State<Arc<AppState>>) -> Json<Vec<CnIpoRecord>> {
    if let Some(cached) = cn_cache().get().await {
        return Json(cached);
    }
    match fetch_cn_ipos().await {
        Ok(records) if !records.is_empty() => {
            cn_cache().set(records.clone()).await;
            Json(records)
        }
        Ok(_empty) => Json(cn_cache().get_stale().await.unwrap_or_default()),
        Err(_) => Json(cn_cache().get_stale().await.unwrap_or_default()),
    }
}

async fn fetch_cn_ipos() -> anyhow::Result<Vec<CnIpoRecord>> {
    let client = reqwest::Client::new();
    let resp = client
        .get(JISILU_CNIPO_URL)
        .headers(jisilu_headers(JISILU_REFERER))
        .send()
        .await?;

    let raw: JiSiLuCnResponse = resp.json().await?;
    Ok(raw.rows.into_iter().map(map_cn_row).collect())
}

// ── CB IPO ───────────────────────────────────────────────────────────────────

const JISILU_CBIPO_URL: &str = "https://www.jisilu.cn/data/cbnew/pre_list/";
const JISILU_CBIPO_REFERER: &str = "https://www.jisilu.cn/web/data/cb/pre";

#[derive(Debug, Deserialize)]
struct JiSiLuCbResponse {
    rows: Vec<JiSiLuCbRow>,
}

#[derive(Debug, Deserialize)]
struct JiSiLuCbRow {
    id: String,
    cell: JiSiLuCbCell,
}

#[derive(Debug, Deserialize)]
struct JiSiLuCbCell {
    #[serde(default)]
    stock_id: Option<String>,
    #[serde(default)]
    bond_id: Option<String>,
    #[serde(default)]
    stock_nm: Option<String>,
    #[serde(default)]
    bond_nm: Option<String>,
    #[serde(default)]
    progress_nm: Option<String>,
    #[serde(default)]
    progress_dt: Option<String>,
    #[serde(default)]
    cb_amount: Option<String>,
    #[serde(default)]
    price: Option<String>,
    #[serde(default)]
    convert_price: Option<String>,
    #[serde(default)]
    increase_rt: Option<String>,
    #[serde(default)]
    year_left: Option<String>,
    #[serde(default)]
    apply10: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CbIpoRecord {
    pub id: String,
    pub code: String,
    pub name: String,
    pub progress: String,
    pub announce_date: String,
    pub issue_size: String,
    pub stock_price: String,
    pub conversion_price: String,
    pub conversion_premium: String,
    pub term: String,
    pub min_market_value: String,
}

fn normalize_display(v: Option<String>) -> String {
    v.filter(|s| !s.trim().is_empty())
        .map(|s| strip_html(&s))
        .filter(|s| !s.is_empty() && s != "-" && s != "--")
        .unwrap_or_else(|| "-".to_string())
}

fn normalize_json_display(v: Option<serde_json::Value>) -> String {
    match v {
        Some(serde_json::Value::String(s)) => normalize_display(Some(s)),
        Some(serde_json::Value::Number(n)) => n.to_string(),
        Some(serde_json::Value::Bool(b)) => {
            if b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        _ => "-".to_string(),
    }
}

fn map_cb_row(row: JiSiLuCbRow) -> CbIpoRecord {
    let c = row.cell;
    CbIpoRecord {
        id: row.id,
        code: normalize_display(c.stock_id.or(c.bond_id)),
        name: normalize_display(c.stock_nm.or(c.bond_nm)),
        progress: normalize_display(c.progress_nm),
        announce_date: normalize_display(c.progress_dt),
        issue_size: normalize_display(c.cb_amount),
        stock_price: normalize_display(c.price),
        conversion_price: normalize_display(c.convert_price),
        conversion_premium: normalize_display(c.increase_rt.map(|v| format!("{v}%"))),
        term: normalize_display(c.year_left),
        min_market_value: normalize_json_display(c.apply10),
    }
}

static CB_IPO_CACHE: OnceLock<TtlCache<Vec<CbIpoRecord>>> = OnceLock::new();
fn cb_cache() -> &'static TtlCache<Vec<CbIpoRecord>> {
    CB_IPO_CACHE.get_or_init(TtlCache::new)
}

async fn list_cb_ipos(_state: axum::extract::State<Arc<AppState>>) -> Json<Vec<CbIpoRecord>> {
    if let Some(cached) = cb_cache().get().await {
        return Json(cached);
    }
    match fetch_cb_ipos().await {
        Ok(records) if !records.is_empty() => {
            cb_cache().set(records.clone()).await;
            Json(records)
        }
        Ok(_empty) => Json(cb_cache().get_stale().await.unwrap_or_default()),
        Err(_) => Json(cb_cache().get_stale().await.unwrap_or_default()),
    }
}

async fn fetch_cb_ipos() -> anyhow::Result<Vec<CbIpoRecord>> {
    let client = reqwest::Client::new();
    let resp = client
        .get(JISILU_CBIPO_URL)
        .headers(jisilu_headers(JISILU_CBIPO_REFERER))
        .send()
        .await?;

    let raw: JiSiLuCbResponse = resp.json().await?;
    Ok(raw.rows.into_iter().map(map_cb_row).collect())
}

// ── REITS ─────────────────────────────────────────────────────────────────────

const JISILU_REITS_URL: &str = "https://www.jisilu.cn/data/cnreits/pre_list/";
const JISILU_REITS_REFERER: &str = "https://www.jisilu.cn/data/cnreits/#PreReits";

#[derive(Debug, Deserialize)]
struct JiSiLuReitsResponse {
    rows: Vec<JiSiLuReitsRow>,
}

#[derive(Debug, Deserialize)]
struct JiSiLuReitsRow {
    id: String,
    cell: JiSiLuReitsCell,
}

#[derive(Debug, Deserialize)]
struct JiSiLuReitsCell {
    code: String,
    name: String,
    #[serde(default)]
    asset_type: Option<String>,
    #[serde(default)]
    apply_date: Option<String>,
    #[serde(default)]
    apply_code: Option<String>,
    #[serde(default)]
    issue_price: Option<String>,
    #[serde(default)]
    max_apply: Option<String>,
    #[serde(default)]
    list_date: Option<String>,
    #[serde(default)]
    underlying_asset: Option<String>,
    #[serde(default)]
    sponsor: Option<String>,
    #[serde(default)]
    apply_flg: Option<i32>,
    #[serde(default)]
    list_flg: Option<i32>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReitsRecord {
    pub id: String,
    pub code: String,
    pub name: String,
    pub asset_type: String,
    pub subscription_date: String,
    pub subscription_code: String,
    pub issue_price: String,
    pub max_subscriptions: String,
    pub listing_date: String,
    pub underlying_asset: String,
    pub sponsor: String,
    pub status: String,
}

fn derive_reits_status(apply_flg: Option<i32>, list_flg: Option<i32>) -> String {
    match (list_flg.unwrap_or(0), apply_flg.unwrap_or(0)) {
        (1, _) => "listed".to_string(),
        (_, 1) => "open".to_string(),
        _ => "upcoming".to_string(),
    }
}

fn map_reits_row(row: JiSiLuReitsRow) -> ReitsRecord {
    let c = row.cell;
    ReitsRecord {
        id: row.id,
        code: c.code,
        name: c.name,
        asset_type: c.asset_type.unwrap_or_default(),
        subscription_date: c.apply_date.unwrap_or_default(),
        subscription_code: c.apply_code.unwrap_or_default(),
        issue_price: c.issue_price.unwrap_or_default(),
        max_subscriptions: c.max_apply.unwrap_or_default(),
        listing_date: c.list_date.unwrap_or_default(),
        underlying_asset: c.underlying_asset.unwrap_or_default(),
        sponsor: c.sponsor.unwrap_or_default(),
        status: derive_reits_status(c.apply_flg, c.list_flg),
    }
}

static REITS_IPO_CACHE: OnceLock<TtlCache<Vec<ReitsRecord>>> = OnceLock::new();
fn reits_cache() -> &'static TtlCache<Vec<ReitsRecord>> {
    REITS_IPO_CACHE.get_or_init(TtlCache::new)
}

async fn list_reits_ipos(_state: axum::extract::State<Arc<AppState>>) -> Json<Vec<ReitsRecord>> {
    if let Some(cached) = reits_cache().get().await {
        return Json(cached);
    }
    match fetch_reits_ipos().await {
        Ok(records) if !records.is_empty() => {
            reits_cache().set(records.clone()).await;
            Json(records)
        }
        Ok(_empty) => Json(reits_cache().get_stale().await.unwrap_or_default()),
        Err(_) => Json(reits_cache().get_stale().await.unwrap_or_default()),
    }
}

async fn fetch_reits_ipos() -> anyhow::Result<Vec<ReitsRecord>> {
    let mut headers = jisilu_headers(JISILU_REITS_REFERER);
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/json, text/javascript, */*; q=0.01"),
    );
    headers.insert(
        HeaderName::from_static("x-requested-with"),
        HeaderValue::from_static("XMLHttpRequest"),
    );

    let client = reqwest::Client::new();
    let resp = client
        .get(JISILU_REITS_URL)
        .headers(headers)
        .send()
        .await?;

    let raw: JiSiLuReitsResponse = resp.json().await?;
    Ok(raw.rows.into_iter().map(map_reits_row).collect())
}

// ── Router ────────────────────────────────────────────────────────────────────

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/ipo/hk", get(list_hk_ipos))
        .route("/ipo/cn", get(list_cn_ipos))
    .route("/ipo/cb", get(list_cb_ipos))
        .route("/ipo/reits", get(list_reits_ipos))
}
