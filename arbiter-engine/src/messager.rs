#![warn(missing_docs)]

//! The messager module contains the core messager layer for the Arbiter Engine.

use artemis_core::types::{Collector, CollectorStream, Executor};
use tokio::sync::broadcast::Sender;

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
pub struct Messager {
    broadcaster: Sender<Message>,
}

impl Messager {
    /// Creates a new messager with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            broadcaster: Sender::new(capacity),
        }
    }
}

impl Default for Messager {
    fn default() -> Self {
        Self::new(32)
    }
}

#[async_trait::async_trait]
impl Collector<Message> for Messager {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Message>> {
        let mut subscription = self.broadcaster.subscribe();
        let stream = async_stream::stream! {
            while let Ok(message) = subscription.recv().await {
                yield message;
            }
        };
        Ok(Box::pin(stream))
    }
}

#[async_trait::async_trait]
impl Executor<Message> for Messager {
    async fn execute(&self, message: Message) -> Result<()> {
        let _buf_len = self.broadcaster.send(message)?;
        Ok(())
    }
}
