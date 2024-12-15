use serde::{Deserialize, Serialize};
use crate::utils::data_conversion::parse_str_to_f64;

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeData {
    pub coin: String,
    pub side: TradeSide,
    pub price: f64,         //px(String)
    pub size: f64,          //sz(String)
    pub trade_hash: String, //hash
    pub timestamp: u64,     //time
    pub trade_id: u64,      //tid
    pub users: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct WsLevel {
    pub price: f64,
    pub size: f64,
    pub order_count: usize,
}

#[derive(Clone, Debug)]
pub struct WsBook {
    pub coin: String,
    pub bid_levels: Vec<WsLevel>,
    pub ask_levels: Vec<WsLevel>,
    pub timestamp: u64,
}

// pub fn parse_str_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let value: Value = Deserialize::deserialize(deserializer)?;
//     match value {
//         Value::String(ref s) => {
//             debug!("Parsing string to f64: {}", s);
//             s.parse::<f64>().map_err(serde::de::Error::custom)
//         }
//         Value::Number(ref n) => {
//             debug!("Parsing number to f64: {}", n);
//             n.as_f64()
//                 .ok_or_else(|| serde::de::Error::custom("Failed to convert number to f64"))
//         }
//         _ => {
//             error!("Unexpected type for f64: {:?}", value);
//             Err(serde::de::Error::custom(
//                 "Expected a string or number for f64",
//             ))
//         }
//     }
// }

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