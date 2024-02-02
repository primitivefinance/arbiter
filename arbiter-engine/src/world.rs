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
use crate::{agent::Agent, messager::Messager};

/// A world is a collection of agents that use the same type of provider, e.g.,
/// operate on the same blockchain or same `Environment`. The world is
/// responsible for managing the agents and their state transitions.
///
/// # How it works
/// The [`World`] holds on to a collection of [`Agent`]s and can run them all
/// concurrently when the [`run`] method is called. The [`World`] takes in
/// [`AgentBuilder`]s and when it does so, it creates [`Agent`]s that are now
/// connected to the world via a client ([`Arc<RevmMiddleware>`]) and a messager
/// ([`Messager`]).
#[derive(Debug)]
pub struct World {
    /// The identifier of the world.
    pub id: String,

    /// The agents in the world.
    pub agents: Option<HashMap<String, Agent>>,

    /// The environment for the world.
    pub environment: Environment,

    /// The messaging layer for the world.
    pub messager: Messager,
}

impl World {
    /// Creates a new [`World`] with the given identifier and provider.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
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

    /// Runs all of the [`Agent`]s and their [`crate::machine::Behavior`]s in
    /// the world in parallel.
    pub async fn run(&mut self) -> Result<()> {
        let mut tasks = vec![];
        let agents = self
            .agents
            .take()
            .ok_or_else(|| anyhow!("No agents found! Has the world already been run?"))?;
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
        Ok(())
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
