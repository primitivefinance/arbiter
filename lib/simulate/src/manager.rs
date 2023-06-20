#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.

use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use bindings::{arbiter_math, arbiter_token, liquid_exchange, weth9};
use bytes::Bytes;
use crossbeam_channel::unbounded;
use ethers::{
    abi::{Token, Tokenize},
    types::U256,
};
use revm::primitives::{AccountInfo, Address, B160, U256 as rU256};

use crate::{
    agent::{
        simple_arbitrageur::SimpleArbitrageur, user::User, Agent, AgentType, IsActive,
        NotActive, TransactSettings,
    },
    environment::{
        contract::{IsDeployed, SimulationContract, NotDeployed},
        EventStream, SimulationEnvironment,
    },
    utils::recast_address,
};

#[derive(Clone, Debug)]
/// Error type for the simulation manager.
/// # Fields
/// * `message` - Error message.
/// * `output` - Byte output of the error.
pub struct ManagerError {
    /// Error message.
    pub message: String,
    /// Byte output of the error.
    pub output: Option<Bytes>,
}

impl Error for ManagerError {}

impl Display for ManagerError {
    /// Display the error message.
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message)
    }
}

/// Manages simulations.
/// # Fields
/// * `environment` - The simulation environment that the manager controls.
/// * `agents` - The agents that are currently running in the simulation environment.
pub struct SimulationManager {
    /// [`SimulationEnvironment`] that the simulation manager controls.
    pub environment: SimulationEnvironment,
    /// The agents that are currently running in the [`SimulationEnvironment`].
    pub agents: HashMap<String, AgentType<IsActive>>,
    /// The collection of different [`SimulationContract`] that are currently deployed in the [`SimulationEnvironment`].
    pub deployed_contracts: HashMap<String, SimulationContract<IsDeployed>>,
}

impl Default for SimulationManager {
    /// Constructor function to instantiate a manager that has a default admin user and a simulation environment.
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationManager {
    /// Constructor function to instantiate a manager that has a default admin user and a simulation environment.
    /// The admin will always be given the 0x0...1 address.
    pub fn new() -> Self {
        let mut simulation_manager = Self {
            environment: SimulationEnvironment::new(),
            agents: HashMap::new(),
            deployed_contracts: HashMap::new(),
        };
        let admin = AgentType::User(User::new("admin", None));
        simulation_manager
            .activate_agent(admin, B160::from_low_u64_be(1))
            .unwrap(); // This unwrap should never fail.
        simulation_manager.environment.run();
        simulation_manager.auto_deploy();

        simulation_manager
    }
    /// generic contract deploy function that takse a vector of contracts and constructor args
    pub fn _deploy_contracts(
        &mut self,
        admin: &AgentType<IsActive>,
        // could be replaced with a struct that holds the contract, args, and name
        contracts: Vec<(SimulationContract<NotDeployed>, Vec<Token>, String)>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (contract, args, name) in contracts {
            let (contract, result) = admin.deploy(contract, args)?;
            assert!(result.is_success());
            self.deployed_contracts.insert(name, contract);
        }
        Ok(())
    }
    /// Deploy all contracts that are needed for any simulation.
    fn auto_deploy(&mut self) {
        let arbiter_math = SimulationContract::new(
            arbiter_math::ARBITERMATH_ABI.clone(),
            arbiter_math::ARBITERMATH_BYTECODE.clone(),
        );
        let admin = self.agents.get("admin").unwrap();
        let (arbiter_math, result) = admin.deploy(arbiter_math, ().into_tokens()).unwrap();
        assert!(result.is_success());
        self.deployed_contracts
            .insert("arbiter_math".to_string(), arbiter_math);

        let decimals = 18_u8;
        let wad: U256 = U256::from(10_i64.pow(decimals as u32));

        let weth = SimulationContract::new(weth9::WETH9_ABI.clone(), weth9::WETH9_BYTECODE.clone());
        let (weth, result) = admin.deploy(weth, vec![]).unwrap();
        assert!(result.is_success());
        self.deployed_contracts.insert("weth".to_string(), weth);

        // Deploy Arbiter Tokens
        let arbiter_token = SimulationContract::new(
            arbiter_token::ARBITERTOKEN_ABI.clone(),
            arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
        );

        // Choose name and symbol and combine into the constructor args required by ERC-20 contracts.
        let arbx_args = ("ArbiterToken".to_string(), "ARBX".to_string(), decimals).into_tokens();
        let (arbiter_token_x, result) = admin.deploy(arbiter_token.clone(), arbx_args).unwrap();
        assert!(result.is_success());
        self.deployed_contracts
            .insert("arbiter_token_x".to_string(), arbiter_token_x.clone());

        let arby_args = ("ArbiterTokenY".to_string(), "ARBY".to_string(), decimals).into_tokens();
        let (arbiter_token_y, result) = admin.deploy(arbiter_token, arby_args).unwrap();
        assert!(result.is_success());
        self.deployed_contracts
            .insert("arbiter_token_y".to_string(), arbiter_token_y.clone());

        // Deploy LiquidExchange
        let initial_price: U256 = wad.checked_mul(U256::from(1)).unwrap();
        let liquid_exchange = SimulationContract::new(
            liquid_exchange::LIQUIDEXCHANGE_ABI.clone(),
            liquid_exchange::LIQUIDEXCHANGE_BYTECODE.clone(),
        );
        let le_args = (
            recast_address(arbiter_token_x.address),
            recast_address(arbiter_token_y.address),
            initial_price,
        )
            .into_tokens();
        let (liquid_exchange_xy, result) = admin.deploy(liquid_exchange, le_args).unwrap();
        assert!(result.is_success());
        self.deployed_contracts
            .insert("liquid_exchange_xy".to_string(), liquid_exchange_xy);
    }

    /// Adds and activates an agent to be put in the collection of agents under the manager's control.
    /// # Arguments
    /// * `new_agent` - The agent to be added to the collection of agents.
    /// * `new_agent_address` - The address that the agent will be given.
    pub fn activate_agent(
        &mut self,
        new_agent: AgentType<NotActive>,
        new_agent_address: Address,
    ) -> Result<(), ManagerError> {
        // Check to make sure we are not creating an agent with an address or name that already exists.
        if self
            .agents
            .values()
            .any(|agent_in_db| agent_in_db.inner().address() == new_agent_address)
        {
            return Err(ManagerError {
                message: "Agent with that address already exists in the simulation environment."
                    .to_string(),
                output: None,
            });
        };
        if self
            .agents
            .keys()
            .any(|name_in_db| *name_in_db == new_agent.inner().name())
        {
            return Err(ManagerError {
                message: "Agent with that name already exists in the simulation environment."
                    .to_string(),
                output: None,
            });
        };

        // Create the agent and add it to the simulation environment so long as we don't throw an error above.
        let (event_sender, event_receiver) = unbounded();
        self.environment
            .event_broadcaster
            .lock()
            .unwrap()
            .add_sender(event_sender);
        let account_info = AccountInfo::default();
        self.environment
            .evm
            .db()
            .unwrap()
            .insert_account_info(new_agent_address, account_info.clone());
        match new_agent {
            AgentType::User(user) => {
                let event_stream = EventStream {
                    receiver: event_receiver,
                    filters: user.event_filters.clone(),
                };
                let new_user = User::<IsActive> {
                    name: user.name,
                    address: new_agent_address,
                    account_info,
                    transact_settings: TransactSettings {
                        gas_limit: u64::MAX,    // TODO: Users should have a gas limit.
                        gas_price: rU256::ZERO, // TODO: Users should have an associated gas price.
                    },
                    event_stream,
                    event_filters: user.event_filters,
                    transaction_sender: self.environment.transaction_channel.0.clone(),
                    result_channel: crossbeam_channel::unbounded(),
                };
                self.agents
                    .insert(new_user.name.clone(), AgentType::User(new_user));
            }
            AgentType::SimpleArbitrageur(simple_arbitrageur) => {
                let event_stream = EventStream {
                    receiver: event_receiver,
                    filters: simple_arbitrageur.event_filters.clone(),
                };
                let new_simple_arbitrageur = SimpleArbitrageur::<IsActive> {
                    name: simple_arbitrageur.name,
                    address: new_agent_address,
                    account_info,
                    transact_settings: TransactSettings {
                        gas_limit: u64::MAX,    // TODO: Users should have a gas limit.
                        gas_price: rU256::ZERO, // TODO: Users should have an associated gas price.
                    },
                    event_stream,
                    event_filters: simple_arbitrageur.event_filters,
                    prices: simple_arbitrageur.prices,
                    transaction_sender: self.environment.transaction_channel.0.clone(),
                    result_channel: crossbeam_channel::unbounded(),
                };
                self.agents.insert(
                    new_simple_arbitrageur.name.clone(),
                    AgentType::SimpleArbitrageur(new_simple_arbitrageur),
                );
            }
        };
        Ok(())
    }
    /// Stops a simulation and drops the channels
    pub fn shutdown(self) {
        drop(self.environment.transaction_channel.1);
    }
}

#[test]
fn agent_address_collision() {
    let mut manager = SimulationManager::default();
    let alice = User::new("alice", None);
    let result = manager.activate_agent(AgentType::User(alice), B160::from_low_u64_be(1));
    assert!(result.is_err());
}
