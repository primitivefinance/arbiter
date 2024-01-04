#![warn(missing_docs)]
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TODO: Notes ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// When we start running an agent, we should have their messager start producing events that can be
// used by any and all behaviors the agent has that takes in messages as an event. Similarly, we
// should have agents start up any streams listeners that they need so those can also produce
// events. Those can then be piped into the behaviors that need them.
// Can perhaps make behaviors come from very specific events (e.g., specific contract events).
// This means each behavior should be a consumer and perhaps the agent itself is the producer (or at
// least relayer).
// This means we should give agents some way to "start streams" that they can then use to produce
// events.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! The agent module contains the core agent abstraction for the Arbiter Engine.

use std::sync::Arc;

use arbiter_core::middleware::RevmMiddleware;
use futures_util::{Stream, StreamExt};

use super::*;
use crate::{
    messager::{Message, Messager},
    world::World,
};

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

    pub streams: Vec<Box<dyn Stream<Item = Message>>>,
}

impl Agent {
    /// Produces a new agent with the given identifier.
    pub(crate) fn connect(id: &str, world: &World) -> Self {
        let messager = world.messager.for_agent(id);
        let client = RevmMiddleware::new(&world.environment, Some(id)).unwrap();
        Self {
            id: id.to_owned(),
            state: AgentState::Uninitialized,
            messager,
            client,
            streams: Vec::new(),
        }
    }

    pub fn add_stream(&mut self, stream: Box<dyn Stream<Item = Message>>) {
        self.streams.push(stream);
    }
}

pub enum AgentState {
    Uninitialized,
    Startup,
    Running,
    Stopped,
}

pub trait Behavior<E, A> {
    /// Used to bring the agent back up to date with the latest state of the
    /// world. This could be used if the world was stopped and later restarted.
    async fn sync_state(&mut self);

    /// Used to start the agent.
    async fn startup(&mut self);

    async fn process(&mut self, event: E) -> Vec<A>;
}
