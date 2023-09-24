use std::fmt::Debug;

use ethers::{
    contract::{builders::Event as EventBuilder, EthLogDecode},
    providers::{Middleware, StreamExt},
};

use futures_channel::mpsc::{Sender, Receiver, channel};
use crate::middleware::RevmMiddlewareError;
use std::sync::Arc;

pub struct EventCapture<
    M: Middleware + std::borrow::Borrow<D>,
    D: Middleware + Debug + Send + Sync,
    E: EthLogDecode + Debug + 'static,
> {
    events: EventBuilder<M, D, E>,
}

impl<
        M: Middleware + std::borrow::Borrow<D>,
        D: Middleware + Debug + Send + Sync,
        E: EthLogDecode + Debug + 'static,
    > EventCapture<M, D, E>
{
    pub fn new(events: EventBuilder<M, D, E>) -> Self {
        Self { events }
    }

    pub async fn run(self) -> Result<(), RevmMiddlewareError> {
        let events = self.events;
        println!("Events: {:?}", events);
        let mut stream = events.stream().await.unwrap();
        while let Some(log) = stream.next().await {
            println!("Log: {:?}", log);
        }

        Ok(())
    }
}
