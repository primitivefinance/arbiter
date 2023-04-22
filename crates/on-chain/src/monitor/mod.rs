#![warn(missing_docs)]
//! The data that describes agents that live in a `SimulationEnvironment`.
//! All agents must implement the `Agent` trait.
use std::{error::Error, fs::File, sync::Arc};

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
    ) -> Result<Vec<U256>, Box<dyn std::error::Error>> {
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

        pub struct Swap {
            pub sender: Address,
            pub recipient: Address,
            pub amount0: I256,
            pub amount1: I256,
            pub sqrt_price_x96: U256,
            pub liquidity: U256,
            pub tick: i32,
        }

        let mut price_data: Vec<U256> = Vec::new();

        for log in past_logs {
            let log_topics: Vec<H256> = log.topics.clone();
            let log_data = log.data.0.into_iter().collect();

            let decoded_swap_event = match contract
                .decode_event::<(H160, H160, I256, I256, U256, U256, i32)>(
                    swap_event, log_topics, log_data,
                ) {
                Ok(event) => event,
                Err(_) => continue, /* Some blocks don't have any events which will throw an error. We're ignoring these */
            };

            let (_sender, _recipient, _amount0, _amount1, _sqrt_price_x96, _liquidity, _tick) =
                decoded_swap_event;
            price_data.push(_sqrt_price_x96);
        }
        Ok(price_data)
    }

    /// Converts sqrt_price_x96 to readable price
    pub fn sqrt_price_x96_to_price(&self, price_data: Vec<u128>) -> Vec<f64> {
        let mut normalized_price_data: Vec<f64> = Vec::new();

        for price in price_data {
            let sqrt_price_x96 = price;
            let sqrtprice = sqrt_price_x96 as f64;
            normalized_price_data.push(sqrtprice);
        }
        normalized_price_data
    }

    /// Save historical data to csv
    pub fn save_price_to_csv(
        &self,
        price_data: &Vec<U256>,
        file_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        let file = File::create(file_path)?;
        let mut writer = WriterBuilder::new().from_writer(file);

        for num in price_data {
            writer.serialize(num)?;
        }

        writer.flush()?;

        Ok(())
    }
}
