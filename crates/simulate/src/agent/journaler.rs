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
        let receiver = self.receiver();
        let event_filters = self.event_filters();
        assert!(event_filters.len() == 1); // TODO: Allow journaler to have more than just a single event filter.

        thread::spawn(move || {
            let decoder = |input| {
                event_filters[0].base_contract.decode_event_raw(
                    event_filters[0].event_name.as_str(),
                    vec![event_filters[0].topic],
                    input,
                )
            };
            while let Ok(logs) = receiver.recv() {
                // Get the logs and filter
                let filtered_logs = filter_events(event_filters.clone(), logs);
                println!("Filtered logs are: {:#?}", filtered_logs);

                if !filtered_logs.is_empty() {
                    println!("Log data is: {:#?}", filtered_logs[0].data);
                    let data = filtered_logs[0].data.clone().into_iter().collect();

                    let decoded_event = decoder(data).unwrap(); // TODO: Fix the error handling here.
                    println!("Decoded event says: {:#?}", decoded_event);
                    let value = decoded_event[0].clone();
                    println!("The value is: {:#?}", value);
                    let value = value.into_string().unwrap();
                    println!("Value is: {:#?}", value,);
                }
            }
            println!("Exited journaling writing thread!");
        })
    }
}

#[cfg(test)]
mod tests {

    use std::{error::Error, sync::Arc};

    use bindings::writer;
    use ethers::prelude::U256;
    use revm::primitives::{ruint::Uint, B160};

    use super::Journaler;
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

        // Get bytecode and abi for the writer contract.
        let writer =
            SimulationContract::new(writer::WRITER_ABI.clone(), writer::WRITER_BYTECODE.clone());

        // Deploy the writer contract.
        let writer = writer.deploy(
            &mut manager.environment,
            manager.agents.get("admin").unwrap(),
            (),
        );

        let event_filters = vec![create_filter(&writer, "WasWritten")];

        // Create the journaler and start listening for events.
        let journaler =
            AgentType::Journaler(Journaler::new("journaler", event_filters, "test.csv"));
        manager.activate_agent(journaler, B160::from_low_u64_be(2))?;
        let journaler = manager.agents.get("journaler").unwrap();
        let base_journaler = match journaler {
            AgentType::Journaler(base_journaler) => base_journaler,
            _ => panic!(),
        };
        base_journaler.journal_events();

        // Generate calldata for the 'echoString' function
        let test_string = "Hello, world!";
        let input_arguments = test_string.to_string();
        let call_data = writer.encode_function("echoString", input_arguments)?;

        // Call the 'echoString' function.
        let _execution_result = manager.agents.get("admin").unwrap().call_contract(
            &mut manager.environment,
            &writer,
            call_data,
            Uint::ZERO,
        );
        Ok(())
    }
}
