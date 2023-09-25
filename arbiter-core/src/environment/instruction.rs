use super::*;

/// [`Instruction`]s that can be sent to the [`Environment`] via the
/// [`Socket`].
/// These instructions can be:
/// - [`Instruction::AddAccount`],
/// - [`Instruction::BlockUpdate`],
/// - [`Instruction::Call`],
/// - [`Instruction::Transaction`],
/// - [`Instruction::Query`].
/// The [`Instruction`]s are sent to the [`Environment`] via the
/// [`Socket::instruction_sender`] and the results are received via the
/// [`crate::middleware::Connection::outcome_receiver`].
#[derive(Debug, Clone)]
pub(crate) enum Instruction {
    /// An `AddAccount` is used to add a default/unfunded account to the
    /// [`EVM`].
    AddAccount {
        /// The address of the account to add to the [`EVM`].
        address: ethers::types::Address,

        /// The sender used to to send the outcome of the account addition back
        /// to.
        outcome_sender: OutcomeSender,
    },

    /// A `BlockUpdate` is used to update the block number and timestamp of the
    /// [`EVM`].
    BlockUpdate {
        /// The block number to update the [`EVM`] to.
        block_number: U256,

        /// The block timestamp to update the [`EVM`] to.
        block_timestamp: U256,

        /// The sender used to to send the outcome of the block update back to.
        outcome_sender: OutcomeSender,
    },

    /// A `Deal` is used to increase the balance of an account in the [`EVM`].
    Deal {
        /// The address of the account to increase the balance of.
        address: ethers::types::Address,

        /// The amount to increase the balance of the account by.
        amount: ethers::types::U256,

        /// The sender used to to send the outcome of the deal back to.
        outcome_sender: OutcomeSender,
    },

    /// A `Call` is processed by the [`EVM`] but will not be state changing and
    /// will not create events.
    Call {
        /// The transaction environment for the call.
        tx_env: TxEnv,

        /// The sender used to to send the outcome of the call back to.
        outcome_sender: OutcomeSender,
    },

    /// A `SetGasPrice` is used to set the gas price of the [`EVM`].
    SetGasPrice {
        /// The gas price to set the [`EVM`] to.
        gas_price: ethers::types::U256,

        /// The sender used to to send the outcome of the gas price setting back
        /// to.
        outcome_sender: OutcomeSender,
    },

    /// A `Transaction` is processed by the [`EVM`] and will be state changing
    /// and will create events.
    Transaction {
        /// The transaction environment for the transaction.
        tx_env: TxEnv,

        /// The sender used to to send the outcome of the transaction back to.
        outcome_sender: OutcomeSender,
    },

    /// A `Query` is used to query the [`EVM`] for some data, the choice of
    /// which data is specified by the inner `EnvironmentData` enum.
    Query {
        /// The data to query the [`EVM`] for.
        environment_data: EnvironmentData,

        /// The sender used to to send the outcome of the query back to.
        outcome_sender: OutcomeSender,
    },
    Stop(OutcomeSender),
}

/// [`Outcome`]s that can be sent back to the the client via the
/// [`Socket`].
/// These outcomes can be from `Call`, `Transaction`, or `BlockUpdate`
/// instructions sent to the [`Environment`]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Outcome {
    /// The outcome of an [`Instruction::AddAccount`] instruction that is used
    /// to signify that the account was added successfully.
    AddAccountCompleted,

    /// The outcome of a `BlockUpdate` instruction that is used to provide a
    /// non-error output of updating the block number and timestamp of the
    /// [`EVM`] to the client.
    BlockUpdateCompleted(ReceiptData),

    /// The outcome of a [`Instruction::Deal`] instruction that is used to
    /// signify that increasing the balance of an account was successful.
    DealCompleted,

    /// The outcome of a `Call` instruction that is used to provide the output
    /// of some [`EVM`] computation to the client.
    CallCompleted(ExecutionResult),

    /// The outcome of a [`Instruction::SetGasPrice`] instruction that is used
    /// to signify that the gas price was set successfully.
    SetGasPriceCompleted,

    /// The outcome of a `Transaction` instruction that is first unpacked to see
    /// if the result is successful, then it can be used to build a
    /// `TransactionReceipt` in the `Middleware`.
    TransactionCompleted(ExecutionResult, ReceiptData),

    /// The outcome of a `Query` instruction that carries a `String`
    /// representation of the data. Currently this may carry the block
    /// number, block timestamp, gas price, or balance of an account.
    QueryReturn(String),

    StopCompleted,
}

/// [`EnvironmentData`] is an enum used inside of the [`Instruction::Query`] to
/// specify what data should be returned to the user.
/// Currently this may be the block number, block timestamp, gas price, or
/// balance of an account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) enum EnvironmentData {
    /// The query is for the block number of the [`EVM`].
    BlockNumber,

    /// The query is for the block timestamp of the [`EVM`].
    BlockTimestamp,

    /// The query is for the gas price of the [`EVM`].
    GasPrice,

    /// The query is for the balance of an account given by the inner `Address`.
    Balance(ethers::types::Address),
}

/// [`ReceiptData`] is a structure that holds the block number, transaction
/// index, and cumulative gas used per block for a transaction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReceiptData {
    /// `block_number` is the number of the block in which the transaction was
    /// included.
    pub(crate) block_number: U64,
    /// `transaction_index` is the index position of the transaction in the
    /// block.
    pub(crate) transaction_index: U64,
    /// [`cumulative_gas_per_block`] is the total amount of gas used in the
    /// block up until and including the transaction.
    pub(crate) cumulative_gas_per_block: U256,
}
