///`Order(uint128,uint128,bool,uint64,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Order {
    pub input: u128,
    pub output: u128,
    pub use_max: bool,
    pub pool_id: u64,
    pub sell_asset: bool,
}
///`PortfolioCurve(uint128,uint16,uint16,uint16,uint16,uint32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct PortfolioCurve {
    pub strike_price: u128,
    pub fee: u16,
    pub duration: u16,
    pub volatility: u16,
    pub priority_fee: u16,
    pub created_at: u32,
}
///`PortfolioPair(address,uint8,address,uint8)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct PortfolioPair {
    pub token_asset: ::ethers::core::types::Address,
    pub decimals_asset: u8,
    pub token_quote: ::ethers::core::types::Address,
    pub decimals_quote: u8,
}
