use hyperliquid::http::client::HttpClient;
use log::{error, info};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let client = HttpClient::new(false);

    //fetch_all_mids
    let all_mids = match client.fetch_all_mids().await {
        Ok(all_mids) => all_mids,
        Err(e) => {
            error!("Failed to fetch all mids: {}", e);
            return;
        }
    };
    info!("Number of all mids: {}", all_mids.len());
    info!("HYPE: {}", all_mids.get("HYPE").unwrap());
}
