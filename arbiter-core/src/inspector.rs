use std::ops::Range;

use revm::{
    inspectors::GasInspector,
    interpreter::{CallInputs, Interpreter, InterpreterResult},
    Database, EvmContext, Inspector,
};
use revm_primitives::Address;

use crate::{console::ConsoleLogs, database::ArbiterDB};

#[derive(Debug, Clone)]
pub struct ArbiterInspector {
    pub console_log: Option<ConsoleLogs>,
    pub gas: Option<GasInspector>,
}

impl ArbiterInspector {
    pub fn new(console_log: bool, gas: bool) -> Self {
        Self {
            console_log: Some(ConsoleLogs::default()),
            gas: Some(GasInspector::default()),
        }
    }
}

impl<DB: Database> Inspector<DB> for ArbiterInspector {
    #[inline]
    fn initialize_interp(&mut self, interp: &mut Interpreter, context: &mut EvmContext<'_, DB>) {
        if let Some(gas) = &mut self.gas {
            gas.initialize_interp(interp, context);
        }
    }

    #[inline]
    fn step_end(&mut self, interp: &mut Interpreter, context: &mut EvmContext<'_, DB>) {
        if let Some(gas) = &mut self.gas {
            gas.step_end(interp, context);
        }
    }

    #[inline]
    fn call(
        &mut self,
        context: &mut EvmContext<'_, DB>,
        inputs: &mut CallInputs,
    ) -> Option<(InterpreterResult, Range<usize>)> {
        if let Some(console_log) = &mut self.console_log {
            console_log.call(context, inputs)
        } else {
            None
        }
    }

    #[inline]
    fn call_end(
        &mut self,
        context: &mut EvmContext<'_, DB>,
        result: InterpreterResult,
    ) -> InterpreterResult {
        if let Some(gas) = &mut self.gas {
            gas.call_end(context, result)
        } else {
            result
        }
    }

    #[inline]
    fn create_end(
        &mut self,
        _context: &mut EvmContext<'_, DB>,
        result: InterpreterResult,
        address: Option<Address>,
    ) -> (InterpreterResult, Option<Address>) {
        (result, address)
    }
}
