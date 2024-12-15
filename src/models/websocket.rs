use crate::models::TradeSide;
use serde::{Deserialize, Serialize};

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
