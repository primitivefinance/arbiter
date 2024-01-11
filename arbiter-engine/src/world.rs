// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TODO: Notes ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// * Probably should move labels to world instead of on the environment.
// * One thing that is different about the Arbiter world is that give a bunch of
//   different channels to communicate with the Environment's tx thread. This is
//   different from a connection to a blockchain where you typically will just
//   have a single HTTP/WS connection. What we want is some kind of way of
//   having the world own a reference to a provider or something
// * Can add a messager as an interconnect and have the manager give each world
//   it owns a clone of the same messager.
// * The worlds now are just going to be revm worlds. We can generalize this
//   later.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! The world module contains the core world abstraction for the Arbiter Engine.

use arbiter_core::environment::{builder::EnvironmentBuilder, Environment};
use futures_util::future::{join_all, JoinAll};
use tokio::task::JoinHandle;

use super::*;
use crate::{
    agent::Agent,
    machine::{State, StateMachine},
    messager::Messager,
};

/// A world is a collection of agents that use the same type of provider, e.g.,
/// operate on the same blockchain or same `Environment`. The world is
/// responsible for managing the agents and their state transitions.
///
/// # How it works
/// The [`World`] works by implementing the [`StateMachine`] trait. When the
/// [`World`] is asked to enter into a new state, it will ask each [`Agent`] it
/// owns to run that state transition by calling [`StateMachine::run_state`].
/// All of the [`Agent`]s at once will then be able to be asked to block and
/// wait to finish their state transition by calling
/// [`StateMachine::transition`]. Ultimately, the [`World`] will transition
/// through the following states:
/// 1. [`State::Uninitialized`]: The [`World`] has been created, but has not
///   been started.
/// 2. [`State::Syncing`]: The [`World`] is syncing with the agents. This is
///  where the [`World`] can be brought up to date with the latest state of the
/// agents. This could be used if the world was stopped and later restarted.
/// 3. [`State::Startup`]: The [`World`] is starting up. This is where the
/// [`World`] can be initialized and setup.
/// 4. [`State::Processing`]: The [`World`] is processing. This is where the
/// [`World`] can process events and produce actions. The [`State::Processing`]
/// stage may run for a long time before all [`World`]s are finished processing.
/// This is the main stage of the [`World`] that predominantly runs automation.
/// 5. [`State::Stopped`]: The [`World`] is stopped. This is where the [`World`]
/// can be stopped and state of the [`World`] and its [`Agent`]s can be
/// offloaded and saved.
pub struct World {
    /// The identifier of the world.
    pub id: String,

    /// The agents in the world.
    pub agents: Option<HashMap<String, Agent>>,

    /// The environment for the world.
    pub environment: Environment,

    /// The messaging layer for the world.
    pub messager: Messager,

    /// Holds onto the tasks that represent the agent running a specific state
    /// transition.
    agent_tasks: Option<JoinAll<JoinHandle<Agent>>>,
}

impl World {
    /// Creates a new world with the given identifier and provider.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            agents: Some(HashMap::new()),
            agent_tasks: None,
            environment: EnvironmentBuilder::new().build(),
            messager: Messager::new(),
        }
    }

    /// Adds an agent to the world.
    pub fn create_agent(&mut self, id: &str) -> &mut Agent {
        let agent = Agent::connect(id, self);
        let agents = self.agents.as_mut().unwrap();
        agents.insert(id.to_owned(), agent);
        agents.get_mut(id).unwrap()
    }
}

#[async_trait::async_trait]
impl StateMachine for World {
    fn run_state(&mut self, state: State) {
        match state {
            State::Uninitialized => {
                unimplemented!("This never gets called.")
            }
            State::Syncing => {
                trace!("World is syncing.");
                let mut agents = self.agents.take().unwrap();
                for agent in agents.values_mut() {
                    agent.run_state(state);
                }
                self.agent_tasks = Some(join_all(agents.into_values().map(|mut agent| {
                    tokio::spawn(async move {
                        agent.transition().await;
                        agent
                    })
                })));
            }
            State::Startup => {
                trace!("World is starting up.");
                let mut agents = self.agents.take().unwrap();
                for agent in agents.values_mut() {
                    agent.run_state(state);
                }
                self.agent_tasks = Some(join_all(agents.into_values().map(|mut agent| {
                    tokio::spawn(async move {
                        agent.transition().await;
                        agent
                    })
                })));
            }
            State::Processing => {
                trace!("World is starting up.");
                let mut agents = self.agents.take().unwrap();
                for agent in agents.values_mut() {
                    agent.run_state(state);
                }
                self.agent_tasks = Some(join_all(agents.into_values().map(|mut agent| {
                    tokio::spawn(async move {
                        agent.transition().await;
                        agent
                    })
                })));
            }
            State::Stopped => {
                trace!("World is starting up.");
                let mut agents = self.agents.take().unwrap();
                for agent in agents.values_mut() {
                    agent.run_state(state);
                }
                self.agent_tasks = Some(join_all(agents.into_values().map(|mut agent| {
                    tokio::spawn(async move {
                        agent.transition().await;
                        agent
                    })
                })));
            }
        }
    }

    async fn transition(&mut self) {
        self.agents = Some(
            self.agent_tasks
                .take()
                .unwrap()
                .await
                .into_iter()
                .map(|res| {
                    let agent = res.unwrap();
                    (agent.id.clone(), agent)
                })
                .collect::<HashMap<String, Agent>>(),
        );
    }
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, sync::Arc};

    use arbiter_bindings::bindings::weth::WETH;
    use ethers::{
        providers::{Middleware, Provider, Ws},
        types::Address,
    };
    use futures_util::StreamExt;

    #[ignore = "This is unnecessary to run on CI currently."]
    #[tokio::test]
    async fn mainnet_ws() {
        let ws_url = std::env::var("MAINNET_WS_URL").expect("MAINNET_WS_URL must be set");
        let ws = Ws::connect(ws_url).await.unwrap();
        let provider = Provider::new(ws);
        let client = Arc::new(provider);
        let weth = WETH::new(
            Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap(),
            client.clone(),
        );
        let filter = weth.approval_filter().filter;
        let mut subscription = client.subscribe_logs(&filter).await.unwrap();
        println!("next: {:?}", subscription.next().await);
    }
}
