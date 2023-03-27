#![warn(missing_docs)]
use std::sync::Arc;

use ethers::{
    abi::{Address, Abi}
    contract::{BaseContract, Contract},
    providers::{Http, Provider, Middleware},
    prelude::{Provider, Client},
    types::{Address, Filter},
};

use eyre::Result;
use futures::stream::StreamExt;

pub mod error;
pub mod utils;

/// Houses a provider to talk to the chain.
pub struct event_monitor {
    pub client: Arc<Provider<Http>>,
}
impl event_monitor {
    /// Public builder function that instantiates a default implementation of `event_monitor`.
    pub async fn default() -> Self {
        let provider = utils::get_provider().await;
        Self { provider }
    }
    /// Monitors events for a given contract from a given provider.
    pub async fn monitor_events(self, contract_address: &str, contract_abi: Abi ) -> Result<()> {
        
        /// Parse ABI
        let contract_address: Address = contract_address.parse()?;
        
        /// Create contract instance
        let i_portfolio = Contract::new(contract_address, contract_abi, self.client.clone());
    
        /// Build event filter
        let event_filter = Filter {
            address: Some(ethers::types::ValueOrArray::Array(vec![
                i_portfolio.address()
            ])),
            topics: [None, None, None, None], // None for all topics
            ..Default::default()
        };
    
        /// Create event stream
        let mut event_stream = self.client
            .watch(&event_filter)
            .await
            .expect("Failed to create event stream");
    
        /// Monitor event logs
        while let Some(log) = event_stream.next().await {
            println!("Event data: {:?}", log);
        }
        Ok(())
    }
}