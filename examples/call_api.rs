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
}
