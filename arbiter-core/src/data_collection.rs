//! The `data_collection` module provides the `EventLogger` struct for logging
//! events from the Ethereum network.
//!
//! The `EventLogger` struct contains a BTreeMap of events, where each event is
//! represented by a string key and a vector of `Event` instances.
//! It also optionally contains a path where the event logs will be stored.
//!
//! This module also provides the implementation of the `EventLogger` struct,
//! including methods for constructing a new `EventLogger`, adding an event to
//! the `EventLogger`, and writing the event logs to a file.
//!
//! # Type Parameters
//!
//! * `M` - Middleware that implements the `Middleware` trait,
//!   `std::borrow::Borrow<D>`, and has a static lifetime.
//! * `D` - Middleware that implements the `Middleware` trait, `Debug`, `Send`,
//!   `Sync`, and has a static lifetime.
//! * `E` - Type that implements the `EthLogDecode`, `Debug`, `Serialize`
//!   traits, and has a static lifetime.
use std::{
    collections::BTreeMap, env::current_dir, fmt::Debug, marker::PhantomData, mem::transmute,
    sync::Arc,
};

use ethers::{
    contract::{builders::Event, EthLogDecode},
    providers::{Middleware, StreamExt as ProviderStreamExt},
    types::{Filter, FilteredParams},
};
use serde::Serialize;
use serde_json::Value;
use tokio::io::AsyncWriteExt;
use tracing::info;

use crate::{
    environment::{Broadcast, Socket},
    middleware::{cast::revm_logs_to_ethers_logs, errors::RevmMiddlewareError, RevmMiddleware},
};

/// `EventLogger` is a struct that logs events from the Ethereum network.
///
/// It contains a BTreeMap of events, where each event is represented by a
/// string key and a vector of `Event` instances. It also optionally contains a
/// path where the event logs will be stored.
///
/// # Type Parameters
///
/// * `M` - Middleware that implements the `Middleware` trait,
///   `std::borrow::Borrow<D>`, and has a static lifetime.
/// * `D` - Middleware that implements the `Middleware` trait, `Debug`, `Send`,
///   `Sync`, and has a static lifetime.
/// * `E` - Type that implements the `EthLogDecode`, `Debug`, `Serialize`
///   traits, and has a static lifetime.
pub struct EventLogger {
    events: tokio::task::JoinSet<()>,
    path: Option<String>,
}

impl EventLogger {
    /// Constructs a new `EventLogger`.
    ///
    /// # Returns
    ///
    /// A fresh `EventLogger` instance with an uninitialized events BTreeMap and
    /// no specified path.
    pub fn builder() -> Self {
        Self {
            events: tokio::task::JoinSet::new(),
            path: None,
        }
    }

    /// Adds an event to the `EventLogger`.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to be added.
    /// * `name` - The name of the event.
    ///
    /// # Returns
    ///
    /// The `EventLogger` instance with the added event.
    pub fn add<S: Into<String>, E: EthLogDecode + Debug + Serialize + 'static>(
        mut self,
        event: Event<Arc<RevmMiddleware>, RevmMiddleware, E>,
        name: S,
    ) -> Self {
        // Grab the connection from the client and add a new event sender so that we have a distinct channel to now receive events over
        let event_transmuted: EventTransmuted<Arc<RevmMiddleware>, RevmMiddleware, E> =
            unsafe { transmute(event) };
        let middleware = event_transmuted.provider.clone();
        let filter = event_transmuted.filter.clone();
        let connection = middleware.provider().as_ref();
        let (event_sender, event_receiver) = crossbeam_channel::unbounded::<Broadcast>();
        connection
            .event_broadcaster
            .lock()
            .unwrap()
            .add_sender(event_sender);

        // In here we build as big of a filter as we want, then pass to run to actually get ALL the events

        std::thread::spawn(move || {
            let mut logs = vec![];
            let filtered_params = FilteredParams::new(Some(filter));
            while let Ok(broadcast) = event_receiver.recv() {
                match broadcast {
                    Broadcast::StopSignal => break,
                    Broadcast::Event(event) => {
                        let ethers_logs = revm_logs_to_ethers_logs(event);
                        for log in ethers_logs {
                            if filtered_params.filter_address(&log)
                                && filtered_params.filter_topics(&log)
                            {
                                logs.push(log);
                            }
                        }
                    }
                }
            }
        });

        // let name = name.into();
        // let event_dir = current_dir()
        //     .unwrap()
        //     .join(self.path.clone().unwrap_or("events".into()))
        //     .join(name);
        // std::fs::create_dir_all(&event_dir).unwrap();
        // self.events.spawn(async move {
        //     let mut stream = event.stream().await.unwrap();
        //     let mut files: BTreeMap<String, tokio::fs::File> = BTreeMap::new();
        //     let mut columns_written: BTreeMap<String, bool> = BTreeMap::new();
        //     while let Some(Ok(log)) = stream.next().await {
        //         let serialized = serde_json::to_string(&log).unwrap();
        //         let deserialized: BTreeMap<String, Value> =
        //             serde_json::from_str(&serialized).unwrap();
        //         let (key, value) = deserialized.iter().next().unwrap();
        //         let file_name = event_dir.join(format!("{}.csv", key));
        //         let file_key = file_name.to_str().unwrap();
        //         let file_value = files.get(file_key);
        //         let toggle_written_columns = columns_written.get(file_key).unwrap_or(&false);
        //         if file_value.is_none() {
        //             files.insert(
        //                 file_key.into(),
        //                 tokio::fs::OpenOptions::new()
        //                     .write(true)
        //                     .create(true)
        //                     .truncate(true)
        //                     .open(&file_name)
        //                     .await
        //                     .unwrap(),
        //             );
        //         }

        //         let file = files.get_mut(file_key).unwrap();

        //         if toggle_written_columns == &true {
        //             let values = value
        //                 .as_object()
        //                 .unwrap()
        //                 .values()
        //                 .map(|x| x.to_string())
        //                 .collect::<Vec<String>>()
        //                 .join(",");
        //             file.write_all(values.as_bytes()).await.unwrap();
        //             file.write_all("\n".as_bytes()).await.unwrap();
        //         } else {
        //             columns_written.entry(file_key.into()).or_insert(true);
        //             let columns = value
        //                 .as_object()
        //                 .unwrap()
        //                 .keys()
        //                 .map(|x| x.to_string())
        //                 .collect::<Vec<String>>()
        //                 .join(",");
        //             file.write_all(columns.as_bytes()).await.unwrap();
        //             file.write_all("\n".as_bytes()).await.unwrap();
        //             let values = value
        //                 .as_object()
        //                 .unwrap()
        //                 .values()
        //                 .map(|x| x.to_string())
        //                 .collect::<Vec<String>>()
        //                 .join(",");
        //             file.write_all(values.as_bytes()).await.unwrap();
        //             file.write_all("\n".as_bytes()).await.unwrap();
        //         }
        //         continue;
        //     }
        // });
        self
    }

    /// Sets the path for the `EventLogger`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path where the event logs will be stored.
    ///
    /// # Returns
    ///
    /// The `EventLogger` instance with the specified path.
    pub fn path<S: Into<String>>(mut self, path: S) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Executes the `EventLogger`.
    ///
    /// This function starts the event logging process. It first deletes the
    /// existing events directory, then creates a new directory for each
    /// event. For each event, it creates a new CSV file and writes
    /// the event data into the file. If the file already exists, it appends the
    /// new data to the file.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    ///
    /// * `Ok(())` if the `EventLogger` ran successfully.
    /// * `Err(RevmMiddlewareError)` if there was an error running the
    ///   `EventLogger`.
    ///
    /// # Errors
    ///
    /// This function will return an error if there is a problem creating the
    /// directories or files, or writing to the files.
    pub fn run(self) -> Result<(), RevmMiddlewareError> {
        tokio::spawn(async move {
            let mut set = self.events;
            while let Some(res) = set.join_next().await {
                info!("task completed: {:?}", res);
            }
        });
        Ok(())
    }
}

pub struct EventTransmuted<B, M, D> {
    /// The event filter's state
    pub filter: Filter,
    pub(crate) provider: B,
    /// Stores the event datatype
    pub(crate) datatype: PhantomData<D>,
    pub(crate) _m: PhantomData<M>,
}
