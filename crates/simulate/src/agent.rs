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
    ) -> Vec<Log>; // TODO: Not sure this needs to be mutable self

    // TODO: Should agents be labeled as `active` or `inactive` similarly to `IsDeployed` and `NotDeployed`?
}
