#![warn(missing_docs)]
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TODO: Notes ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Is it possible to have the messager track the ID of agents connected to it? Perhaps there is some
// kind of reference counter with identifiers. Then we can offload the messager delivery service to
// the messager instead of broadcasting all messages. We'd have to do this as we call
// "get_event_stream" on the messager maybe.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// TODO: Need to create the pipeline for agent to get all of the streams they need to listen to from
// the behaviors and then have the events piped to the behaviors upon process stage

//! The messager module contains the core messager layer for the Arbiter Engine.

use std::pin::Pin;

use async_broadcast::{broadcast, Receiver as BroadcastReceiver, Sender as BroadcastSender};
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures_util::Stream;

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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum To {
    All,
    Agent(String),
}

/// A messager that can be used to send messages between agents.
#[derive(Clone, Debug)]
pub struct Messager {
    // TODO: Right now these are for broadcasting. We could add a way to send to a specific agent.
    // In fact, if we just have all agents join the messager, then we can send to all agents if the
    // `to` is `None`
    pub(crate) broadcast_sender: BroadcastSender<Message>,
    broadcast_receiver: BroadcastReceiver<Message>,
    pub id: Option<String>,
}

impl Messager {
    // TODO: Allow for modulating the capacity of the messager.
    // TODO: It might be nice to have some kind of messaging header so that we can
    // pipe messages to agents and pipe messages across worlds.
    /// Creates a new messager with the given capacity.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (broadcast_sender, broadcast_receiver) = broadcast(512);
        Self {
            broadcast_sender,
            broadcast_receiver,
            id: None,
        }
    }

    // TODO: Okay if we do something kinda like this, then agents don't even need to
    // filter the `to` field or set the `from` field. Let's give this a shot!
    pub(crate) fn for_agent(&self, id: &str) -> Self {
        Self {
            broadcast_sender: self.broadcast_sender.clone(),
            broadcast_receiver: self.broadcast_receiver.clone(),
            id: Some(id.to_owned()),
        }
    }

    pub fn stream(&self) -> Pin<Box<dyn Stream<Item = Message> + Send + '_>> {
        let mut receiver = self.broadcast_receiver.clone();
        let stream = async_stream::stream! {
            while let Ok(message) = receiver.recv().await {
                println!("Received message: {:?}", message);

                match &message.to {
                    To::All => {
                        println!("Yielding the broadcasted message");
                        yield message;
                    }
                    To::Agent(id) => {
                        if let Some(self_id) = &self.id {
                            if id == self_id {
                                println!("Yielding the message for the agent");
                                yield message;
                            }
                        }
                    }
                }
            }
        };
        Box::pin(stream)
    }

    pub async fn send(&self, message: Message) {
        trace!("Sending message");
        self.broadcast_sender.broadcast(message).await.unwrap();
    }
}
