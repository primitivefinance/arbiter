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
