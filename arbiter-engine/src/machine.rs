//! The [`StateMachine`] trait, [`Behavior`] trait, and the [`Engine`] that runs
//! [`Behavior`]s.

use std::pin::Pin;

use futures_util::{Stream, StreamExt};
use tokio::task::JoinHandle;

use self::{errors::ArbiterEngineError, messager::Messager};
use super::*;

/// A type alias for a pinned, boxed stream of events.
///
/// This stream is capable of handling items of any type that implements the
/// `Stream` trait, and it is both sendable across threads and synchronizable
/// between threads.
///
/// # Type Parameters
///
/// * `E`: The type of the items in the stream.
pub type EventStream<E> = Pin<Box<dyn Stream<Item = E> + Send + Sync>>;

/// The instructions that can be sent to a [`StateMachine`].
#[derive(Clone, Debug)]
pub enum MachineInstruction {
    /// Used to make a [`StateMachine`] start up.
    Start(Arc<RevmMiddleware>, Messager),

    /// Used to make a [`StateMachine`] process events.
    /// This will offload the process into a task that can be halted by sending
    /// a [`ControlFlow::Halt`] message from the [`Messager`]. For our purposes,
    /// the [`crate::world::World`] will handle this.
    Process,
}

/// The message that is used in a [`StateMachine`] to continue or halt its
/// processing.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ControlFlow {
    /// Used to halt the processing of a [`StateMachine`].
    Halt,

    /// Used to continue on the processing of a [`StateMachine`].
    Continue,
}

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
pub trait Behavior<E>: Serialize + DeserializeOwned + Send + Sync + Debug + 'static {
    /// Used to start the agent.
    /// This is where the agent can engage in its specific start up activities
    /// that it can do given the current state of the world.
    async fn startup(
        &mut self,
        client: Arc<RevmMiddleware>,
        messager: Messager,
    ) -> Result<EventStream<E>, ArbiterEngineError>;

    /// Used to process events.
    /// This is where the agent can engage in its specific processing
    /// of events that can lead to actions being taken.
    async fn process(&mut self, event: E) -> Result<ControlFlow, ArbiterEngineError>;
}
/// A trait for creating a state machine.
///
/// This trait is intended to be implemented by types that can be converted into
/// a state machine. A state machine, in this context, is an entity capable of
/// executing a set of instructions or operations based on its current state and
/// inputs it receives.
///
/// Implementers of this trait should provide the logic to initialize and return
/// a new instance of a state machine, encapsulated within a `Box<dyn
/// StateMachine>`. This allows for dynamic dispatch to the state machine's
/// methods, enabling polymorphism where different types of state machines can
/// be used interchangeably at runtime.
///
/// # Returns
///
/// - `Box<dyn StateMachine>`: A boxed state machine object that can be
///   dynamically dispatched.
pub trait CreateStateMachine {
    /// Creates and returns a new state machine instance.
    ///
    /// This method consumes the implementer and returns a new instance of a
    /// state machine encapsulated within a `Box<dyn StateMachine>`. The
    /// specific type of the state machine returned can vary, allowing for
    /// flexibility and reuse of the state machine logic across
    /// different contexts.
    fn create_state_machine(self) -> Box<dyn StateMachine>;
}
#[async_trait::async_trait]
/// A trait defining the capabilities of a state machine within the system.
///
/// This trait is designed to be implemented by entities that can execute
/// instructions based on their current state and inputs they receive. The
/// execution of these instructions is asynchronous, allowing for non-blocking
/// operations within the state machine's logic.
///
/// Implementers of this trait must be able to be sent across threads and shared
/// among threads safely, hence the `Send`, `Sync`, and `'static` bounds. They
/// should also support debugging through the `Debug` trait.
pub trait StateMachine: Send + Sync + Debug + 'static {
    /// Executes a given instruction asynchronously.
    ///
    /// This method takes a mutable reference to self, allowing the state
    /// machine to modify its state in response to the instruction. The
    /// instruction to be executed is passed as an argument, encapsulating the
    /// action to be performed by the state machine.
    ///
    /// # Parameters
    ///
    /// - `instruction`: The instruction that the state machine is to execute.
    ///
    /// # Returns
    ///
    /// This method does not return a value, but it may result in state changes
    /// within the implementing type or the generation of further instructions
    /// or events.
    async fn execute(&mut self, instruction: MachineInstruction) -> Result<(), ArbiterEngineError>;
}

/// The `Engine` struct represents the core logic unit of a state machine-based
/// entity, such as an agent. It encapsulates a behavior and manages the flow
/// of events to and from this behavior, effectively driving the entity's
/// response to external stimuli.
///
/// The `Engine` is generic over a behavior type `B` and an event type `E`,
/// allowing it to be used with a wide variety of behaviors and event sources.
/// It is itself a state machine, capable of executing instructions that
/// manipulate its behavior or react to events.
///
/// # Fields
///
/// - `behavior`: An optional behavior that the engine is currently managing.
///   This is where the engine's logic is primarily executed in response to
///   events.
pub struct Engine<B, E>
where
    B: Behavior<E>,
{
    /// The behavior the `Engine` runs.
    pub behavior: Option<B>,

    /// The current state of the [`Engine`].
    pub state: State,

    /// The receiver of events that the [`Engine`] will process.
    /// The [`State::Processing`] stage will attempt a decode of the [`String`]s
    /// into the event type `<E>`.
    event_stream: Option<EventStream<E>>,

    phantom: std::marker::PhantomData<E>,
}

impl<B, E> Debug for Engine<B, E>
where
    B: Behavior<E>,
    E: DeserializeOwned + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine")
            .field("behavior", &self.behavior)
            .field("state", &self.state)
            .finish()
    }
}

impl<B, E> Engine<B, E>
where
    B: Behavior<E> + Debug,
    E: DeserializeOwned + Send + Sync + 'static,
{
    /// Creates a new [`Engine`] with the given [`Behavior`] and [`Receiver`].
    pub(crate) fn new(behavior: B) -> Self {
        Self {
            behavior: Some(behavior),
            state: State::Uninitialized,
            event_stream: None,
            phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<B, E> StateMachine for Engine<B, E>
where
    B: Behavior<E> + Debug + Serialize + DeserializeOwned,
    E: DeserializeOwned + Serialize + Send + Sync + Debug + 'static,
{
    async fn execute(&mut self, instruction: MachineInstruction) -> Result<(), ArbiterEngineError> {
        match instruction {
            MachineInstruction::Start(client, messager) => {
                self.state = State::Starting;
                let mut behavior = self.behavior.take().unwrap();
                let behavior_task: JoinHandle<Result<(EventStream<E>, B), ArbiterEngineError>> =
                    tokio::spawn(async move {
                        let id = messager.id.clone();
                        let stream = behavior.startup(client, messager).await?;
                        debug!("startup complete for {:?}!", id);
                        Ok((stream, behavior))
                    });
                let (stream, behavior) = behavior_task.await??;
                self.event_stream = Some(stream);
                self.behavior = Some(behavior);
                self.execute(MachineInstruction::Process).await;
                Ok(())
            }
            MachineInstruction::Process => {
                let mut behavior = self.behavior.take().unwrap();
                let mut stream = self.event_stream.take().unwrap();
                let behavior_task: JoinHandle<Result<B, ArbiterEngineError>> =
                    tokio::spawn(async move {
                        while let Some(event) = stream.next().await {
                            match behavior.process(event).await? {
                                ControlFlow::Halt => {
                                    break;
                                }
                                ControlFlow::Continue => {}
                            }
                        }
                        Ok(behavior)
                    });
                // TODO: We don't have to store the behavior again here, we could just discard
                // it.
                self.behavior = Some(behavior_task.await??);
                Ok(())
            }
        }
    }
}
