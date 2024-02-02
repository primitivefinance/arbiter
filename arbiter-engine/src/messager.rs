//! The messager module contains the core messager layer for the Arbiter Engine.

// TODO: Allow for modulating the capacity of the messager.
// TODO: It might be nice to have some kind of messaging header so that we can
// pipe messages to agents and pipe messages across worlds.

use futures_util::Stream;
use tokio::sync::broadcast::{channel, Receiver, Sender};

use super::*;

/// A message that can be sent between agents.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    /// The sender of the message.
    pub from: String,

    /// The recipient of the message.
    pub to: To,

    /// The data of the message.
    /// This can be a struct serialized into JSON.
    pub data: String,
}

/// The recipient of the message.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum To {
    /// Send the message to all agents who are listening for broadcasts.
    All,

    /// Send the message to a specific agent.
    Agent(String),
}

/// A messager that can be used to send messages between agents.
#[derive(Debug)]
pub struct Messager {
    /// The identifier of the entity that is using the messager.
    pub id: Option<String>,

    pub(crate) broadcast_sender: Sender<Message>,

    broadcast_receiver: Option<Receiver<Message>>,
}

impl Clone for Messager {
    fn clone(&self) -> Self {
        Self {
            broadcast_sender: self.broadcast_sender.clone(),
            broadcast_receiver: Some(self.broadcast_sender.subscribe()),
            id: self.id.clone(),
        }
    }
}

impl Messager {
    /// Creates a new messager with the given capacity.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (broadcast_sender, broadcast_receiver) = channel(512);
        Self {
            broadcast_sender,
            broadcast_receiver: Some(broadcast_receiver),
            id: None,
        }
    }

    /// Returns a [`Messager`] interface connected to the same instance but with
    /// the `id` provided.
    pub(crate) fn for_agent(&self, id: &str) -> Self {
        Self {
            broadcast_sender: self.broadcast_sender.clone(),
            broadcast_receiver: Some(self.broadcast_sender.subscribe()),
            id: Some(id.to_owned()),
        }
    }

    /// utility function for getting the next value from the broadcast_receiver
    /// without streaming
    pub async fn get_next(&mut self) -> Message {
        while let Ok(message) = self.broadcast_receiver.as_mut().unwrap().recv().await {
            match &message.to {
                To::All => {
                    return message;
                }
                To::Agent(id) => {
                    if let Some(self_id) = &self.id {
                        if id == self_id {
                            return message;
                        }
                    }
                    continue;
                }
            }
        }
        unreachable!()
    }

    /// Returns a stream of messages that are either sent to [`To::All`] or to
    /// the agent via [`To::Agent(id)`].
    pub fn stream(mut self) -> impl Stream<Item = Message> + Send {
        let mut receiver = self.broadcast_receiver.take().unwrap();
        async_stream::stream! {
            while let Ok(message) = receiver.recv().await {
                match &message.to {
                    To::All => {
                        yield message;
                    }
                    To::Agent(id) => {
                        if let Some(self_id) = &self.id {
                            if id == self_id {
                                yield message;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Sends a message to the messager.
    pub async fn send(&self, message: Message) {
        trace!("Sending message via messager.");
        self.broadcast_sender.send(message).unwrap();
    }
}
