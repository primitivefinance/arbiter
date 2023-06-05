#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Describes the most basic type of user agent.

use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crossbeam_channel::{Receiver, Sender};
use futures::StreamExt;
use revm::primitives::{Address, ExecutionResult, Log, TxEnv, U256};
use tokio::sync::broadcast;

use super::{AgentStatus, Identifiable, IsActive, NotActive};
use crate::agent::{filter_events, Agent, SimulationEventFilter, TransactSettings};

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
        }
    }
}

// impl Clone for SimpleArbitrageur<IsActive> {
//     fn clone(&self) -> Self {
//         Self {
//             name: self.name.clone(),
//             address: self.address.clone(),
//             account_info: self.account_info.clone(),
//             transact_settings: self.transact_settings.clone(),
//             transaction_sender: self.transaction_sender.clone(),
//             result_channel: self.result_channel.clone(),
//             event_stream: self.event_receiver,
//             event_filters: self.event_filters.clone(),
//             prices: self.prices.clone(),
//         }
//     }
// }

impl SimpleArbitrageur<IsActive> {
    /// A basic implementation that will detect price discprepencies from events emitted from pools.
    /// Currently implemented and tested only against the `liquid_exchange`.
    pub fn detect_arbitrage(&self) -> tokio::task::JoinHandle<(NextTx, Option<SwapDirection>)> {
        println!("Beginning arbitrage detection.");
        let prices = Arc::clone(&self.prices);
        let event_filters = self.event_filters();
        // let arbitrageur = self.clone();
        let mut event_stream = Box::pin(self.event_stream().into_stream());

        let mut return_value = (NextTx::None, None);
        tokio::spawn(async move {
            println!("Inside the detect_arbitrage tokio::spawn.");
            while let Ok((tokens, address)) = event_stream.next().await.unwrap() {
                println!("Received a new message in the detect_arbitrage while let.");
                let new_price = tokens[0].clone().into_uint().unwrap();
                println!("New price: {:#?}", new_price);
                let mut prices = prices.lock().unwrap();
                // If the first event is from the first pool, pool_number is 0 else it will be pool 1.
                let pool_number = address == event_filters.clone()[0].address;
                prices[pool_number as usize] = new_price.into();
                let (is_arbitrage, swap_direction) = is_arbitrage(prices.clone());
                if is_arbitrage {
                    println!("Arbitrage detected.");
                    return_value = (NextTx::Swap, swap_direction);
                    break;
                } else {
                    println!("No arbitrage detected.");
                    return_value = (NextTx::UpdatePrice, None);
                    break;
                }
            }
            return_value
        })
    }
}

/// Basic function that checks for price differences between the two pools.
// TODO: Will need to take into account the no-arbitrage bounds.
pub fn is_arbitrage(prices: [U256; 2]) -> (bool, Option<SwapDirection>) {
    let price_difference = prices[0].checked_sub(prices[1]);
    if price_difference.is_none() {
        // If this difference is `None`, then the subtraction overflowed so prices[0]<prices[1].
        return (true, Some(SwapDirection::ZeroToOne));
    } else if price_difference != Some(U256::ZERO) {
        // If the price difference is still nonzero, then we must swap with price[0]>price[1].
        return (true, Some(SwapDirection::OneToZero));
    } else {
        return (false, None);
    }
}

// #[cfg(test)]
// mod tests {

//     use std::{error::Error, sync::Arc};

//     use bindings::{arbiter_token, liquid_exchange};
//     use ethers::{abi::Tokenize, prelude::U256};
//     use revm::primitives::B160;

//     use super::SimpleArbitrageur;
//     use crate::{
//         agent::{
//             filter_events, simple_arbitrageur::NextTx, Agent, AgentType, SimulationEventFilter,
//         },
//         environment::contract::SimulationContract,
//         manager::SimulationManager,
//         utils::recast_address,
//     };

//     #[tokio::test]
//     async fn simple_arbitrageur_event_filter() -> Result<(), Box<dyn Error>> {
//         // Set up the liquid exchange.
//         let decimals = 18_u8;
//         let wad: U256 = U256::from(10_i64.pow(decimals as u32));

//         // Set up the execution manager and a user address.
//         let mut manager = SimulationManager::default();

//         // Create arbiter token general contract.
//         let arbiter_token = SimulationContract::new(
//             arbiter_token::ARBITERTOKEN_ABI.clone(),
//             arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
//         );

//         // Deploy token_x.
//         let name = "Token X";
//         let symbol = "TKNX";
//         let args = (name.to_string(), symbol.to_string(), decimals);
//         let (token_x, execution_result) = manager
//             .agents
//             .get("admin")
//             .unwrap()
//             .deploy(arbiter_token.clone(), args.into_tokens())?;
//         assert!(execution_result.is_success());

//         // Deploy token_y.
//         let name = "Token Y";
//         let symbol = "TKNY";
//         let args = (name.to_string(), symbol.to_string(), decimals);
//         let (token_y, execution_result) = manager
//             .agents
//             .get("admin")
//             .unwrap()
//             .deploy(arbiter_token, args.into_tokens())?;
//         assert!(execution_result.is_success());

//         // Create `NotDeployed` LiquidExchange
//         let liquid_exchange = SimulationContract::new(
//             liquid_exchange::LIQUIDEXCHANGE_ABI.clone(),
//             liquid_exchange::LIQUIDEXCHANGE_BYTECODE.clone(),
//         );

//         // Deploy two exchanges so they can list different prices.
//         let price_to_check = 1000;
//         let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
//         let args0 = (
//             recast_address(token_x.address),
//             recast_address(token_y.address),
//             initial_price,
//         );
//         let (liquid_exchange_xy0, execution_result) = manager
//             .agents
//             .get("admin")
//             .unwrap()
//             .deploy(liquid_exchange.clone(), args0.into_tokens())?;
//         assert!(execution_result.is_success());
//         let price_to_check = 123;
//         let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
//         let args1 = (
//             recast_address(token_x.address),
//             recast_address(token_y.address),
//             initial_price,
//         );
//         let (liquid_exchange_xy1, execution_result) = manager
//             .agents
//             .get("admin")
//             .unwrap()
//             .deploy(liquid_exchange, args1.into_tokens())?;
//         assert!(execution_result.is_success());

//         // Create a simple arbitrageur agent.
//         let event_filters = vec![
//             SimulationEventFilter::new(&liquid_exchange_xy0, "PriceChange"),
//             SimulationEventFilter::new(&liquid_exchange_xy1, "PriceChange"),
//         ];

//         let arbitrageur =
//             AgentType::SimpleArbitrageur(SimpleArbitrageur::new("arbitrageur", event_filters));
//         manager.activate_agent(arbitrageur, B160::from_low_u64_be(2))?;
//         let arbitrageur = manager.agents.get("arbitrageur").unwrap();
//         let admin = manager.agents.get("admin").unwrap();

//         // Start the arbitrageur listening for events.
//         let mut receiver = arbitrageur.receiver();

//         // Make a price change to the first exchange.
//         let new_price0 = wad.checked_mul(U256::from(42069)).unwrap();
//         admin.call(&liquid_exchange_xy0, "setPrice", new_price0.into_tokens())?;
//         // Test that the arbitrageur doesn't filter out these logs.
//         let unfiltered_events = receiver.recv().await?;
//         let filtered_events = filter_events(arbitrageur.event_filters(), unfiltered_events.clone());
//         println!(
//             "The filtered events for the first call are: {:#?}",
//             &filtered_events
//         );
//         assert_eq!(filtered_events, unfiltered_events);

//         // Make a price change to the second exchange.
//         let new_price1 = wad.checked_mul(U256::from(69420)).unwrap();
//         admin.call(&liquid_exchange_xy1, "setPrice", new_price1.into_tokens())?;
//         // Test that the arbitrageur doesn't filter out these logs.
//         let unfiltered_events = receiver.recv().await?;
//         let filtered_events = filter_events(arbitrageur.event_filters(), unfiltered_events.clone());
//         println!(
//             "The filtered events for the second call are: {:#?}",
//             &filtered_events
//         );
//         assert_eq!(filtered_events, unfiltered_events);

//         // Make calls that the arbitrageur should filter out.
//         // Make a call to mint tokens.
//         admin.call(
//             &token_x,
//             "mint",
//             (recast_address(arbitrageur.address()), U256::from(1)).into_tokens(),
//         )?;
//         // Test that the arbitrageur does filter out these logs.
//         let unfiltered_events = receiver.recv().await?;
//         let filtered_events = filter_events(arbitrageur.event_filters(), unfiltered_events);
//         println!(
//             "The filtered events for the third call are: {:#?}",
//             &filtered_events
//         );
//         assert_eq!(filtered_events, vec![]);
//         Ok(())
//     }

//     #[tokio::test]
//     async fn simple_arbitrage_detection() -> Result<(), Box<dyn Error>> {
//         // Set up the liquid exchange.
//         let decimals = 18_u8;
//         let wad: U256 = U256::from(10_i64.pow(decimals as u32));

//         // Set up the execution manager and a user address.
//         let mut manager = SimulationManager::default();

//         // Create arbiter token general contract.
//         let arbiter_token = SimulationContract::new(
//             arbiter_token::ARBITERTOKEN_ABI.clone(),
//             arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
//         );

//         // Deploy token_x.
//         let name = "Token X";
//         let symbol = "TKNX";
//         let args = (name.to_string(), symbol.to_string(), decimals);
//         let (token_x, execution_result) = manager
//             .agents
//             .get("admin")
//             .unwrap()
//             .deploy(arbiter_token.clone(), args.into_tokens())?;
//         assert!(execution_result.is_success());

//         // Deploy token_y.
//         let name = "Token Y";
//         let symbol = "TKNY";
//         let args = (name.to_string(), symbol.to_string(), decimals);
//         let (token_y, execution_result) = manager
//             .agents
//             .get("admin")
//             .unwrap()
//             .deploy(arbiter_token, args.into_tokens())?;
//         assert!(execution_result.is_success());

//         // Deploy LiquidExchange
//         let price_to_check = 1000;
//         let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
//         let liquid_exchange = SimulationContract::new(
//             liquid_exchange::LIQUIDEXCHANGE_ABI.clone(),
//             liquid_exchange::LIQUIDEXCHANGE_BYTECODE.clone(),
//         );
//         let args0 = (
//             recast_address(token_x.address),
//             recast_address(token_y.address),
//             initial_price,
//         );

//         // Deploy two exchanges so they can list different prices.
//         let (liquid_exchange_xy0, execution_result) = manager
//             .agents
//             .get("admin")
//             .unwrap()
//             .deploy(liquid_exchange.clone(), args0.into_tokens())?;
//         assert!(execution_result.is_success());
//         let price_to_check = 123;
//         let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
//         let args1 = (
//             recast_address(token_x.address),
//             recast_address(token_y.address),
//             initial_price,
//         );
//         let (liquid_exchange_xy1, execution_result) = manager
//             .agents
//             .get("admin")
//             .unwrap()
//             .deploy(liquid_exchange, args1.into_tokens())?;
//         assert!(execution_result.is_success());

//         // Create a simple arbitrageur agent.
//         let event_filters = vec![
//             SimulationEventFilter::new(&liquid_exchange_xy0, "PriceChange"),
//             SimulationEventFilter::new(&liquid_exchange_xy1, "PriceChange"),
//         ];
//         let arbitrageur =
//             AgentType::SimpleArbitrageur(SimpleArbitrageur::new("arbitrageur", event_filters));
//         manager.activate_agent(arbitrageur, B160::from_low_u64_be(2))?;
//         let arbitrageur = manager.agents.get("arbitrageur").unwrap();
//         let admin = manager.agents.get("admin").unwrap();

//         // Have the arbitrageur check for arbitrage events.
//         let base_arbitrageur = match arbitrageur {
//             AgentType::SimpleArbitrageur(base_arbitrageur) => base_arbitrageur,
//             _ => panic!(),
//         };

//         // Verify that the initial prices are correct
//         let prices = Arc::clone(&base_arbitrageur.prices);
//         let prices = prices.lock().unwrap();
//         assert_eq!(prices[0], U256::MAX.into());
//         assert_eq!(prices[1], U256::MAX.into());
//         drop(prices);

//         // Start the arbitrageur to detect price changes.
//         let arbitrage = base_arbitrageur.watch();


//         std::thread::sleep(std::time::Duration::from_secs(1));
//         println!("Sending price change events.");
//         let new_price0 = wad.checked_mul(U256::from(42069)).unwrap();
//         let execution_result =
//             admin.call(&liquid_exchange_xy0, "setPrice", new_price0.into_tokens())?;
//         assert!(execution_result.is_success());
//         std::thread::sleep(std::time::Duration::from_secs(1));
//         // Make a price change to the second exchange.
//         let new_price1 = wad.checked_mul(U256::from(69420)).unwrap();
//         let execution_result =
//             admin.call(&liquid_exchange_xy1, "setPrice", new_price1.into_tokens())?;
//         assert!(execution_result.is_success());

//         println!("output from detect_arbitrage: {:#?}", arbitrage.await);
//         // while let (next_tx, swap_direction) = arbitrage.await {
//         //     match next_tx {
//         //         NextTx::None => {
//         //             println!("None");
//         //         }
//         //         NextTx::Swap => {
//         //             println!("Swap");
//         //         }
//         //         NextTx::UpdatePrice => {
//         //             println!("Update price");
//         //         }
//         //     };
//         // }
//         let prices = Arc::clone(&base_arbitrageur.prices);
//         let prices = prices.lock().unwrap();
//         println!("Arbitrageur prices: {:#?}", prices);
//         assert_eq!(
//             prices[0],
//             wad.checked_mul(U256::from(42069)).unwrap().into()
//         );
//         assert_eq!(
//             prices[1],
//             wad.checked_mul(U256::from(69420)).unwrap().into()
//         );

//         Ok(())
//     }
// }
