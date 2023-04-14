#![warn(missing_docs)]
//! The data that describes agents that live in a `SimulationEnvironment`.
//! All agents must implement the `Agent` trait.
use std::sync::Arc;

use ethers::{
    abi::Abi,
    contract::Contract,
    prelude::Provider,
    providers::{Http, Middleware},
    types::{Address, Filter, BlockNumber},
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
    pub async fn new(rpc_type: utils::RpcTypes) -> Self {
        let provider = utils::get_provider(rpc_type).await;
        Self { provider }
    }
    /// Monitors events for a given contract from a given provider.
    pub async fn monitor_events(
        &self,
        contract_address: &str,
        contract_abi: Abi,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

/// Houses a provider to talk to the chain.
pub struct HistoricalMonitor {
    /// Provider to talk to the chain.
    pub provider: Arc<Provider<Http>>,
}
impl HistoricalMonitor {
    /// Public builder function that instantiates a default implementation of `historical_monitor`.
    pub async fn new(rpc_type: utils::RpcTypes) -> Self {
        let provider = utils::get_provider(rpc_type).await;
        Self { provider }
    }
    /// pulls historical events for a given contract from a given provider.
    pub async fn historical_monitor(
        &self,
        contract_address: &str,
        contract_abi: Abi,
        from_block: u64,
        to_block: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let contract_address: Address = contract_address.parse()?;
        let contract = Contract::new(contract_address, contract_abi, self.provider.clone());
    
        let event_filter = Filter {
            address: Some(ethers::types::ValueOrArray::Array(vec![
                contract.address(),
            ])),
            from_block: Some(ethers::types::BlockNumber::Number(from_block.into())),
            to_block: Some(ethers::types::BlockNumber::Number(to_block.into())),
            topics: [None, None, None, None], // None for all topics
            ..Default::default()
        };
        let past_logs = self
            .provider
            .get_logs(&event_filter)
            .await
            .expect("Failed to query past logs");

        for log in past_logs {
            println!("Past event data: {:?}", log);
        }

        Ok(())

    }
}