// use serde::{Deserialize, Serialize};
use crate::http::client::HttpClient;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarketType {
    Spot,
    Perp,
}

#[derive(Debug, Default)]
pub struct TokenManager {
    spot_symbol_to_internal: HashMap<String, String>,
    perp_symbol_to_internal: HashMap<String, String>,
    spot_internal_to_symbol: HashMap<String, String>,
    perp_internal_to_symbol: HashMap<String, String>,
    spot_symbol_to_index: HashMap<String, u32>,
    perp_symbol_to_index: HashMap<String, u32>,
}

impl TokenManager {
    pub fn new() -> Self {
        Self::default()
    }

    //Get Spot and Perp information from API and initialize
    pub async fn from_api(client: &HttpClient) -> Result<Self, Box<dyn std::error::Error>> {
        let spot_meta = client.fetch_spot_meta().await?;
        let perp_meta = client.fetch_perp_meta().await?;

        let mut manager = TokenManager::new();

        // Spot.
        for universe in spot_meta.universe {
            let tokens: Vec<String> = universe
                .tokens
                .iter()
                .map(|&index| spot_meta.tokens[index as usize].name.clone())
                .collect();
            let pair_name = format!("{}/{}", tokens[0], tokens[1]);
            manager.add_mapping(
                MarketType::Spot,
                &pair_name,
                &universe.name,
                10_000 + universe.index, // Index of Spot is 10_000 offset
            );
        }

        // Perp.
        for (index, universe) in perp_meta.universe.iter().enumerate() {
            manager.add_mapping(
                MarketType::Perp,
                &universe.name,
                &universe.name,
                index as u32,
            );
        }

        Ok(manager)
    }

    pub fn add_mapping(
        &mut self,
        market_type: MarketType,
        symbol: &str,
        internal_code: &str,
        index: u32,
    ) {
        match market_type {
            MarketType::Spot => {
                self.spot_symbol_to_internal
                    .insert(symbol.to_string(), internal_code.to_string());
                self.spot_internal_to_symbol
                    .insert(internal_code.to_string(), symbol.to_string());
                self.spot_symbol_to_index.insert(symbol.to_string(), index);
            }
            MarketType::Perp => {
                self.perp_symbol_to_internal
                    .insert(symbol.to_string(), internal_code.to_string());
                self.perp_internal_to_symbol
                    .insert(internal_code.to_string(), symbol.to_string());
                self.perp_symbol_to_index.insert(symbol.to_string(), index);
            }
        }
    }

    pub fn get_internal_code(&self, market_type: MarketType, symbol: &str) -> Option<&String> {
        match market_type {
            MarketType::Spot => self.spot_symbol_to_internal.get(symbol),
            MarketType::Perp => self.perp_symbol_to_internal.get(symbol),
        }
    }

    pub fn get_symbol(&self, market_type: MarketType, internal_code: &str) -> Option<&String> {
        match market_type {
            MarketType::Spot => self.spot_internal_to_symbol.get(internal_code),
            MarketType::Perp => self.perp_internal_to_symbol.get(internal_code),
        }
    }

    pub fn get_token_index(&self, market_type: MarketType, symbol: &str) -> Option<&u32> {
        match market_type {
            MarketType::Spot => self.spot_symbol_to_index.get(symbol),
            MarketType::Perp => self.perp_symbol_to_index.get(symbol),
        }
    }

    pub fn get_available_symbols(&self, market_type: MarketType) -> Vec<String> {
        match market_type {
            MarketType::Spot => self.spot_symbol_to_internal.keys().cloned().collect(),
            MarketType::Perp => self.perp_symbol_to_internal.keys().cloned().collect(),
        }
    }
}
