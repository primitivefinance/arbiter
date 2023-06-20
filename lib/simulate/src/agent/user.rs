#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Describes the most basic type of user agent.

use crossbeam_channel::Receiver;
use revm::primitives::Address;

use super::{AgentStatus, Identifiable, IsActive, NotActive};
use crate::agent::{Agent, SimulationEventFilter, TransactSettings};

/// A user is an agent that can interact with the simulation environment generically.
#[derive(Debug, Clone)]
pub struct User<AgentState: AgentStatus> {
    /// Name of the agent.
    pub name: String,
    /// Public address of the simulation manager.
    pub address: AgentState::Address,
    /// [`revm::primitives`] account of the simulation manager.
    pub account_info: AgentState::AccountInfo,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    pub transact_settings: AgentState::TransactSettings,
    /// The [`crossbeam_channel::Sender`] for sending transactions to the `SimulationEnvironment`.
    pub transaction_sender: AgentState::TransactionSender,
    /// The [`crossbeam_channel`] for getting [`ExecutionResult`] back from the `SimulationEnvironment`.
    pub result_channel: AgentState::ResultChannel,
    /// The [`crossbeam_channel::Receiver`] for the events are sent down from [`SimulationEnvironment`]'s dispatch.
    pub event_stream: AgentState::EventStream,
    /// The filter for the events that the agent is interested in.
    pub event_filters: Vec<SimulationEventFilter>,
}

impl<AgentState: AgentStatus> Identifiable for User<AgentState> {
    fn name(&self) -> String {
        self.name.clone()
    }
}

// TODO: I'm not sure we need all the things cloned here.
impl Agent for User<IsActive> {
    fn address(&self) -> Address {
        self.address
    }
    fn transact_settings(&self) -> &TransactSettings {
        &self.transact_settings
    }
    fn event_stream(&self) -> crate::environment::EventStream {
        self.event_stream.clone()
    }
    fn event_filters(&self) -> Vec<SimulationEventFilter> {
        self.event_filters.clone()
    }
    fn transaction_sender(
        &self,
    ) -> crossbeam_channel::Sender<(
        revm::primitives::TxEnv,
        crossbeam_channel::Sender<revm::primitives::ExecutionResult>,
    )> {
        self.transaction_sender.clone()
    }
    fn result_channel(
        &self,
    ) -> (
        crossbeam_channel::Sender<revm::primitives::ExecutionResult>,
        Receiver<revm::primitives::ExecutionResult>,
    ) {
        self.result_channel.clone()
    }
}

impl User<NotActive> {
    /// Creates a new [`User`] which takes an optional vector of [`SimulationEventFilter`] and automatically sets default initial stored prices.
    pub fn new<S: Into<String>>(
        name: S,
        event_filters: Option<Vec<SimulationEventFilter>>,
    ) -> User<NotActive> {
        User::<NotActive> {
            name: name.into(),
            address: (),
            account_info: (),
            transact_settings: (),
            event_stream: (),
            event_filters: event_filters.unwrap_or_default(),
            transaction_sender: (),
            result_channel: (),
        }
    }
}
