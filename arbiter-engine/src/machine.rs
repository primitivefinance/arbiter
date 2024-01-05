use std::{fmt::Debug, pin::Pin};

use async_broadcast::Receiver;
use futures_util::{Stream, StreamExt};
use serde::de::DeserializeOwned;
use tokio::task::JoinHandle;

use super::*;

#[derive(Debug, Copy, Clone)]
pub enum State {
    Uninitialized,
    Syncing,
    Startup,
    Processing,
    Stopped,
}

// This makes the trait object safe even though rust 1.75 has async trait
// stabilized
#[async_trait::async_trait]
pub trait Behavior<E>: Send + Sync + 'static {
    /// Used to bring the agent back up to date with the latest state of the
    /// world. This could be used if the world was stopped and later restarted.
    async fn sync(&mut self);

    /// Used to start the agent.
    async fn startup(&mut self);

    async fn process(&mut self, event: E);
}

#[async_trait::async_trait]
pub(crate) trait StateMachine: Send + Sync + 'static {
    fn run_state(&mut self, state: State);
    async fn transition(&mut self);
}

pub struct Engine<B, E>
where
    B: Behavior<E>,
{
    pub behavior: Option<B>,
    pub behavior_task: Option<JoinHandle<B>>,
    event_receiver: Receiver<String>,
    phantom: std::marker::PhantomData<E>,
}

impl<B, E> Engine<B, E>
where
    B: Behavior<E>,
    E: DeserializeOwned + Send + Sync + 'static,
{
    pub fn new(behavior: B, event_receiver: Receiver<String>) -> Self {
        Self {
            behavior: Some(behavior),
            behavior_task: None,
            event_receiver,
            phantom: std::marker::PhantomData,
        }
    }
}

// TODO: These states can all be wrapped in tokio::task::spawn for concurrency.
#[async_trait::async_trait]
impl<B, E> StateMachine for Engine<B, E>
where
    B: Behavior<E>,
    E: DeserializeOwned + Send + Sync + Debug + 'static,
{
    fn run_state(&mut self, state: State) {
        match state {
            State::Uninitialized => {
                unimplemented!("This never gets called.")
            }
            State::Syncing => {
                // TODO: This is basically doing Option ping-pong where we move behaviors into a
                // task, hold onto the task, then regain the behavior only upon completion of
                // the task.
                trace!("Behavior is syncing.");
                let mut behavior = self.behavior.take().unwrap();
                self.behavior_task = Some(tokio::spawn(async move {
                    behavior.sync().await;
                    behavior
                }));
            }
            State::Startup => {
                trace!("Behavior is starting up.");
                let mut behavior = self.behavior.take().unwrap();
                self.behavior_task = Some(tokio::spawn(async move {
                    behavior.startup().await;
                    behavior
                }));
            }
            State::Processing => {
                trace!("Behavior is processing.");
                let mut behavior = self.behavior.take().unwrap();
                let mut receiver = self.event_receiver.clone(); // TODO Could use Option::take() if we don't want to clone.
                self.behavior_task = Some(tokio::spawn(async move {
                    while let Ok(event) = receiver.recv().await {
                        trace!("Behavior has gotten event: {:?}", event);
                        let decoding_result = serde_json::from_str::<E>(&event);
                        match decoding_result {
                            Ok(event) => behavior.process(event).await,
                            Err(e) => {
                                tracing::error!("Error decoding event: {:?}", e);
                                continue;
                            }
                        }
                    }
                    behavior
                }));
            }
            State::Stopped => {
                todo!()
            }
        }
    }

    // Take the task and wait for it to finish. Then take the behavior and put it
    // back into the engine.
    async fn transition(&mut self) {
        let behavior_task = self.behavior_task.take().unwrap();
        let behavior = behavior_task.await.unwrap();
        self.behavior = Some(behavior);
    }
}
