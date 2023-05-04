#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Describes the most basic type of user agent.

use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crossbeam_channel::Receiver;
use revm::primitives::{Address, Log, U256};

use super::{AgentStatus, Identifiable, IsActive, NotActive};
use crate::agent::{filter_events, Agent, SimulationEventFilter, TransactSettings};

/// A user is an agent that can interact with the simulation environment generically.
pub struct Journaler<AgentState: AgentStatus> {
    /// Name of the agent.
    pub name: String,
    /// Public address of the simulation manager.
    pub address: AgentState::Address,
    /// [`revm::primitives`] account of the simulation manager.
    pub account_info: AgentState::AccountInfo,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    pub transact_settings: AgentState::TransactSettings,
    /// The [`crossbeam_channel::Receiver`] for the events are sent down from [`SimulationEnvironment`]'s dispatch.
    pub event_receiver: AgentState::EventReceiver,
    /// The filter for the events that the agent is interested in.
    pub event_filters: Vec<SimulationEventFilter>,
    /// Storage of prices of the two pools the [`SimpleArbitrageur`] tracks.
    pub csv_name: String,
}

impl<AgentState: AgentStatus> Identifiable for Journaler<AgentState> {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Agent for Journaler<IsActive> {
    fn address(&self) -> Address {
        self.address
    }
    fn transact_settings(&self) -> &TransactSettings {
        &self.transact_settings
    }
    fn receiver(&self) -> Receiver<Vec<Log>> {
        self.event_receiver.clone()
    }
    fn event_filters(&self) -> Vec<SimulationEventFilter> {
        self.event_filters.clone()
    }
}

impl Journaler<NotActive> {
    /// Creates a new [`SimpleArbitrageur`] which requires a vector of [`SimulationEventFilter`] and automatically sets default initial stored prices.
    pub fn new<S: Into<String>>(
        name: S,
        event_filters: Vec<SimulationEventFilter>,
        csv_name: S,
    ) -> Journaler<NotActive> {
        Journaler::<NotActive> {
            name: name.into(),
            address: (),
            account_info: (),
            transact_settings: (),
            event_receiver: (),
            event_filters,
            csv_name: csv_name.into(),
        }
    }
}

impl Journaler<IsActive> {
    /// A basic implementation that will detect price discprepencies from events emitted from pools.
    /// Currently implemented and tested only against the `liquid_exchange`.
    pub fn journal_events(&self) -> JoinHandle<()> {
        // let receiver = self.receiver();
        // let event_filters = self.event_filters();

        // let prices = Arc::clone(&self.prices);

        // thread::spawn(move || {
        //     let mut prices = prices.lock().unwrap();
        //     let decoder = |input, filter_num: usize| {
        //         event_filters[filter_num].base_contract.decode_event_raw(
        //             event_filters[filter_num].event_name.as_str(),
        //             vec![event_filters[filter_num].topic],
        //             input,
        //         )
        //     };
        //     while let Ok(logs) = receiver.recv() {
        //         // Get the logs and filter
        //         let filtered_logs = filter_events(event_filters.clone(), logs);
        //         println!("Filtered logs are: {:#?}", filtered_logs);

        //         if !filtered_logs.is_empty() {
        //             let data = filtered_logs[0].data.clone().into_iter().collect();

        //             // See which pool this came from
        //             let pool_number =
        //                 if filtered_logs[0].address == event_filters.clone()[0].address {
        //                     0
        //                 } else {
        //                     1
        //                 };

        //             let decoded_event = decoder(data, pool_number).unwrap(); // TODO: Fix the error handling here.
        //             println!("Decoded event says: {:#?}", decoded_event);
        //             let value = decoded_event[0].clone();
        //             println!("The value is: {:#?}", value);
        //             let value = value.into_uint().unwrap();
        //             prices[pool_number] = value.into();
        //             println!(
        //                 "Price for pool number {:#?} is {:#?}",
        //                 pool_number, prices[pool_number]
        //             );

        //             // look to see if this gives an arbitrage event
        //             // First filter out if one of the prices is MAX as this is the default state.
        //             if prices[0] != U256::MAX && prices[1] != U256::MAX {
        //                 let price_difference = prices[0].overflowing_sub(prices[1]);
        //                 println!("Price difference = {:#?}", price_difference);
        //                 if price_difference.1 {
        //                     println!("Arbitrage with price_0 < price_1");
        //                     break;
        //                 } else if price_difference.1 && price_difference.0 != U256::ZERO {
        //                     println!("Arbitrage with price_0 > price_1");
        //                     break;
        //                 }
        //             }
        //         }
        //     }
        //     println!("Exited arbitrage detection thread!");
        // })
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use std::{error::Error, sync::Arc};

    use bindings::{arbiter_token, liquid_exchange};
    use ethers::prelude::U256;
    use revm::primitives::B160;

    use super::SimpleArbitrageur;
    use crate::{
        agent::{create_filter, filter_events, Agent, AgentType},
        contract::SimulationContract,
        manager::SimulationManager,
        utils::recast_address,
    };

    #[test]
    fn write_csv() -> Result<(), Box<dyn Error>> {
        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();
        let admin = manager.agents.get("admin").unwrap();
        let environment = &mut manager.environment;

        // Get bytecode and abi for the writer contract.
        let writer =
            SimulationContract::new(writer::WRITER_ABI.clone(), writer::WRITER_BYTECODE.clone());

        // Deploy the writer contract.
        let writer = writer.deploy(environment, admin, ());

        let event_filters = vec![create_filter(
            &writer,
            "WasWritten",
        )];

        // Generate calldata for the 'echoString' function
        let test_string = "Hello, world!";
        let input_arguments = test_string.to_string();
        let call_data = writer.encode_function("echoString", input_arguments)?;

        // Call the 'echoString' function.
        let _execution_result = admin.call_contract(environment, &writer, call_data, Uint::ZERO);

        // Read logs twice since the first time is just the contract creation which gives no log.
        let _logs = admin.read_logs()?;
        let logs = admin.read_logs()?;

        // Decode the logs
        let log_topics = logs[0].topics.clone();
        let log_data = logs[0].data.clone();
        let log_output: String = writer.decode_event("WasWritten", log_topics, log_data)?;
        println!("Log Response: {:#?}", log_output);

        assert_eq!(log_output, test_string);
        Ok(())
    }
}
