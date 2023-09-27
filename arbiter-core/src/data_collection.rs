use std::fmt::Debug;

use ethers::{
    contract::{builders::Event as EventBuilder, EthLogDecode},
    providers::{Middleware, StreamExt as ProviderStreamExt},
};
use futures_util::FutureExt;

use crate::middleware::RevmMiddlewareError;
use tokio::sync::oneshot::{Receiver, Sender};
use stream_cancel::{StreamExt, Tripwire};
use tokio::task::JoinHandle;
use tracing::info;

pub struct EventCapture<
    M: Middleware + std::borrow::Borrow<D> + 'static,
    D: Middleware + Debug + Send + Sync + 'static,
    E: EthLogDecode + Debug + 'static,
> {
    events: EventBuilder<M, D, E>,
    channel: (Sender<()>, Receiver<()>),
}

impl<
        M: Middleware + std::borrow::Borrow<D>,
        D: Middleware + Debug + Send + Sync,
        E: EthLogDecode + Debug + 'static,
    > EventCapture<M, D, E>
{
    pub fn new(events: EventBuilder<M, D, E>) -> Self {
        Self { events, channel: tokio::sync::oneshot::channel() }
    }

    pub async fn run(
        self,
    ) -> Result<(std::thread::JoinHandle<()>, Sender<()>), RevmMiddlewareError> {
        let events = self.events;
        let (sender, mut receiver) = self.channel;
        let handle = std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let mut stream = events.stream().await.unwrap();
                loop {
                    info!("looping");
                    tokio::select! {
                        biased;
                        Some(log) = stream.next() => {
                            info!("log: {:?}", log);
                        }
                        _ = &mut receiver => {
                            info!("received message");
                            let remainder = stream.next().now_or_never();
                            info!("remainder: {:?}", remainder);
                            break;
                        }
                    }
                }
                info!("exiting");
                let remainder = stream.next().now_or_never();
                info!("remainder: {:?}", remainder);
            });
        });
        Ok((handle, sender))
    }
}
