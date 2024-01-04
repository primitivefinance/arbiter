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
// Can the agent engines basically ask the central hub of events to "if let" into whatever `E` they
// define and if it doesn't match, then it passes on.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! The agent module contains the core agent abstraction for the Arbiter Engine.

use std::{fmt::Debug, pin::Pin, sync::Arc};

use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
use arbiter_core::{data_collection::EventLogger, middleware::RevmMiddleware};
use ethers::{
    contract::{EthLogDecode, Event},
    types::U256,
};
use futures::stream::{self, Stream, StreamExt};
use futures_util::stream::{Map, Select};

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

    pub event_streamer: Option<EventLogger>,
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
            event_streamer: Some(EventLogger::builder()),
        }
    }

    pub fn add_event<D: EthLogDecode + Debug + Serialize + 'static>(
        &mut self,
        event: Event<Arc<RevmMiddleware>, RevmMiddleware, D>,
    ) {
        self.event_streamer = Some(self.event_streamer.take().unwrap().add_stream(event));
    }

    pub(crate) fn start_streams(&mut self) -> Pin<Box<dyn Stream<Item = String> + Send + '_>> {
        let event_stream = self.event_streamer.take().unwrap().stream();
        let message_stream = self
            .messager
            .stream()
            .map(|msg| serde_json::to_string(&msg).unwrap_or_else(|e| e.to_string()));

        Box::pin(futures::stream::select(event_stream, message_stream))
    }
}

pub enum AgentState {
    Uninitialized,
    Startup,
    Running,
    Stopped,
}

pub trait Engine<E, A> {
    /// Used to bring the agent back up to date with the latest state of the
    /// world. This could be used if the world was stopped and later restarted.
    async fn sync_state(&mut self);

    /// Used to start the agent.
    async fn startup(&mut self);

    async fn process(&mut self, event: E) -> Vec<A>;
}

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
    let mut streamer = agent.start_streams();
    // let mut message_streamer = agent.messager.stream();
    // let mut event_streamer = agent.event_streamer.take().unwrap().stream();
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
    // let thing = streamer.enumerate().collect::<Vec<_>>().await;
    // println!("Thing: {:?}", thing);
    while let Some(msg) = streamer.next().await {
        println!("Printing message in test: {:?}", msg);
    }
    // while let Some(event) = event_streamer.next().await {
    //     println!("Event: {:?}", event);
    // }
}
