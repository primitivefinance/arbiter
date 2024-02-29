//! The [`Coprocessor`] is used to process calls and can access read-only from
//! the [`Environment`]'s database while staying up to date with the
//! latest state of the [`Environment`]'s database.

use std::convert::Infallible;

use revm::inspector_handle_register;
use revm::primitives::{EVMError, ResultAndState};

use self::database::inspector::ArbiterInspector;

use super::*;
use crate::environment::Environment;

/// A [`Coprocessor`] is used to process calls and can access read-only from the
/// [`Environment`]'s database. This can eventually be used for things like
/// parallelized compute for agents that are not currently sending transactions
/// that need to be processed by the [`Environment`], but are instead using the
/// current state to make decisions.
#[derive(Debug)]
pub struct Coprocessor<'a> {
    pub(crate) evm: Evm<'a, ArbiterInspector, ArbiterDB>,
}

impl<'a> Coprocessor<'a> {
    /// Create a new `Coprocessor` with the given `Environment`.
    pub fn new(environment: &Environment) -> Self {
        let inspector = ArbiterInspector::new(true, false);
        let db = environment.db.clone();
        let evm = Evm::builder()
            .with_db(db)
            .with_external_context(inspector)
            .append_handler_register(inspector_handle_register)
            .build();
        Self { evm }
    }

    /// Used as an entrypoint to process a call with the `Coprocessor`.
    pub fn transact(&mut self, tx_env: TxEnv) -> Result<ResultAndState, EVMError<Infallible>> {
        *self.evm.tx_mut() = tx_env;
        self.evm.transact()
    }
}

unsafe impl Sync for Coprocessor<'_> {}
unsafe impl Send for Coprocessor<'_> {}

#[cfg(test)]
mod tests {
    use revm::primitives::{InvalidTransaction, U256};

    use super::*;

    #[test]
    fn coprocessor() {
        let environment = Environment::builder().build();
        let mut coprocessor = Coprocessor::new(&environment);
        let tx_env = TxEnv {
            value: U256::from(100),
            ..Default::default()
        };
        let outcome = coprocessor.transact(tx_env);
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
