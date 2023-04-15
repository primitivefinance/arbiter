#![warn(missing_docs)]
//! This module contains the `SimulationContract` struct that is used to wrap around the ethers `BaseContract` and add some additional information relevant for revm and the simulation.
use std::marker::PhantomData;

use bytes::Bytes;
use ethers::{
    abi::{Contract, Detokenize, Token, Tokenize},
    prelude::{BaseContract},
    types::{Bytes as EthersBytes, H256},
};
use revm::primitives::{B160, B256};

#[derive(Debug, Clone)]
/// A struct use for `PhantomData` to indicate a lock on contracts that are not deployed.
pub struct NotDeployed;
#[derive(Debug)]
// TODO: It would be good to also allow the `IsDeployed` to also mention which `SimulationManager` has deployed it (when we have multiple managers).
/// A struct use for `PhantomData` to indicate an unlocked contract that is deployed.
pub struct IsDeployed;

/// Trait that is used to allow for different statuses of contract fields depending on whether a contract is deployed or not.
pub trait DeploymentStatus {
    /// The type of the address field.
    type Address;
    /// The type of the ABI field that is used once deployed.
    type Abi;
    /// The type of the base contract field that is used before deployment.
    type BaseContract;
    /// The type of the bytecode field used only before deployment.
    type Bytecode;
    /// The type of the constructor arguments field used only before deployment.
    type ConstructorArguments;
}

impl DeploymentStatus for NotDeployed {
    type Address = ();
    type Abi = ();
    type BaseContract = BaseContract;
    type Bytecode = Vec<u8>;
    type ConstructorArguments = ();
}

impl DeploymentStatus for IsDeployed {
    type Address = B160;
    type Abi = BaseContract;
    type BaseContract = ();
    type Bytecode = ();
    type ConstructorArguments = Vec<Token>;
}

#[derive(Debug, Clone)]
/// A struct that wraps around the ethers `BaseContract` and adds some additional information relevant for revm and the simulation.
pub struct SimulationContract<Deployed: DeploymentStatus> {
    /// The address of the contract within the relevant `SimulationEnvironment`.
    pub address: Deployed::Address,
    /// The ABI of the contract (same as the base contract, but private once deployed for a better API)
    abi: Deployed::Abi,
    /// The ethers `BaseContract` that holds the ABI.
    pub base_contract: Deployed::BaseContract,
    // TODO: Bytecode is really only used for deployment. Maybe we don't need to store it like this.
    /// The contract's deployed bytecode.
    pub bytecode: Deployed::Bytecode,
    /// A `PhantomData` marker to indicate whether the contract is deployed or not.
    pub deployed: PhantomData<Deployed>,
    /// The constructor arguments for the contract.
    pub constructor_arguments: Deployed::ConstructorArguments,
}

impl SimulationContract<NotDeployed> {
    /// A constructor function for `SimulationContract` that takes a `BaseContract` and the deployment bytecode.
    pub fn new(contract: Contract, bytecode: EthersBytes) -> Self {
        Self {
            abi: (),
            base_contract: BaseContract::from(contract),
            bytecode: bytecode.to_vec(),
            address: (),
            deployed: std::marker::PhantomData,
            constructor_arguments: (),
        }
    }
    // TODO: This function can probably be made private.
    /// Creates a new `SimulationContract` that is marked as deployed.
    /// This is only called by implicitly by the `SimulationManager` inside of the `deploy` function.
    pub(crate) fn to_deployed(
        simulation_contract: SimulationContract<NotDeployed>,
        address: B160,
        constructor_arguments: Vec<Token>,
    ) -> SimulationContract<IsDeployed> {
        SimulationContract {
            abi: simulation_contract.base_contract,
            bytecode: (),
            address,
            deployed: std::marker::PhantomData,
            base_contract: (),
            constructor_arguments,
        }
    }
}

impl SimulationContract<IsDeployed> {
    /// Encodes the arguments for a function call for the [`SimulationContract`].
    pub fn encode_function(&self, function_name: &str, args: impl Tokenize) -> Bytes {
        self.abi
            .encode(function_name, args)
            .unwrap()
            .into_iter()
            .collect()
    }
    /// Decodes the output of a function call for the [`SimulationContract`].
    pub fn decode_output<D: Detokenize>(&self, function_name: &str, value: Bytes) -> D {
        self.abi.decode_output(function_name, value).unwrap()
    }
    /// Decodes the logs for an event with the [`SimulationContract`].
    pub fn decode_event<D: Detokenize>(
        &self,
        function_name: &str,
        log_topics: Vec<B256>,
        log_data: Bytes,
    ) -> D {
        let log_topics: Vec<H256> = log_topics
            .into_iter()
            .map(|topic| H256::from_slice(&topic.0))
            .collect();
        self.abi
            .decode_event(function_name, log_topics, log_data.into())
            .unwrap()
    }
}
