use std::convert::Infallible;

use revm::EVM;
use revm_primitives::{EVMError, ResultAndState};

use crate::environment::{ArbiterDB, Environment};

pub struct Coprocessor {
    evm: EVM<ArbiterDB>,
}

impl Coprocessor {
    pub fn new(environment: &Environment) -> Self {
        let db = environment.db.as_ref().unwrap().clone();
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
    use super::*;
    use crate::environment::builder::EnvironmentBuilder;

    #[test]
    fn coprocessor() {
        let environment = EnvironmentBuilder::new().build();
        let coprocessor = Coprocessor::new(&environment);
    }
}
