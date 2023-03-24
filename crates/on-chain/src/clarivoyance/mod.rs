#![allow(clippy::useless_attribute)]
use std::sync::Arc;

use ethers::{providers::{Http, Provider}, contract::{BaseContract, Contract}, abi::Address};

#[allow(warnings)]
pub mod error;
pub mod utils;

#[deprecated(since = "0.0.1", note = "will be useful for actors in the future")]
pub struct Clairvoyance {
    pub provider: Arc<Provider<Http>>,
}
#[allow(warnings)]
impl Clairvoyance {
    pub async fn default() -> Self {
        let provider = utils::get_provider().await;
        Self { provider }
    }
    pub async fn monitor_events(&self, contract: BaseContract, address: Address) -> Self {
        let thing: ethers::contract::Contract<Provider<ethers::providers::Http>> = contract.into_contract(address, self.provider.clone());
        // still meditating on the right way to abstract this
        todo!()
    }
}