#![warn(missing_docs)]
#![warn(unsafe_code)]

//! ## Agent trait and associated functionality
//!
//! An abstract representation of an agent on the EVM, to be used in simulations.
//! Some examples of agents are market makers or arbitrageurs.
//! All agents must implement the [`Agent`] traits and be included in the [`AgentType`] enum.
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    thread,
};

use bytes::Bytes;
use crossbeam_channel::Receiver;
use ethers::{prelude::BaseContract, types::H256};
use revm::primitives::{AccountInfo, Address, ExecutionResult, Log, TransactTo, TxEnv, B160, U256};

use self::{journaler::Journaler, simple_arbitrageur::SimpleArbitrageur, user::User};
use crate::{
    contract::{IsDeployed, SimulationContract},
    environment::SimulationEnvironment,
};

pub mod journaler;
pub mod simple_arbitrageur;
pub mod user;

#[derive(Debug)]
/// Error type for the simulation manager.
pub struct AgentError(String);

impl Error for AgentError {}

impl Display for AgentError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

/// A marker trait for [`Agent`] types.
/// Allows the agent to be in two states: [`NotActive`] and [`IsActive`].
/// These two states have different properties.
pub trait AgentStatus {
    /// The address information of an agent.
    type Address;
    /// The [`revm`] [`AccountInfo`] of the agent.
    type AccountInfo;
    /// The receiver for the log stream emitted by the [`SimulationEnvironment`].
    type EventReceiver;
    /// The settings that define how a given agent will transact with the [`SimulationEnvironment`].
    type TransactSettings;
}

/// Marker for an [`Agent`] when they are not yet added to the `SimulationManager` agent list.
pub struct NotActive;
/// Marker for an [`Agent`] that has been added to the `SimulationManager` agent list and has all the relevant information needed to be used in a simulation.
pub struct IsActive;

impl AgentStatus for NotActive {
    type Address = ();
    type AccountInfo = ();
    type EventReceiver = ();
    type TransactSettings = ();
}

impl AgentStatus for IsActive {
    type Address = B160;
    type AccountInfo = AccountInfo;
    type EventReceiver = Receiver<Vec<Log>>;
    type TransactSettings = TransactSettings;
}

/// Gives a function to retrieve the name of an [`Agent`].
pub trait Identifiable {
    /// Returns the name of an [`IsActive`] or [`NotActive`] [`Agent`] (or otherwise identifiable type).
    fn name(&self) -> String;
}

/// An agent is an entity that can interact with the simulation environment.
/// Agents can be various entities such as users, market makers, arbitrageurs, etc.
/// The [`User`] and [`SimpleArbitrageur`] agents are currently implemented.
pub enum AgentType<AgentState: AgentStatus> {
    /// A [`User`] is the most basic agent that can interact with the simulation environment.
    User(User<AgentState>),
    /// A [`SimpleArbitrageur`] is an agent that can perform arbitrage between two pools.
    SimpleArbitrageur(SimpleArbitrageur<AgentState>),
    /// A [`Journaler`] is an agent that can be used to log events to a file.
    Journaler(Journaler<AgentState>),
}

impl AgentType<IsActive> {
    /// Retrieves the inner `&dyn Agent` struct inside of the [`AgentType`] enum.
    pub fn inner(&self) -> &dyn Agent {
        match self {
            AgentType::User(inner) => inner,
            AgentType::SimpleArbitrageur(inner) => inner,
            AgentType::Journaler(inner) => inner,
        }
    }
}

impl AgentType<NotActive> {
    /// Retrieves the inner `&dyn Identifiable` struct inside of the [`AgentType`] enum.
    pub fn inner(&self) -> &dyn Identifiable {
        match self {
            AgentType::User(inner) => inner,
            AgentType::SimpleArbitrageur(inner) => inner,
            AgentType::Journaler(inner) => inner,
        }
    }
}

impl Identifiable for AgentType<IsActive> {
    fn name(&self) -> String {
        self.inner().name()
    }
}

impl Identifiable for AgentType<NotActive> {
    fn name(&self) -> String {
        self.inner().name()
    }
}

impl Agent for AgentType<IsActive> {
    fn address(&self) -> Address {
        self.inner().address()
    }

    fn transact_settings(&self) -> &TransactSettings {
        self.inner().transact_settings()
    }

    fn receiver(&self) -> Receiver<Vec<Log>> {
        self.inner().receiver()
    }

    fn event_filters(&self) -> Vec<SimulationEventFilter> {
        self.inner().event_filters()
    }
}

/// Describes the gas settings for a transaction.
pub struct TransactSettings {
    /// Gas limit for the transaction for a simulation.
    pub gas_limit: u64,
    /// Gas limit for the transaction for a simulation.
    pub gas_price: U256,
}

/// Basic traits that every `Agent` must implement in order to properly interact with an EVM.
pub trait Agent: Identifiable {
    /// Returns the address of the agent.
    fn address(&self) -> Address;
    /// Returns the transaction settings of the agent.
    fn transact_settings(&self) -> &TransactSettings;
    /// The event's channel receiver for the agent.
    fn receiver(&self) -> Receiver<Vec<Log>>;
    /// Gets the event filter for the [`Agent`]
    fn event_filters(&self) -> Vec<SimulationEventFilter>;

    /// Used to allow agents to make a generic call a specific smart contract.
    fn call_contract(
        &self,
        simulation_environment: &mut SimulationEnvironment,
        contract: &SimulationContract<IsDeployed>,
        call_data: Bytes,
        value: U256,
    ) -> ExecutionResult {
        let tx = self.build_call_transaction(contract.address, call_data, value);
        simulation_environment.execute(tx)
    }

    /// A constructor to build a `TxEnv` for an agent (uses agent data like `address` and `TransactSettings`).
    fn build_call_transaction(
        &self,
        receiver_address: B160,
        call_data: Bytes,
        value: U256,
    ) -> TxEnv {
        TxEnv {
            caller: self.address(),
            gas_limit: self.transact_settings().gas_limit,
            gas_price: self.transact_settings().gas_price,
            gas_priority_fee: None,
            transact_to: TransactTo::Call(receiver_address),
            value,
            data: call_data,
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        }
    }

    // TODO: May be defunct to read logs now
    /// Gets the most current event (which is all that is stored in the event buffer).
    fn read_logs(&self) -> Result<Vec<Log>, AgentError> {
        match self.receiver().recv() {
            Ok(logs) => Ok(logs),
            Err(e) => Err(AgentError(format!("Error reading logs for agent: {}", e))),
        }
    }
    // TODO: Right now this is only detecting an arb event and instead...
    // TODO: add a condition as a bool valued function?
    // TODO: It would be optimal to not build functions inside of the monitor events since it could get called often. Ideally we just don't call it often?
    /// Monitor events for the agent.
    fn monitor_events(&self) {
        let receiver = self.receiver();
        let event_filters = self.event_filters();
        thread::spawn(move || {
            let decoder = |input, filter_num: usize| {
                event_filters[filter_num].base_contract.decode_event_raw(
                    event_filters[filter_num].event_name.as_str(),
                    vec![event_filters[filter_num].topic],
                    input,
                )
            };
            while let Ok(logs) = receiver.recv() {
                let filtered_logs = filter_events(event_filters.clone(), logs);
                println!("Filtered logs are: {:#?}", filtered_logs);
                let data = filtered_logs[0].data.clone().into_iter().collect();
                let decoded_event = decoder(data, 0).unwrap(); // TODO: Fix the error handling here.
                println!("Decoded event says: {:#?}", decoded_event);
                let value = decoded_event[0].clone();
                println!("The value is: {:#?}", value);
            }
        });
    }
}

#[derive(Debug, Clone)]
/// The filtering implmentation to be used with the [`Agent`] trait.
pub struct SimulationEventFilter {
    /// The addresses to filter for.
    pub address: B160,
    /// The event names to filter for.
    pub topic: H256,
    /// A private copy of the [`BaseContract`] for whichever contract is used to generate filters.
    base_contract: BaseContract,
    /// The name of the event emitted by a contract.
    pub event_name: String,
}

/// Creates a filter for the agent to use to filter out events.
pub fn create_filter(
    contract: &SimulationContract<IsDeployed>,
    event_name: &str,
) -> SimulationEventFilter {
    let topic = contract
        .base_contract
        .abi()
        .event(event_name)
        .unwrap()
        .signature();
    // let decoder = |input| contract.decode_event::<[Token]>(event_name, vec![topic.into()], input);
    SimulationEventFilter {
        address: contract.address,
        topic,
        base_contract: contract.base_contract.clone(),
        event_name: event_name.to_string(),
    }
}

/// Used to allow agents to filter out the events they choose to monitor.
pub fn filter_events(event_filters: Vec<SimulationEventFilter>, logs: Vec<Log>) -> Vec<Log> {
    if event_filters.is_empty() {
        return logs;
    }

    let mut events = vec![];

    for log in logs {
        for event_filter in event_filters.iter() {
            if event_filter.address == log.address && event_filter.topic == log.topics[0].into()
            // TODO: Needs to not just be log.topics[0]
            {
                events.push(log.clone());
                break;
            }
        }
    }

    events
}

#[cfg(test)]
mod tests {

    use std::error::Error;

    use bindings::{arbiter_token, writer};
    use revm::primitives::{ruint::Uint, B160};

    use crate::{
        agent::{create_filter, user::User, Agent, AgentType},
        contract::SimulationContract,
        manager::SimulationManager,
    };

    #[test]
    fn agent_event_filter_through() -> Result<(), Box<dyn Error>> {
        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();

        // Create writer contract.
        let writer =
            SimulationContract::new(writer::WRITER_ABI.clone(), writer::WRITER_BYTECODE.clone());

        // Deploy the writer.
        let test_string = "Hello, world..?".to_string();
        let writer = writer.deploy(
            &mut manager.environment,
            manager.agents.get("admin").unwrap(),
            test_string,
        );

        // Create two agents with a filter.
        let alice = User::new("alice", None);
        manager.activate_agent(AgentType::User(alice), B160::from_low_u64_be(2))?;

        let event_filters = vec![create_filter(&writer, "WasWritten")];
        let bob = User::new("bob", Some(event_filters));
        manager.activate_agent(AgentType::User(bob), B160::from_low_u64_be(3))?;

        let alice = manager.agents.get("alice").unwrap();
        let bob = manager.agents.get("bob").unwrap();

        println!("Alice's event filter: {:#?}", alice.event_filters());
        println!("Bob's event filter: {:#?}", bob.event_filters());

        // Make calls that alice and bob won't filter out.
        let new_string = "Hello, world!".to_string();
        let call_data = writer.encode_function("echoString", new_string)?;
        manager.agents.get("admin").unwrap().call_contract(
            &mut manager.environment,
            &writer,
            call_data,
            Uint::ZERO,
        );
        // Test that the alice doesn't filter out these logs.
        let unfiltered_events = alice.read_logs()?;
        let filtered_events =
            super::filter_events(alice.event_filters(), unfiltered_events.clone());
        println!(
            "The filtered events for alice on the first call are: {:#?}",
            &filtered_events
        );
        assert_eq!(filtered_events, unfiltered_events);
        // Test that bob filters out these logs.
        let unfiltered_events = bob.read_logs()?;
        let filtered_events = super::filter_events(bob.event_filters(), unfiltered_events.clone());
        println!(
            "The filtered events for bob on the first call are: {:#?}",
            &filtered_events
        );

        // Also try to filter out a different address.

        Ok(())
    }

    #[test]
    fn agent_event_filter_out() -> Result<(), Box<dyn Error>> {
        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();

        // Create writer contract.
        let writer =
            SimulationContract::new(writer::WRITER_ABI.clone(), writer::WRITER_BYTECODE.clone());
        let test_string = "Hello, world..?".to_string();
        let writer = writer.deploy(
            &mut manager.environment,
            manager.agents.get("admin").unwrap(),
            test_string,
        );

        // Create writer contract.
        let arbt = SimulationContract::new(
            arbiter_token::ARBITERTOKEN_ABI.clone(),
            arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
        );
        let arbt = arbt.deploy(
            &mut manager.environment,
            manager.agents.get("admin").unwrap(),
            ("ArbiterToken".to_string(), "ARBT".to_string(), 18_u8),
        );

        // Create agent with a filter.
        let event_filters = vec![create_filter(&arbt, "Approval")];
        let alice = User::new("alice", Some(event_filters));
        manager.activate_agent(AgentType::User(alice), B160::from_low_u64_be(2))?;
        let alice = manager.agents.get("alice").unwrap();

        println!("Alice's event filter: {:#?}", alice.event_filters());

        // Make calls that alice and bob won't filter out.
        let new_string = "Hello, world!".to_string();
        let call_data = writer.encode_function("echoString", new_string)?;
        manager.agents.get("admin").unwrap().call_contract(
            &mut manager.environment,
            &writer,
            call_data,
            Uint::ZERO,
        );
        // Test that the alice doesn't filter out these logs.
        let unfiltered_events = alice.read_logs()?;
        let filtered_events =
            super::filter_events(alice.event_filters(), unfiltered_events.clone());
        println!(
            "The filtered events for alice on the first call are: {:#?}",
            &filtered_events
        );
        assert_eq!(filtered_events, vec![]);
        Ok(())
    }
}
