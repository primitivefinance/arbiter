use std::convert::Infallible;

use revm::EVM;
use revm_primitives::{EVMError, ResultAndState};

use crate::{database::ArbiterDB, environment::Environment};

pub struct Coprocessor {
    evm: EVM<ArbiterDB>,
}

impl Coprocessor {
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

    pub fn transact_ref(&self) -> Result<ResultAndState, EVMError<Infallible>> {
        self.evm.transact_ref()
    }
}

#[cfg(test)]
mod tests {
    use revm_primitives::{InvalidTransaction, U256};

    use super::*;
    use crate::environment::builder::EnvironmentBuilder;

    #[test]
    fn coprocessor() {
        let environment = EnvironmentBuilder::new().build();
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
