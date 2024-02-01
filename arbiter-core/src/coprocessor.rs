//! The `Coprocessor` is used to process calls and can access read-only from the
//! `Environment`'s database. The `Coprocessor` will stay up to date with the
//! latest state of the `Environment`'s database.

use std::convert::Infallible;

use revm::EVM;
use revm_primitives::{EVMError, ResultAndState};

use crate::{database::ArbiterDB, environment::Environment};

// TODO: I have not tested that the coprocessor actually maintains state parity
// with the environment. At the moment, it is only able to be constructed and
// can certainly act as a read-only EVM.

/// A `Coprocessor` is used to process calls and can access read-only from the
/// `Environment`'s database. This can eventually be used for things like
/// parallelized compute for agents that are not currently sending transactions
/// that need to be processed by the `Environment`, but are instead using the
/// current state to make decisions.
pub struct Coprocessor {
    evm: EVM<ArbiterDB>,
}

impl Coprocessor {
    /// Create a new `Coprocessor` with the given `Environment`.
    pub fn new(environment: &Environment) -> Self {
        let db = ArbiterDB(
            environment
                .db
                .as_ref()
                .unwrap_or(&ArbiterDB::new())
                .0
                .clone(),
        );
        let mut evm = EVM::new();
        evm.database(db);
        Self { evm }
    }

    /// Used as an entrypoint to process a call with the `Coprocessor`.
    pub fn transact_ref(&self) -> Result<ResultAndState, EVMError<Infallible>> {
        self.evm.transact_ref()
    }
}

#[cfg(test)]
mod tests {
    use revm_primitives::{InvalidTransaction, U256};

    use super::*;

    #[test]
    fn coprocessor() {
        let environment = Environment::builder().build();
        let mut coprocessor = Coprocessor::new(&environment);
        coprocessor.evm.env.tx.value = U256::from(100);
        let outcome = coprocessor.transact_ref();
        if let Err(EVMError::Transaction(InvalidTransaction::LackOfFundForMaxFee {
            fee,
            balance,
        })) = outcome
        {
            assert_eq!(*fee, U256::from(100));
            assert_eq!(*balance, U256::from(0));
        }
    }
}
