//! Cheatcodes are a direct way to access the underlying [`EVM`] environment
// and database. ! Use them via the `apply_cheatcode` method on a `client`.

#![warn(missing_docs)]

use revm_primitives::{alloy_primitives, AccountInfo, HashMap, U256};

/// Cheatcodes are a direct way to access the underlying [`EVM`] environment and
/// database.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Cheatcodes {
    /// A `Deal` is used to increase the balance of an account in the [`EVM`].
    Deal {
        /// The address of the account to increase the balance of.
        address: alloy_primitives::Address,

        /// The amount to increase the balance of the account by.
        amount: alloy_primitives::U256,
    },
    /// Fetches the value of a storage slot of an account.
    Load {
        /// The address of the account to fetch the storage slot from.
        account: alloy_primitives::Address,
        /// The storage slot to fetch.
        key: alloy_primitives::B256,

        /// The block to fetch the storage slot from.
        /// todo: implement storage slots at blocks.
        block: Option<ethers::types::BlockId>,
    },
    /// Overwrites a storage slot of an account.
    /// TODO: for more complicated data types, like structs, there's more work
    /// to do.
    Store {
        /// The address of the account to overwrite the storage slot of.
        account: alloy_primitives::Address,
        /// The storage slot to overwrite.
        key: alloy_primitives::B256,
        /// The value to overwrite the storage slot with.
        value: alloy_primitives::B256,
    },
    /// Fetches the `DbAccount` account at the given address.
    Access {
        /// The address of the account to fetch.
        address: alloy_primitives::Address,
    },
}

/// Wrapper around [`AccountState`] that can be serialized and deserialized.
#[derive(Debug, Clone, Default, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AccountStateSerializable {
    /// Before Spurious Dragon hardfork there was a difference between empty and
    /// not existing. And we are flagging it here.
    NotExisting,
    /// EVM touched this account. For newer hardfork this means it can be
    /// cleared/removed from state.
    Touched,
    /// EVM cleared storage of this account, mostly by selfdestruct, we don't
    /// ask database for storage slots and assume they are U256::ZERO
    StorageCleared,
    /// EVM didn't interacted with this account
    #[default]
    None,
}

/// Return values of applying cheatcodes.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CheatcodesReturn {
    /// A `Load` returns the value of a storage slot of an account.
    Load {
        /// The value of the storage slot.
        value: revm_primitives::alloy_primitives::U256,
    },
    /// A `Store` returns nothing.
    Store,
    /// A `Deal` returns nothing.
    Deal,
    /// Gets the DbAccount associated with an address.
    Access {
        /// Basic account information like nonce, balance, code hash, bytcode.
        info: AccountInfo,
        /// todo: revm must be updated with serde deserialize, then `DbAccount`
        /// can be used.
        account_state: AccountStateSerializable,
        /// Storage slots of the account.
        storage: HashMap<U256, U256>,
    },
}
