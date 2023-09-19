use std::{
    any::type_name,
    collections::BTreeMap,
    fmt::Debug,
    marker::PhantomData,
    mem::transmute,
    sync::{atomic::AtomicBool, Arc},
};

use ethers::{
    contract::{EthEvent, EthLogDecode, Event},
    providers::{Middleware, MiddlewareError, StreamExt},
    types::Filter,
};
use serde::Serialize;
use serde_json::Value;

use crate::{
    environment::{Instruction, InstructionSender, Outcome},
    middleware::{RevmMiddleware, RevmMiddlewareError},
};

pub type ArbiterEvent<F> = Event<Arc<RevmMiddleware>, RevmMiddleware, F>;

pub struct EventCapture<F: EthEvent + EthLogDecode + Debug + 'static> {
    event: ArbiterEvent<F>,
    capture: Vec<BTreeMap<String, Value>>,
    pub state: Arc<AtomicBool>,
}

impl<F: Serialize + EthEvent + EthLogDecode + Debug> EventCapture<F> {
    pub fn new(event: ArbiterEvent<F>) -> Self {
        Self {
            event,
            capture: vec![],
            state: Arc::new(AtomicBool::new(false)),
        }
    }

    // TODO: We should probalby set the state only once everything is sent to the environment properly
    pub async fn run(self) -> Result<(), RevmMiddlewareError> {
        let event_transmuter: EventTransmuter<Arc<RevmMiddleware>, RevmMiddleware, F> =
            unsafe { transmute(self.event) };
        let provider = event_transmuter.provider.clone();
        let event: Event<Arc<RevmMiddleware>, RevmMiddleware, F> =
            unsafe { transmute(event_transmuter) };
        self.state.store(true, std::sync::atomic::Ordering::SeqCst);
        let mut capture = self.capture;
        let running = self.state.clone();
        let handle = tokio::spawn(async move {
            println!("Listening for events");
            let mut stream = event.stream().await.unwrap();
            loop {
                tokio::select! {
                    // Check if we should stop running
                    _ = async {
                        if !running.load(std::sync::atomic::Ordering::SeqCst) {
                            Some(())
                        } else {
                            None
                        }
                    } => {
                        for (name, value) in deserialized.iter() {
                            series_vec.push(Series::new(
                                name,
                                value
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .map(|x| x.as_str().unwrap().to_string())
                                    .collect::<Vec<String>>(),
                            ));
                        }
                        let mut dataframe = DataFrame::new(series_vec)?;

                        // Create a directory in the CWD to store the CSV file.
                        let current_dir = env::current_dir()?;
                        let output_dir = current_dir.join("output");
                        fs::create_dir_all(&output_dir)?;

                        // Write out the CSV file using the environment label.
                        let file_path = output_dir.join(format!("{}.csv", label));
                        let file = fs::File::create(file_path)?;
                        let mut writer = CsvWriter::new(file);
                        writer.finish(&mut dataframe)?;
                        break capture;
                    }
                    // Or continue to process events
                    next_event = stream.next() => {
                        if let Some(Ok(event)) = next_event {
                            let serialized = serde_json::to_string(&event).unwrap();
                            let deserialized: BTreeMap<String, Value> = serde_json::from_str(&serialized).unwrap();
                            println!("deserialized: {:?}", deserialized);
                            capture.push(deserialized);
                        }
                    }
                }
            }
        });
        let instruction = Instruction::AttachCapture {
            handle,
            state: self.state,
            outcome_sender: provider.outcome_sender(),
        };
        let outcome = provider.send_instruction(instruction).await?;
        match outcome {
            Outcome::AttachCaptureComplete => Ok(()),
            _ => Err(RevmMiddlewareError::MissingData(
                "Wrong variant returned via instruction outcome!".to_string(),
            )),
        }
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
