use std::fmt::Debug;

use ethers::{
    contract::{builders::Event as EventBuilder, EthLogDecode},
    providers::{Middleware, StreamExt as ProviderStreamExt},
};

use crate::middleware::RevmMiddlewareError;
use futures_channel::mpsc::{channel, Receiver, Sender};
use stream_cancel::{StreamExt, Tripwire};
use tokio::task::JoinHandle;
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

    pub async fn run(
        self,
        tripwire: Tripwire,
    ) -> Result<std::thread::JoinHandle<()>, RevmMiddlewareError> {
        let events = self.events;
        let handle = std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let mut stream = events.stream().await.unwrap().take_until_if(tripwire);
                let mut i = 0;
                loop {
                    if let Some(log) = stream.next().await {
                        info!("log: {:?}", log);
                        info!("i: {}", i);
                        i += 1;
                    } else {
                        continue
                    }
                }
            });
        });
        Ok(handle)
    }
}
