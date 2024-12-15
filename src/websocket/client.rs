use crate::models::{Candle, TradeData, TradeSide, WsBook, WsLevel};
use futures_util::SinkExt;
use futures_util::StreamExt;
use log::{debug, error, info};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::Duration;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

#[derive(Clone)]
pub struct WebSocketConnection {
    ws_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    pub url: String,
    pub all_mids: Arc<Mutex<HashMap<String, String>>>,
    pub trades: Arc<Mutex<HashMap<String, Vec<TradeData>>>>,
    pub l2_books: Arc<Mutex<HashMap<String, WsBook>>>,
    pub candles: Arc<Mutex<HashMap<String, Vec<Candle>>>>,
    max_trades: usize,
    max_candles: usize,
}

impl WebSocketConnection {
    pub async fn connect(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (ws_stream, _) = connect_async(url).await?;
        info!("WebSocket connection established to {}", url);

        Ok(WebSocketConnection {
            ws_stream: Arc::new(Mutex::new(ws_stream)),
            url: url.to_string(),
            all_mids: Arc::new(Mutex::new(HashMap::new())),
            trades: Arc::new(Mutex::new(HashMap::new())),
            l2_books: Arc::new(Mutex::new(HashMap::new())),
            candles: Arc::new(Mutex::new(HashMap::new())),
            max_trades: 1000,
            max_candles: 1000,
        })
    }

    pub async fn connect_with_retries(is_test: bool) -> Arc<Self> {
        let url = if is_test {
            "wss://api.hyperliquid-testnet.xyz/ws"
        } else {
            "wss://api.hyperliquid.xyz/ws"
        };

        let mut attempts = 0;

        loop {
            match WebSocketConnection::connect(url).await {
                Ok(connection) => {
                    info!("WebSocket connection established successfully.");
                    return Arc::new(connection);
                }
                Err(e) => {
                    attempts += 1;
                    error!("Connection error: {}, retrying attempt: {}", e, attempts);

                    if attempts >= 5 {
                        panic!("Exceeded maximum reconnection attempts.");
                    }

                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    pub async fn receive_messages(&self) {
        loop {
            let mut ws_stream = self.ws_stream.lock().await;

            while let Some(msg) = ws_stream.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Err(e) = self.process_message(&text).await {
                            error!("Message processing error: {}", e);
                        }
                    }

                    Ok(Message::Close(_)) => {
                        info!("Connection closed. Attempting to reconnect.");
                        break;
                    }
                    Err(e) => {
                        error!("Message reception error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }

            match connect_async(&self.url).await {
                Ok((new_ws_stream, _)) => {
                    *ws_stream = new_ws_stream;
                    info!("WebSocket reconnected successfully.");
                }
                Err(e) => {
                    error!("WebSocket reconnection failed: {}", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    pub async fn subscribe(
        &self,
        subscription_type: &str,
        params: HashMap<&str, &str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut subscription_msg = serde_json::json!({
            "method": "subscribe",
            "subscription": {
                "type": subscription_type
            }
        });

        if let Some(subscription_obj) = subscription_msg
            .get_mut("subscription")
            .and_then(|v| v.as_object_mut())
        {
            for (key, value) in params {
                subscription_obj.insert(
                    key.to_string(),
                    serde_json::Value::String(value.to_string()),
                );
            }
        }

        info!("Subscription message: {}", subscription_msg);

        let mut ws_stream = self.ws_stream.lock().await;
        ws_stream
            .send(Message::Text(subscription_msg.to_string()))
            .await?;
        info!("Subscription sent for: {}", subscription_type);

        Ok(())
    }

    async fn process_message(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let parsed: Value = serde_json::from_str(message)?;

        if let Some(channel) = parsed.get("channel").and_then(|v| v.as_str()) {
            match channel {
                "allMids" => self.update_all_mids(message).await?,
                "trades" => self.update_trades(message).await?,
                "l2Book" => self.update_l2_book(message).await?,
                "candle" => self.update_candles(message).await?,
                "pong" => info!("Received pong message: {}", message),
                _ => {
                    info!("Unknown channel: {}", channel);
                }
            }
        }
        Ok(())
    }

    async fn update_all_mids(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let parsed: Value = serde_json::from_str(message)?;

        if let Some(data) = parsed.get("data").and_then(|v| v.get("mids")) {
            if let Some(mids) = data.as_object() {
                let mut all_mids = self.all_mids.lock().await;
                all_mids.clear();
                for (key, value) in mids {
                    if let Some(price) = value.as_str() {
                        all_mids.insert(key.clone(), price.to_string());
                    }
                }
            }
        }
        Ok(())
    }

    async fn update_trades(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let parsed: Value = serde_json::from_str(message)?;

        if let Some(data) = parsed.get("data").and_then(|v| v.as_array()) {
            let mut trades = self.trades.lock().await;

            for trade in data {
                if let Ok(trade_raw) = serde_json::from_value::<Value>(trade.clone()) {
                    if let (
                        Some(coin),
                        Some(side_raw),
                        Some(px),
                        Some(sz),
                        Some(hash),
                        Some(time),
                        Some(tid),
                        Some(users),
                    ) = (
                        trade_raw
                            .get("coin")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        trade_raw.get("side").and_then(|v| v.as_str()),
                        trade_raw
                            .get("px")
                            .and_then(|v| v.as_str())
                            .and_then(|v| v.parse::<f64>().ok()),
                        trade_raw
                            .get("sz")
                            .and_then(|v| v.as_str())
                            .and_then(|v| v.parse::<f64>().ok()),
                        trade_raw
                            .get("hash")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        trade_raw.get("time").and_then(|v| v.as_u64()),
                        trade_raw.get("tid").and_then(|v| v.as_u64()),
                        trade_raw
                            .get("users")
                            .and_then(|v| v.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|v| v.as_str().map(String::from))
                                    .collect::<Vec<String>>()
                            }),
                    ) {
                        if let Some(side) = TradeSide::from_code(side_raw) {
                            let trade_data = TradeData {
                                coin,
                                side,
                                price: px,
                                size: sz,
                                trade_hash: hash,
                                timestamp: time,
                                trade_id: tid,
                                users,
                            };

                            let coin_trades = trades
                                .entry(trade_data.coin.clone())
                                .or_insert_with(Vec::new);
                            coin_trades.push(trade_data);

                            if coin_trades.len() > self.max_trades {
                                coin_trades.remove(0);
                            }
                        } else {
                            error!("Invalid side value: {}", side_raw);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn update_l2_book(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let parsed: Value = match serde_json::from_str(message) {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to parse message as JSON: {}", e);
                return Ok(());
            }
        };

        let data = match parsed.get("data") {
            Some(data) => data,
            None => {
                error!("No 'data' field in the message: {}", message);
                return Ok(());
            }
        };

        let coin = match data.get("coin").and_then(|v| v.as_str()) {
            Some(coin) => coin,
            None => {
                error!("No 'coin' field in the data: {}", data);
                return Ok(());
            }
        };

        let levels = match data.get("levels").and_then(|v| v.as_array()) {
            Some(levels) => levels,
            None => {
                error!("No 'levels' field or not an array in the data: {}", data);
                return Ok(());
            }
        };

        if levels.len() < 2 {
            error!("Levels data does not contain both bids and asks: {}", data);
            return Ok(());
        }

        let parse_levels = |level_data: &Value| -> Vec<WsLevel> {
            level_data
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|level| {
                    let price = level
                        .get("px")
                        .and_then(|v| v.as_str())
                        .and_then(|v| v.parse().ok());
                    let size = level
                        .get("sz")
                        .and_then(|v| v.as_str())
                        .and_then(|v| v.parse().ok());
                    let order_count = level.get("n").and_then(|v| v.as_u64()).map(|v| v as usize);

                    match (price, size, order_count) {
                        (Some(price), Some(size), Some(order_count)) => Some(WsLevel {
                            price,
                            size,
                            order_count,
                        }),
                        _ => {
                            error!("Invalid level format: {}", level);
                            None
                        }
                    }
                })
                .collect()
        };

        let bid_levels = parse_levels(&levels[0]); // bids
        let ask_levels = parse_levels(&levels[1]); // asks

        let book = WsBook {
            coin: coin.to_string(),
            bid_levels,
            ask_levels,
            timestamp: data
                .get("time")
                .and_then(|v| v.as_u64())
                .unwrap_or_default(),
        };

        let mut l2_books = self.l2_books.lock().await;
        l2_books.insert(coin.to_string(), book);

        Ok(())
    }

    async fn update_candles(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let parsed: Value = serde_json::from_str(message)?;

        if let Some(data) = parsed.get("data") {
            if let Ok(candle_data) = serde_json::from_value::<Candle>(data.clone()) {
                let mut candles = self.candles.lock().await;

                let coin_candles = candles
                    .entry(candle_data.coin.clone())
                    .or_insert_with(Vec::new);

                // 未確定足の更新または新規追加
                if let Some(existing_candle) = coin_candles
                    .iter_mut()
                    .find(|c| c.open_time == candle_data.open_time)
                {
                    // 未確定足を更新
                    debug!(
                        "Updating candle for coin: {}, open_time: {}",
                        candle_data.coin, candle_data.open_time
                    );
                    *existing_candle = candle_data;
                } else {
                    // 新しい Candle を追加
                    info!("Added new candle for coin: {}", candle_data.coin);
                    coin_candles.push(candle_data);
                }

                // 最大サイズを超えたら古いデータを削除
                if coin_candles.len() > self.max_candles {
                    coin_candles.remove(0);
                }
            } else {
                error!("Failed to parse candle data: {:?}", data);
            }
        }

        Ok(())
    }

    pub async fn start_ping_task(&self) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            info!("Attempting to send WebSocket Ping...");

            let heartbeat_msg = serde_json::json!({ "method": "ping" }).to_string();
            let mut ws_stream = self.ws_stream.lock().await;
            match ws_stream.send(Message::Text(heartbeat_msg)).await {
                Ok(_) => info!("Heartbeat (ping) sent."),
                Err(e) => {
                    error!("Failed to send heartbeat: {}", e);
                    break;
                }
            }
        }
    }
}
