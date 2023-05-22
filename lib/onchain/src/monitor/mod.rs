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
/// # Fields
/// * `provider` - Provider to talk to the chain. (Arc<Provider<Http>>)
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
    /// Monitors all live events for a given Uniswap V3 pool contract from a given provider.
    /// # Arguments
    /// * `contract_address` - Address of the contract to monitor. (String)
    /// * `contract_abi` - ABI of the contract to monitor. (Abi)
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
            .watch(&event_filter) // stream events live
            .await
            .expect("Failed to create event stream");

        while let Some(log) = event_stream.next().await {
            println!("Event data: {:?}", log);
        }
        Ok(())
    }
}

/// Houses a provider to talk to the chain.
/// # Fields
/// * `provider` - Provider to talk to the chain. (Arc<Provider<Http>>)
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
    /// pulls historical swap event price data for a given contract from a given provider.
    /// # Arguments
    /// * `contract_address` - Address of the contract to monitor. (String)
    /// * `contract_abi` - ABI of the contract to monitor. (Abi)
    /// * `from_block` - Block number to start from. (u64)
    /// * `to_block` - Block number to end at. (u64)
    /// # Returns
    /// * `Vec<U256>` - Vector of U256 values that represent the historical sqrt_price_x96 for all swap events between specified block range.
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
                // Filter events between block range for above contract address
                from_block: Some(ethers::types::BlockNumber::Number(from_block.into())),
                to_block: Some(ethers::types::BlockNumber::Number(to_block.into())),
            },
            topics: [None, None, None, None], // None for all topics
        };
        let past_logs = self
            .provider
            .get_logs(&event_filter)
            .await
            .expect("Failed to query past logs");

        let swap_event = "Swap";

        /// Struct that represents the Swap event from the Uniswap V3 contract.
        /// # Fields
        /// * `sender` - Address of the sender.
        /// * `recipient` - Address of the recipient.
        /// * `amount0` - Amount of token0.
        /// * `amount1` - Amount of token1.
        /// * `sqrt_price_x96` - Price of the swap.
        /// * `liquidity` - Liquidity of the swap.
        /// * `tick` - Tick of the swap.
        pub struct _Swap {
            /// Address of the sender.
            pub sender: Address,
            /// Address of the recipient.
            pub recipient: Address,
            /// Amount of token0.
            pub amount0: I256,
            /// Amount of token1.
            pub amount1: I256,
            /// Price of the swap.
            pub sqrt_price_x96: U256,
            /// Liquidity of the swap.
            pub liquidity: U256,
            /// Tick of the swap.
            pub tick: i32,
        }

        let mut price_data: Vec<U256> = Vec::new();

        for log in past_logs {
            let log_topics: Vec<H256> = log.topics.clone();
            let log_data = log.data.0.into_iter().collect();

            let decoded_swap_event = match contract // Decode Swap event from Uniswap V3 contract
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

    /// Converts sqrt_price_x96 from U256 to float64
    /// # Arguments
    /// * `price_data` - Vector of U256 values that represent the historical sqrt_price_x96 for all swap events between specified block range.
    /// # Returns
    /// * `Vec<f64>` - Vector of f64 values that represent the historical price for all swap events between specified block range.
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
    /// # Arguments
    /// * `price_data` - Vector of U256 values that represent the historical price.
    /// * `file_path` - File path to save csv to.
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
