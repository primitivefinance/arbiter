#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Describes the most basic type of user agent.

use crossbeam_channel::Receiver;
use revm::primitives::{Address, Log};

use super::{AgentStatus, Identifiable, IsActive, NotActive};
use crate::agent::{Agent, SimulationEventFilter, TransactSettings};

/// A user is an agent that can interact with the simulation environment generically.
pub struct User<AgentState: AgentStatus> {
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
}

impl<AgentState: AgentStatus> Identifiable for User<AgentState> {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Agent for User<IsActive> {
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
            event_receiver: (),
            event_filters: event_filters.unwrap_or_default(),
        }
    }
}
