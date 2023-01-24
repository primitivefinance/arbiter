//! error handling and support for clairvoyance
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ClairvoyanceError {
    // When a token string as specified by the input does not exist in arbiter
    TokenDoesNotExist {
        token_name: String,
    },
    FeeTierDoesNotExist {
        bp: u32,
    },
    PoolDoesNotExist {
        token_0_name: String,
        token_1_name: String,
        bp: u32,
    },
}

impl Display for ClairvoyanceError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ClairvoyanceError::TokenDoesNotExist{ token_name } =>
                write!(f, "\n\nErrorType : TokenDoesNotExist\nError Message : \"{token_name}\" is not a valid token that exists in arbiter!\n\n"),
            ClairvoyanceError::FeeTierDoesNotExist{ bp } =>
                write!(f, "\n\nError Type : FeeTierDoesNotExist\nError Message : {bp} is not a basis point that exists on Uniswap Pools!\n\t\tThe valid basis points are 1, 5, 30, 100.\n\n"),
            ClairvoyanceError::PoolDoesNotExist{ token_0_name, token_1_name, bp } =>
                write!(f, "\n\nError Type : PoolDoesNotExist\nError Message : ({token_0_name}, {token_1_name}, {bp}), is not a valid pool that exists on Uniswap!\n\n"),
        }
    }
}
