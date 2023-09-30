//! Cheatcodes are a direct way to access the underlying [`EVM`] environment
// and database. ! Use them via the `apply_cheatcode` method on a `client`.

/// Cheatcodes are a direct way to access the underlying [`EVM`] environment and
/// database.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Cheatcodes {
    /// A `Deal` is used to increase the balance of an account in the [`EVM`].
    Deal {
        /// The address of the account to increase the balance of.
        address: ethers::types::Address,

        /// The amount to increase the balance of the account by.
        amount: ethers::types::U256,
    },
    /// Fetches the value of a storage slot of an account.
    Load {
        /// The address of the account to fetch the storage slot from.
        account: ethers::types::Address,
        /// The storage slot to fetch.
        key: ethers::types::H256,
        /// The block to fetch the storage slot from.
        /// todo: implement storage slots at blocks.
        block: Option<ethers::types::BlockId>,
    },
    /// Overwrites a storage slot of an account.
    /// todo: for more complicated data types, like structs, there's more work
    /// to do.
    Store {
        /// The address of the account to overwrite the storage slot of.
        account: ethers::types::Address,
        /// The storage slot to overwrite.
        key: ethers::types::H256,
        /// The value to overwrite the storage slot with.
        value: ethers::types::H256,
    },
}

/// Return values of applying cheatcodes.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CheatcodesReturn {
    /// A `Load` returns the value of a storage slot of an account.
    Load {
        /// The value of the storage slot.
        value: revm::primitives::U256,
    },
    /// A `Store` returns nothing.
    Store,
    /// A `Deal` returns nothing.
    Deal,
}
