use serde::Deserialize;

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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
