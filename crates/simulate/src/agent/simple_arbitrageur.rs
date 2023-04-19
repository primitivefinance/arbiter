#![warn(missing_docs)]
//! Describes the most basic type of user agent.

use std::marker::PhantomData;

use crossbeam_channel::Receiver;
use ethers::types::Filter;
use revm::primitives::{Account, AccountInfo, Address, Log, B160, U256};

use crate::{
    agent::{Agent, TransactSettings},
    contract::{IsDeployed, SimulationContract},
    utils::recast_address,
};

/// A user is an agent that can interact with the simulation environment generically.
pub struct SimpleArbitrageur {
    /// Name of the agent.
    pub name: String,
    /// Public address of the simulation manager.
    pub address: B160,
    /// [`revm::primitives`] account of the [`SimulationManager`].
    pub account_info: AccountInfo,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    pub transact_settings: TransactSettings,
    /// The receiver for the crossbeam channel that events are sent down from manager's dispatch.
    pub event_receiver: Receiver<Vec<Log>>,
    pub event_filter: Filter,
}

impl Agent for SimpleArbitrageur {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn address(&self) -> Address {
        self.address
    }
    fn transact_settings(&self) -> &TransactSettings {
        &self.transact_settings
    }
    fn receiver(&self) -> Receiver<Vec<Log>> {
        self.event_receiver.clone()
    }
    fn filter_events(&self, logs: Vec<Log>) -> Vec<Log> {
        todo!();
    }
}

impl SimpleArbitrageur {
    /// Constructor function to instantiate a user agent.
    pub fn new(
        name: String,
        address: B160,
        pools: (
            SimulationContract<IsDeployed>,
            SimulationContract<IsDeployed>,
        ),
        event_strings: (&str, &str),
    ) -> Self {
        todo!();
        // TODO: The commented code here is a start to building the correct type of filter -- let's leave it in.
        // let event_filter = Filter {
        //     address: Some(ethers::types::ValueOrArray::Array(vec![
        //         recast_address(pools.0.address),
        //         recast_address(pools.1.address),
        //     ])),
        //     topics: [None, None, None, None], // None for all topics
        //     ..Default::default()
        // };
        // event_filter.event(event_strings.0);
        // event_filter.event(event_strings.1);
        // Self {
        //     name,
        //     address,
        //     account_info: (),
        //     transact_settings: TransactSettings {
        //         gas_limit: u64::MAX,   // TODO: Users should have a gas limit.
        //         gas_price: U256::ZERO, // TODO: Users should have an associated gas price.
        //     },
        //     event_receiver: (),
        //     event_filter,
        //     active: PhantomData,
        // }
    }
}

#[cfg(test)]

mod test {
    use super::*;
    use bindings::liquid_exchange::{LiquidExchangeEvents, PriceChangeFilter};
    // use ethers::prelude::contract::base::decode_event;

    #[test]
    fn simple_arbitrageur_event_filter() {
        // SimpleArbitrageur::new(event_receiver, address, pools, event_strings)
        assert_eq!(1,1);
        todo!();
    }
}
