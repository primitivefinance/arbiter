#![warn(missing_docs)]
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TODO: Notes ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// We may need traits for Events and Actions (e.g., "Event" and "Action"
// which have a method like "parse()" and "produce()" or something.).
// Need an init signal or something.
// We can give agents a "calculator" evm to send "Actions" to when they are just
// doing compute so they aren't blocking the main tx thread.
// Maybe by default we should give agents a messager as part of their engine so we can call a
// "start" and "stop" with them.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! The agent module contains the core agent abstraction for the Arbiter Engine.

use std::sync::Arc;

use arbiter_core::middleware::RevmMiddleware;

use super::*;
use crate::{messager::Messager, world::World};

// TODO: For the time being, these agents are just meant to be for arbiter
// instances. We can generalize later.

/// An agent is an entity capable of processing events and producing actions.
/// These are the core actors in simulations or in onchain systems.
/// Agents can be connected of other agents either as a dependent, or a
/// dependency.
pub struct Agent {
    /// Identifier for this agent.
    /// Used for routing messages.
    pub id: String,

    /// The status of the agent.
    pub state: AgentState,

    /// The messager the agent uses to send and receive messages from other
    /// agents.
    pub messager: Messager,

    pub client: Arc<RevmMiddleware>,
}

impl Agent {
    /// Produces a new agent with the given identifier.
    pub(crate) fn connect<P>(id: &str, world: &World) -> Self {
        let messager = world.messager.for_agent(id);
        let client = RevmMiddleware::new(&world.environment, Some(id)).unwrap();
        Self {
            id: id.to_owned(),
            state: AgentState::Uninitialized,
            messager,
            client,
        }
    }
}

pub enum AgentState {
    Uninitialized,
    Startup,
    Running,
    Stopped,
}

pub trait Behavior {
    /// Used to bring the agent back up to date with the latest state of the
    /// world. This could be used if the world was stopped and later restarted.
    fn sync_state(&mut self);

    /// Used to start the agent.
    fn startup(&mut self);

    fn process(&mut self);
}
