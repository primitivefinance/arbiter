use ethers::types::{Address, H160};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents an ERC20 token.
pub struct Token {
    /// Address of the token.
    pub address: H160,
    /// Decimals of the token.
    pub decimals: u16,
    /// Name of the token.
    pub name: String,
    /// Whether the token is a stablecoin.
    pub is_stable: bool,
}

impl Token {
    /// Public builder function that instantiates a `Token`.
    fn new(address: H160, decimals: u16, name: String, is_stable: bool) -> Self {
        Self {
            address,
            decimals,
            name,
            is_stable,
        }
    }
}

// return hashmap, name = key, value = token object
pub fn get_tokens() -> HashMap<String, Token> {
    let mut tokens = HashMap::new();

    // ETH
    let eth_address = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
        .parse::<Address>()
        .unwrap();
    let eth = Token::new(eth_address, 18, String::from("ETH"), false);

    // Wrapped Bitcoin
    //wBTC https://wbtc.network/
    let wbtc_address = "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599"
        .parse::<Address>()
        .unwrap();
    let wbtc = Token::new(wbtc_address, 8, String::from("WBTC"), false);

    // DAI https://makerdao.com/en/
    let dai_address = "0x6B175474E89094C44Da98b954EedeAC495271d0F"
        .parse::<Address>()
        .unwrap();
    let dai = Token::new(dai_address, 18, String::from("DAI"), true);

    // USDT https://tether.to/
    let usdt_address = "0xdAC17F958D2ee523a2206206994597C13D831ec7"
        .parse::<Address>()
        .unwrap();
    let usdt = Token::new(usdt_address, 6, String::from("USDT"), true);

    // USDC https://www.centre.io/
    let usdc_address = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
        .parse::<Address>()
        .unwrap();
    let usdc = Token::new(usdc_address, 6, String::from("USDC"), true);

    // FRAX https://frax.finance/
    let frax_address = "0x853d955aCEf822Db058eb8505911ED77F175b99e"
        .parse::<Address>()
        .unwrap();
    let frax = Token::new(frax_address, 18, String::from("FRAX"), true);

    // Chainlink
    // LINK https://chain.link/
    let link_address = "0x853d955aCEf822Db058eb8505911ED77F175b99e"
        .parse::<Address>()
        .unwrap();
    let link = Token::new(link_address, 18, String::from("LINK"), false);

    // stETH2 https://app.stakewise.io/
    let steth2_address = "0xFe2e637202056d30016725477c5da089Ab0A043A"
        .parse::<Address>()
        .unwrap();
    let steth2 = Token::new(steth2_address, 18, String::from("sETH2"), false);

    // USDM https://www.mappedswap.io/
    let usdm_address = "0xbbAec992fc2d637151dAF40451f160bF85f3C8C1"
        .parse::<Address>()
        .unwrap();
    let usdm = Token::new(usdm_address, 6, String::from("USDM"), true);

    // Binance US Dollar
    // BUSD https://paxos.com/busd/
    let busd_address = "0x4Fabb145d64652a948d72533023f6E7A623C7C53"
        .parse::<Address>()
        .unwrap();
    let busd = Token::new(busd_address, 18, String::from("USDM"), true);

    // Uniswap
    // UNI https://uniswap.org/
    let uni_address = "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984"
        .parse::<Address>()
        .unwrap();
    let uni = Token::new(uni_address, 18, String::from("UNI"), false);

    // BTT https://www.bittorrent.com/
    let btt_address = "0xC669928185DbCE49d2230CC9B0979BE6DC797957"
        .parse::<Address>()
        .unwrap();
    let btt = Token::new(btt_address, 18, String::from("BTT"), false);

    // Wrapped Staked Ethereum
    // wstETH https://www.lido.fi/
    let wsteth_address = "0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0"
        .parse::<Address>()
        .unwrap();
    let wsteth = Token::new(wsteth_address, 18, String::from("WSTETH"), false);

    // Staked Ethereum
    // stETH https://www.lido.fi/
    // Etherscan: https://etherscan.io/token/0xae7ab96520de3a18e5e111b5eaab095312d7fe84
    let steth_address = "0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84"
        .parse::<Address>()
        .unwrap();
    let steth = Token::new(steth_address, 18, String::from("STETH"), false);

    // Rocket Pool Ethereum
    // rETH https://rocketpool.net/#header
    // Etherscan: https://etherscan.io/token/0xae78736cd615f374d3085123a210448e74fc6393
    let reth_address = "0xae78736Cd615f374D3085123A210448E74Fc6393"
        .parse::<Address>()
        .unwrap();
    let reth = Token::new(reth_address, 18, String::from("RETH"), false);

    tokens.insert("ETH".to_string(), eth);
    tokens.insert("USDC".to_string(), usdc);
    tokens.insert("USDT".to_string(), usdt);
    tokens.insert("WBTC".to_string(), wbtc);
    tokens.insert("DAI".to_string(), dai);
    tokens.insert("FRAX".to_string(), frax);
    tokens.insert("LINK".to_string(), link);
    tokens.insert("SETH2".to_string(), steth2);
    tokens.insert("USDM".to_string(), usdm);
    tokens.insert("BUSD".to_string(), busd);
    tokens.insert("UNI".to_string(), uni);
    tokens.insert("BTT".to_string(), btt);
    tokens.insert("WSTETH".to_string(), wsteth);
    tokens.insert("STETH".to_string(), steth);
    tokens.insert("RETH".to_string(), reth);

    tokens
}
