//! Errors that can occur when managing or interfacing with Arbiter's sandboxed
//! Ethereum environment.

use std::any::Any;

// use crossbeam_channel::SendError;
use crossbeam_channel::{RecvError, SendError};
use ethers::{
    providers::{MiddlewareError, ProviderError},
    signers::WalletError,
};
use revm_primitives::{EVMError, HaltReason};
use thiserror::Error;

use self::environment::instruction::{Instruction, Outcome};
use super::*;

#[derive(Error, Debug)]
pub enum ArbiterCoreError {
    #[error("Account already exists!")]
    AccountCreationError,

    #[error("Account doesn't exist!")]
    AccountDoesNotExistError,

    #[error("Can't sign with a forked EOA!")]
    ForkedEOASignError,

    #[error("Failed to upgrade sender to a strong reference!")]
    UpgradeSenderError,

    #[error("Data missing when calling a transaction!")]
    MissingDataError,

    #[error("Invalid `get_balance()` request!")]
    InvalidQueryError,

    #[error("Failed to join environment thread on stop!")]
    JoinError,

    #[error("Execution failed with revert: {gas_used:?} gas used, {output:?}")]
    ExecutionRevert { gas_used: u64, output: Vec<u8> },

    #[error("Execution failed with halt: {reason:?}, {gas_used:?} gas used")]
    ExecutionHalt { reason: HaltReason, gas_used: u64 },

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    EVMError(#[from] EVMError<Infallible>),

    #[error(transparent)]
    ProviderError(#[from] ProviderError),

    #[error(transparent)]
    WalletError(#[from] WalletError),

    #[error(transparent)]
    SendError(#[from] SendError<Instruction>),

    #[error(transparent)]
    RecvError(#[from] RecvError),

    #[error(transparent)]
    FromStrRadixError(#[from] uint::FromStrRadixErr),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("{0}")]
    ReplyError(String),
}

impl From<SendError<Result<Outcome, ArbiterCoreError>>> for ArbiterCoreError {
    fn from(e: SendError<Result<Outcome, ArbiterCoreError>>) -> Self {
        ArbiterCoreError::ReplyError(e.to_string())
    }
}

impl MiddlewareError for ArbiterCoreError {
    type Inner = ProviderError;

    fn from_err(e: Self::Inner) -> Self {
        ArbiterCoreError::from(e)
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        None
    }
}
