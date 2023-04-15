#![warn(missing_docs)]
//! The data that describes agents that live in a `SimulationEnvironment`.
//! All agents must implement the `Agent` trait.
use std::sync::Arc;
use std::error::Error;
use std::fs::File;
use csv::WriterBuilder;


use ethers::{
    abi::Abi,
    contract::Contract,
    prelude::*,
    providers::{Http, Middleware},
    types::{Address, Filter, H256},
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
            address: Some(ethers::types::ValueOrArray::Array(vec![contract.address()])),
            block_option: ethers::types::FilterBlockOption::Range {
                from_block: Some(ethers::types::BlockNumber::Number(from_block.into())),
                to_block: Some(ethers::types::BlockNumber::Number(to_block.into())),
            },
            topics: [None, None, None, None], // None for all topics
            ..Default::default()
        };
        let past_logs = self
            .provider
            .get_logs(&event_filter)
            .await
            .expect("Failed to query past logs");

        let swap_event = "Swap";

        for log in past_logs {
            let log_topics: Vec<H256> = log.topics.clone();
            let log_data = log.data.0
                .into_iter()
                .collect();
            
                let decoded_swap_event = contract.decode_event(swap_event, log_topics, log_data)?.0;

                if let Some(event) = decoded_swap_event {
                    let sender = event.sender;
                    let recipient = event.recipient;
                    let amount0 = event.amount0.as_u128();
                    let amount1 = event.amount1.as_u128();
                    let sqrt_price_x96 = event.sqrt_price_x96.as_u128();

                    let encoded = ethers::abi::Tokenizable::from_tokens(&[
                        sender.into(),
                        recipient.into(),
                        amount0.into(),
                        amount1.into(),
                        sqrt_price_x96.into(),
                    ])
                    .encode()?;
                    println!("sqrt_price_x96: {:?}", encoded.sqrt_price_x96);
                    }
            
        }
        Ok(())
    }

    /// Converts sqrt_price_x96 to readable price
    pub fn sqrt_price_x96_to_price(&self, sqrt_price_x96: u128) -> f64 {
        let sqrtprice = (sqrt_price_x96 as f64) / (2.0_f64.powi(96) as f64);
        let price = sqrtprice * sqrtprice;
        price
    }

    /// Save historical data to csv
    fn save_price_to_csv(price_data: &Vec<f64>, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(file_path)?;
        let mut writer = WriterBuilder::new().from_writer(file);
    
        for num in price_data {
            writer.serialize(num)?;
        }
    
        writer.flush()?;
    
        Ok(())
    }
}
