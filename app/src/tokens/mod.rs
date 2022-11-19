use ethers::types::{Address, H160};
// use std::collections::hash_map;
use std::collections::{linked_list, HashMap};

#[derive(Debug)]
pub struct Token {
    pub address: H160, //ethers::core::types::Address,
    pub decimals: u16,
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
        decimals: 18,
        name: "ETH".to_string(),
    };
    //wBTC
    let wbtc_address = "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599"
        .parse::<Address>()
        .unwrap();
    let wbtc = Token {
        address: wbtc_address,
        decimals: 8,
        name: "WBTC".to_string(),
    };
    // Dai
    let dai_address = "0x6B175474E89094C44Da98b954EedeAC495271d0F"
        .parse::<Address>()
        .unwrap();
    let dai = Token {
        address: dai_address,
        decimals: 18,
        name: "DAI".to_string(),
    };
    // USDT
    let tether_address = "0xdAC17F958D2ee523a2206206994597C13D831ec7"
        .parse::<Address>()
        .unwrap();
    let tether = Token {
        address: tether_address,
        decimals: 6,
        name: "USDT".to_string(),
    };

    // USDC
    let usdc_address = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
        .parse::<Address>()
        .unwrap();
    let usdc = Token {
        address: usdc_address,
        decimals: 6,
        name: "USDC".to_string(),
    };

    // Frax
    let frax_address = "0x853d955aCEf822Db058eb8505911ED77F175b99e"
        .parse::<Address>()
        .unwrap();
    let frax = Token {
        address: frax_address,
        decimals: 18,
        name: "FRAX".to_string(),
    };

    // link
    let link_address = "0x853d955aCEf822Db058eb8505911ED77F175b99e"
        .parse::<Address>()
        .unwrap();
    let link = Token {
        address: link_address,
        decimals: 18,
        name: "LINK".to_string(),
    };

    let mut tokens = HashMap::new();
    tokens.insert("ETH".to_string(), eth);
    tokens.insert("USDC".to_string(), usdc);
    tokens.insert("USDT".to_string(), tether);
    tokens.insert("WBTC".to_string(), wbtc);
    tokens.insert("DAI".to_string(), dai);
    tokens.insert("FRAX".to_string(), frax);
    tokens.insert("LINK".to_string(), link);

    tokens
}
