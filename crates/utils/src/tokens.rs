use std::collections::HashMap;

use ethers::types::{Address, H160};

/// Insert a token to the HashMap.
macro_rules! token_insert {
    (
        $address:expr,
        $decimals:expr,
        $name:expr,
        $is_stable:expr,
        $tokens:expr
    ) => {
        let address = $address.parse::<Address>().unwrap();
        let token = Token::new(address, $decimals, String::from($name), $is_stable);

        $tokens.insert(String::from($name), token);
    };
}

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
    pub fn new(address: H160, decimals: u16, name: String, is_stable: bool) -> Self {
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
    token_insert!(
        "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        18,
        "ETH",
        false,
        tokens
    );

    //wBTC https://wbtc.network/
    token_insert!(
        "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599",
        8,
        "WBTC",
        false,
        tokens
    );

    // DAI https://makerdao.com/en/
    token_insert!(
        "0x6B175474E89094C44Da98b954EedeAC495271d0F",
        18,
        "DAI",
        true,
        tokens
    );

    // USDT https://tether.to/
    token_insert!(
        "0xdAC17F958D2ee523a2206206994597C13D831ec7",
        6,
        "USDT",
        true,
        tokens
    );

    // USDC https://www.centre.io/
    token_insert!(
        "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
        6,
        "USDC",
        true,
        tokens
    );

    // FRAX https://frax.finance/
    token_insert!(
        "0x853d955aCEf822Db058eb8505911ED77F175b99e",
        18,
        "FRAX",
        true,
        tokens
    );

    // LINK https://chain.link/
    token_insert!(
        "0x514910771af9ca656af840dff83e8264ecf986ca",
        18,
        "LINK",
        false,
        tokens
    );

    // stETH2 https://app.stakewise.io/
    token_insert!(
        "0xFe2e637202056d30016725477c5da089Ab0A043A",
        18,
        "sETH2",
        false,
        tokens
    );

    // USDM https://www.mappedswap.io/
    token_insert!(
        "0xbbAec992fc2d637151dAF40451f160bF85f3C8C1",
        6,
        "USDM",
        true,
        tokens
    );

    // BUSD https://paxos.com/busd/
    token_insert!(
        "0x4Fabb145d64652a948d72533023f6E7A623C7C53",
        18,
        "BUSD",
        true,
        tokens
    );

    // UNI https://uniswap.org/
    token_insert!(
        "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984",
        18,
        "UNI",
        false,
        tokens
    );

    // BTT https://www.bittorrent.com/
    token_insert!(
        "0xC669928185DbCE49d2230CC9B0979BE6DC797957",
        18,
        "BTT",
        false,
        tokens
    );

    // wstETH https://www.lido.fi/
    token_insert!(
        "0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0",
        18,
        "WSTETH",
        false,
        tokens
    );

    // stETH https://www.lido.fi/
    token_insert!(
        "0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84",
        18,
        "STETH",
        false,
        tokens
    );

    // rETH https://rocketpool.net/#header
    token_insert!(
        "0xae78736Cd615f374D3085123A210448E74Fc6393",
        18,
        "RETH",
        false,
        tokens
    );

    // RPL https://rocketpool.net/#header
    token_insert!(
        "0xD33526068D116cE69F19A9ee46F0bd304F21A51f",
        18,
        "RPL",
        false,
        tokens
    );

    tokens
}
