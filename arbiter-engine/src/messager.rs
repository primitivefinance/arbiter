#![warn(missing_docs)]

//! The messager module contains the core messager layer for the Arbiter Engine.

use artemis_core::types::{Collector, CollectorStream, Executor};
use flume::{unbounded, Receiver, Sender};

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
#[derive(Clone, Debug)]
pub struct Messager {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl Messager {
    /// Creates a new messager with the given capacity.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self { sender, receiver }
    }
}

#[async_trait::async_trait]
impl Collector<Message> for Messager {
    #[tracing::instrument(skip(self), level = "debug", target = "messager")]
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Message>> {
        debug!("Getting the event stream for the messager.");
        let stream = self.receiver.clone().into_stream();
        Ok(Box::pin(stream))
    }
}

#[async_trait::async_trait]
impl Executor<Message> for Messager {
    #[tracing::instrument(skip(self), level = "trace", target = "messager")]
    async fn execute(&self, message: Message) -> Result<()> {
        trace!("Broadcasting message.");
        self.sender.send(message)?;
        Ok(())
    }
}

