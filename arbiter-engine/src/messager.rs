#![warn(missing_docs)]
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TODO: Notes ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Is it possible to have the messager track the ID of agents connected to it? Perhaps there is some kind of reference counter with identifiers. Then we can offload the messager delivery service to the messager instead of broadcasting all messages. We'd have to do this as we call "get_event_stream" on the messager maybe.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! The messager module contains the core messager layer for the Arbiter Engine.

use artemis_core::types::{Collector, CollectorStream, Executor};
use async_broadcast::{broadcast, Receiver as BroadcastReceiver, Sender as BroadcastSender};
use crossbeam_channel::{unbounded, Receiver, Sender};

// use tokio::sync::broadcast::{channel, Receiver, Sender};
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum To {
    All,
    Agent(String),
}

/// A messager that can be used to send messages between agents.
#[derive(Clone, Debug)]
pub struct Messager {
    // TODO: Right now these are for broadcasting. We could add a way to send to a specific agent.
    // In fact, if we just have all agents join the messager, then we can send to all agents if the `to` is `None`.
    pub(crate) broadcast_sender: BroadcastSender<Message>,
    broadcast_receiver: BroadcastReceiver<Message>,
    agent_channels: HashMap<String, (Sender<Message>, Receiver<Message>)>,
}

impl Messager {
    // TODO: Allow for modulating the capacity of the messager.
    /// Creates a new messager with the given capacity.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (broadcast_sender, broadcast_receiver) = broadcast(512);
        Self {
            broadcast_sender,
            broadcast_receiver,
            agent_channels: HashMap::new(),
        }
    }

    pub fn join(&mut self, id: &str) {
        let agent_channel = unbounded();
        self.agent_channels.insert(id.to_owned(), agent_channel);
    }
}

#[async_trait::async_trait]
impl Collector<Message> for Messager {
    // TODO: In here we need to get the stream for the broadcast receiver and the SPECIFIC stream for the SPECIFIC agent given the ID and fuse the two streams.
    #[tracing::instrument(skip(self), level = "debug", target = "messager")]
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Message>> {
        debug!("Getting the event stream for the messager.");
        let mut receiver = self.broadcast_receiver.clone();
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

// TODO: This should pass on a message with jsut data to the relevant SPECIFIC agent or broadcast it to all. The agent should be able to handle the message and do what it wants with it and not have to do filtering.
#[async_trait::async_trait]
impl Executor<Message> for Messager {
    #[tracing::instrument(skip(self), level = "debug", target = "messager")]
    async fn execute(&self, message: Message) -> Result<()> {
        match self.broadcast_sender.broadcast(message.clone()).await {
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
