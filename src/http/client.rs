use crate::models::{SpotAssetResponse, SpotMetaResponse};
use reqwest::Client;

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

    pub async fn fetch_spot_meta(&self) -> Result<SpotMetaResponse, reqwest::Error> {
        let request_body = serde_json::json!({"type": "spotMeta"});

        let response = self
            .client
            .post(format!("{}/info", self.base_url))
            .json(&request_body)
            .send()
            .await?
            .json::<SpotMetaResponse>()
            .await?;

        Ok(response)
    }

    pub async fn fetch_spot_asset_contexts(&self) -> Result<SpotAssetResponse, reqwest::Error> {
        let request_body = serde_json::json!({"type": "spotAssetContexts"});

        let response = self
            .client
            .post(format!("{}/info", self.base_url))
            .json(&request_body)
            .send()
            .await?
            .json::<SpotAssetResponse>()
            .await?;

        Ok(response)
    }
}
