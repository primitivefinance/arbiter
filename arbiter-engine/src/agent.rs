//! The agent module contains the core agent abstraction for the Arbiter Engine.

use std::{fmt::Debug, sync::Arc};

use arbiter_core::middleware::RevmMiddleware;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

use crate::{
    machine::{Behavior, Engine, StateMachine},
    messager::Messager,
};

/// An agent is an entity capable of processing events and producing actions.
/// These are the core actors in simulations or in onchain systems.
/// Agents can be connected of other agents either as a dependent, or a
/// dependency.
///
/// # How it works
/// When the [`World`] that owns the [`Agent`] is ran, it has each [`Agent`] run
/// each of its [`Behavior`]s `startup()` methods. The [`Behavior`]s themselves
/// will return a stream of events that then let the [`Behavior`] move into the
/// `State::Processing` stage.
pub struct Agent {
    /// Identifier for this agent.
    /// Used for routing messages.
    pub id: String,

    /// The messager the agent uses to send and receive messages from other
    /// agents.
    pub messager: Messager,

    /// The client the agent uses to interact with the blockchain.
    pub client: Arc<RevmMiddleware>,

    /// The engines/behaviors that the agent uses to sync, startup, and process
    /// events.
    pub(crate) behavior_engines: Vec<Box<dyn StateMachine>>,
}

impl Debug for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Agent")
            .field("id", &self.id)
            .field("messager", &self.messager)
            .field("client", &self.client)
            .field("behavior_engines", &self.behavior_engines)
            .finish()
    }
}

impl Agent {
    /// Produces a minimal agent builder with the given identifier.
    pub fn builder(id: &str) -> AgentBuilder {
        AgentBuilder {
            id: id.to_owned(),
            behavior_engines: None,
        }
    }
}

/// [`AgentBuilder`] represents the intermediate state of agent creation before
/// it is converted into a full on [`Agent`]
#[derive(Serialize, Deserialize)]
pub struct AgentBuilder {
    /// Identifier for this agent.
    /// Used for routing messages.
    pub id: String,
    /// The engines/behaviors that the agent uses to sync, startup, and process
    /// events.
    behavior_engines: Option<Vec<Box<dyn StateMachine>>>,
}

impl AgentBuilder {
    /// Appends a behavior onto an [`AgentBuilder`]. Behaviors are initialized
    /// when the agent builder is added to the [`crate::world::World`]
    pub fn with_behavior<E: DeserializeOwned + Send + Sync + Debug + 'static>(
        mut self,
        behavior: impl Behavior<E> + 'static,
    ) -> Self {
        let engine = Engine::new(behavior);
        if let Some(engines) = &mut self.behavior_engines {
            engines.push(Box::new(engine));
        } else {
            self.behavior_engines = Some(vec![Box::new(engine)]);
        };
        self
    }

    /// Produces a new [`Agent`] with the given identifier.
    pub fn build(
        self,
        client: Arc<RevmMiddleware>,
        messager: Messager,
    ) -> Result<Agent, AgentBuildError> {
        match self.behavior_engines {
            Some(engines) => Ok(Agent {
                id: self.id,
                messager,
                client,
                behavior_engines: engines,
            }),
            None => Err(AgentBuildError::MissingBehaviorEngines),
        }
    }
}

/// enum representing the possible error states encountered by the agent builder
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum AgentBuildError {
    /// Error representing the case where the agent is missing behavior engines;
    /// an agent has to have behaviors to be useful!
    #[error("Agent is missing behavior engines")]
    MissingBehaviorEngines,
}
