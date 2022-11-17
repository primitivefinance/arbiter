use std::collections::hash_map;

use ethers::types::{Address, H160};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Token {
    pub address: H160, //ethers::core::types::Address,
    pub base_units: u16,
    pub name: String,
}

// return hashmap, name = key, value = token object
pub fn get_tokens() -> HashMap<String, Token> {
    // Ether
    let eth_address = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
        .parse::<Address>()
        .unwrap();
    let eth = Token {
        address: eth_address,
        base_units: 18,
        name: "ETH".to_string(),
    };

    // USDC
    let usdc_address = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
        .parse::<Address>()
        .unwrap();
    let usdc = Token {
        address: usdc_address,
        base_units: 6,
        name: "USDC".to_string(),
    };
    let mut tokens = HashMap::new();
    tokens.insert("ETH".to_string(), eth);
    tokens.insert("USDC".to_string(), usdc);
    tokens
}
