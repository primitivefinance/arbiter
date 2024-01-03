#![warn(missing_docs)]

//! The messager module contains the core messager layer for the Arbiter Engine.

use artemis_core::types::{Collector, CollectorStream, Executor};
use async_broadcast::{broadcast, Receiver, Sender};

// use tokio::sync::broadcast::{channel, Receiver, Sender};
use super::*;

/// A message that can be sent between agents.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    /// The sender of the message.
    pub from: String,

    /// The recipient of the message.
    pub to: String,

    /// The data of the message.
    /// This can be a struct serialized into JSON.
    pub data: String,
}

/// A messager that can be used to send messages between agents.
#[derive(Debug)]
pub struct Messager {
    pub(crate) sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl Clone for Messager {
    fn clone(&self) -> Self {
        let sender = self.receiver.new_sender();
        let receiver = self.receiver.new_receiver();
        Self { sender, receiver }
    }
}

impl Messager {
    // TODO: Allow for modulating the capacity of the messager.
    /// Creates a new messager with the given capacity.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (sender, receiver) = broadcast(512);
        Self { sender, receiver }
    }
}

#[async_trait::async_trait]
impl Collector<Message> for Messager {
    #[tracing::instrument(skip(self), level = "debug", target = "messager")]
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Message>> {
        debug!("Getting the event stream for the messager.");
        let mut receiver = self.receiver.clone();
        let stream = async_stream::stream! {
            loop {
                let message = receiver.recv().await;
                match message {
                    Ok(message) => yield message,
                    Err(_) => break,
                }
            }
        };
        Ok(Box::pin(stream))
    }
}

#[async_trait::async_trait]
impl Executor<Message> for Messager {
    #[tracing::instrument(skip(self), level = "trace", target = "messager")]
    async fn execute(&self, message: Message) -> Result<()> {
        match self.sender.broadcast(message.clone()).await {
            Ok(_) => {
                trace!("The message was successfully broadcasted: {:?}", message);
                Ok(())
            }
            Err(e) => {
                trace!("An error occurred while broadcasting the message: {:?}", e);
                Err(e.into())
            }
        }
    }
}
