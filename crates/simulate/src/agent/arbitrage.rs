#![warn(missing_docs)]
//! Describes the most basic type of user agent.

use std::{sync::{Arc, RwLock}, thread};

use ethers::types::H256;
use revm::primitives::{Account, AccountInfo, Address, B160, U256};

use crate::{
    agent::{Agent, TransactSettings},
    environment::{SimulationEnvironment, SimulationContract, IsDeployed},
};

/// A user is an agent that can interact with the simulation environment generically.
pub struct Arbitrageur {
    /// Public address of the simulation manager.
    pub address: B160,
    /// revm-primitive account of the simulation manager.
    pub account: Account,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    transact_settings: TransactSettings,
    // TODO: is this useful? environment: Arc<Mutex<Environment>>,
    _environment: Arc<RwLock<SimulationEnvironment>>,
    /// read thread for logs
    _read_thread: Option<std::thread::JoinHandle<()>>,
    /// Write thread for execution of transactions
    _write_thread: Option<std::thread::JoinHandle<()>>,
    /// Boolean value to indicate if the agent is currently running
    _running: bool,
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
    pub fn new(environment: Arc<RwLock<SimulationEnvironment>>, address: B160) -> Self {
        Self {
            address,
            account: Account::from(AccountInfo::default()),
            transact_settings: TransactSettings {
                gas_limit: u64::MAX,
                gas_price: U256::ZERO,
            },
            _environment: environment,
            _read_thread: None,
            _write_thread: None,
            _running: false,
        }
    }
    /// Watch for arbitrage opportunities.
    pub async fn run(&self, market: SimulationContract<IsDeployed>) {
        let reader = self.receiver();
        let writer_base_contract = market.base_contract.clone();

        let handle = thread::spawn(move || {
            let mut i = 0;
            while let Ok(logs) = reader.recv() {
                println!("Got logs in alice's thread!");
                println!("{:?}", logs);
                println!("Got the right log in alice's thread!!");
                println!("Decoding logs!");
                let log_topics: Vec<H256> = logs.clone()[0]
                    .topics
                    .clone()
                    .into_iter()
                    .map(|x| H256::from_slice(x.as_slice()))
                    .collect();
                let log_data = logs[0].data.clone().into();
                let log_output = writer_base_contract
                    .decode_event::<String>("WasWritten", log_topics, log_data)
                    .unwrap();
                assert_eq!(log_output, "Hello, world!".to_string());
                println!("Got the right log in alice's thread!");
                println!("Decoding logs!");
                let log_topics: Vec<H256> = logs.clone()[0]
                    .topics
                    .clone()
                    .into_iter()
                    .map(|x| H256::from_slice(x.as_slice()))
                    .collect();
                let log_data = logs[0].data.clone().into();
                let log_output = writer_base_contract
                    .decode_event::<String>("WasWritten", log_topics, log_data)
                    .unwrap();
                assert_eq!(log_output, "Hello, world! again...".to_string());
                println!("Got the right log!");
            }
        });
    }
}