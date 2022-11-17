use ethers::types::H160;

#[derive(Debug)]
pub struct Token {
    pub address: H160, //ethers::core::types::Address,
    pub base_units: u16,
    pub name: String,
}