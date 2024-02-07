//! Errors that can occur when managing or interfacing with Arbiter's sandboxed
//! Ethereum environment.
use std::convert::Infallible;

use ethers::providers::{MiddlewareError, ProviderError};
use revm_primitives::{EVMError, InvalidTransaction};
use thiserror::Error;

use self::environment::Broadcast;

use super::*;

/// Errors that can occur when managing or interfacing with the Ethereum
/// environment.
///
/// ## What are we trying to catch?
/// The errors here are at a fairly low level and should be quite rare (if
/// possible). Errors that come from smart contracts (e.g., reverts or halts)
/// will not be caught here and will instead carried out into the
/// [`RevmMiddleware`]. Please bring up if you catch errors here by sending a
/// message in the [Telegram group](https://t.me/arbiter_rs) or on
/// [GitHub](https://github.com/primitivefinance/arbiter/).
#[derive(Error, Debug, Clone)]
pub enum EnvironmentError {
    /// [`EnvironmentError::Execution`] is thrown when the [`EVM`] itself
    /// throws an error in execution. To be clear, this is not a contract
    /// revert or halt, this is likely an error in `revm`. Please report
    /// this type of error.
    #[error(transparent)]
    Execution(#[from] EVMError<Infallible>),

    /// [`EnvironmentError::Transaction`] is thrown when a transaction fails
    /// to be processed by the [`EVM`]. This could be due to a insufficient
    /// funds to pay for gas, an invalid nonce, or other reasons. This error
    /// can be quite common and should be handled gracefully.
    #[error("transaction error! the source error is: {0:?}")]
    Transaction(InvalidTransaction),

    /// [`EnvironmentError::Account`] is thrown when there is an issue handling
    /// accounts in the [`EVM`]. This could be due to an account already
    /// existing or other reasons.
    #[error("account error! due to: {0:?}")]
    Account(String),

    /// [`EnvironmentError::Stop`] is thrown when the [`Environment`]
    /// fails to stop. This error could occur due to an invalid state transition
    /// or other unexpected conditions. If this error is thrown, it indicates
    /// a serious issue that needs to be investigated. Please report this error!
    #[error("error stopping! due to: {0:?}")]
    Stop(String),

    /// [`EnvironmentError::Communication`] is thrown when a channel for
    /// receiving or broadcasting fails in some way. This error could happen
    /// due to a channel being closed accidentally. If this is thrown, a
    /// restart of the simulation and an investigation into what caused a
    /// dropped channel is necessary.
    #[error("error communicating! due to: {0}")]
    Communication(String),

    /// [`EnvironmentError::Broadcast`] is thrown when the
    /// [`EventBroadcaster`] fails to broadcast events. This should be
    /// rare (if not impossible). If this is thrown, please report this error!
    #[error(transparent)]
    Broadcast(#[from] crossbeam_channel::SendError<Broadcast>),

    /// [`EnvironmentError::Conversion`] is thrown when a type fails to
    /// convert into another (typically a type used in `revm` versus a type used
    /// in [`ethers-rs`](https://github.com/gakonst/ethers-rs)).
    /// This error should be rare (if not impossible).
    /// Furthermore, after a switch to [`alloy`](https://github.com/alloy-rs)
    /// this will be (hopefully) unnecessary!
    #[error("conversion error! the source error is: {0}")]
    Conversion(String),

    /// [`EnvironmentError::ShutDownReceiverError`] is thrown when a malformed
    /// shutdown receiver is sent to the event broadcaster. This error could
    /// occur due to an invalid shutdown receiver.
    #[error("error in the environment! malformed shutdown receiver sent to event broadcaster")]
    ShutDownReceiverError,
}

/// Possible errors thrown by interacting with the revm middleware client.
/// Errors that can occur while using the [`RevmMiddleware`].
/// These errors are likely to be more common than other errors in
/// `arbiter-core` as they can come from simple issues such as contract reverts
/// or halts. Certain errors such as [`RevmMiddlewareError::Send`],
/// [`RevmMiddlewareError::Receive`], [`RevmMiddlewareError::Conversion`],
/// [`RevmMiddlewareError::Json`], and [`RevmMiddlewareError::EventBroadcaster`]
/// are considered more worrying. If these are achieved, please feel free to
/// contact our team via the [Telegram group](https://t.me/arbiter_rs) or on
/// [GitHub](https://github.com/primitivefinance/arbiter/).
#[derive(Error, Debug)]
pub enum RevmMiddlewareError {
    /// An error occurred while attempting to interact with the [`Environment`].
    #[error("an error came from the environment! due to: {0}")]
    Environment(#[from] EnvironmentError),

    /// An error occurred while attempting to interact with the provider:
    /// [`Connection`].
    #[error("an error came from the provider! due to: {0}")]
    Provider(#[from] ProviderError),

    /// An error occurred while attempting to send a transaction.
    #[error("failed to send transaction! due to: {0}")]
    Send(String),

    /// There was an issue receiving an [`ExecutionResult`], possibly from
    /// another service or module.
    #[error("failed to receive `ExecutionResult`! due to: {0}")]
    Receive(#[from] crossbeam_channel::RecvError),

    /// There was a failure trying to obtain a lock on the [`EventBroadcaster`],
    /// possibly due to concurrency issues.
    #[error("failed to gain event broadcaster lock! due to: {0}")]
    EventBroadcaster(String),

    /// The required data or functionality for an instruction was missing or
    /// incomplete.
    #[error("missing data! due to: {0}")]
    MissingData(String),

    /// An error occurred during type conversion, possibly when translating
    /// between domain-specific types.
    #[error("failed to convert types! due to: {0}")]
    Conversion(String),

    /// An error occurred while trying to serialize or deserialize JSON data.
    #[error("failed to handle with JSON data! due to: {0:?}")]
    Json(serde_json::Error),

    /// The execution of a transaction was reverted, indicating that the
    /// transaction was not successful.
    #[error("execution failed to succeed due to revert!\n gas used is: {gas_used}\n output is {output:?}")]
    ExecutionRevert {
        /// Provides the amount of gas used by the transaction.
        gas_used: u64,

        /// Provides the output or reason why the transaction was reverted.
        output: revm::primitives::Bytes,
    },

    /// The execution of a transaction halted unexpectedly.
    #[error("execution failed to succeed due to halt!\n reason is: {reason:?}\n gas used is: {gas_used}")]
    ExecutionHalt {
        /// Provides the reason for the halt.
        reason: revm::primitives::Halt,

        /// Provides the amount of gas used by the transaction.
        gas_used: u64,
    },

    /// There was an error with a signature.
    #[error("signature error! due to: {0}")]
    Signing(String),
}

impl MiddlewareError for RevmMiddlewareError {
    type Inner = ProviderError;

    fn from_err(e: Self::Inner) -> Self {
        RevmMiddlewareError::Provider(e)
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        None
    }
}
