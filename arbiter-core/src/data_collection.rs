use std::{
    any::type_name,
    collections::BTreeMap,
    fmt::Debug,
    sync::{atomic::AtomicBool, Arc},
};

use ethers::{
    contract::{EthEvent, EthLogDecode, Event},
    providers::StreamExt,
};
use serde::Serialize;
use serde_json::Value;

use crate::middleware::RevmMiddleware;

pub type ArbiterEvent<F> = Event<Arc<RevmMiddleware>, RevmMiddleware, F>;

pub struct EventCapture<F: EthEvent + EthLogDecode + Debug + 'static> {
    event: ArbiterEvent<F>,
    name: String,
    capture: Vec<BTreeMap<String, Value>>,
    pub running: Arc<AtomicBool>,
}

impl<F: Serialize + EthEvent + EthLogDecode + Debug> EventCapture<F> {
    pub fn new(event: ArbiterEvent<F>) -> Self {
        Self {
            event,
            name: type_name::<F>().to_owned(),
            capture: vec![],
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn run(
        self,
    ) -> (
        tokio::task::JoinHandle<Vec<BTreeMap<String, Value>>>,
        Arc<AtomicBool>,
    ) {
        self.running
            .store(true, std::sync::atomic::Ordering::SeqCst);
        let event = self.event;
        let mut capture = self.capture;
        let running = self.running.clone();
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
                        break capture;
                    }
                    // Or continue to process events
                    next_event = stream.next() => {
                        if let Some(Ok(event)) = next_event {
                            let serialized = serde_json::to_string(&event).unwrap();
                            let deserialized: BTreeMap<String, Value> = serde_json::from_str(&serialized).unwrap();
                            capture.push(deserialized);
                        }
                    }
                }
            }
        });
        (handle, self.running)
    }
}
