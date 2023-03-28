#![allow(deprecated)]
#![warn(missing_docs)]
#![warn(unsafe_code)]
#![allow(clippy::useless_attribute)]
use std::sync::Arc;

use ethers::{
    abi::Address,
    contract::BaseContract,
    providers::{Http, Provider},
};

#[allow(warnings)]
pub mod error;
pub mod utils;

#[deprecated(since = "0.0.1", note = "will be useful for actors in the future")]
/// Houses a provider to talk to the chain.
pub struct Clairvoyance {
    pub provider: Arc<Provider<Http>>,
}
#[allow(warnings)]
impl Clairvoyance {
    /// Public builder function that instantiates a default implementation of `Clairvoyance`.
    pub async fn default() -> Self {
        let provider = utils::get_provider().await;
        Self { provider }
    }
    /// Monitors events for a given contract from a given provider.
    pub async fn monitor_events(&self, contract: BaseContract, address: Address) -> Self {
        let thing: ethers::contract::Contract<Provider<ethers::providers::Http>> =
            contract.into_contract(address, self.provider.clone());
        // still meditating on the right way to abstract this
        todo!()
    }
}
