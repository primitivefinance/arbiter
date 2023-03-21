use bytes::Bytes;
use revm::primitives::{ExecutionResult, Log, Output, TxEnv, B160, U256};

use crate::environment::{IsDeployed, SimulationContract};

// use crate::environment::SimulationEnvironment;

pub struct TransactSettings {
    pub gas_limit: u64,
    pub gas_price: U256,
    pub value: U256,
}

pub trait Agent {
    fn call_contract(
        &mut self,
        contract: &SimulationContract<IsDeployed>,
        call_data: Bytes,
        value: U256,
    ) -> ExecutionResult;
    fn build_call_transaction(
        &self,
        receiver_address: B160,
        call_data: Bytes,
        value: U256,
    ) -> TxEnv;
    fn read_logs(
        &mut self,
        contract: SimulationContract<IsDeployed>,
        event_name: &str,
        execution_result: ExecutionResult,
    ) -> &Vec<revm::primitives::Log>; // TODO: Not sure this needs to be mutable self

    // TODO: Should agents be labeled as `active` or `inactive` similarly to `IsDeployed` and `NotDeployed`?

    // TODO: We may never actually need to return logs here depending on how we handle echoing to a buffer.
    fn unpack_execution(&self, execution_result: ExecutionResult) -> (Bytes, Vec<Log>) {
        // unpack output call enum into raw bytes
        match execution_result {
            ExecutionResult::Success { output, logs, .. } => match output {
                Output::Call(value) => (value, logs),
                Output::Create(_, Some(_)) => {
                    panic!("Failed. This was a 'Create' call, use 'Deploy' instead.")
                }
                _ => panic!("This call has failed."),
            },
            _ => panic!("This call generated no execution result. This should not happen."),
        }
    }
}
