#![warn(missing_docs)]
//! Describes the most basic type of user agent.

use std::thread;

use ethers::types::H256;
use revm::primitives::{Address, B160, U256};

use crate::{
    agent::{Agent, TransactSettings},
    environment::{IsDeployed, SimulationContract},
};

/// A user is an agent that can interact with the simulation environment generically.
pub struct Arbitrageur {
    /// Public address of the simulation manager.
    pub address: B160,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    transact_settings: TransactSettings,
}

impl Agent for Arbitrageur {
    fn address(&self) -> Address {
        self.address
    }
    fn transact_settings(&self) -> &TransactSettings {
        &self.transact_settings
    }

    fn receiver(&self) -> crossbeam_channel::Receiver<Vec<revm::primitives::Log>> {
        todo!()
    }

    fn filter_events(&self) {
        todo!()
    }
}

impl Arbitrageur {
    /// Constructor function to instantiate a
    pub fn new(address: B160) -> Self {
        Self {
            address,
            transact_settings: TransactSettings {
                gas_limit: u64::MAX,
                gas_price: U256::ZERO,
            },
        }
    }
    /// Watch for arbitrage opportunities.
    pub async fn run(&self, market: SimulationContract<IsDeployed>) {
        let reader = self.receiver();
        let market_base_contract = market.base_contract;

        let _ = thread::spawn(move || {
            while let Ok(logs) = reader.recv() {
                println!("Got logs");
                println!("{:?}", logs);
                println!("Decoding logs!");
                let log_topics: Vec<H256> = logs.clone()[0]
                    .topics
                    .clone()
                    .into_iter()
                    .map(|x| H256::from_slice(x.as_slice()))
                    .collect();
                let log_data = logs[0].data.clone().into();
                let _ = market_base_contract
                    .decode_event::<String>("REPLACE WITH EVENT NAME", log_topics, log_data)
                    .unwrap();
                println!("Got the right log in alice's thread!");
                println!("Decoding logs!");
                let log_topics: Vec<H256> = logs.clone()[0]
                    .topics
                    .clone()
                    .into_iter()
                    .map(|x| H256::from_slice(x.as_slice()))
                    .collect();
                let log_data = logs[0].data.clone().into();
                let _ = market_base_contract
                    .decode_event::<String>("REPLACE WITH EVENT NAME", log_topics, log_data)
                    .unwrap();
                println!("Got the right log!");
            }
        });
    }
}
