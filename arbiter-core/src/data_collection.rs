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
    collections::BTreeMap, env::current_dir, fmt::Debug, fs::File, io::BufWriter,
    marker::PhantomData, mem::transmute, sync::Arc,
};

use ethers::{
    abi::RawLog,
    contract::{builders::Event, EthLogDecode},
    providers::{Middleware, StreamExt as ProviderStreamExt},
    types::{Filter, FilteredParams},
};
use serde::Serialize;
use serde_json::Value;

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
    directory: Option<String>,
    file_name: Option<String>,
    decoder: BTreeMap<String, (FilteredParams, Box<dyn Fn(&RawLog) -> String + Send + Sync>)>,
    receiver: Option<crossbeam_channel::Receiver<Broadcast>>,
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
            directory: None,
            file_name: None,
            decoder: BTreeMap::new(),
            receiver: None,
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
    pub fn add<S: Into<String>, D: EthLogDecode + Debug + Serialize + 'static>(
        mut self,
        event: Event<Arc<RevmMiddleware>, RevmMiddleware, D>,
        name: S,
    ) -> Self {
        // Grab the connection from the client and add a new event sender so that we have a distinct channel to now receive events over
        let event_transmuted: EventTransmuted<Arc<RevmMiddleware>, RevmMiddleware, D> =
            unsafe { transmute(event) };
        let middleware = event_transmuted.provider.clone();
        let decoder = |x: &_| serde_json::to_string(&D::decode_log(x).unwrap()).unwrap();
        let filter = event_transmuted.filter.clone();
        self.decoder.insert(
            name.into(),
            (FilteredParams::new(Some(filter)), Box::new(decoder)),
        );
        let connection = middleware.provider().as_ref();
        if self.receiver.is_none() {
            let (event_sender, event_receiver) = crossbeam_channel::unbounded::<Broadcast>();
            connection
                .event_broadcaster
                .lock()
                .unwrap()
                .add_sender(event_sender);
            self.receiver = Some(event_receiver);
        }
        self
    }
    /// Sets the directory for the `EventLogger`.
    ///
    /// # Arguments
    ///
    /// * `directory` - The directory where the event logs will be stored.
    ///
    /// # Returns
    ///
    /// The `EventLogger` instance with the specified directory.
    pub fn directory<S: Into<String>>(mut self, path: S) -> Self {
        self.directory = Some(path.into());
        self
    }

    /// Sets the output file name for the `EventLogger`.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The file where the event logs will be stored.
    ///
    /// # Returns
    ///
    /// The `EventLogger` instance with the specified file.
    pub fn file_name<S: Into<String>>(mut self, path: S) -> Self {
        self.file_name = Some(path.into());
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
        let receiver = self.receiver.unwrap();
        let dir = self.directory.unwrap_or("./out".into());
        let file_name = self.file_name.unwrap_or("output".into());
        std::thread::spawn(move || {
            let mut logs: BTreeMap<String, BTreeMap<String, Vec<Value>>> = BTreeMap::new();
            while let Ok(broadcast) = receiver.recv() {
                match broadcast {
                    Broadcast::StopSignal => {
                        // create new directory with path
                        let output_dir = std::env::current_dir().unwrap().join(dir);
                        std::fs::create_dir_all(&output_dir).unwrap();
                        let file_path = output_dir.join(format!("{}.json", file_name));
                        let file = std::fs::File::create(file_path).unwrap();
                        let writer = BufWriter::new(file);
                        serde_json::to_writer(writer, &logs).expect("Unable to write data");
                        break;
                    }
                    Broadcast::Event(event) => {
                        let ethers_logs = revm_logs_to_ethers_logs(event);
                        for log in ethers_logs {
                            for (contract_name, (filter, decoder)) in self.decoder.iter() {
                                if filter.filter_address(&log) && filter.filter_topics(&log) {
                                    let cloned_logs = log.clone();
                                    let event_as_value = serde_json::from_str::<Value>(&decoder(
                                        &cloned_logs.into(),
                                    ))
                                    .unwrap();
                                    let event_as_object = event_as_value.as_object().unwrap();

                                    let contract = logs.get(contract_name);
                                    if contract.is_none() {
                                        logs.insert(contract_name.clone(), BTreeMap::new());
                                    }
                                    let contract = logs.get_mut(contract_name).unwrap();

                                    let event_name =
                                        event_as_object.clone().keys().collect::<Vec<&String>>()[0]
                                            .clone();

                                    let event = contract.get_mut(&event_name);
                                    if event.is_none() {
                                        contract.insert(event_name.to_string(), vec![]);
                                    }
                                    let event = contract.get_mut(&event_name).unwrap();

                                    for (_key, value) in event_as_object {
                                        event.push(value.clone());
                                    }
                                }
                            }
                        }
                    }
                }
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
