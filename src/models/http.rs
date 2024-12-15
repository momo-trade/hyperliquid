use crate::http::client::HttpClient;
use crate::utils::data_conversion::{parse_str_to_f64, parse_str_to_option_f64};
use ethers::types::H160;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TokenManager {
    symbol_to_internal: HashMap<String, String>,
    internal_to_symbol: HashMap<String, String>,
}

impl TokenManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn from_api(client: &HttpClient) -> Result<Self, Box<dyn std::error::Error>> {
        let spot_meta = client.fetch_spot_meta().await?;
        let mut mapper = TokenManager::new();

        for universe in spot_meta.universe {
            if universe.name.starts_with("@") {
                let tokens: Vec<String> = universe
                    .tokens
                    .iter()
                    .map(|&index| spot_meta.tokens[index as usize].name.clone())
                    .collect();
                let pair_name = format!("{}/{}", tokens[0], tokens[1]);
                mapper.add_mapping(&pair_name, &universe.name);
            } else {
                mapper.add_mapping(&universe.name, &universe.name);
            }
        }

        Ok(mapper)
    }

    pub fn add_mapping(&mut self, symbol: &str, internal_code: &str) {
        self.symbol_to_internal
            .insert(symbol.to_string(), internal_code.to_string());
        self.internal_to_symbol
            .insert(internal_code.to_string(), symbol.to_string());
    }

    pub fn get_internal_code(&self, symbol: &str) -> Option<&String> {
        self.symbol_to_internal.get(symbol)
    }

    pub fn get_symbol(&self, internal_code: &str) -> Option<&String> {
        self.internal_to_symbol.get(internal_code)
    }

    pub fn list_available_pairs(&self) -> Vec<String> {
        self.symbol_to_internal.keys().cloned().collect()
    }
}

#[derive(Deserialize, Debug)]
pub struct Token {
    pub name: String,
    #[serde(rename = "szDecimals")]
    pub sz_decimals: u8,
    #[serde(rename = "weiDecimals")]
    pub wei_decimals: u8,
    pub index: u32,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "isCanonical")]
    pub is_canonical: bool,
    #[serde(rename = "evmContract")]
    pub evm_contract: Option<String>,
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Universe {
    pub name: String,
    pub tokens: Vec<u32>,
    pub index: u32,
    #[serde(rename = "isCanonical")]
    pub is_canonical: bool,
}

#[derive(Deserialize)]
pub struct MarketData {
    #[serde(rename = "dayNtlVlm")]
    pub day_ntl_vlm: String,
    #[serde(rename = "markPx")]
    pub mark_px: String,
    #[serde(rename = "midPx")]
    pub mid_px: String,
    #[serde(rename = "prevDayPx")]
    pub prev_day_px: String,
}

#[derive(Deserialize)]
pub struct SpotMetaResponse {
    pub tokens: Vec<Token>,
    pub universe: Vec<Universe>,
}

#[derive(Deserialize)]
pub struct SpotAssetResponse {
    pub tokens: Vec<Token>,
    pub universe: Vec<Universe>,
    pub market_data: Vec<MarketData>,
}

#[derive(Debug, Deserialize)]
pub struct OpenOrder {
    pub coin: String,
    #[serde(rename = "limitPx", deserialize_with = "parse_str_to_f64")]
    pub limit_price: f64,
    #[serde(rename = "oid")]
    pub order_id: u64,
    #[serde(deserialize_with = "parse_trade_side")]
    pub side: TradeSide,
    #[serde(rename = "sz", deserialize_with = "parse_str_to_f64")]
    pub size: f64,
    pub timestamp: u64,
}
pub type OpenOrdersResponse = Vec<OpenOrder>;
pub type HistoricalOrdersResponse = Vec<OpenOrder>;

#[derive(Debug, Deserialize)]
pub struct RateLimitResponse {
    #[serde(rename = "cumVlm", deserialize_with = "parse_str_to_f64")]
    pub cum_volume: f64,
    #[serde(rename = "nRequestsUsed")]
    pub n_requests_used: u64,
    #[serde(rename = "nRequestsCap")]
    pub n_requests_cap: u64,
}

#[derive(Debug, Deserialize)]
pub struct SpotTokenBalance {
    pub coin: String,
    pub token: u64,
    #[serde(deserialize_with = "parse_str_to_f64")]
    pub hold: f64,
    #[serde(deserialize_with = "parse_str_to_f64")]
    pub total: f64,
    #[serde(rename = "entryNtl", deserialize_with = "parse_str_to_f64")]
    pub entry_notional: f64,
}

#[derive(Debug, Deserialize)]
pub struct SpotTokenBalancesResponse {
    pub balances: Vec<SpotTokenBalance>,
}

#[derive(Debug, Deserialize)]
pub struct UserFills {
    #[serde(rename = "closedPnl", deserialize_with = "parse_str_to_f64")]
    pub closed_pnl: f64,
    pub coin: String,
    pub crossed: bool,
    pub dir: String,
    pub hash: String,
    #[serde(rename = "oid")]
    pub order_id: u64,
    #[serde(rename = "px", deserialize_with = "parse_str_to_f64")]
    pub price: f64,
    #[serde(deserialize_with = "parse_trade_side")]
    pub side: TradeSide,
    #[serde(rename = "startPosition", deserialize_with = "parse_str_to_f64")]
    pub start_position: f64,
    #[serde(rename = "sz", deserialize_with = "parse_str_to_f64")]
    pub size: f64,
    #[serde(rename = "time")]
    pub timestamp: u64,
    #[serde(deserialize_with = "parse_str_to_f64")]
    pub fee: f64,
    #[serde(rename = "feeToken")]
    pub fee_token: String,
    #[serde(
        rename = "builderFee",
        deserialize_with = "parse_str_to_option_f64",
        default
    )]
    pub builder_fee: Option<f64>,
    pub tid: u64,
}
pub type UserFillsResponse = Vec<UserFills>;

#[derive(Serialize)]
pub struct OrderStatusRequest {
    #[serde(rename = "type")]
    pub request_type: String,
    pub user: H160,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloid: Option<String>,
}

impl OrderStatusRequest {
    pub fn new(user: H160, oid: Option<u64>, cloid: Option<String>) -> Result<Self, String> {
        if oid.is_none() && cloid.is_none() {
            return Err("Either oid or cloid must be provided".to_string());
        }
        Ok(Self {
            request_type: "orderStatus".to_string(),
            user,
            oid,
            cloid,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct OrderStatusResponse {
    pub status: String, // order or unknownOid
    pub order: Option<OrderDetail>,
}

#[derive(Debug, Deserialize)]
pub struct OrderDetail {
    pub order: OrderInfo,
    pub status: String, // "filled" | "open" | "canceled" | "triggered" | "rejected" | "marginCanceled"
    #[serde(rename = "statusTimestamp")]
    pub status_timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct OrderInfo {
    pub coin: String,
    #[serde(deserialize_with = "parse_trade_side")]
    pub side: TradeSide,
    #[serde(rename = "limitPx", deserialize_with = "parse_str_to_f64")]
    pub limit_price: f64,
    #[serde(rename = "sz", deserialize_with = "parse_str_to_f64")]
    pub size: f64,
    #[serde(rename = "oid")]
    pub order_id: u64,
    pub timestamp: u64,
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: String,
    #[serde(rename = "isTrigger")]
    pub is_trigger: bool,
    #[serde(rename = "triggerPx", deserialize_with = "parse_str_to_f64")]
    pub trigger_price: f64,
    pub children: Vec<OrderInfo>,
    #[serde(rename = "isPositionTpsl")]
    pub is_position_tpsl: bool,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    #[serde(rename = "orderType")]
    pub order_type: String, //Market, Limit
    #[serde(rename = "origSz", deserialize_with = "parse_str_to_f64")]
    pub original_size: f64,
    pub tif: String, //FrontendMarketなど
    pub cloid: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrderLevel {
    #[serde(rename = "px", deserialize_with = "parse_str_to_f64")]
    pub price: f64,
    #[serde(rename = "sz", deserialize_with = "parse_str_to_f64")]
    pub size: f64,
    #[serde(rename = "n")]
    pub order_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct L2BookResponse {
    pub coin: String,
    #[serde(rename = "time")]
    pub timestamp: u64,
    pub levels: [Vec<OrderLevel>; 2], // bids: levels[0], asks: levels[1]
}

#[derive(Serialize)]
pub struct L2BookRequest {
    #[serde(rename = "type")]
    pub request_type: String,
    pub coin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_sig_figs: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mantissa: Option<u8>,
}

impl L2BookRequest {
    pub fn new(coin: &str, n_sig_figs: Option<u8>, mantissa: Option<u8>) -> Self {
        Self {
            request_type: "l2Book".to_string(),
            coin: coin.to_string(),
            n_sig_figs,
            mantissa,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct CandleSnapshotRequest {
    #[serde(rename = "type")]
    request_type: String,
    req: CandleRequestDetails,
}

impl CandleSnapshotRequest {
    pub fn new(coin: &str, interval: &str, start_time: Option<u64>, end_time: Option<u64>) -> Self {
        let now = current_epoch_millis();
        Self {
            request_type: "candleSnapshot".to_string(),
            req: CandleRequestDetails {
                coin: coin.to_string(),
                interval: interval.to_string(),
                start_time: start_time.unwrap_or(now - 24 * 60 * 60 * 1000), // デフォルトで24時間前
                end_time: end_time.unwrap_or(now),                           // デフォルトで現在時刻
            },
        }
    }
}

#[derive(Serialize, Debug)]
pub struct CandleRequestDetails {
    coin: String,
    interval: String,
    #[serde(rename = "startTime")]
    start_time: u64,
    #[serde(rename = "endTime")]
    end_time: u64,
}

// 現在のエポックミリ秒を取得するヘルパー関数
fn current_epoch_millis() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Candle {
    #[serde(rename = "t")]
    pub open_time: u64,
    #[serde(rename = "T")]
    pub close_time: u64,
    #[serde(rename = "s")]
    pub coin: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "o", deserialize_with = "parse_str_to_f64")]
    pub open: f64,
    #[serde(rename = "h", deserialize_with = "parse_str_to_f64")]
    pub high: f64,
    #[serde(rename = "l", deserialize_with = "parse_str_to_f64")]
    pub low: f64,
    #[serde(rename = "c", deserialize_with = "parse_str_to_f64")]
    pub close: f64,
    #[serde(rename = "v", deserialize_with = "parse_str_to_f64")]
    pub volume: f64,
    #[serde(rename = "n")]
    pub number_of_trades: u64,
}

pub type CandleSnapshotResponse = Vec<Candle>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TradeSide {
    Buy,
    Sell,
}

impl TradeSide {
    pub fn from_code(value: &str) -> Option<Self> {
        match value {
            "B" => Some(TradeSide::Buy),
            "A" => Some(TradeSide::Sell),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            TradeSide::Buy => "buy",
            TradeSide::Sell => "sell",
        }
    }
}

// カスタムデシリアライズ関数
pub fn parse_trade_side<'de, D>(deserializer: D) -> Result<TradeSide, D::Error>
where
    D: Deserializer<'de>,
{
    let value: String = Deserialize::deserialize(deserializer)?;
    TradeSide::from_code(&value)
        .ok_or_else(|| serde::de::Error::custom(format!("Invalid trade side code: {}", value)))
}

#[derive(Debug, Deserialize)]
pub struct Trade {
    #[serde(rename = "side", deserialize_with = "parse_trade_side")]
    pub side: TradeSide, // "A" や "B" を `TradeSide` に変換
    #[serde(rename = "px")]
    pub price: String, // 他のフィールド例
    #[serde(rename = "sz")]
    pub size: String,
}
