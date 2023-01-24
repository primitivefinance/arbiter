//! Error handling for clairvoyance.

/// Error enumeration type for the `uniswap` module of `clairvoyance`.
#[derive(Debug, Error)]
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
}