use std::ops::Range;

use revm::{
    interpreter::{CallInputs, InterpreterResult},
    Database, EvmContext, Inspector,
};
use revm_primitives::{address, Address, Bytes};

const CONSOLE_ADDRESS: Address = address!("000000000000000000636F6e736F6c652e6c6f67");

#[allow(clippy::all)]
#[rustfmt::skip]
pub mod abi;

/// An inspector that collects `console2.log`s during execution.
#[derive(Debug, Clone, Default)]
pub struct ConsoleLogs(pub Vec<Bytes>);

impl<DB: Database> Inspector<DB> for ConsoleLogs {
    #[inline]
    fn call(
        &mut self,
        _context: &mut EvmContext<'_, DB>,
        call: &mut CallInputs,
    ) -> Option<(InterpreterResult, Range<usize>)> {
        if call.contract == CONSOLE_ADDRESS {
            self.0.push(call.input.clone());
        }
        None
    }
}