#![warn(missing_docs)]
#![warn(unsafe_code)]
//! ## module for the environment
//!
//! An abstraction on the EVM, to be used in simulations.
pub mod contract;

use crossbeam_channel::{unbounded, Receiver, Sender};
use ethers::{abi::Token, prelude::AbiError};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, Log, TxEnv, U256},
    EVM,
};
use std::{thread, pin::Pin};
use tokio::sync::broadcast;
use futures::{task::{Context, Poll}, Stream};
use futures::stream::StreamExt;

use crate::agent::{SimulationEventFilter, AgentError, filter_events};

/// The simulation environment that houses the execution environment and event logs.
/// # Fields
/// * `evm` - The EVM that is used for the simulation.
/// * `event_senders` - The senders on the event channel that is used to send events to the agents and simulation manager.
pub struct SimulationEnvironment {
    /// The EVM that is used for the simulation.
    pub(crate) evm: EVM<CacheDB<EmptyDB>>,
    /// The sender on the event channel that is used to send events to the agents and simulation manager.
    pub(crate) event_broadcaster: EventBroadcaster,
    /// The receiver of txs from agents.
    /// Bundles with a sender to send the execution result back to the agent.
    pub(crate) transaction_channel: (
        Sender<(TxEnv, Sender<ExecutionResult>)>,
        Receiver<(TxEnv, Sender<ExecutionResult>)>,
    ),
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
            event_broadcaster: EventBroadcaster::new(),
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
                event_broadcaster.broadcast(execution_result.logs()); 
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

    let execution_result = match evm.transact_commit() {
        Ok(val) => val,
        // URGENT: change this to a custom error
        Err(_) => panic!("failed"),
    };

    execution_result
}

#[derive(Clone, Debug)]
pub struct EventBroadcaster {
    senders: Vec<crossbeam_channel::Sender<Vec<Log>>>,
}

impl EventBroadcaster {
    pub(crate) fn new() -> Self {
        Self { senders: vec![] }
    }

    pub(crate) fn add_sender(&mut self, sender: crossbeam_channel::Sender<Vec<Log>>) {
        self.senders.push(sender);
    }

    pub(crate) fn broadcast(&self, logs: Vec<Log>) {
        for sender in &self.senders {
            sender.send(logs.clone()).unwrap();
        }
    }
}

#[derive(Clone, Debug)]
pub struct EventStream {
    pub(crate) receiver: crossbeam_channel::Receiver<Vec<Log>>,
    pub(crate) filters: Vec<SimulationEventFilter>,
}

impl EventStream {
    fn next(&mut self) -> Option<Result<(Vec<Token>, revm::primitives::Address), AbiError>> {
        let event_filters = self.filters.clone();
        assert!(!event_filters.is_empty());
        let decoder = |input| {
            event_filters[0].base_contract.decode_event_raw(
                event_filters[0].event_name.as_str(),
                vec![event_filters[0].topic],
                input,
            )
        };

        self.receiver.recv().ok().map(|logs| {
            let filtered_logs = filter_events(event_filters.clone(), logs);
            if filtered_logs.is_empty() {
                return Ok((vec![], Default::default())); // Return default address if no logs
            }
            let data = filtered_logs[0].data.clone().into_iter().collect();
            let address = filtered_logs[0].address; // Extract address here
            decoder(data).map(|tokens| (tokens, address))
        })
    }

    pub fn into_stream(self) -> impl Stream<Item = Result<(Vec<Token>, revm::primitives::Address), AbiError>> {
        futures::stream::unfold(self, |mut state| async {
            match state.next() {
                Some(item) => Some((item, state)),
                None => None,
            }
        })
    }
    
}


#[cfg(test)]
mod tests {
    use crate::agent::{AgentType, Agent};
    use crate::agent::user::User;
    use crate::environment::contract::SimulationContract;
    use crate::manager::{self, SimulationManager};

    use super::*;
    use bindings::writer;
    use ethers::abi::{Function, Param, ParamType, StateMutability, Tokenize};
    use ethers::types::{Address, H256, U256};
    use revm::primitives::{TxEnv, B160};
    use std::error::Error;
    use std::str::FromStr;

    #[tokio::test]
    async fn event_stream() -> Result<(), Box<dyn Error>> {
        // Start a new environment
        let mut manager = SimulationManager::new();
        // let admin = manager.agents["admin"].inner();

        // copypastge
        // Create writer contract.
        let writer =
            SimulationContract::new(writer::WRITER_ABI.clone(), writer::WRITER_BYTECODE.clone());

        // Deploy the writer.
        let test_string = "Hello, world..?".to_string();
        let (writer, execution_result) = manager.agents["admin"]
            .deploy(writer, test_string.into_tokens())?;
        assert!(execution_result.is_success());

        // Create two agents with a filter.
        let filters = vec![SimulationEventFilter::new(&writer, "WasWritten")];

        // Create a new event stream
        let (sender, receiver) = crossbeam_channel::unbounded();
        let mut event_stream = Box::pin(EventStream {
            receiver,
            filters,
        }.into_stream());

        // Have the environment broadcast an event to populate the stream


        // Check that the event stream has the event
        let handle = tokio::spawn(async move {
            println!("Waiting for event...");
            while let Ok((tokens, address)) = event_stream.next().await.unwrap() {
                println!("Got event!");
                println!("Tokens: {:?}\nAddress: {:?}", tokens, address);
                // assert_eq!(event, vec![]);
            }
        });


        // Check that the event stream records events async and we can get next events        
        for i in 0..3 {
            println!("Calling echoString... {i}");
            let new_string = format!("Hello, world..? {}", i);
            manager.agents["admin"].call(
                &writer,
                "echoString",
                new_string.into_tokens(),
            )?;
        }

        handle.await?;


        Ok(())
    }

    fn event_stream_filter() {
        todo!()
        // Check that the filter works

        // Check that the decoder works


    }


}