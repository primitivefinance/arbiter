//! The [`StateMachine`] trait, [`Behavior`] trait, and the [`Engine`] that runs
//! [`Behavior`]s.

use std::fmt::Debug;

use serde::de::DeserializeOwned;
use tokio::sync::broadcast::Receiver;

use super::*;

/// The instructions that can be sent to a [`StateMachine`].
#[derive(Clone, Copy, Debug)]
pub enum MachineInstruction {
    /// Used to make a [`StateMachine`] sync with the world.
    Sync,

    /// Used to make a [`StateMachine`] start up.
    Start,

    /// Used to make a [`StateMachine`] process events.
    /// This will offload the process into a task that can be halted by sending
    /// a [`MachineHalt`] message from the [`Messager`]. For our purposes, the
    /// [`crate::world::World`] will handle this.
    Process,

    /// Used to make a [`StateMachine`] stop. Only applicable for the
    /// [`crate::world::World`] currently.
    Stop,
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

    /// The entity is syncing with the world.
    /// This can be used to bring the entity back up to date with the latest
    /// state of the world. This could be used if the world was stopped and
    /// later restarted.
    Syncing,

    /// The entity is starting up.
    /// This is where the entity can engage in its specific start up activities
    /// that it can do given the current state of the world.
    /// These are usually quick one-shot activities that are not repeated.
    Starting,

    /// The entity is processing.
    /// This is where the entity can engage in its specific processing
    /// of events that can lead to actions being taken.
    Processing,

    /// The entity is stopped.
    /// This is where state can be offloaded and saved if need be.
    Stopped,
}

// NOTE: `async_trait::async_trait` is used throughout to make the trait object
// safe even though rust >=1.75 has async trait stabilized

/// The [`Behavior`] trait is the lowest level functionality that will be used
/// by a [`StateMachine`]. This constitutes what each state transition will do.
#[async_trait::async_trait]
pub trait Behavior<E>: Send + Sync + 'static {
    /// Used to bring the agent back up to date with the latest state of the
    /// world. This could be used if the world was stopped and later restarted.
    async fn sync(&mut self) {}

    /// Used to start the agent.
    /// This is where the agent can engage in its specific start up activities
    /// that it can do given the current state of the world.
    async fn startup(&mut self) {}

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
            MachineInstruction::Sync => {
                trace!("Behavior is syncing.");
                self.state = State::Syncing;
                let mut behavior = self.behavior.take().unwrap();
                let behavior_task = tokio::spawn(async move {
                    behavior.sync().await;
                    behavior
                });
                self.behavior = Some(behavior_task.await.unwrap());
            }
            MachineInstruction::Start => {
                trace!("Behavior is starting up.");
                self.state = State::Starting;
                let mut behavior = self.behavior.take().unwrap();
                let behavior_task = tokio::spawn(async move {
                    behavior.startup().await;
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
            MachineInstruction::Stop => {
                unreachable!("This is never explicitly called on an engine.")
            }
        }
    }
}
