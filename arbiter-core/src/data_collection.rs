use std::fmt::Debug;

use ethers::{
    contract::{builders::Event as EventBuilder, EthLogDecode},
    providers::{Middleware, StreamExt as ProviderStreamExt},
};

use futures_channel::mpsc::{Sender, Receiver, channel};
use tokio::task::JoinHandle;
use crate::middleware::RevmMiddlewareError;
use stream_cancel::{StreamExt, Tripwire};
use tracing::info;

pub struct EventCapture<
    M: Middleware + std::borrow::Borrow<D> + 'static,
    D: Middleware + Debug + Send + Sync + 'static,
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

    pub async fn run(self, tripwire: Tripwire) -> Result<JoinHandle<()>, RevmMiddlewareError> {
        let events = self.events;

        let handle = tokio::spawn(async move {
            let mut stream = events.stream().await.unwrap().take_until_if(tripwire);
            loop {
                if let Some(log) = stream.next().await {
                        info!("got inside stream!");
                        println!("log: {:?}", log);
                } else {
                    continue;
                }
            }         
        });
        Ok(handle)
    }
}
