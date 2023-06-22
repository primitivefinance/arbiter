#![warn(unsafe_code)]
//! Describes the most basic type of user agent.

use ethers::prelude::AbiError;
use std::sync::Arc;
use tokio::sync::Mutex;

use crossbeam_channel::{Receiver, Sender};
use futures::StreamExt;
use revm::primitives::{Address, ExecutionResult, TxEnv, U256};

use super::{AgentStatus, Identifiable, IsActive, NotActive};
use crate::agent::{Agent, SimulationEventFilter, TransactSettings};

/// Used to report back to another [`Agent`] what the next transaction of the [`SimpleArbitrageur`] should be.

#[derive(Debug, Clone)]
pub enum NextTx {
    /// Arbitrageur is going to swap next.
    Swap,
    /// Arbitrageur is okay with a price update.
    UpdatePrice,
    /// Arbitrageur is asking for no action to take place for the moment.
    None,
}

/// Used to indicate the swap directio for the [`SimpleArbitrageur`].
#[derive(Debug, Clone)]
pub enum SwapDirection {
    /// Arbitrageur is swapping from token 0 to token 1.
    ZeroToOne,
    /// Arbitrageur is swapping from token 1 to token 0.
    OneToZero,
}

/// A user is an agent that can interact with the simulation environment generically.
#[derive(Debug)]
pub struct SimpleArbitrageur<AgentState: AgentStatus> {
    /// Name of the agent.
    pub name: String,
    /// Public address of the simulation manager.
    pub address: AgentState::Address,
    /// [`revm::primitives`] account of the simulation manager.
    pub account_info: AgentState::AccountInfo,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    pub transact_settings: AgentState::TransactSettings,
    /// The [`crossbeam_channel::Sender`] for sending transactions to the `SimulationEnvironment`.
    pub transaction_sender: AgentState::TransactionSender,
    /// The [`crossbeam_channel`] for getting [`ExecutionResult`] back from the `SimulationEnvironment`.
    pub result_channel: AgentState::ResultChannel,
    /// The [`crossbeam_channel::Receiver`] for the events are sent down from [`SimulationEnvironment`]'s dispatch.
    pub event_stream: AgentState::EventStream,
    /// The filter for the events that the agent is interested in.
    pub event_filters: Vec<SimulationEventFilter>,
    /// Storage of prices of the two pools the [`SimpleArbitrageur`] tracks.
    pub prices: Arc<Mutex<[U256; 2]>>,
    /// Fee parameter.
    pub fee: U256,
}

impl<AgentState: AgentStatus> Identifiable for SimpleArbitrageur<AgentState> {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Agent for SimpleArbitrageur<IsActive> {
    fn address(&self) -> Address {
        self.address
    }
    fn transact_settings(&self) -> &TransactSettings {
        &self.transact_settings
    }
    fn event_stream(&self) -> crate::environment::EventStream {
        self.event_stream.clone()
    }
    fn event_filters(&self) -> Vec<SimulationEventFilter> {
        self.event_filters.clone()
    }
    fn transaction_sender(&self) -> Sender<(TxEnv, Sender<ExecutionResult>)> {
        self.transaction_sender.clone()
    }
    fn result_channel(&self) -> (Sender<ExecutionResult>, Receiver<ExecutionResult>) {
        self.result_channel.clone()
    }
}

impl SimpleArbitrageur<NotActive> {
    /// Creates a new [`SimpleArbitrageur`] which requires a vector of [`SimulationEventFilter`] and automatically sets default initial stored prices.
    pub fn new<S: Into<String>>(
        name: S,
        event_filters: Vec<SimulationEventFilter>,
        fee: U256,
    ) -> SimpleArbitrageur<NotActive> {
        SimpleArbitrageur::<NotActive> {
            name: name.into(),
            address: (),
            account_info: (),
            transact_settings: (),
            event_stream: (),
            event_filters,
            prices: Arc::new(Mutex::new([U256::MAX, U256::MAX])), // TODO: This is a bad way to do this. Fix it.
            transaction_sender: (),
            result_channel: (),
            fee,
        }
    }
}

impl SimpleArbitrageur<IsActive> {
    /// A basic implementation that will detect price discprepencies from events emitted from pools.
    /// Currently implemented and tested only against the `liquid_exchange`.
    pub async fn detect_price_change(&self) -> Result<(NextTx, Option<SwapDirection>), AbiError> {
        let prices = Arc::clone(&self.prices);
        let mut watcher = self.watch();
        let mut return_value = (NextTx::None, None);

        if let Some(result) = watcher.next().await {
            let (tokens, pool_number) = result?;
            let new_price = tokens[0].clone().into_uint().unwrap();
            let mut prices = prices.lock().await;
            prices[pool_number] = new_price.into();
            let (is_arbitrage, swap_direction) = is_arbitrage(*prices, self.fee);
            if is_arbitrage {
                return_value = (NextTx::Swap, swap_direction);
            } else {
                return_value = (NextTx::UpdatePrice, None);
            }
        }
        Ok(return_value)
    }
}

/// Basic function that checks for price differences between the two pools exceeds the no-arb bounds.
pub fn is_arbitrage(prices: [U256; 2], fee: U256) -> (bool, Option<SwapDirection>) {
    if prices[0] == U256::MAX || prices[1] == U256::MAX {
        // If either of the prices are uninitialized, then we can't check for arbitrage.
        return (false, None);
    }
    // Check the no-arbitrage bounds
    let upper_arb_bound = prices[0] * U256::from(10e17) / fee;
    let lower_arb_bound = prices[0] * fee / U256::from(10e17);
    if (prices[1] > upper_arb_bound) | (prices[1] < lower_arb_bound) {
        // If the prices are outside of the no-arbitrage bounds, then we can arbitrage.
        let price_difference = prices[0].checked_sub(prices[1]);
        if price_difference.is_none() {
            // If this difference is `None`, then the subtraction overflowed so prices[0]<prices[1].
            (true, Some(SwapDirection::ZeroToOne))
        } else {
            // If the price difference is still nonzero, then we must swap with price[0]>price[1].
            (true, Some(SwapDirection::OneToZero))
        }
    } else {
        // Prices are within the no-arbitrage bounds, so we don't have an arbitrage.
        (false, None)
    }
}

#[cfg(test)]
mod tests {

    use std::{error::Error, sync::Arc};

    use bindings::{arbiter_token, liquid_exchange};
    use ethers::{abi::Tokenize, prelude::U256};
    use futures::StreamExt;
    use revm::primitives::B160;

    use super::SimpleArbitrageur;
    use crate::{
        agent::{simple_arbitrageur::is_arbitrage, Agent, AgentType, SimulationEventFilter},
        environment::contract::SimulationContract,
        manager::SimulationManager,
        utils::recast_address,
    };

    #[tokio::test]
    // Tests that both filters are working correctly.
    async fn simple_arbitrageur_event_filter() -> Result<(), Box<dyn Error>> {
        // Set up the liquid exchange.
        let decimals = 18_u8;
        let wad: U256 = U256::from(10_i64.pow(decimals as u32));

        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();

        // Create arbiter token general contract.
        let arbiter_token = SimulationContract::new(
            arbiter_token::ARBITERTOKEN_ABI.clone(),
            arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
        );

        // Deploy token_x.
        let name = "Token X";
        let symbol = "TKNX";
        let args = (name.to_string(), symbol.to_string(), decimals);
        let (token_x, execution_result) = manager
            .agents
            .get("admin")
            .unwrap()
            .deploy(arbiter_token.clone(), args.into_tokens())?;
        assert!(execution_result.is_success());

        // Deploy token_y.
        let name = "Token Y";
        let symbol = "TKNY";
        let args = (name.to_string(), symbol.to_string(), decimals);
        let (token_y, execution_result) = manager
            .agents
            .get("admin")
            .unwrap()
            .deploy(arbiter_token, args.into_tokens())?;
        assert!(execution_result.is_success());

        // Create `NotDeployed` LiquidExchange
        let liquid_exchange = SimulationContract::new(
            liquid_exchange::LIQUIDEXCHANGE_ABI.clone(),
            liquid_exchange::LIQUIDEXCHANGE_BYTECODE.clone(),
        );

        // Deploy two exchanges so they can list different prices.
        let price_to_check = 1000;
        let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
        let args0 = (
            recast_address(token_x.address),
            recast_address(token_y.address),
            initial_price,
        );
        let (liquid_exchange_xy0, execution_result) = manager
            .agents
            .get("admin")
            .unwrap()
            .deploy(liquid_exchange.clone(), args0.into_tokens())?;
        assert!(execution_result.is_success());
        let price_to_check = 123;
        let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
        let args1 = (
            recast_address(token_x.address),
            recast_address(token_y.address),
            initial_price,
        );
        let (liquid_exchange_xy1, execution_result) = manager
            .agents
            .get("admin")
            .unwrap()
            .deploy(liquid_exchange, args1.into_tokens())?;
        assert!(execution_result.is_success());

        // Create a simple arbitrageur agent.
        let event_filters = vec![
            SimulationEventFilter::new(&liquid_exchange_xy0, "PriceChange"),
            SimulationEventFilter::new(&liquid_exchange_xy1, "PriceChange"),
        ];

        let arbitrageur = AgentType::SimpleArbitrageur(SimpleArbitrageur::new(
            "arbitrageur",
            event_filters,
            U256::from(997_000_000_000_000_000u128).into(),
        ));
        manager.activate_agent(arbitrageur, B160::from_low_u64_be(2))?;
        let arbitrageur = manager.agents.get("arbitrageur").unwrap();
        let admin = manager.agents.get("admin").unwrap();

        // Get the filter watcher
        let mut watcher = arbitrageur.watch();

        // Make a price change to the first exchange.
        let new_price0 = wad.checked_mul(U256::from(42069)).unwrap();
        admin.call(&liquid_exchange_xy0, "setPrice", new_price0.into_tokens())?;

        // Test that the arbitrageur doesn't filter out these logs.
        let filtered_events = watcher.next().await;
        println!(
            "The filtered events for the first call are: {:#?}",
            &filtered_events
        );
        assert!(filtered_events.is_some());

        // Make a price change to the second exchange.
        let new_price1 = wad.checked_mul(U256::from(69420)).unwrap();
        admin.call(&liquid_exchange_xy1, "setPrice", new_price1.into_tokens())?;
        // Test that the arbitrageur doesn't filter out these logs.
        let filtered_events = watcher.next().await;
        println!(
            "The filtered events for the second call are: {:#?}",
            &filtered_events
        );
        assert!(filtered_events.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn simple_arbitrage_detection() -> Result<(), Box<dyn Error>> {
        // Set up the liquid exchange.
        let decimals = 18_u8;
        let wad: U256 = U256::from(10_i64.pow(decimals as u32));

        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();

        // Create arbiter token general contract.
        let arbiter_token = SimulationContract::new(
            arbiter_token::ARBITERTOKEN_ABI.clone(),
            arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
        );

        // Deploy token_x.
        let name = "Token X";
        let symbol = "TKNX";
        let args = (name.to_string(), symbol.to_string(), decimals);
        let (token_x, execution_result) = manager
            .agents
            .get("admin")
            .unwrap()
            .deploy(arbiter_token.clone(), args.into_tokens())?;
        assert!(execution_result.is_success());

        // Deploy token_y.
        let name = "Token Y";
        let symbol = "TKNY";
        let args = (name.to_string(), symbol.to_string(), decimals);
        let (token_y, execution_result) = manager
            .agents
            .get("admin")
            .unwrap()
            .deploy(arbiter_token, args.into_tokens())?;
        assert!(execution_result.is_success());

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
        let (liquid_exchange_xy0, execution_result) = manager
            .agents
            .get("admin")
            .unwrap()
            .deploy(liquid_exchange.clone(), args0.into_tokens())?;
        assert!(execution_result.is_success());
        let price_to_check = 123;
        let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
        let args1 = (
            recast_address(token_x.address),
            recast_address(token_y.address),
            initial_price,
        );
        let (liquid_exchange_xy1, execution_result) = manager
            .agents
            .get("admin")
            .unwrap()
            .deploy(liquid_exchange, args1.into_tokens())?;
        assert!(execution_result.is_success());

        // Create a simple arbitrageur agent.
        let event_filters = vec![
            SimulationEventFilter::new(&liquid_exchange_xy0, "PriceChange"),
            SimulationEventFilter::new(&liquid_exchange_xy1, "PriceChange"),
        ];
        let arbitrageur = AgentType::SimpleArbitrageur(SimpleArbitrageur::new(
            "arbitrageur",
            event_filters,
            U256::from(997_000_000_000_000_000u128).into(),
        ));
        manager.activate_agent(arbitrageur, B160::from_low_u64_be(2))?;
        let arbitrageur = manager.agents.get("arbitrageur").unwrap();
        let admin = manager.agents.get("admin").unwrap();

        // Have the arbitrageur check for arbitrage events.
        let base_arbitrageur = match arbitrageur {
            AgentType::SimpleArbitrageur(base_arbitrageur) => base_arbitrageur,
            _ => panic!(),
        };

        // Verify that the initial prices are correct
        let prices = Arc::clone(&base_arbitrageur.prices);
        let prices = prices.lock().await;
        assert_eq!(prices[0], U256::MAX.into());
        assert_eq!(prices[1], U256::MAX.into());
        drop(prices);

        println!("Sending price change events.");
        let new_price0 = wad.checked_mul(U256::from(42069)).unwrap();
        let execution_result =
            admin.call(&liquid_exchange_xy0, "setPrice", new_price0.into_tokens())?;
        assert!(execution_result.is_success());

        // Make a price change to the second exchange.
        let new_price1 = wad.checked_mul(U256::from(69420)).unwrap();
        let execution_result =
            admin.call(&liquid_exchange_xy1, "setPrice", new_price1.into_tokens())?;
        assert!(execution_result.is_success());

        // Run the detect_arbitrage function twice to pick up both price changes.
        println!(
            "output from detect_arbitrage: {:#?}",
            base_arbitrageur.detect_price_change().await
        );
        println!(
            "output from detect_arbitrage: {:#?}",
            base_arbitrageur.detect_price_change().await
        );
        let prices = Arc::clone(&base_arbitrageur.prices);
        let prices = prices.lock().await;
        println!("Arbitrageur prices: {:#?}", prices);
        assert_eq!(
            prices[0],
            wad.checked_mul(U256::from(42069)).unwrap().into()
        );
        assert_eq!(
            prices[1],
            wad.checked_mul(U256::from(69420)).unwrap().into()
        );

        // Check that the arbitrageur detects the arbitrage opportunity.
        assert!(is_arbitrage(*prices, U256::from(997_000_000_000_000_000u128).into()).0);

        Ok(())
    }
}
