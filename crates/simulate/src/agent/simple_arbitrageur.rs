#![warn(missing_docs)]
//! Describes the most basic type of user agent.

use std::error::Error;

use crossbeam_channel::Receiver;
use revm::primitives::{AccountInfo, Address, Log, B160, U256};

use crate::agent::{Agent, SimulationEventFilter, TransactSettings};

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
    /// The filter for the events that the agent is interested in.
    pub event_filters: Vec<SimulationEventFilter>,
    
    pub prices: (U256, U256),
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
    fn event_filters(&self) -> Vec<SimulationEventFilter> {
        self.event_filters.clone()
    }
}

impl SimpleArbitrageur {
    async fn detect_arbitrage(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::error::Error;

    use bindings::{arbiter_token, liquid_exchange};
    use ethers::prelude::U256;
    use revm::primitives::B160;

    use crate::{
        agent::{create_filter, AgentType},
        contract::SimulationContract,
        manager::SimulationManager,
        utils::recast_address,
    };

    // #[test]
    // fn simple_arbitrageur_event_filter() -> Result<(), Box<dyn Error>> {
    //     // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
    //     // Set up the liquid exchange.
    //     let decimals = 18_u8;
    //     let wad: U256 = U256::from(10_i64.pow(decimals as u32));

    //     // Set up the execution manager and a user address.
    //     let mut manager = SimulationManager::default();
    //     // let admin = manager.agents.get("admin").unwrap();

    //     // Create arbiter token general contract.
    //     let arbiter_token = SimulationContract::new(
    //         arbiter_token::ARBITERTOKEN_ABI.clone(),
    //         arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
    //     );

    //     // Deploy token_x.
    //     let name = "Token X";
    //     let symbol = "TKNX";
    //     let args = (name.to_string(), symbol.to_string(), decimals);
    //     let token_x = arbiter_token.deploy(
    //         &mut manager.environment,
    //         manager.agents.get("admin").unwrap(),
    //         args,
    //     );

    //     // Deploy token_y.
    //     let name = "Token Y";
    //     let symbol = "TKNY";
    //     let args = (name.to_string(), symbol.to_string(), decimals);
    //     let token_y = arbiter_token.deploy(
    //         &mut manager.environment,
    //         manager.agents.get("admin").unwrap(),
    //         args,
    //     );

    //     // Deploy LiquidExchange
    //     let price_to_check = 1000;
    //     let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
    //     let liquid_exchange = SimulationContract::new(
    //         liquid_exchange::LIQUIDEXCHANGE_ABI.clone(),
    //         liquid_exchange::LIQUIDEXCHANGE_BYTECODE.clone(),
    //     );
    //     let args0 = (
    //         recast_address(token_x.address),
    //         recast_address(token_y.address),
    //         initial_price,
    //     );

    //     // Deploy two exchanges so they can list different prices.
    //     let liquid_exchange_xy0 = liquid_exchange.deploy(
    //         &mut manager.environment,
    //         manager.agents.get("admin").unwrap(),
    //         args0,
    //     );
    //     let price_to_check = 123;
    //     let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
    //     let args1 = (
    //         recast_address(token_x.address),
    //         recast_address(token_y.address),
    //         initial_price,
    //     );
    //     let liquid_exchange_xy1 = liquid_exchange.deploy(
    //         &mut manager.environment,
    //         manager.agents.get("admin").unwrap(),
    //         args1,
    //     );
    //     // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

    //     // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
    //     // Create a simple arbitrageur agent.
    //     let event_filters = vec![
    //         create_filter(&liquid_exchange_xy0, "PriceChange"),
    //         create_filter(&liquid_exchange_xy1, "PriceChange"),
    //     ];
    //     manager.create_agent(
    //         "arbitrageur",
    //         B160::from_low_u64_be(2),
    //         AgentType::SimpleArbitrageur,
    //         Some(event_filters),
    //     )?;
    //     let arbitrageur = manager.agents.get("arbitrageur").unwrap();
    //     // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

    //     // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
    //     // Make calls that the arbitrageur should not filter out.
    //     // Make a price change to the first exchange.
    //     let new_price0 = wad.checked_mul(U256::from(42069)).unwrap();
    //     let call_data = liquid_exchange_xy0.encode_function("setPrice", new_price0)?;
    //     manager.agents.get("admin").unwrap().call_contract(
    //         &mut manager.environment,
    //         &liquid_exchange_xy0,
    //         call_data,
    //         U256::zero().into(),
    //     );
    //     // Test that the arbitrageur doesn't filter out these logs.
    //     let unfiltered_events = arbitrageur.read_logs()?;
    //     let filtered_events = arbitrageur.filter_events(unfiltered_events.clone());
    //     println!(
    //         "The filtered events for the first call are: {:#?}",
    //         &filtered_events
    //     );
    //     assert_eq!(filtered_events, unfiltered_events);

    //     // Make a price change to the second exchange.
    //     let new_price1 = wad.checked_mul(U256::from(69420)).unwrap();
    //     let call_data = liquid_exchange_xy1.encode_function("setPrice", new_price1)?;
    //     manager.agents.get("admin").unwrap().call_contract(
    //         &mut manager.environment,
    //         &liquid_exchange_xy1,
    //         call_data,
    //         U256::zero().into(),
    //     );
    //     // Test that the arbitrageur doesn't filter out these logs.
    //     let unfiltered_events = arbitrageur.read_logs()?;
    //     let filtered_events = arbitrageur.filter_events(unfiltered_events.clone());
    //     println!(
    //         "The filtered events for the second call are: {:#?}",
    //         &filtered_events
    //     );
    //     assert_eq!(filtered_events, unfiltered_events);
    //     // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

    //     // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
    //     // Make calls that the arbitrageur should filter out.
    //     // Make a call to mint tokens.
    //     let call_data = token_x.encode_function(
    //         "mint",
    //         (
    //             recast_address(manager.agents.get("arbitrageur").unwrap().address()),
    //             U256::from(1),
    //         ),
    //     )?;
    //     manager.agents.get("admin").unwrap().call_contract(
    //         &mut manager.environment,
    //         &token_x,
    //         call_data,
    //         U256::zero().into(),
    //     );
    //     // Test that the arbitrageur does filter out these logs.
    //     let unfiltered_events = arbitrageur.read_logs()?;
    //     let filtered_events = arbitrageur.filter_events(unfiltered_events.clone());
    //     println!(
    //         "The filtered events for the second call are: {:#?}",
    //         &filtered_events
    //     );
    //     assert_eq!(filtered_events, vec![]);
    //     Ok(())
    // }

    #[test]
    fn simple_arbitrage_detection() -> Result<(), Box<dyn Error>> {
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
        // Set up the liquid exchange.
        let decimals = 18_u8;
        let wad: U256 = U256::from(10_i64.pow(decimals as u32));

        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();
        // let admin = manager.agents.get("admin").unwrap();

        // Create arbiter token general contract.
        let arbiter_token = SimulationContract::new(
            arbiter_token::ARBITERTOKEN_ABI.clone(),
            arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
        );

        // Deploy token_x.
        let name = "Token X";
        let symbol = "TKNX";
        let args = (name.to_string(), symbol.to_string(), decimals);
        let token_x = arbiter_token.deploy(
            &mut manager.environment,
            manager.agents.get("admin").unwrap(),
            args,
        );

        // Deploy token_y.
        let name = "Token Y";
        let symbol = "TKNY";
        let args = (name.to_string(), symbol.to_string(), decimals);
        let token_y = arbiter_token.deploy(
            &mut manager.environment,
            manager.agents.get("admin").unwrap(),
            args,
        );

        // Deploy LiquidExchange
        let price_to_check = 1000;
        let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
        let liquid_exchange = SimulationContract::new(
            liquid_exchange::LIQUIDEXCHANGE_ABI.clone(),
            liquid_exchange::LIQUIDEXCHANGE_BYTECODE.clone(),
        );
        let args0 = (
            recast_address(token_x.address),
            recast_address(token_y.address),
            initial_price,
        );

        // Deploy two exchanges so they can list different prices.
        let liquid_exchange_xy0 = liquid_exchange.deploy(
            &mut manager.environment,
            manager.agents.get("admin").unwrap(),
            args0,
        );
        let price_to_check = 123;
        let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
        let args1 = (
            recast_address(token_x.address),
            recast_address(token_y.address),
            initial_price,
        );
        let liquid_exchange_xy1 = liquid_exchange.deploy(
            &mut manager.environment,
            manager.agents.get("admin").unwrap(),
            args1,
        );
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
        // Create a simple arbitrageur agent.
        let event_filters = vec![
            create_filter(&liquid_exchange_xy0, "PriceChange"),
            create_filter(&liquid_exchange_xy1, "PriceChange"),
        ];
        manager.create_agent(
            "arbitrageur",
            B160::from_low_u64_be(2),
            AgentType::SimpleArbitrageur,
            Some(event_filters),
        )?;
        let arbitrageur = manager.agents.get("arbitrageur").unwrap();
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

        arbitrageur.monitor_events();


        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
        // Make calls that the arbitrageur should not filter out.
        // Make a price change to the first exchange.
        let new_price0 = wad.checked_mul(U256::from(42069)).unwrap();
        let call_data = liquid_exchange_xy0.encode_function("setPrice", new_price0)?;
        manager.agents.get("admin").unwrap().call_contract(
            &mut manager.environment,
            &liquid_exchange_xy0,
            call_data,
            U256::zero().into(),
        );
        // Test that the arbitrageur doesn't filter out these logs.
        // let unfiltered_events = arbitrageur.read_logs()?;
        // let filtered_events = arbitrageur.filter_events(unfiltered_events.clone());
        // println!(
        //     "The filtered events for the first call are: {:#?}",
        //     &filtered_events
        // );
        // assert_eq!(filtered_events, unfiltered_events);

        // Make a price change to the second exchange.
        let new_price1 = wad.checked_mul(U256::from(69420)).unwrap();
        let call_data = liquid_exchange_xy1.encode_function("setPrice", new_price1)?;
        manager.agents.get("admin").unwrap().call_contract(
            &mut manager.environment,
            &liquid_exchange_xy1,
            call_data,
            U256::zero().into(),
        );
        // Test that the arbitrageur doesn't filter out these logs.
        // let unfiltered_events = arbitrageur.read_logs()?;
        // let filtered_events = arbitrageur.filter_events(unfiltered_events.clone());
        // println!(
        //     "The filtered events for the second call are: {:#?}",
        //     &filtered_events
        // );
        // assert_eq!(filtered_events, unfiltered_events);
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
        // Make calls that the arbitrageur should filter out.
        // Make a call to mint tokens.
        let call_data = token_x.encode_function(
            "mint",
            (
                recast_address(manager.agents.get("arbitrageur").unwrap().address()),
                U256::from(1),
            ),
        )?;
        manager.agents.get("admin").unwrap().call_contract(
            &mut manager.environment,
            &token_x,
            call_data,
            U256::zero().into(),
        );
        // Test that the arbitrageur does filter out these logs.
        // let unfiltered_events = arbitrageur.read_logs()?;
        // let filtered_events = arbitrageur.filter_events(unfiltered_events.clone());
        // println!(
        //     "The filtered events for the second call are: {:#?}",
        //     &filtered_events
        // );
        // assert_eq!(filtered_events, vec![]);

        Ok(())
    }
}
