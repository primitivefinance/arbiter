use std::{
    any::type_name,
    collections::BTreeMap,
    env,
    fmt::Debug,
    fs,
    marker::PhantomData,
    mem::transmute,
    sync::{atomic::AtomicBool, Arc},
    thread::JoinHandle,
};

use ethers::{
    contract::{EthEvent, EthLogDecode, Event},
    providers::{Middleware, MiddlewareError, StreamExt},
    types::{Filter, Log},
};
use futures_util::future::Either;
use futures_util::stream::select;
use polars::{
    prelude::{CsvWriter, DataFrame, NamedFrom, SerWriter},
    series::Series,
};
use serde::Serialize;
use serde_json::Value;

use crate::{
    environment::{Instruction, InstructionSender, Outcome},
    middleware::{RevmMiddleware, RevmMiddlewareError},
};

enum EventCaptureItem<T> {
    Event(T),
    StopSignal,
}

pub type ArbiterEvent<F> = Event<Arc<RevmMiddleware>, RevmMiddleware, F>;

pub struct EventCapture<F: EthEvent + EthLogDecode + Debug + 'static> {
    event: ArbiterEvent<F>,
    client: Arc<RevmMiddleware>,
    stop_signal_sender: futures_channel::mpsc::Sender<()>,
    stop_signal_receiver: futures_channel::mpsc::Receiver<()>,
}

impl<F: Serialize + EthEvent + EthLogDecode + Debug> EventCapture<F> {
    pub fn new(client: Arc<RevmMiddleware>, event: ArbiterEvent<F>) -> Self {
        let (stop_signal_sender, stop_signal_receiver) = futures_channel::mpsc::channel(1);
        Self {
            event,
            client,
            stop_signal_sender,
            stop_signal_receiver,
        }
    }

    pub async fn run(
        self,
    ) -> Result<(JoinHandle<()>, futures_channel::mpsc::Sender<()>), RevmMiddlewareError> {
        // let event_stream = Box::leak(Box::new(self.event))
        //     .stream()
        //     .await
        //     .unwrap()
        //     .map(EventCaptureItem::Event);
        let event_stream = Box::leak(Box::new(self.event))
            .stream()
            .await
            .unwrap()
            .map(EventCaptureItem::Event);
        let mut stream = select(
            event_stream,
            self.stop_signal_receiver
                .map(|_| EventCaptureItem::StopSignal),
        );
        let mut capture = BTreeMap::new();
        println!("entering task");
        let handle = std::thread::spawn(move || {
            println!("Listening for events");
            let rt = tokio::runtime::Runtime::new().unwrap();

            rt.block_on(async {
                loop {
                    if let Some(event_capture_item) = stream.next().await {
                        match event_capture_item {
                            EventCaptureItem::Event(Ok(event)) => {
                                // Initialize the capture upon first event
                                println!("got event: {:?}", event);
                                if capture.is_empty() {
                                    let serialized = serde_json::to_string(&event).unwrap();
                                    let deserialized: BTreeMap<String, Value> =
                                        serde_json::from_str(&serialized).unwrap();
                                    for (name, value) in deserialized.into_iter() {
                                        capture.insert(name, vec![value]);
                                    }
                                } else {
                                    let serialized = serde_json::to_string(&event).unwrap();
                                    let deserialized: BTreeMap<String, Value> =
                                        serde_json::from_str(&serialized).unwrap();
                                    for (name, value) in deserialized.into_iter() {
                                        let storage = capture.get_mut(&name).unwrap();
                                        storage.push(value);
                                    }
                                }
                            }
                            EventCaptureItem::StopSignal => {
                                println!("got stop signal");
                                let mut series_vec = vec![];
                                // Create a directory in the CWD to store the CSV file.
                                let current_dir = env::current_dir().unwrap();
                                let output_dir = current_dir.join("output");
                                fs::create_dir_all(&output_dir).unwrap();

                                if !capture.is_empty() {
                                    for (name, storage) in capture.iter() {
                                        println!("name: {:?}", name);
                                        println!("storage: {:?}", storage);
                                        series_vec.push(Series::new(
                                            name,
                                            storage
                                                .iter()
                                                .map(|x| x.as_str().unwrap().to_string())
                                                .collect::<Vec<String>>(),
                                        ));
                                    }
                                    // Write out the CSV file using the environment label.
                                    let mut dataframe = DataFrame::new(series_vec).unwrap();
                                    let file_path =
                                        output_dir.join(format!("{}.csv", "test_label"));
                                    let file = fs::File::create(file_path).unwrap();
                                    let mut writer = CsvWriter::new(file);
                                    writer.finish(&mut dataframe).unwrap();
                                }

                                break;
                            }
                            EventCaptureItem::Event(Err(e)) => {
                                println!("error: {:?}", e); //  TODO: handle properly
                            }
                        }
                    } else {
                        continue;
                    }
                }
            });
        });

        println!("got here.");
        // let instruction = Instruction::AttachCapture {
        //     handle,
        //     stop_signal_sender: self.stop_signal_sender,
        //     outcome_sender: self.client.outcome_sender(),
        // };
        // let outcome = self.client.send_instruction(instruction).await?;
        // match outcome {
        //     Outcome::AttachCaptureComplete => Ok(()),
        //     _ => Err(RevmMiddlewareError::MissingData(
        //         "Wrong variant returned via instruction outcome!".to_string(),
        //     )),
        // }
        Ok((handle, self.stop_signal_sender))
    }
}

struct EventTransmuter<B, M, D> {
    /// The event filter's state
    pub filter: Filter,
    pub provider: B,
    /// Stores the event datatype
    pub(crate) datatype: PhantomData<D>,
    pub(crate) _m: PhantomData<M>,
}
