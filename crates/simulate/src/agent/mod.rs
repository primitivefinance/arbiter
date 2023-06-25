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
    pin::Pin,
};

use bytes::Bytes;
use crossbeam_channel::{Receiver, Sender};
use ethers::{abi::Token, prelude::BaseContract, types::H256};

use ethers::core::abi::AbiError;

use revm::primitives::{
    AccountInfo, Address, ExecutionResult, Log, Output, TransactTo, TxEnv, B160, U256,
};

use self::{simple_arbitrageur::SimpleArbitrageur, user::User};
use crate::environment::contract::{IsDeployed, NotDeployed, SimulationContract};
use futures::Stream;

pub mod simple_arbitrageur;
pub mod user;

/// Error type for the simulation manager.
#[derive(Debug)]
pub struct AgentError(String);

/// Type Alias for the watch result.
pub type WatchResult = Result<(Vec<Token>, usize), AbiError>;
/// Type Alias for the event channel.
pub type WatchStream = dyn Stream<Item = WatchResult> + Send + Sync;

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
    type EventStream;
    /// The settings that define how a given agent will transact with the [`SimulationEnvironment`].
    type TransactSettings;
    /// A sender for the agent to send transactions to the [`SimulationEnvironment`].
    type TransactionSender;
    /// A channel for the agent to receive the result of a transaction from the [`SimulationEnvironment`].
    type ResultChannel;
}

/// Marker for an [`Agent`] when they are not yet added to the `SimulationManager` agent list.
#[derive(Clone)]
pub struct NotActive;
/// Marker for an [`Agent`] that has been added to the `SimulationManager` agent list and has all the relevant information needed to be used in a simulation.
#[derive(Clone)]
pub struct IsActive;

impl AgentStatus for NotActive {
    type Address = ();
    type AccountInfo = ();
    type EventStream = ();
    type TransactSettings = ();
    type TransactionSender = ();
    type ResultChannel = ();
}

impl AgentStatus for IsActive {
    type Address = B160;
    type AccountInfo = AccountInfo;
    type EventStream = crate::environment::EventStream;
    type TransactSettings = TransactSettings;
    type TransactionSender = Sender<(TxEnv, Sender<ExecutionResult>)>;
    type ResultChannel = (Sender<ExecutionResult>, Receiver<ExecutionResult>);
}

/// Gives a function to retrieve the name of an [`Agent`].
pub trait Identifiable {
    /// Returns the name of an [`IsActive`] or [`NotActive`] [`Agent`] (or otherwise identifiable type).
    fn name(&self) -> String;
}

/// An agent is an entity that can interact with the simulation environment.
/// Agents can be various entities such as users, market makers, arbitrageurs, etc.
/// The [`User`] and [`SimpleArbitrageur`] agents are currently implemented.]
pub enum AgentType<AgentState: AgentStatus> {
    /// A [`User`] is the most basic agent that can interact with the simulation environment.
    User(User<AgentState>),
    /// A [`SimpleArbitrageur`] is an agent that can perform arbitrage between two pools.
    SimpleArbitrageur(SimpleArbitrageur<AgentState>),
}

impl AgentType<IsActive> {
    /// Retrieves the inner `&dyn Agent` struct inside of the [`AgentType`] enum.
    pub fn inner(&self) -> &dyn Agent {
        match self {
            AgentType::User(inner) => inner,
            AgentType::SimpleArbitrageur(inner) => inner,
        }
    }
}

impl AgentType<NotActive> {
    /// Retrieves the inner `&dyn Identifiable` struct inside of the [`AgentType`] enum.
    pub fn inner(&self) -> &dyn Identifiable {
        match self {
            AgentType::User(inner) => inner,
            AgentType::SimpleArbitrageur(inner) => inner,
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

    fn event_stream(&self) -> crate::environment::EventStream {
        self.inner().event_stream()
    }

    fn event_filters(&self) -> Vec<SimulationEventFilter> {
        self.inner().event_filters()
    }

    fn transaction_sender(&self) -> Sender<(TxEnv, Sender<ExecutionResult>)> {
        self.inner().transaction_sender()
    }

    fn result_channel(&self) -> (Sender<ExecutionResult>, Receiver<ExecutionResult>) {
        self.inner().result_channel()
    }
}

/// Describes the gas settings for a transaction.
#[derive(Debug, Clone)]
pub struct TransactSettings {
    /// Gas limit for the transaction for a simulation.
    pub gas_limit: u64,
    /// Gas limit for the transaction for a simulation.
    pub gas_price: U256,
}

/// Basic traits that every `Agent` must implement in order to properly interact with an EVM.
#[async_trait::async_trait]
pub trait Agent: Identifiable {
    // type FilterWatcher: Stream<Item = Result<(Vec<Token>, usize), AbiError>> + Send + Sync;
    /// Returns the address of the agent.
    fn address(&self) -> Address;
    /// Returns the transaction settings of the agent.
    fn transact_settings(&self) -> &TransactSettings;
    /// The event's channel receiver for the agent.
    fn event_stream(&self) -> crate::environment::EventStream;
    /// Gets the event filter for the [`Agent`]
    fn event_filters(&self) -> Vec<SimulationEventFilter>;
    /// Used to allow agents to make a txn with the evm.
    fn transaction_sender(&self) -> Sender<(TxEnv, Sender<ExecutionResult>)>;
    /// Used to allow agents to receive the result of a txn.
    fn result_channel(&self) -> (Sender<ExecutionResult>, Receiver<ExecutionResult>);

    /// Used to allow agents to make a generic call a specific smart contract.
    fn call(
        &self,
        contract: &SimulationContract<IsDeployed>,
        function_name: &str,
        args: Vec<Token>,
    ) -> Result<ExecutionResult, Box<dyn Error>> {
        let function = contract.base_contract.abi().function(function_name)?;
        let call_data = function.encode_input(&args)?.into_iter().collect();
        let tx = self.build_call_transaction(contract.address, call_data);
        self.transaction_sender()
            .send((tx, self.result_channel().0))?;
        Ok(self.result_channel().1.recv()?)
    }

    /// A constructor to build a `TxEnv` for an agent (uses agent data like `address` and `TransactSettings`).
    fn build_call_transaction(
        &self,
        receiver_address: B160,
        call_data: Bytes,
        // value: U256,
    ) -> TxEnv {
        TxEnv {
            caller: self.address(),
            gas_limit: self.transact_settings().gas_limit,
            gas_price: self.transact_settings().gas_price,
            gas_priority_fee: None,
            transact_to: TransactTo::Call(receiver_address),
            value: U256::ZERO, // TODO: I doubt we want to ever send any raw eth to a contract.
            data: call_data,
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        }
    }

    /// This returns a `filtered watcher` stream for the agent.
    fn watch(&self) -> Pin<Box<WatchStream>> {
        Box::pin(self.event_stream().into_stream())
    }

    /// Deploy a contract to the current simulation environment and return a new [`SimulationContract<IsDeployed>`].
    /// Does not consume the current [`SimulationContract<NotDeployed>`] so that more copies can be deployed later.
    /// # Arguments
    /// * `simulation_environment` - The [`SimulationEnvironment`] to deploy the contract to.
    /// * `deployer` - The [`AgentType`] that will deploy the contract.
    /// * `constructor_arguments` - The constructor arguments for the contract.
    /// # Returns
    /// * `SimulationContract<IsDeployed>` - The deployed contract.
    fn deploy(
        &self,
        contract: SimulationContract<NotDeployed>,
        constructor_arguments: Vec<Token>,
    ) -> Result<(SimulationContract<IsDeployed>, ExecutionResult), Box<dyn Error>> {
        // Append constructor args (if available) to generate the deploy bytecode.
        let bytecode = match contract.base_contract.abi().constructor.clone() {
            Some(constructor) => Bytes::from(
                constructor.encode_input(contract.bytecode.clone(), &constructor_arguments)?,
            ),
            None => Bytes::from(contract.bytecode.clone()),
        };

        // Take the execution result and extract the contract address.
        let deploy_txenv = TxEnv {
            caller: self.address(),
            gas_limit: self.transact_settings().gas_limit,
            gas_price: self.transact_settings().gas_price,
            gas_priority_fee: None,
            transact_to: TransactTo::create(),
            value: U256::ZERO,
            data: bytecode,
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        };
        self.transaction_sender()
            .send((deploy_txenv, self.result_channel().0))?;
        let execution_result = self.result_channel().1.recv().unwrap(); // TODO: bad error handling here.
                                                                        // let execution_result = simulation_environment.execute(deploy_txenv);
        let output = match execution_result.clone() {
            ExecutionResult::Success { output, .. } => output,
            ExecutionResult::Revert { output, .. } => panic!("Failed due to revert: {:?}", output),
            ExecutionResult::Halt { reason, .. } => panic!("Failed due to halt: {:?}", reason),
        };
        let address = match output {
            Output::Create(_, address) => address.unwrap(),
            _ => panic!("failed"),
        };

        Ok((
            SimulationContract {
                bytecode: (),
                address,
                base_contract: contract.base_contract,
                constructor_arguments,
            },
            execution_result,
        ))
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
    pub(crate) base_contract: BaseContract,
    /// The name of the event emitted by a contract.
    pub event_name: String,
}

impl SimulationEventFilter {
    /// Creates a filter for the agent to use to filter out events.
    pub fn new(
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
}

/// Used to allow agents to filter out the events they choose to monitor.
pub fn filter_events(
    event_filters: Vec<SimulationEventFilter>,
    logs: Vec<Log>,
) -> (Vec<Log>, usize) {
    assert!(!event_filters.is_empty());

    let mut events = vec![];
    let mut event_index = 0;

    for log in logs {
        for (index, event_filter) in event_filters.iter().enumerate() {
            if event_filter.address == log.address && event_filter.topic == log.topics[0].into() {
                events.push(log.clone());
                event_index = index;
                break;
            }
        }
    }

    (events, event_index)
}

#[cfg(test)]
mod tests {

    use std::error::Error;

    use bindings::{arbiter_token, writer};
    use ethers::abi::Tokenize;
    use revm::primitives::B160;

    use crate::{
        agent::{user::User, Agent, AgentType, SimulationEventFilter},
        environment::contract::SimulationContract,
        manager::SimulationManager,
    };

    use futures::stream::StreamExt;

    #[tokio::test]
    // Tests that a single filter is working correctly.
    async fn agent_event_filtering() -> Result<(), Box<dyn Error>> {
        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();

        // Create writer contract.
        let writer =
            SimulationContract::new(writer::WRITER_ABI.clone(), writer::WRITER_BYTECODE.clone());

        // Deploy the writer.
        let test_string = "Hello, world..?".to_string();
        let (writer, execution_result) = manager
            .agents
            .get("admin")
            .unwrap()
            .deploy(writer, test_string.into_tokens())?;
        assert!(execution_result.is_success());

        // Deploy arbiter token contract.
        let arbt = SimulationContract::new(
            arbiter_token::ARBITERTOKEN_ABI.clone(),
            arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
        );
        let (arbt, execution_result) = manager.agents.get("admin").unwrap().deploy(
            arbt,
            ("ArbiterToken".to_string(), "ARBT".to_string(), 18_u8).into_tokens(),
        )?;
        assert!(execution_result.is_success());

        // Create two agents with a filter.
        // Create agent with a filter.
        let event_filters = vec![SimulationEventFilter::new(&arbt, "Approval")];
        let alice = User::new("alice", Some(event_filters));
        manager.activate_agent(AgentType::User(alice), B160::from_low_u64_be(2))?;

        let event_filters = vec![SimulationEventFilter::new(&writer, "WasWritten")];
        let bob = User::new("bob", Some(event_filters));
        manager.activate_agent(AgentType::User(bob), B160::from_low_u64_be(3))?;

        let alice = manager.agents.get("alice").unwrap();
        let bob = manager.agents.get("bob").unwrap();
        println!("Alice's event filter: {:#?}", alice.event_filters());
        println!("Bob's event filter: {:#?}", bob.event_filters());

        // Make calls that alice will filter out and bob won't filter out.
        let new_string = "Hello, world!".to_string();
        manager.agents.get("admin").unwrap().call(
            &writer,
            "echoString",
            new_string.into_tokens(),
        )?;

        // Test that the alice doesn't filter out these logs.
        let mut alice_watcher = alice.watch();
        let alice_next_event = alice_watcher.next().await;
        print!("Alice's next event: {:#?}", alice_next_event);
        assert!(alice_next_event.is_none());
        // Test that bob filters out these logs.
        let mut bob_watcher = bob.watch();
        let bob_next_event = bob_watcher.next().await;
        print!("Bob's next event: {:#?}", bob_next_event);
        assert!(bob_next_event.is_some());
        Ok(())
    }
}
