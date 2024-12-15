use crate::models::{
    OpenOrder, RateLimitResponse, SpotAssetResponse, SpotMetaResponse, SpotTokenBalancesResponse,
};
use ethers::types::H160;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

/// HttpClientError 型を定義
#[derive(Debug)]
pub enum HttpClientError {
    RequestFailed(reqwest::Error),
    JsonParse(String),
}

impl std::fmt::Display for HttpClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpClientError::RequestFailed(e) => write!(f, "Request failed: {}", e),
            HttpClientError::JsonParse(e) => write!(f, "Failed to parse JSON: {}", e),
        }
    }
}

impl Error for HttpClientError {}

pub struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(is_test: bool) -> Self {
        let base_url = if is_test {
            "https://api.hyperliquid-testnet.xyz".to_string()
        } else {
            "https://api.hyperliquid.xyz".to_string()
        };
        HttpClient {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn send_info_request<T: for<'de> Deserialize<'de>, U: Serialize>(
        &self,
        info_request: U,
    ) -> Result<T, HttpClientError> {
        // JSONに変換
        let data = serde_json::to_string(&info_request)
            .map_err(|e| HttpClientError::JsonParse(e.to_string()))?;

        // POSTリクエストを送信
        let response_text = self
            .client
            .post(format!("{}{}", self.base_url, "/info"))
            .header("Content-Type", "application/json")
            .body(data)
            .send()
            .await
            .map_err(HttpClientError::RequestFailed)?
            .text()
            .await
            .map_err(HttpClientError::RequestFailed)?;

        // レスポンスを JSON にデシリアライズ
        serde_json::from_str::<T>(&response_text)
            .map_err(|e| HttpClientError::JsonParse(e.to_string()))
    }

    pub async fn fetch_all_mids(&self) -> Result<HashMap<String, String>, HttpClientError> {
        let request_body = serde_json::json!({"type": "allMids"});
        self.send_info_request(request_body).await
    }

    pub async fn fetch_open_orders(
        &self,
        address: H160,
    ) -> Result<Vec<OpenOrder>, HttpClientError> {
        let request_body = serde_json::json!({"type": "openOrders", "user": address});
        self.send_info_request(request_body).await
    }

    pub async fn fetch_user_fills(&self) {
        todo!("fetch_user_fills");
    }

    pub async fn fetch_frontend_open_orders(&self) {
        todo!("fetch_frontend_open_orders");
    }

    pub async fn fetch_user_fills_by_time(&self) {
        todo!("fetch_user_fills_by_time");
    }

    pub async fn fetch_rate_limits(
        &self,
        address: H160,
    ) -> Result<RateLimitResponse, HttpClientError> {
        let request_body = serde_json::json!({"type": "userRateLimit", "user": address});
        self.send_info_request(request_body).await
    }

    pub async fn fetch_order_status(&self) {
        todo!("fetch_order_status");
    }

    pub async fn fetch_l2_book(&self) {
        todo!("fetch_l2_book");
    }

    pub async fn fetch_candle_snapshot(&self) {
        todo!("fetch_candle_snapshot");
    }

    pub async fn fetch_builder_fee_approval(&self) {
        todo!("fetch_builder_fee_approval");
    }

    pub async fn fetch_historical_orders(&self) {
        todo!("fetch_historical_orders");
    }

    pub async fn fetch_twap_slice_fills(&self) {
        todo!("fetch_twap_slice_fills");
    }

    pub async fn fetch_subaccounts(&self) {
        todo!("fetch_user_subaccounts");
    }

    pub async fn fetch_spot_meta(&self) -> Result<SpotMetaResponse, HttpClientError> {
        let request_body = serde_json::json!({"type": "spotMeta"});
        self.send_info_request(request_body).await
    }

    pub async fn fetch_spot_asset_contexts(&self) -> Result<SpotAssetResponse, HttpClientError> {
        let request_body = serde_json::json!({"type": "spotAssetContexts"});
        self.send_info_request(request_body).await
    }

    pub async fn fetch_spot_token_balances(
        &self,
        address: H160,
    ) -> Result<SpotTokenBalancesResponse, HttpClientError> {
        let request_body = serde_json::json!({"type": "spotClearinghouseState", "user": address});
        self.send_info_request(request_body).await
    }

    pub async fn fetch_auction_info(&self) {
        todo!("fetch_spot_deploy_auction_info");
    }

    pub async fn fetch_perpetuals_meta(&self) {
        todo!("fetch_perpetuals_meta");
    }

    pub async fn fetch_perpetuals_asset_contexts(&self) {
        todo!("fetch_perpetuals_asset_contexts");
    }

    pub async fn fetch_perp_account_summary(&self) {
        todo!("fetch_user_perpetuals_account_summary");
    }

    pub async fn fetch_funding_history(&self) {
        todo!("fetch_user_funding_history");
    }

    pub async fn fetch_historical_funding_rates(&self) {
        todo!("fetch_historical_funding_rates");
    }

    pub async fn fetch_funding_rate_predictions(&self) {
        todo!("fetch_predicted_funding_rates");
    }

    pub async fn place_order(&self) {
        todo!("place_order");
    }

    pub async fn cancel_order(&self) {
        todo!("cancel_order");
    }

    pub async fn cancel_order_by_client_order_id(&self) {
        todo!("cancel_order_by_cloid");
    }

    pub async fn cancel_all_orders(&self) {
        todo!("cancel_all_orders");
    }

    pub async fn modify_order(&self) {
        todo!("modify_order");
    }

    pub async fn modify_orders(&self) {
        todo!("modify_orders");
    }

    pub async fn update_leverage(&self) {
        todo!("update_leverage");
    }

    pub async fn update_isolated_margin(&self) {
        todo!("update_isolated_margin");
    }

    pub async fn transfer_usd(&self) {
        todo!("transfer_usd");
    }

    pub async fn transfer_spot_asset(&self) {
        todo!("transfer_spot_asset");
    }

    pub async fn initiate_withdrawal(&self) {
        todo!("initiate_withdrawal");
    }

    pub async fn transfer_between_spot_and_perp(&self) {
        todo!("transfer_spot_to_perp");
    }

    pub async fn deposit_or_withdraw_vault(&self) {
        todo!("manage_vault_funds");
    }

    pub async fn approve_api_wallet(&self) {
        todo!("approve_api_wallet");
    }

    pub async fn approve_builder_fee(&self) {
        todo!("approve_builder_fee");
    }

    pub async fn place_twap_order(&self) {
        todo!("place_twap_order");
    }

    pub async fn cancel_twap_order(&self) {
        todo!("cancel_twap_order");
    }
}
