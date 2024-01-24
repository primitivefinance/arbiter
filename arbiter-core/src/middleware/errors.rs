use super::*;

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
    Environment(#[from] crate::environment::errors::EnvironmentError),

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
