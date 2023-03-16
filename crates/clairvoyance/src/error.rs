//! Error handling for clairvoyance.

use ethers::{prelude::*, providers::Provider};
use thiserror::Error;

/// Error enumeration type for the `uniswap` module of `clairvoyance`.
#[derive(Debug, Error)]
#[deprecated(since = "0.0.1", note = "will be useful for actors in the future")]
#[allow(warnings)]
pub enum UniswapError {
    /// Error occured when fetching data from the Uniswap contract.
    #[error(transparent)]
    ContractInteractionError(#[from] ContractError<Provider<Http>>),

    /// Error occured when trying to fetch a pool with an invalid fee tier.
    #[error("the specified fee tier does not exist")]
    FeeTierError,

    /// Error occured when attempting to fetch an invalid pool.
    #[error("a pool with the given parameters does not exist")]
    PoolError,

    /// Error occured when attempting to fetch an invalid token.
    #[error("a token with the given parameters does not exist")]
    TokenError,
}
