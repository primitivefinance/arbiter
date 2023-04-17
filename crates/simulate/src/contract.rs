#![warn(missing_docs)]
//! This module contains the `SimulationContract` struct that is used to wrap around the ethers `BaseContract` and add some additional information relevant for revm and the simulation.
use std::marker::PhantomData;

use ethers::prelude::BaseContract;
use revm::primitives::B160;

#[derive(Debug, Clone)]
/// A struct use for [`PhantomData`] to indicate a lock on contracts that are not deployed.
pub struct NotDeployed;
#[derive(Debug)]
// TODO: It would be good to also allow the `IsDeployed` to also mention which `SimulationManager` has deployed it (when we have multiple managers).
/// A struct use for `PhantomData` to indicate an unlocked contract that is deployed.
pub struct IsDeployed;

/// Trait that is used to allow for different statuses of contract fields depending on whether a contract is deployed or not.
pub trait DeploymentStatus {
    /// The type of the address field.
    type Address;
}

impl DeploymentStatus for NotDeployed {
    type Address = ();
}

impl DeploymentStatus for IsDeployed {
    type Address = B160;
}

#[derive(Debug, Clone)]
/// A struct that wraps around the ethers `BaseContract` and adds some additional information relevant for revm and the simulation.
pub struct SimulationContract<Deployed: DeploymentStatus> {
    /// The ethers `BaseContract` that holds the ABI.
    pub base_contract: BaseContract,
    // TODO: Bytecode is really only used for deployment. Maybe we don't need to store it like this.
    /// The contract's deployed bytecode.
    pub bytecode: Vec<u8>,
    /// The address of the contract within the relevant `SimulationEnvironment`.
    pub address: Deployed::Address,
    /// A `PhantomData` marker to indicate whether the contract is deployed or not.
    pub deployed: PhantomData<Deployed>,
}

impl SimulationContract<NotDeployed> {
    /// A constructor function for `SimulationContract` that takes a `BaseContract` and the deployment bytecode.
    pub fn new(base_contract: BaseContract, bytecode: Vec<u8>) -> Self {
        Self {
            base_contract,
            bytecode,
            address: (),
            deployed: std::marker::PhantomData,
        }
    }
    // TODO: This function can probably be made private.
    /// Creates a new `SimulationContract` that is marked as deployed.
    /// This is only called by implicitly by the `SimulationManager` inside of the `deploy` function.
    pub(crate) fn to_deployed(&self, address: B160) -> SimulationContract<IsDeployed> {
        SimulationContract {
            base_contract: self.base_contract.clone(),
            bytecode: self.bytecode.clone(),
            address,
            deployed: std::marker::PhantomData,
        }
    }
}
