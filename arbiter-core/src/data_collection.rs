use std::{fmt::Debug, sync::Arc};

use ethers::{
    contract::{builders::Event as EventBuilder, EthLogDecode},
    providers::{Middleware, StreamExt as ProviderStreamExt}, types::{Filter, U256},
    prelude::k256::sha2::{Digest, Sha256}
};
use futures_util::FutureExt;

use crate::middleware::{RevmMiddlewareError, RevmMiddleware};
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
    id: U256,
    client: Arc<RevmMiddleware>
}

impl<
        M: Middleware + std::borrow::Borrow<D>,
        D: Middleware + Debug + Send + Sync,
        E: EthLogDecode + Debug + 'static,
    > EventCapture<M, D, E>
{
    pub fn new(events: EventBuilder<M, D, E>, client: Arc<RevmMiddleware>) -> Self {
        let filter = events.filter.clone();
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(&filter).map_err(RevmMiddlewareError::Json).unwrap());
        let hash = hasher.finalize();
        let id = ethers::types::U256::from(ethers::types::H256::from_slice(&hash).as_bytes());
        Self { events, channel: tokio::sync::oneshot::channel(), id, client }
    }

    pub async fn run(
        self,
    ) -> Result<(Sender<()>, JoinHandle<()>), RevmMiddlewareError> {
        let events = self.events;
        let (sender, mut recv) = self.channel;
        let handle = tokio::spawn(async move {
            let client = self.client.clone();
            let connection = client.provider().as_ref();
            let mut stream = events.stream().await.unwrap();
            info!("here1");
            loop {
                info!("looping");
                tokio::select! {
                    Some(log) = stream.next() => {
                        info!("log: {:?}", log);
                    }
                    _ = &mut recv => {
                        info!("received message");
                        loop {
                            let receivers = connection.filter_receivers.lock().await;
                            if let Some (receiver) = receivers.get(&self.id) {
                                if receiver.receiver.len() == 0 {
                                    break;
                                } else {
                                    drop(receivers);
                                    if let Some(log) = stream.next().await {
                                        info!("log: {:?}", log);
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
            }
        });
        Ok((sender, handle))
    }
}
