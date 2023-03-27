#![warn(missing_docs)]
//! The data that describes agents that live in a `SimulationEnvironment`.
//! All agents must implement the `Agent` trait.
use std::sync::Arc;

use ethers::{
    abi::Abi,
    contract::Contract,
    prelude::Provider,
    providers::{Http, Middleware},
    types::{Address, Filter},
};
use eyre::Result;
use futures::stream::StreamExt;

pub mod utils;

/// Houses a provider to talk to the chain.
pub struct EventMonitor {
    /// Provider to talk to the chain.
    pub provider: Arc<Provider<Http>>,
}
impl EventMonitor {
    /// Public builder function that instantiates a default implementation of `event_monitor`.
    pub async fn default() -> Self {
        let provider = utils::get_provider().await;
        Self { provider }
    }
    /// Monitors events for a given contract from a given provider.
    pub async fn monitor_events(self, contract_address: &str, contract_abi: Abi) -> Result<()> {
        let contract_address: Address = contract_address.parse()?;
        let i_portfolio = Contract::new(contract_address, contract_abi, self.provider.clone());

        let event_filter = Filter {
            address: Some(ethers::types::ValueOrArray::Array(vec![
                i_portfolio.address()
            ])),
            topics: [None, None, None, None], // None for all topics
            ..Default::default()
        };

        let mut event_stream = self
            .provider
            .watch(&event_filter)
            .await
            .expect("Failed to create event stream");

        while let Some(log) = event_stream.next().await {
            println!("Event data: {:?}", log);
        }
        Ok(())
    }
}
