// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TODO: Notes ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// When we start running an agent, we should have their messager start producing
// events that can be used by any and all behaviors the agent has that takes in
// messages as an event. Similarly, we should have agents start up any streams
// listeners that they need so those can also produce events. Those can then be
// piped into the behaviors that need them. Can perhaps make behaviors come from
// very specific events (e.g., specific contract events). This means each
// behavior should be a consumer and perhaps the agent itself is the producer
// (or at least relayer).
// This means we should give agents some way to "start streams" that they can
// then use to produce events.
// Can the agent engines basically ask the central hub of events to "if let"
// into whatever `E` they define and if it doesn't match, then it passes on.
// Also, all of this should be abstracted so that the user really only ever does
// something like "Start" the simulation and the agents state machine can
// progress through the stages of startup, running, and stopped. This way they
// get their streams and do all this under the hood. The streams should be
// placed inside of a `tokio::task` when the agents want to run them so they can
// be watched (i.e., `stream.next().await`) concurrently. In the startup step
// for the engine, it might be there that we have the behavior tell the event
// streamer what events it wants to listen to since this is where we can have
// contracts be added (they can be deployed in sync_state).
// Behaviors definitely need to be able to reference the agent's client and
// messager so that they can send messages and interact with the blockchain.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! The agent module contains the core agent abstraction for the Arbiter Engine.

use std::{fmt::Debug, pin::Pin, sync::Arc};

use arbiter_core::{data_collection::EventLogger, middleware::RevmMiddleware};
use ethers::contract::{EthLogDecode, Event};
use futures::stream::{Stream, StreamExt};
use futures_util::{
    future::{join_all, JoinAll},
    Future,
};
use serde::de::DeserializeOwned;
use tokio::task::JoinHandle;

use super::*;
use crate::{
    machine::{Behavior, Engine, State, StateMachine},
    messager::Messager,
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
    pub state: State,

    /// The messager the agent uses to send and receive messages from other
    /// agents.
    pub messager: Messager,

    pub client: Arc<RevmMiddleware>,

    pub event_streamer: Option<EventLogger>,

    pub behavior_engines: Option<Vec<Box<dyn StateMachine>>>,

    pub behavior_tasks: Option<JoinAll<JoinHandle<Box<dyn StateMachine>>>>,

    pub distributor: (
        async_broadcast::Sender<String>,
        async_broadcast::Receiver<String>,
    ),
}

impl Agent {
    /// Produces a new agent with the given identifier.
    pub(crate) fn connect(id: &str, world: &World) -> Self {
        let messager = world.messager.for_agent(id);
        let client = RevmMiddleware::new(&world.environment, Some(id)).unwrap();
        let distributor = async_broadcast::broadcast(512);
        Self {
            id: id.to_owned(),
            state: State::Uninitialized,
            messager,
            client,
            event_streamer: Some(EventLogger::builder()),
            behavior_engines: None,
            distributor,
            behavior_tasks: None,
        }
    }

    pub fn add_event<D: EthLogDecode + Debug + Serialize + 'static>(
        &mut self,
        event: Event<Arc<RevmMiddleware>, RevmMiddleware, D>,
    ) {
        self.event_streamer = Some(self.event_streamer.take().unwrap().add_stream(event));
    }

    pub(crate) fn start_event_stream(&mut self) -> Pin<Box<dyn Stream<Item = String> + Send + '_>> {
        let event_stream = self.event_streamer.take().unwrap().stream();
        let message_stream = self
            .messager
            .stream()
            .map(|msg| serde_json::to_string(&msg).unwrap_or_else(|e| e.to_string()));

        Box::pin(futures::stream::select(event_stream, message_stream))
    }

    pub fn add_behavior<E: DeserializeOwned + Send + Sync + Debug + 'static>(
        &mut self,
        behavior: impl Behavior<E> + 'static,
    ) {
        let event_receiver = self.distributor.0.new_receiver();

        let engine = Engine::new(behavior, event_receiver);
        if let Some(engines) = &mut self.behavior_engines {
            engines.push(Box::new(engine));
        } else {
            self.behavior_engines = Some(vec![Box::new(engine)]);
        }
    }
}

// TODO: Now at this piont, `behavior_engine` should have a task
// inside of it and we can collect all of the tasks for all of
// the behaviors. These tasks should be all running
// concurrently, so we need to await all of them simultaneously
// using the `transition()` method

#[async_trait::async_trait]
impl StateMachine for Agent {
    fn run_state(&mut self, state: State) {
        match state {
            State::Uninitialized => {
                unimplemented!("This never gets called.")
            }
            State::Syncing => {
                self.state = state;
                trace!("Agent is syncing.");
                let mut behavior_engines = self.behavior_engines.take().unwrap();
                for engine in behavior_engines.iter_mut() {
                    engine.run_state(state);
                }
                self.behavior_tasks =
                    Some(join_all(behavior_engines.into_iter().map(|mut engine| {
                        tokio::spawn(async move {
                            engine.transition().await;
                            engine
                        })
                    })));
            }
            State::Startup => {
                trace!("Agent is starting up.");
                self.state = state;
                let mut behavior_engines = self.behavior_engines.take().unwrap();
                for engine in behavior_engines.iter_mut() {
                    engine.run_state(state);
                }
                self.behavior_tasks =
                    Some(join_all(behavior_engines.into_iter().map(|mut engine| {
                        tokio::spawn(async move {
                            engine.transition().await;
                            engine
                        })
                    })));
            }
            State::Processing => {
                trace!("Agent is processing.");
                self.state = state;
                let mut behavior_engines = self.behavior_engines.take().unwrap();
                for engine in behavior_engines.iter_mut() {
                    engine.run_state(state);
                }
                self.behavior_tasks =
                    Some(join_all(behavior_engines.into_iter().map(|mut engine| {
                        tokio::spawn(async move {
                            engine.transition().await;
                            engine
                        })
                    })));
            }
            State::Stopped => {
                todo!()
            }
        }
    }

    async fn transition(&mut self) {
        // TODO: Idea, go through all the behaviors and see if they are done. Processing
        // may not explicitly finish, but will need awaited nonetheless
        self.behavior_engines = Some(
            self.behavior_tasks
                .take()
                .unwrap()
                .await
                .into_iter()
                .map(|res| res.unwrap())
                .collect::<Vec<_>>(),
        );
    }
}

#[cfg(test)]
mod tests {
    use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
    use ethers::types::U256;

    use super::*;
    use crate::messager::Message;
    #[ignore]
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn streaming() {
        std::env::set_var("RUST_LOG", "trace");
        tracing_subscriber::fmt::init();

        let mut world = World::new("world");
        let messager = world.messager.clone();
        println!(
            "Receiver count: {:?}",
            messager.broadcast_sender.receiver_count()
        );

        let agent = world.create_agent("agent");

        let arb = ArbiterToken::deploy(
            agent.client.clone(),
            ("ArbiterToken".to_string(), "ARB".to_string(), 18u8),
        )
        .unwrap()
        .send()
        .await
        .unwrap();

        agent.add_event(arb.events());
        let address = agent.client.address();
        let mut streamer = agent.start_event_stream();

        for _ in 0..5 {
            messager
                .send(Message {
                    from: "me".to_string(),
                    to: messager::To::All,
                    data: "hello".to_string(),
                })
                .await;
            arb.approve(address, U256::from(1))
                .send()
                .await
                .unwrap()
                .await
                .unwrap();
        }

        while let Some(msg) = streamer.next().await {
            println!("Printing message in test: {:?}", msg);
        }
    }
}
