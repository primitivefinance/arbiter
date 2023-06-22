#![warn(missing_docs)]
#![warn(unsafe_code)]
//! ## module for the environment
//!
//! An abstraction on the EVM, to be used in simulations.
pub mod contract;

use crossbeam_channel::{unbounded, Receiver, Sender};
use ethers::{abi::Token, prelude::AbiError};
use futures::Stream;
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, Log, TxEnv, U256},
    EVM,
};
use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::agent::{filter_events, SimulationEventFilter};
/// Type Aliases for the event channel.
pub(crate) type ExecutionSender = Sender<ExecutionResult>;
pub(crate) type TxEnvSender = Sender<(TxEnv, ExecutionSender)>;
pub(crate) type TxEnvReceiver = Receiver<(TxEnv, ExecutionSender)>;

/// The simulation environment that houses the execution environment and event logs.
/// # Fields
/// * `evm` - The EVM that is used for the simulation.
/// * `event_senders` - The senders on the event channel that is used to send events to the agents and simulation manager.
pub struct SimulationEnvironment {
    /// The EVM that is used for the simulation.
    pub(crate) evm: EVM<CacheDB<EmptyDB>>,
    /// The sender on the event channel that is used to send events to the agents and simulation manager.
    pub(crate) event_broadcaster: Arc<Mutex<EventBroadcaster>>,
    /// The receiver of txs from agents.
    /// Bundles with a sender to send the execution result back to the agent.
    pub(crate) transaction_channel: (TxEnvSender, TxEnvReceiver),
}

impl SimulationEnvironment {
    pub(crate) fn new() -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.cfg.limit_contract_code_size = Some(0x100000); // This is a large contract size limit, beware!
        evm.env.block.gas_limit = U256::MAX;
        evm.database(db);
        let transaction_channel = unbounded::<(TxEnv, Sender<ExecutionResult>)>();
        Self {
            evm,
            event_broadcaster: Arc::new(Mutex::new(EventBroadcaster::new())),
            transaction_channel,
        }
    }

    pub(crate) fn run(&self) {
        let tx_receiver = self.transaction_channel.1.clone();
        let mut evm = self.evm.clone();
        let event_broadcaster = self.event_broadcaster.clone();
        thread::spawn(move || {
            while let Ok((tx, sender)) = tx_receiver.recv() {
                // Execute the transaction, echo the logs to all agents, and report the execution result to the agent who made the call.
                let execution_result = execute(&mut evm, tx);
                event_broadcaster
                    .lock()
                    .unwrap()
                    .broadcast(execution_result.logs());
                sender.send(execution_result).unwrap();
            }
        });
    }
}

/// Execute a transaction in the execution environment.
/// # Arguments
/// * `tx` - The transaction environment that is used to execute the transaction.
/// # Returns
/// * `ExecutionResult` - The execution result of the transaction.
fn execute(evm: &mut EVM<CacheDB<EmptyDB>>, tx: TxEnv) -> ExecutionResult {
    evm.env.tx = tx;

    match evm.transact_commit() {
        Ok(val) => val,
        // URGENT: change this to a custom error
        Err(_) => panic!("failed"),
    }
}

/// The event broadcaster that is used to broadcast events to the agents from the simulation manager.
#[derive(Clone, Debug)]
pub struct EventBroadcaster(Vec<crossbeam_channel::Sender<Vec<Log>>>);

impl EventBroadcaster {
    pub(crate) fn new() -> Self {
        Self(vec![])
    }

    pub(crate) fn add_sender(&mut self, sender: crossbeam_channel::Sender<Vec<Log>>) {
        self.0.push(sender);
    }

    pub(crate) fn broadcast(&self, logs: Vec<Log>) {
        for sender in &self.0 {
            sender.send(logs.clone()).unwrap();
        }
    }
}

/// The event stream is an agents individual stream of events that it owns.
#[derive(Clone, Debug)]
pub struct EventStream {
    pub(crate) receiver: crossbeam_channel::Receiver<Vec<Log>>,
    pub(crate) filters: Vec<SimulationEventFilter>,
}

impl EventStream {
    fn next(&mut self) -> Option<Result<(Vec<Token>, usize), AbiError>> {
        let event_filters = self.filters.clone();
        assert!(!event_filters.is_empty());
        let decoder = |input, filter_number: usize| {
            event_filters[filter_number].base_contract.decode_event_raw(
                event_filters[filter_number].event_name.as_str(),
                vec![event_filters[filter_number].topic],
                input,
            )
        };

        self.receiver.recv().ok().and_then(|logs| {
            let (filtered_events, index) = filter_events(event_filters.clone(), logs);
            if filtered_events.is_empty() {
                None // Skip logs that don't pass the filter
            } else {
                let data = filtered_events[0].data.clone().into_iter().collect();
                Some(decoder(data, index).map(|tokens| (tokens, index)))
            }
        })
    }

    /// Converts the event stream into a stream of events.
    pub fn into_stream(self) -> impl Stream<Item = Result<(Vec<Token>, usize), AbiError>> {
        futures::stream::unfold(self, |mut state| async {
            state.next().map(|item| (item, state))
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::agent::Agent;
    use crate::environment::contract::SimulationContract;
    use crate::manager::SimulationManager;

    use super::*;
    use bindings::writer;
    use ethers::abi::Tokenize;
    use futures::StreamExt;
    use std::error::Error;

    #[tokio::test]
    async fn event_stream() -> Result<(), Box<dyn Error>> {
        // Start a new environment
        let manager = SimulationManager::new();
        // let admin = manager.agents["admin"].inner();

        // Create writer contract.
        let writer =
            SimulationContract::new(writer::WRITER_ABI.clone(), writer::WRITER_BYTECODE.clone());

        // Deploy the writer.
        let test_string = "Hello, world..?".to_string();
        let (writer, execution_result) =
            manager.agents["admin"].deploy(writer, test_string.into_tokens())?;
        assert!(execution_result.is_success());

        // Create a filter.
        let filters = vec![SimulationEventFilter::new(&writer, "WasWritten")];

        // Create a new event stream and add the sender to the event broadcaster.
        let (sender, receiver) = crossbeam_channel::unbounded();
        let mut event_stream = Box::pin(EventStream { receiver, filters }.into_stream());
        manager
            .environment
            .event_broadcaster
            .lock()
            .unwrap()
            .add_sender(sender);

        // Check that the event stream has the event
        let handle = tokio::spawn(async move {
            println!("Waiting for event...");
            let mut i = 0;
            while let Some(event_result) = event_stream.next().await {
                match event_result {
                    Ok((tokens, filter_index)) => {
                        println!("Got event!");
                        println!("Tokens: {:?}\nAddress: {:?}", tokens, filter_index);
                        assert_eq!(
                            tokens[0].clone().into_string().unwrap(),
                            format!("Hello, world..? {}", i)
                        );
                        i += 1;
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        });

        // Check that the event stream records events async and we can get next events
        for i in 0..3 {
            println!("Calling echoString... {i}");
            let new_string = format!("Hello, world..? {}", i);
            manager.agents["admin"].call(&writer, "echoString", new_string.into_tokens())?;
        }

        manager.shutdown();
        handle.await?;

        Ok(())
    }
}
