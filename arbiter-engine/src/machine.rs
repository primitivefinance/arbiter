//! The [`StateMachine`] trait, [`Behavior`] trait, and the [`Engine`] that runs
//! [`Behavior`]s.

// TODO: Notes
// I think we should have the `sync` stage of the behavior receive the client
// and messager and then the user can decide if it wants to use those in their
// behavior.

// Could typestate pattern help here at all? Sync could produce a `Synced` state
// behavior that can then not have options for client and messager. Then the
// user can decide if they want to use those in their behavior and get a bit
// simpler UX.

use std::{fmt::Debug, sync::Arc};

use arbiter_core::middleware::RevmMiddleware;
use serde::de::DeserializeOwned;
use tokio::sync::broadcast::Receiver;

use self::messager::Messager;
use super::*;

/// The instructions that can be sent to a [`StateMachine`].
#[derive(Clone, Debug)]
pub enum MachineInstruction {
    /// Used to make a [`StateMachine`] start up.
    Start(Option<Arc<RevmMiddleware>>, Option<Messager>),

    /// Used to make a [`StateMachine`] process events.
    /// This will offload the process into a task that can be halted by sending
    /// a [`MachineHalt`] message from the [`Messager`]. For our purposes, the
    /// [`crate::world::World`] will handle this.
    Process,
}

/// The message that can be used in a [`StateMachine`] to halt its processing.
/// Optionally returned by [`Behavior::process`] to close tasks.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MachineHalt;

/// The state used by any entity implementing [`StateMachine`].
#[derive(Clone, Copy, Debug)]
pub enum State {
    /// The entity is not yet running any process.
    /// This is the state adopted by the entity when it is first created.
    Uninitialized,

    /// The entity is starting up.
    /// This is where the entity can engage in its specific start up activities
    /// that it can do given the current state of the world.
    /// These are usually quick one-shot activities that are not repeated.
    Starting,

    /// The entity is processing.
    /// This is where the entity can engage in its specific processing
    /// of events that can lead to actions being taken.
    Processing,
}

// NOTE: `async_trait::async_trait` is used throughout to make the trait object
// safe even though rust >=1.75 has async trait stabilized

/// The [`Behavior`] trait is the lowest level functionality that will be used
/// by a [`StateMachine`]. This constitutes what each state transition will do.
#[async_trait::async_trait]
pub trait Behavior<E>: Send + Sync + 'static {
    /// Used to start the agent.
    /// This is where the agent can engage in its specific start up activities
    /// that it can do given the current state of the world.
    async fn startup(&mut self, client: Arc<RevmMiddleware>, messager: Messager) {}

    /// Used to process events.
    /// This is where the agent can engage in its specific processing
    /// of events that can lead to actions being taken.
    async fn process(&mut self, event: E) -> Option<MachineHalt>;
}

#[async_trait::async_trait]
pub(crate) trait StateMachine: Send + Sync + 'static {
    async fn execute(&mut self, instruction: MachineInstruction);
}

/// The idea of the [`Engine`] is that it drives the [`Behavior`] of a
/// [`StateMachine`]-based entity (like an [`agent::Agent`]).
/// The [`Engine`] specifically wraps a [`Behavior`] and a [`Receiver`] of
/// events into a cohesive unit that can listen to events and pass them onto the
/// processor stage. Since the [`Engine`] is also a [`StateMachine`], its
/// generics can be collapsed into a `dyn` trait object so that, for example,
/// [`agent::Agent`]s can own multiple [`Behavior`]s with different event `<E>`
/// types.
pub struct Engine<B, E>
where
    B: Behavior<E>,
{
    /// The behavior the [`Engine`] runs.
    pub behavior: Option<B>,

    /// The current state of the [`Engine`].
    pub state: State,

    /// The receiver of events that the [`Engine`] will process.
    /// The [`State::Processing`] stage will attempt a decode of the [`String`]s
    /// into the event type `<E>`.
    event_receiver: Option<Receiver<String>>,

    phantom: std::marker::PhantomData<E>,
}

impl<B, E> Engine<B, E>
where
    B: Behavior<E>,
    E: DeserializeOwned + Send + Sync + 'static,
{
    /// Creates a new [`Engine`] with the given [`Behavior`] and [`Receiver`].
    pub(crate) fn new(behavior: B, event_receiver: Receiver<String>) -> Self {
        Self {
            behavior: Some(behavior),
            state: State::Uninitialized,
            event_receiver: Some(event_receiver),
            phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<B, E> StateMachine for Engine<B, E>
where
    B: Behavior<E>,
    E: DeserializeOwned + Send + Sync + Debug + 'static,
{
    async fn execute(&mut self, instruction: MachineInstruction) {
        match instruction {
            MachineInstruction::Start(client, messager) => {
                let mut behavior = self.behavior.take().unwrap();
                trace!("Behavior is starting up.");
                self.state = State::Starting;
                let mut behavior = self.behavior.take().unwrap();
                let behavior_task = tokio::spawn(async move {
                    behavior.startup(client.unwrap(), messager.unwrap()).await;
                    behavior
                });
                self.behavior = Some(behavior_task.await.unwrap());
            }
            MachineInstruction::Process => {
                trace!("Behavior is processing.");
                let mut behavior = self.behavior.take().unwrap();
                let mut receiver = self.event_receiver.take().unwrap();
                let behavior_task = tokio::spawn(async move {
                    while let Ok(event) = receiver.recv().await {
                        let decoding_result = serde_json::from_str::<E>(&event);
                        match decoding_result {
                            Ok(event) => {
                                let halt_option = behavior.process(event).await;
                                if halt_option.is_some() {
                                    break;
                                }
                            }
                            Err(_) => match serde_json::from_str::<MachineHalt>(&event) {
                                Ok(_) => {
                                    warn!("Behavior received `MachineHalt` message. Breaking!");
                                    break;
                                }
                                Err(_) => {
                                    trace!(
                                        "Event received by behavior that could not be deserialized."
                                    );
                                    continue;
                                }
                            },
                        }
                    }
                    behavior
                });
                self.behavior = Some(behavior_task.await.unwrap());
            }
        }
    }
}
