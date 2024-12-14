use hyperliquid::http::client::HttpClient;
use hyperliquid::models::TokenManager;
use hyperliquid::utils::time::unix_time_to_jst;
use hyperliquid::websocket::client::WebSocketConnection;
use log::{info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init(); // Initialize the logger
                                                                                                // let url = "wss://api.hyperliquid.xyz/ws";

    let connection = WebSocketConnection::connect_with_retries(false).await;
    let connection = Arc::new(connection);

    let http_client = HttpClient::new(false);
    let token_manager = TokenManager::from_api(&http_client).await.unwrap();

    let symbol = "HYPE/USDC";
    let coin = match token_manager.get_internal_code(symbol) {
        Some(code) => code.clone(),
        None => {
            println!("Token not found");
            return Err(Box::from("Token not found"));
        }
    };

    // Subscriptions
    connection.subscribe("allMids", HashMap::new()).await?;

    connection
        .subscribe("trades", HashMap::from([("coin", &*coin)]))
        .await?;

    connection
        .subscribe("l2Book", HashMap::from([("coin", &*coin)]))
        .await?;

    let mut params = HashMap::new();
    params.insert("coin", coin.as_str());
    params.insert("interval", "5m");
    connection.subscribe("candle", params).await?;

    // Message receiving task (with reconnection support)
    let connection_clone = Arc::clone(&connection);
    tokio::spawn(async move {
        connection_clone.receive_messages().await;
    });

    // Heartbeat sending task
    let connection_clone = Arc::clone(&connection);
    tokio::spawn(async move {
        connection_clone.start_ping_task().await;
    });

    // Periodically access `all_mids` and `trades`
    let connection_clone = Arc::clone(&connection);
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;

            // Access `all_mids`
            let all_mids = connection_clone.all_mids.lock().await;
            // info!("allMids: {:?}", all_mids);
            if let Some(mid_price) = all_mids.get(coin.as_str()) {
                info!("{} mid_price: {}", symbol, mid_price);
            } else {
                warn!("{} mid_price not found.", symbol);
            }

            // Access `trades`
            let trades = connection_clone.trades.lock().await;
            if trades.is_empty() {
                warn!("No trade data available.");
            } else if let Some(latest_trades) = trades.get(&*coin) {
                if latest_trades.is_empty() {
                    warn!("No trade data available for {}.", symbol);
                } else {
                    let latest_trade = latest_trades.last().unwrap();
                    info!(
                        "Latest trade data for {} ({} entries): Side: {}, Price: {}, Size: {}",
                        symbol,
                        latest_trades.len(),
                        latest_trade.side.as_str(),
                        latest_trade.price,
                        latest_trade.size,
                    );
                }
            } else {
                warn!("No trade data available.");
            }

            let l2_books = connection_clone.l2_books.lock().await;
            // info!("l2Books: {:#?}", l2_books);
            if let Some(book) = l2_books.get(&*coin) {
                let best_ask = book.ask_levels.first().map(|level| {
                    format!(
                        "Price: {:.3}, Size: {:.2}, Orders: {}",
                        level.price, level.size, level.order_count
                    )
                });

                let best_bid = book.bid_levels.first().map(|level| {
                    format!(
                        "Price: {:.3}, Size: {:.2}, Orders: {}",
                        level.price, level.size, level.order_count
                    )
                });

                info!(
                    "Best Ask: {}",
                    best_ask.unwrap_or_else(|| "No Asks".to_string())
                );
                info!(
                    "Best Bid: {}",
                    best_bid.unwrap_or_else(|| "No Bids".to_string())
                );
            } else {
                warn!("No book data available.");
            }

            let candles = connection_clone.candles.lock().await;
            match candles.get(&*coin) {
                Some(candle_list) if candle_list.len() > 1 => {
                    // 確定した最後の足（未確定足の1つ前）
                    if let Some(confirmed_candle) = candle_list.get(candle_list.len() - 2) {
                        let open_time_jst = unix_time_to_jst(confirmed_candle.open_time);
                        info!(
                            "Time: {}, Open: {}, High: {}, Low: {}, Close: {}",
                            open_time_jst,
                            confirmed_candle.open,
                            confirmed_candle.high,
                            confirmed_candle.low,
                            confirmed_candle.close
                        );
                    }
                }
                Some(candle_list) if candle_list.len() == 1 => {
                    // 足が1つしかない場合、それが未確定かどうかに関わらず表示
                    if let Some(single_candle) = candle_list.first() {
                        let open_time_jst = unix_time_to_jst(single_candle.open_time);
                        warn!(
                "Only one candle available (possibly incomplete): Time: {}, Open: {}, High: {}, Low: {}, Close: {}",
                open_time_jst,
                single_candle.open,
                single_candle.high,
                single_candle.low,
                single_candle.close
            );
                    }
                }
                Some(_) => warn!("Candle list is empty."),
                None => warn!("No candle data available."),
            }

            // 板情報っぽく表示
            // if let Some(book) = l2_books.get("@107") {
            //     const DISPLAY_LIMIT: usize = 10; // 表示する上限

            //     // ASKを価格の大きい順に並べて表示
            //     let best_asks: Vec<String> = book
            //         .ask_levels
            //         .iter()
            //         .take(DISPLAY_LIMIT)
            //         .rev() // 価格の小さい順で並んでいるため、逆順にする
            //         .map(|level| {
            //             format!(
            //                 "Price: {:.3}, Size: {:.2}, Orders: {}",
            //                 level.price, level.size, level.order_count
            //             )
            //         })
            //         .collect();

            //     // BIDをそのまま表示（価格の大きい順で保持されているためそのまま）
            //     let best_bids: Vec<String> = book
            //         .bid_levels
            //         .iter()
            //         .take(DISPLAY_LIMIT)
            //         .map(|level| {
            //             format!(
            //                 "Price: {:.3}, Size: {:.2}, Orders: {}",
            //                 level.price, level.size, level.order_count
            //             )
            //         })
            //         .collect();

            //     info!(
            //         "HYPE Book:\nASKs:\n{}\nBIDs:\n{}",
            //         best_asks.join("\n"),
            //         best_bids.join("\n")
            //     );
            // } else {
            //     warn!("No book data available for HYPE.");
            // }
        }
    });

    // Main task infinite loop
    loop {
        time::sleep(Duration::from_secs(10)).await;
    }
}
