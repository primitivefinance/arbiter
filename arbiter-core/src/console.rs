use std::ops::Range;

use ethers::{abi::AbiDecode, contract::EthLogDecode, types::H256};
use hex::FromHex;
use revm::{
    interpreter::{CallInputs, Gas, InstructionResult, InterpreterResult},
    Database, EvmContext, Inspector,
};
use revm_primitives::{Address, Bytes, FixedBytes, Log, B256};

use crate::database::ArbiterDB;

// const CONSOLE_ADDRESS: Address = 0x000000000000000000636F6e736F6c652e6c6f67;

/// An inspector that collects logs during execution.
///
/// The inspector collects logs from the LOG opcodes as well as Hardhat-style
/// logs.
#[derive(Debug, Clone, Default)]
pub struct ConsoleLogs(pub Vec<Bytes>);

impl<DB: Database> Inspector<DB> for ConsoleLogs {
    #[inline]
    fn call(
        &mut self,
        context: &mut EvmContext<'_, DB>,
        call: &mut CallInputs,
    ) -> Option<(InterpreterResult, Range<usize>)> {
        if call.contract == Address::from_hex("0x000000000000000000636F6e736F6c652e6c6f67").unwrap()
        {
            self.0.push(call.input.clone());
        }
        None
    }
}
