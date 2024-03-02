//! The [`environment`] module provides abstractions and functionality for
//! handling the Ethereum execution environment. This includes managing its
//! state, interfacing with the EVM, and broadcasting events to subscribers.
//! Other features include the ability to control block rate and gas settings
//! and execute other database modifications from external agents.
//!
//! The key integration for the environment is the Rust EVM [`revm`](https://github.com/bluealloy/revm).
//! This is an implementation of the EVM in Rust that we utilize for processing
//! raw smart contract bytecode.
//!
//! Core structures:
//! - [`Environment`]: Represents the Ethereum execution environment, allowing
//!   for its management (e.g., starting, stopping) and interfacing with agents.
//! - [`EnvironmentParameters`]: Parameters necessary for creating or modifying
//!  an [`Environment`].
//! - [`Instruction`]: Enum indicating the type of instruction that is being
//!   sent
//!  to the EVM.

use revm::{
    inspector_handle_register,
    primitives::{Env, HashMap},
};
use tokio::sync::broadcast::channel;

use super::*;
use crate::database::inspector::ArbiterInspector;

#[derive(Debug)]
pub struct Environment<'a> {
    /// The label used to define the [`Environment`].
    pub parameters: EnvironmentParameters,

    event_broadcaster: BroadcastSender<Event>,

    /// The [`EVM`] that is used as an execution environment and database for
    /// calls and transactions.
    pub(crate) evm: Arc<RwLock<Evm<'a, ArbiterInspector, ArbiterDB>>>,
}

/// Parameters to create [`Environment`]s with different settings.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EnvironmentParameters {
    /// The label used to define the [`Environment`].
    pub label: Option<String>,

    /// The gas limit for the blocks in the [`Environment`].
    pub gas_limit: Option<U256>,

    /// The contract size limit for the [`Environment`].
    pub contract_size_limit: Option<usize>,

    /// Enables inner contract logs to be printed to the console.
    pub console_logs: bool,

    /// Allows for turning off any gas payments for transactions so no inspector
    /// is needed.
    pub pay_gas: bool,
}

/// A builder for creating an [`Environment`].
///
/// This builder allows for the configuration of an [`Environment`] before it is
/// instantiated. It provides methods for setting the label, gas limit, contract
/// size limit, and a database for the [`Environment`].
pub struct EnvironmentBuilder {
    parameters: EnvironmentParameters,
    db: ArbiterDB,
}

impl EnvironmentBuilder {
    /// Builds and runs an [`Environment`] with the parameters set in the
    /// [`EnvironmentBuilder`].
    pub fn build<'a>(self) -> Environment<'a> {
        Environment::create(self.parameters, self.db)
    }

    /// Sets the label for the [`Environment`].
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.parameters.label = Some(label.into());
        self
    }

    /// Sets the gas limit for the [`Environment`].
    pub fn with_gas_limit(mut self, gas_limit: U256) -> Self {
        self.parameters.gas_limit = Some(gas_limit);
        self
    }

    /// Sets the contract size limit for the [`Environment`].
    pub fn with_contract_size_limit(mut self, contract_size_limit: usize) -> Self {
        self.parameters.contract_size_limit = Some(contract_size_limit);
        self
    }

    /// Sets the state for the [`Environment`]. This can come from a saved state
    /// of a simulation or a [`database::fork::Fork`].
    pub fn with_state(mut self, state: impl Into<CacheDB<EmptyDB>>) -> Self {
        self.db.state = Arc::new(RwLock::new(state.into()));
        self
    }

    /// Sets the logs for the [`Environment`]. This can come from a saved state
    /// of a simulation and can be useful for doing analysis.
    pub fn with_logs(mut self, logs: impl Into<std::collections::HashMap<U256, Vec<Log>>>) -> Self {
        self.db.logs = Arc::new(RwLock::new(logs.into()));
        self
    }

    /// Sets the entire database for the [`Environment`] including both the
    /// state and logs. This can come from the saved state of a simulation and
    /// can be useful for doing analysis.
    pub fn with_arbiter_db(mut self, db: ArbiterDB) -> Self {
        self.db = db;
        self
    }

    /// Enables inner contract logs to be printed to the console as `trace`
    /// level logs prepended with "Console logs: ".
    pub fn with_console_logs(mut self) -> Self {
        self.parameters.console_logs = true;
        self
    }

    /// Turns on gas payments for transactions so that the [`EVM`] will
    /// automatically pay for gas and revert if balance is not met by sender.
    pub fn with_pay_gas(mut self) -> Self {
        self.parameters.pay_gas = true;
        self
    }
}

impl<'a> Environment<'a> {
    /// Creates a new [`EnvironmentBuilder`] with default parameters that can be
    /// used to build an [`Environment`].
    pub fn builder() -> EnvironmentBuilder {
        EnvironmentBuilder {
            parameters: EnvironmentParameters::default(),
            db: ArbiterDB::default(),
        }
    }

    fn create(parameters: EnvironmentParameters, db: ArbiterDB) -> Self {
        let (event_broadcaster, _) = channel(512);
        let inspector = if parameters.console_logs || parameters.pay_gas {
            ArbiterInspector::new(parameters.console_logs, parameters.pay_gas)
        } else {
            ArbiterInspector::new(false, false)
        };
        let mut env = Env::default();
        env.cfg.limit_contract_code_size = parameters.contract_size_limit;
        env.block.gas_limit = parameters.gas_limit.unwrap_or(U256::MAX);
        let mut evm = Arc::new(RwLock::new(
            Evm::builder()
                .with_db(db.clone())
                .with_env(Box::new(env))
                .with_external_context(inspector)
                .append_handler_register(inspector_handle_register)
                .build(),
        ));

        Self {
            parameters,
            event_broadcaster,
            evm,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub logs: Vec<Log>,
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) const TEST_ENV_LABEL: &str = "test";
    const TEST_CONTRACT_SIZE_LIMIT: usize = 42069;
    const TEST_GAS_LIMIT: u64 = 1_333_333_333_337;

    #[test]
    fn new_with_parameters() {
        let environment = Environment::builder()
            .with_label(TEST_ENV_LABEL)
            .with_contract_size_limit(TEST_CONTRACT_SIZE_LIMIT)
            .with_gas_limit(U256::from(TEST_GAS_LIMIT));
        assert_eq!(environment.parameters.label, Some(TEST_ENV_LABEL.into()));
        assert_eq!(
            environment.parameters.contract_size_limit.unwrap(),
            TEST_CONTRACT_SIZE_LIMIT
        );
        assert_eq!(
            environment.parameters.gas_limit.unwrap(),
            U256::from(TEST_GAS_LIMIT)
        );
    }
}
