use std::{collections::BTreeMap, fmt::Debug};

use ethers::{
    contract::{builders::Event, EthLogDecode},
    providers::{Middleware, StreamExt as ProviderStreamExt},
};
use serde::Serialize;
use serde_json::Value;
use tokio::io::AsyncWriteExt;
use tracing::info;

use crate::middleware::errors::RevmMiddlewareError;

pub struct EventLogger<
    M: Middleware + std::borrow::Borrow<D> + 'static,
    D: Middleware + Debug + Send + Sync + 'static,
    E: EthLogDecode + Debug + Serialize + 'static,
> {
    events: BTreeMap<String, Vec<Event<M, D, E>>>,
    path: Option<String>,
}

impl<
        M: Middleware + std::borrow::Borrow<D>,
        D: Middleware + Debug + Send + Sync,
        E: EthLogDecode + Debug + Serialize + 'static,
    > EventLogger<M, D, E>
{
    pub fn builder() -> Self {
        Self {
            events: BTreeMap::new(),
            path: None,
        }
    }

    pub fn add<S: Into<String>>(mut self, event: Event<M, D, E>, name: S) -> Self {
        let name = name.into();
        self.events.entry(name).or_insert_with(Vec::new).push(event);
        self
    }

    pub fn path<S: Into<String>>(mut self, path: S) -> Self {
        self.path = Some(path.into());
        self
    }

    pub async fn run(self) -> Result<(), RevmMiddlewareError> {
        // Delete the ./events path before kicking off the run loop
        let path = self.path.unwrap_or("./events".to_string());
        tokio::fs::remove_dir_all(&path).await.unwrap_or_default();

        tokio::spawn(async move {
            let mut set = tokio::task::JoinSet::new();
            for (name, events) in self.events {
                let dir_path = format!("{}/{}", path, name);
                tokio::fs::create_dir_all(&dir_path).await.unwrap();
                for event in events {
                    let dir_path = dir_path.clone();
                    set.spawn(async move {
                        let mut stream = event.stream().await.unwrap();
                        while let Some(Ok(log)) = stream.next().await {
                            let serialized = serde_json::to_string(&log).unwrap();
                            let deserialized: BTreeMap<String, Value> =
                                serde_json::from_str(&serialized).unwrap();
                            let (key, value) = deserialized.iter().next().unwrap();
                            let file_name = format!("{}/{}.csv", dir_path, key);
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
            }
            while let Some(res) = set.join_next().await {
                info!("task completed: {:?}", res);
            }
        });
        Ok(())
    }
}
