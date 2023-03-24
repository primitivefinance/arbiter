#![allow(clippy::useless_attribute)]
use std::sync::Arc;

use ethers::{providers::{Http, Provider}, contract::BaseContract};

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
    pub async fn monitor_events(contract: BaseContract) -> Self {
        todo!()
    }
}