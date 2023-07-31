#![warn(missing_docs)]
//! Module for utility functionality.
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use bytes::Bytes;
use ethers::{
    prelude::Address,
    types::{H256, U64},
};
use revm::primitives::{ExecutionResult, Output, B160, B256, U256};

#[derive(Debug)]
// We should use anyhow / thisError instead
/// Error type for the simulation manager.
/// # Fields
/// * `message` - Error message.
/// * `output` - Byte output of the error.
pub struct UnpackError {
    /// Error message.
    pub message: String,
    /// Byte output of the error.
    pub output: Option<Bytes>,
}

impl Error for UnpackError {}

impl Display for UnpackError {
    /// Display the error message.
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message)
    }
}

/// Recast a logs from Revm into the ethers.rs Log type.
/// # Arguments
/// * `revm_logs` - Logs from Revm. (Vec<revm::primitives::Log>)
/// # Returns
/// * `Vec<ethers::core::types::Log>` - Logs recasted into ethers.rs Log type.
pub fn revm_logs_to_ethers_logs(
    revm_logs: Vec<revm::primitives::Log>,
) -> Vec<ethers::core::types::Log> {
    let mut logs: Vec<ethers::core::types::Log> = vec![];
    for revm_log in revm_logs {
        let topics = revm_log.topics.into_iter().map(recast_b256).collect();
        let log = ethers::core::types::Log {
            address: recast_address(revm_log.address),
            topics,
            data: ethers::core::types::Bytes::from(revm_log.data),
            block_hash: None,
            block_number: None,
            transaction_hash: None,
            transaction_index: None,
            log_index: None,
            transaction_log_index: None,
            log_type: None,
            removed: None,
        };
        logs.push(log);
    }
    logs
}

// Certainly will go away with alloy-types
/// Recast a B160 into an Address type
/// # Arguments
/// * `address` - B160 to recast. (B160)
/// # Returns
/// * `Address` - Recasted Address.
pub fn recast_address(address: B160) -> Address {
    let temp: [u8; 20] = address.as_bytes().try_into().unwrap();
    Address::from(temp)
}

/// Recast a B256 into an H256 type
/// # Arguments
/// * `input` - B256 to recast. (B256)  
/// # Returns
/// * `H256` - Recasted H256.
pub fn recast_b256(input: B256) -> H256 {
    let temp: [u8; 32] = input.as_bytes().try_into().unwrap();
    H256::from(temp)
}
// TODO: Can maybe get rid of this with middleware
/// Takes an `ExecutionResult` and returns the raw bytes of the output that can then be decoded.
/// # Arguments
/// * `execution_result` - The `ExecutionResult` that we want to unpack.
/// # Returns
/// * `Ok(Bytes)` - The raw bytes of the output.
pub fn unpack_execution(execution_result: ExecutionResult) -> Result<Bytes, UnpackError> {
    match execution_result {
        ExecutionResult::Success { output, .. } => match output {
            Output::Call(value) => Ok(value),
            Output::Create(value, _address) => Ok(value),
        },
        ExecutionResult::Halt { reason, gas_used } => Err(UnpackError {
            message: format!(
                "This call halted for {:#?} and used {} gas.",
                reason, gas_used
            ),
            output: None,
        }),
        ExecutionResult::Revert { output, gas_used } => Err(UnpackError {
            message: format!(
                "This call reverted with output {:#?} and used {} gas.",
                output, gas_used
            ),
            output: Some(output),
        }),
    }
}

/// Convert a U256 to a U64, discarding the higher bits if the number is larger than 2^64
/// # Arguments
/// * `input` - The U256 to convert.
/// # Returns
/// * `Ok(U64)` - The converted U64.
/// Used for block number which is a U64.
pub fn convert_uint_to_u64(input: U256) -> Result<U64, &'static str> {
    // Convert to base 2^64 digits
    let digits: Vec<_> = input.to_base_le(2u64.pow(64)).collect();

    // If there are no digits, the value was 0
    if digits.is_empty() {
        return Ok(U64::from(0));
    }

    // Otherwise, return the least significant 64 bits
    // If the number was larger than 2^64, this will silently discard the higher bits
    Ok(U64::from(digits[0]))
}
