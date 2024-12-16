use hyperliquid::http::client::HttpClient;
use hyperliquid::models::MarketType;
use hyperliquid::models::TokenManager;
use log::info;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let client = HttpClient::new(false);
    let token_manager = TokenManager::from_api(&client).await.unwrap();

    //利用可能なトークン一覧(Spot)
    let spot_token_list = token_manager.get_available_symbols(MarketType::Spot);
    info!("Number of Spot tokens: {}", spot_token_list.len());

    for token in spot_token_list {
        let internal_code = token_manager.get_internal_code(MarketType::Spot, &token);
        let index = token_manager.get_token_index(MarketType::Spot, &token);
        info!(
            "Name: {}, Internal Code: {}, Index: {}",
            token,
            internal_code.unwrap(),
            index.unwrap()
        );
    }

    //利用可能なトークン一覧(Perp)
    let perp_token_list = token_manager.get_available_symbols(MarketType::Perp);
    info!("Number of Perp tokens: {}", perp_token_list.len());

    for token in perp_token_list {
        let internal_code = token_manager.get_internal_code(MarketType::Perp, &token);
        let index = token_manager.get_token_index(MarketType::Perp, &token);
        info!(
            "Name: {}, Internal Code: {}, Index: {}",
            token,
            internal_code.unwrap(),
            index.unwrap()
        );
    }
}
