#![warn(missing_docs)]
//! Module for utility functionality.
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use bytes::Bytes;
use ethers::prelude::{Address, U256};
use revm::primitives::{ExecutionResult, Output, B160};

#[derive(Debug)]
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

/// Recast a B160 into an Address type
/// # Arguments
/// * `address` - B160 to recast. (B160)
/// # Returns
/// * `Address` - Recasted Address.
pub fn recast_address(address: B160) -> Address {
    let temp: [u8; 20] = address.as_bytes().try_into().unwrap();
    Address::from(temp)
}

/// Converts a float to a WAD fixed point prepared U256 number.
/// # Arguments
/// * `x` - Float to convert. (f64)
/// # Returns
/// * `U256` - Converted U256 number.
pub fn float_to_wad(x: f64) -> U256 {
    U256::from((x * 1e18) as u128)
}

/// Converts a float to a WAD fixed point prepared U256 number.
/// # Arguments
/// * `x` - WAD to convert. (U256)
/// # Returns
/// * `f64` - Converted f64 number.
pub fn wad_to_float(x: U256) -> f64 {
    x.as_u128() as f64 / 1e18
}

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
