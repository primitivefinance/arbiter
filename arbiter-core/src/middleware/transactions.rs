use revm::primitives::{ExecutionResult, Output};

/// ! Unwraps the result of the EVM execution into a more structured `Success`
/// type.
use super::cast::revm_logs_to_ethers_logs;
use super::errors::RevmMiddlewareError;

/// Contains the result of a successful transaction execution.
#[derive(Debug)]
pub struct Success {
    pub _reason: revm::primitives::Eval,
    pub _gas_used: u64,
    pub _gas_refunded: u64,
    pub logs: Vec<ethers::types::Log>,
    pub output: Output,
}

/// Unpacks the result of the EVM execution.
///
/// This function converts the raw execution result from the EVM into a more
/// structured [`Success`] type or an error indicating the failure of the
/// execution.
pub fn unpack_execution_result(
    execution_result: ExecutionResult,
) -> Result<Success, RevmMiddlewareError> {
    match execution_result {
        ExecutionResult::Success {
            reason,
            gas_used,
            gas_refunded,
            logs,
            output,
        } => {
            let logs = revm_logs_to_ethers_logs(logs);
            Ok(Success {
                _reason: reason,
                _gas_used: gas_used,
                _gas_refunded: gas_refunded,
                logs,
                output,
            })
        }
        ExecutionResult::Revert { gas_used, output } => {
            Err(RevmMiddlewareError::ExecutionRevert { gas_used, output })
        }
        ExecutionResult::Halt { reason, gas_used } => {
            Err(RevmMiddlewareError::ExecutionHalt { reason, gas_used })
        }
    }
}
