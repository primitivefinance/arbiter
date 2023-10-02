use std::{collections::BTreeMap, fmt::Debug};

use ethers::{
    contract::{builders::Event, EthLogDecode},
    providers::{Middleware, StreamExt as ProviderStreamExt},
};
use serde::Serialize;

use crate::middleware::errors::RevmMiddlewareError;
use serde_json::Value;
use tokio::{io::AsyncWriteExt, task::JoinHandle};
use tracing::info;

pub struct EventLogger<
    M: Middleware + std::borrow::Borrow<D> + 'static,
    D: Middleware + Debug + Send + Sync + 'static,
    E: EthLogDecode + Debug + Serialize + 'static,
> {
    events: Vec<Event<M, D, E>>,
}

impl<
        M: Middleware + std::borrow::Borrow<D>,
        D: Middleware + Debug + Send + Sync,
        E: EthLogDecode + Debug + Serialize + 'static,
    > EventLogger<M, D, E>
{
    pub fn builder() -> Self {
        Self { events: Vec::new() }
    }

    pub fn add(mut self, event: Event<M, D, E>) -> Self {
        self.events.push(event);
        self
    }

    pub async fn run(self) -> Result<JoinHandle<()>, RevmMiddlewareError> {
        let handle = tokio::spawn(async move {
            let mut set = tokio::task::JoinSet::new();
            for event in self.events {
                set.spawn(async move {
                    let mut stream = event.stream().await.unwrap();
                    while let Some(Ok(log)) = stream.next().await {
                        let serialized = serde_json::to_string(&log).unwrap();
                        let deserialized: BTreeMap<String, Value> =
                            serde_json::from_str(&serialized).unwrap();
                        let (key, value) = deserialized.iter().next().unwrap();
                        let file_name = format!("{}.csv", key);
                        match tokio::fs::metadata(&file_name).await {
                            Ok(_) => {
                                let mut file = tokio::fs::OpenOptions::new()
                                    .write(true)
                                    .append(true)
                                    .open(&file_name)
                                    .await
                                    .unwrap();
                                let values = value
                                    .as_object()
                                    .unwrap()
                                    .values()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>()
                                    .join(",");
                                file.write_all(values.as_bytes()).await.unwrap();
                                file.write_all("\n".as_bytes()).await.unwrap();
                            }
                            Err(_) => {
                                let mut file = tokio::fs::OpenOptions::new()
                                    .write(true)
                                    .create(true)
                                    .append(true)
                                    .open(&file_name)
                                    .await
                                    .unwrap();
                                let columns = value
                                    .as_object()
                                    .unwrap()
                                    .keys()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>()
                                    .join(",");
                                file.write_all(columns.as_bytes()).await.unwrap();
                                file.write_all("\n".as_bytes()).await.unwrap();
                                let values = value
                                    .as_object()
                                    .unwrap()
                                    .values()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>()
                                    .join(",");
                                file.write_all(values.as_bytes()).await.unwrap();
                                file.write_all("\n".as_bytes()).await.unwrap();
                                continue;
                            }
                        }
                    }
                });
            }
            while let Some(res) = set.join_next().await {
                info!("task completed: {:?}", res);
            }
        });
        Ok(handle)
    }
}
