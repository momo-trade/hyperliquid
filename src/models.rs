use crate::http::client::HttpClient;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TokenManager {
    symbol_to_internal: HashMap<String, String>,
    internal_to_symbol: HashMap<String, String>,
}

impl TokenManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn from_api(client: &HttpClient) -> Result<Self, Box<dyn std::error::Error>> {
        let spot_meta = client.fetch_spot_meta().await?;
        let mut mapper = TokenManager::new();

        for universe in spot_meta.universe {
            if universe.name.starts_with("@") {
                let tokens: Vec<String> = universe
                    .tokens
                    .iter()
                    .map(|&index| spot_meta.tokens[index as usize].name.clone())
                    .collect();
                let pair_name = format!("{}/{}", tokens[0], tokens[1]);
                mapper.add_mapping(&pair_name, &universe.name);
            } else {
                mapper.add_mapping(&universe.name, &universe.name);
            }
        }

        Ok(mapper)
    }

    pub fn add_mapping(&mut self, symbol: &str, internal_code: &str) {
        self.symbol_to_internal
            .insert(symbol.to_string(), internal_code.to_string());
        self.internal_to_symbol
            .insert(internal_code.to_string(), symbol.to_string());
    }

    pub fn get_internal_code(&self, symbol: &str) -> Option<&String> {
        self.symbol_to_internal.get(symbol)
    }

    pub fn get_symbol(&self, internal_code: &str) -> Option<&String> {
        self.internal_to_symbol.get(internal_code)
    }

    pub fn list_available_pairs(&self) -> Vec<String> {
        self.symbol_to_internal.keys().cloned().collect()
    }
}

#[derive(Deserialize, Debug)]
pub struct Token {
    pub name: String,
    #[serde(rename = "szDecimals")]
    pub sz_decimals: u8,
    #[serde(rename = "weiDecimals")]
    pub wei_decimals: u8,
    pub index: u32,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "isCanonical")]
    pub is_canonical: bool,
    #[serde(rename = "evmContract")]
    pub evm_contract: Option<String>,
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Universe {
    pub name: String,
    pub tokens: Vec<u32>,
    pub index: u32,
    #[serde(rename = "isCanonical")]
    pub is_canonical: bool,
}

#[derive(Deserialize)]
pub struct MarketData {
    #[serde(rename = "dayNtlVlm")]
    pub day_ntl_vlm: String,
    #[serde(rename = "markPx")]
    pub mark_px: String,
    #[serde(rename = "midPx")]
    pub mid_px: String,
    #[serde(rename = "prevDayPx")]
    pub prev_day_px: String,
}

#[derive(Deserialize)]
pub struct SpotMetaResponse {
    pub tokens: Vec<Token>,
    pub universe: Vec<Universe>,
}

#[derive(Deserialize)]
pub struct SpotAssetResponse {
    pub tokens: Vec<Token>,
    pub universe: Vec<Universe>,
    pub market_data: Vec<MarketData>,
}
