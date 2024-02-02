//! The agent module contains the core agent abstraction for the Arbiter Engine.

use std::{fmt::Debug, sync::Arc};

use arbiter_core::middleware::RevmMiddleware;
use serde::de::DeserializeOwned;

use crate::{
    machine::{Behavior, Engine, State, StateMachine},
    messager::Messager,
};
use thiserror::Error;

/// An agent is an entity capable of processing events and producing actions.
/// These are the core actors in simulations or in onchain systems.
/// Agents can be connected of other agents either as a dependent, or a
/// dependency.
///
/// # How it works
/// The [`Agent`] works by implementing the [`StateMachine`] trait. When the
/// [`World`] that owns the [`Agent`] is asked to enter into a new state, the
/// [`World`] will ask each [`Agent`] it owns to run that state transition by
/// calling [`StateMachine::run_state`]. All of the [`Agent`]s at once will then
/// will be able to be asked to block and wait to finish their state transition
/// by calling [`StateMachine::transition`]. Ultimately, the [`Agent`] will
/// transition through the following states:
/// 1. [`State::Uninitialized`]: The [`Agent`] has been created, but has not
///   been started.
/// 2. [`State::Syncing`]: The [`Agent`] is syncing with the world. This is
///  where the [`Agent`] can be brought up to date with the latest state of the
/// world. This could be used if the world was stopped and later restarted.
/// 3. [`State::Startup`]: The [`Agent`] is starting up. This is where the
/// [`Agent`] can be initialized and setup.
/// 4. [`State::Processing`]: The [`Agent`] is processing. This is where the
/// [`Agent`] can process events and produce actions. The [`State::Processing`]
/// stage may run for a long time before all [`Agent`]s are finished processing.
/// This is the main stage of the [`Agent`] that predominantly runs automation.
/// 5. [`State::Stopped`]: The [`Agent`] is stopped. This is where the [`Agent`]
/// can be stopped and state of the [`World`] and its [`Agent`]s can be
/// offloaded and saved.
pub struct Agent {
    /// Identifier for this agent.
    /// Used for routing messages.
    pub id: String,

    /// The status of the agent.
    pub state: State,

    /// The messager the agent uses to send and receive messages from other
    /// agents.
    pub messager: Messager,

    /// The client the agent uses to interact with the blockchain.
    pub client: Arc<RevmMiddleware>,

    /// The engines/behaviors that the agent uses to sync, startup, and process
    /// events.
    pub(crate) behavior_engines: Vec<Box<dyn StateMachine>>,
}

impl Agent {
    /// Produces a minimal agent builder with the given identifier.
    pub fn builder(id: &str) -> Result<AgentBuilder, AgentBuildError> {
        Ok(AgentBuilder {
            id: id.to_owned(),
            behavior_engines: None,
        })
    }
}

/// [`AgentBuilder`] represents the intermediate state of agent creation before it is converted into a full on [`Agent`]
pub struct AgentBuilder {
    /// Identifier for this agent.
    /// Used for routing messages.
    pub id: String,
    /// The engines/behaviors that the agent uses to sync, startup, and process
    /// events.
    behavior_engines: Option<Vec<Box<dyn StateMachine>>>,
}

impl AgentBuilder {
    /// Appends a behavior onto an [`AgentBuilder`]. Behaviors are initialized when the agent builder is added to the [`crate::world::World`]
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
                state: State::Uninitialized,
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
    /// Error representing the case where the agent is missing behavior engines; an agent has to have behaviors to be useful!
    #[error("Agent is missing behavior engines")]
    MissingBehaviorEngines,
}
