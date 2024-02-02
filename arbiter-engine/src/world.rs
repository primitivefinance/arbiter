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
// * Can we give the world an address book??
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! The world module contains the core world abstraction for the Arbiter Engine.

use std::collections::VecDeque;

use arbiter_core::{environment::Environment, middleware::RevmMiddleware};
use futures_util::future::join_all;
use tokio::spawn;

use self::{agent::AgentBuilder, machine::MachineInstruction};
use super::*;
use crate::{agent::Agent, machine::State, messager::Messager};

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

    /// The state of the [`World`].
    pub state: State,

    /// The agents in the world.
    pub agents: Option<HashMap<String, Agent>>,

    /// The environment for the world.
    pub environment: Environment,

    /// The messaging layer for the world.
    pub messager: Messager,
}

impl World {
    /// Creates a new [World] with the given identifier and provider.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            state: State::Uninitialized,
            agents: Some(HashMap::new()),
            environment: Environment::builder().build(),
            messager: Messager::new(),
        }
    }

    /// Adds an agent to the world.
    pub fn add_agent(&mut self, agent_builder: AgentBuilder) {
        let id = agent_builder.id.clone();
        let client = RevmMiddleware::new(&self.environment, Some(&id)).unwrap();
        let messager = self.messager.for_agent(&id);
        let agent = agent_builder.build(client, messager).unwrap();
        let agents = self.agents.as_mut().unwrap();
        agents.insert(id.to_owned(), agent);
    }

    /// Runs the world through up to the [`State::Processing`] stage.
    pub async fn run(&mut self) {
        let mut tasks = vec![];
        // TODO: This unwrap should be checked.
        let agents = self.agents.take().unwrap();
        let mut messagers = VecDeque::new();
        for (_, agent) in agents.iter() {
            for _ in &agent.behavior_engines {
                messagers.push_back(agent.messager.clone());
            }
        }
        for (_, mut agent) in agents {
            for mut engine in agent.behavior_engines.drain(..) {
                let client = agent.client.clone();
                let messager = messagers.pop_front().unwrap();
                tasks.push(spawn(async move {
                    engine
                        .execute(MachineInstruction::Start(client, messager))
                        .await
                }));
            }
        }
        join_all(tasks).await;
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
