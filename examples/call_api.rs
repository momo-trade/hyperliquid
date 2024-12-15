use dotenv::dotenv;
use ethers::types::H160;
use hyperliquid::http::client::HttpClient;
use log::{error, info};
use std::env;
use std::str::FromStr;

fn address_from_env() -> H160 {
    let wallet_address = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS not set");
    H160::from_str(&wallet_address).expect("Invalid Ethereum address")
}

#[tokio::main]
async fn main() {
    dotenv().ok(); //Load environment variables from .env file
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let client = HttpClient::new(false);
    let wallet_address = address_from_env();
    info!("Wallet address: {:#x}", wallet_address);

    let order_id = 51243188761; // order情報を取得したいorder_idを設定。無い場合はfetch_order_status毎コメントアウト

    //fetch_all_mids
    info!("Fetching all mids...");
    let all_mids = match client.fetch_all_mids().await {
        Ok(all_mids) => all_mids,
        Err(e) => {
            error!("Failed to fetch all mids: {}", e);
            return;
        }
    };
    info!("Number of all mids: {}", all_mids.len());
    info!("HYPE: {}", all_mids.get("HYPE").unwrap());

    //open_orders
    info!("Fetching open orders...");
    let open_orders = match client.fetch_open_orders(wallet_address).await {
        Ok(open_orders) => open_orders,
        Err(e) => {
            error!("Failed to fetch open orders: {}", e);
            return;
        }
    };
    info!("Number of open orders: {}", open_orders.len());
    info!("Open orders: {:#?}", open_orders);

    //rate_limit
    info!("Fetching rate limit...");
    let rate_limit = match client.fetch_rate_limits(wallet_address).await {
        Ok(rate_limit) => rate_limit,
        Err(e) => {
            error!("Failed to fetch rate limit: {}", e);
            return;
        }
    };
    info!(
        "Rate limit -> cum_volume: {}, n_requests_used: {}, n_requests_cap: {}",
        rate_limit.cum_volume, rate_limit.n_requests_used, rate_limit.n_requests_cap
    );

    //spot_token_balance
    info!("Fetching spot token balance...");
    let spot_token_balance = match client.fetch_spot_token_balances(wallet_address).await {
        Ok(spot_token_balance) => spot_token_balance,
        Err(e) => {
            error!("Failed to fetch spot token balance: {}", e);
            return;
        }
    };
    info!("Spot token balance: {:#?}", spot_token_balance.balances);

    //user_fills
    info!("Fetching user fills...");
    let user_fills = match client.fetch_user_fills(wallet_address, None).await {
        Ok(user_fills) => user_fills,
        Err(e) => {
            error!("Failed to fetch user fills: {}", e);
            return;
        }
    };
    info!("Number of user fills: {}", user_fills.len());
    info!("User fills: {:#?}", user_fills);

    //order_status
    info!("Fetching order status...");
    let order_status = match client
        .fetch_order_status(wallet_address, Some(order_id), None)
        .await
    {
        Ok(order_status) => order_status,
        Err(e) => {
            error!("Failed to fetch order status: {}", e);
            return;
        }
    };
    info!("Order status: {:#?}", order_status);

    //l2_book
    info!("Fetching L2 book...");
    let l2_book = match client.fetch_l2_book("HYPE", None, None).await {
        Ok(l2_book) => l2_book,
        Err(e) => {
            error!("Failed to fetch L2 book: {}", e);
            return;
        }
    };
    info!("Bids:");
    for bid in &l2_book.levels[0] {
        info!(
            "Price: {:.2}, Size: {:.2}, Orders: {}",
            bid.price, bid.size, bid.order_count
        );
    }

    // Asks
    info!("Asks:");
    for ask in &l2_book.levels[1] {
        info!(
            "Price: {:.2}, Size: {:.2}, Orders: {}",
            ask.price, ask.size, ask.order_count
        );
    }

    //candle snapshot
    info!("Fetching candle snapshot...");
    let candle_snapshot = match client
        .fetch_candle_snapshot("HYPE", "15m", None, None)
        .await
    {
        Ok(candle_snapshot) => candle_snapshot,
        Err(e) => {
            error!("Failed to fetch candle snapshot: {}", e);
            return;
        }
    };
    info!("Candle snapshot: {:#?}", candle_snapshot);
}
