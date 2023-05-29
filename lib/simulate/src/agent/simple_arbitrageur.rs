#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Describes the most basic type of user agent.

use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crossbeam_channel::{Receiver, Sender};
use revm::primitives::{Address, ExecutionResult, Log, TxEnv, U256};
use tokio::sync::broadcast;

use super::{AgentStatus, Identifiable, IsActive, NotActive};
use crate::agent::{filter_events, Agent, SimulationEventFilter, TransactSettings};

/// Used to report back to another [`Agent`] what the next transaction of the [`SimpleArbitrageur`] should be.
pub enum NextTx {
    /// Arbitrageur is going to swap next.
    Swap,
    /// Arbitrageur is okay with a price update.
    UpdatePrice,
    /// Arbitrageur is asking for no action to take place for the moment.
    None,
}
/// A user is an agent that can interact with the simulation environment generically.
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
    pub event_receiver: AgentState::EventReceiver,
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
    fn receiver(&self) -> broadcast::Receiver<Vec<Log>> {
        self.event_receiver.resubscribe()
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
            event_receiver: (),
            event_filters,
            prices: Arc::new(Mutex::new([U256::MAX, U256::MAX])), // TODO: This is a bad way to do this. Fix it.
            transaction_sender: (),
            result_channel: (),
        }
    }
}

impl SimpleArbitrageur<IsActive> {
    /// A basic implementation that will detect price discprepencies from events emitted from pools.
    /// Currently implemented and tested only against the `liquid_exchange`.
    pub fn detect_arbitrage(
        &self,
    ) -> (
        tokio::task::JoinHandle<()>,
        crossbeam_channel::Receiver<(NextTx, Option<bool>)>,
    ) {
        let (tx, rx) = crossbeam_channel::unbounded::<(NextTx, Option<bool>)>();
        let mut receiver = self.receiver();
        let event_filters = self.event_filters();
        let prices = Arc::clone(&self.prices);

        (
            tokio::spawn(async move {
                let decoder = |input, filter_num: usize| {
                    event_filters[filter_num].base_contract.decode_event_raw(
                        event_filters[filter_num].event_name.as_str(),
                        vec![event_filters[filter_num].topic],
                        input,
                    )
                };
                while let Ok(logs) = receiver.recv().await {
                    // Get the logs and filter
                    let filtered_logs = filter_events(event_filters.clone(), logs);

                    if !filtered_logs.is_empty() {
                        let data = filtered_logs[0].data.clone().into_iter().collect();

                        // See which pool this came from
                        let pool_number =
                            if filtered_logs[0].address == event_filters.clone()[0].address {
                                0
                            } else {
                                1
                            };

                        let decoded_event = decoder(data, pool_number).unwrap(); // TODO: Fix the error handling here.
                        let value = decoded_event[0].clone();
                        let value = value.into_uint().unwrap();
                        let mut prices = prices.lock().unwrap();
                        prices[pool_number] = value.into();

                        // look to see if this gives an arbitrage event
                        // First filter out if one of the prices is MAX as this is the default state.
                        // TODO: This is a really bad way to do this that just adds extra nesting. Fix it.
                        if prices[0] != U256::MAX && prices[1] != U256::MAX {
                            let price_difference = prices[0].overflowing_sub(prices[1]);
                            if price_difference.1 {
                                match tx.send((NextTx::Swap, Some(false))) {
                                    Ok(_) => {}
                                    Err(_) => {
                                        println!("Error sending arbitrage event to channel.\nReceiver must have stopped listening and no more prices are going to be sent.\nBreaking.\n");
                                        break;
                                    }
                                }
                                continue;
                            } else if !price_difference.1 && price_difference.0 != U256::ZERO {
                                match tx.send((NextTx::Swap, Some(true))) {
                                    Ok(_) => {}
                                    Err(_) => {
                                        println!("Error sending arbitrage event to channel.\nReceiver must have stopped listening and no more prices are going to be sent.\nBreaking.\n");
                                        break;
                                    }
                                }
                                continue;
                            } else {
                                match tx.send((NextTx::UpdatePrice, None)) {
                                    Ok(_) => {}
                                    Err(_) => {
                                        println!("Error sending arbitrage event to channel.\nReceiver must have stopped listening and no more prices are going to be sent.\nBreaking.\n");
                                        break;
                                    }
                                }
                            }
                        }
                        drop(prices);
                    } else {
                        match tx.send((NextTx::None, None)) {
                            Ok(_) => {}
                            Err(_) => {
                                println!("Error sending arbitrage event to channel.\nReceiver must have stopped listening and no more prices are going to be sent.\nBreaking.\n");
                                break;
                            }
                        }
                        continue;
                    }
                }
            }),
            rx,
        )
    }
}

// #[cfg(test)]
// mod tests {

//     use std::{error::Error, sync::Arc};

//     use bindings::{arbiter_token, liquid_exchange};
//     use ethers::prelude::U256;
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

//     #[test]
//     fn simple_arbitrageur_event_filter() -> Result<(), Box<dyn Error>> {
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
//         let token_x = arbiter_token.deploy(
//             &mut manager.environment,
//             manager.agents.get("admin").unwrap(),
//             args,
//         );

//         // Deploy token_y.
//         let name = "Token Y";
//         let symbol = "TKNY";
//         let args = (name.to_string(), symbol.to_string(), decimals);
//         let token_y = arbiter_token.deploy(
//             &mut manager.environment,
//             manager.agents.get("admin").unwrap(),
//             args,
//         );

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
//         let liquid_exchange_xy0 = liquid_exchange.deploy(
//             &mut manager.environment,
//             manager.agents.get("admin").unwrap(),
//             args0,
//         );
//         let price_to_check = 123;
//         let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
//         let args1 = (
//             recast_address(token_x.address),
//             recast_address(token_y.address),
//             initial_price,
//         );
//         let liquid_exchange_xy1 = liquid_exchange.deploy(
//             &mut manager.environment,
//             manager.agents.get("admin").unwrap(),
//             args1,
//         );

//         // Create a simple arbitrageur agent.
//         let event_filters = vec![
//             SimulationEventFilter::new(&liquid_exchange_xy0, "PriceChange"),
//             SimulationEventFilter::new(&liquid_exchange_xy1, "PriceChange"),
//         ];

//         let arbitrageur =
//             AgentType::SimpleArbitrageur(SimpleArbitrageur::new("arbitrageur", event_filters));
//         manager.activate_agent(arbitrageur, B160::from_low_u64_be(2))?;
//         let arbitrageur = manager.agents.get("arbitrageur").unwrap();

//         // Make calls that the arbitrageur should not filter out.
//         // Make a price change to the first exchange.
//         let new_price0 = wad.checked_mul(U256::from(42069)).unwrap();
//         let call_data = liquid_exchange_xy0.encode_function("setPrice", new_price0)?;
//         manager.agents.get("admin").unwrap().call_contract(
//             &mut manager.environment,
//             &liquid_exchange_xy0,
//             call_data,
//             U256::zero().into(),
//         );
//         // Test that the arbitrageur doesn't filter out these logs.
//         let unfiltered_events = arbitrageur.read_logs()?;
//         let filtered_events = filter_events(arbitrageur.event_filters(), unfiltered_events.clone());
//         println!(
//             "The filtered events for the first call are: {:#?}",
//             &filtered_events
//         );
//         assert_eq!(filtered_events, unfiltered_events);

//         // Make a price change to the second exchange.
//         let new_price1 = wad.checked_mul(U256::from(69420)).unwrap();
//         let call_data = liquid_exchange_xy1.encode_function("setPrice", new_price1)?;
//         manager.agents.get("admin").unwrap().call_contract(
//             &mut manager.environment,
//             &liquid_exchange_xy1,
//             call_data,
//             U256::zero().into(),
//         );
//         // Test that the arbitrageur doesn't filter out these logs.
//         let unfiltered_events = arbitrageur.read_logs()?;
//         let filtered_events = filter_events(arbitrageur.event_filters(), unfiltered_events.clone());
//         println!(
//             "The filtered events for the second call are: {:#?}",
//             &filtered_events
//         );
//         assert_eq!(filtered_events, unfiltered_events);

//         // Make calls that the arbitrageur should filter out.
//         // Make a call to mint tokens.
//         let call_data = token_x.encode_function(
//             "mint",
//             (
//                 recast_address(manager.agents.get("arbitrageur").unwrap().address()),
//                 U256::from(1),
//             ),
//         )?;
//         manager.agents.get("admin").unwrap().call_contract(
//             &mut manager.environment,
//             &token_x,
//             call_data,
//             U256::zero().into(),
//         );
//         // Test that the arbitrageur does filter out these logs.
//         let unfiltered_events = arbitrageur.read_logs()?;
//         let filtered_events = filter_events(arbitrageur.event_filters(), unfiltered_events);
//         println!(
//             "The filtered events for the second call are: {:#?}",
//             &filtered_events
//         );
//         assert_eq!(filtered_events, vec![]);
//         Ok(())
//     }

//     #[test]
//     fn simple_arbitrage_detection() -> Result<(), Box<dyn Error>> {
//         // Set up the liquid exchange.
//         let decimals = 18_u8;
//         let wad: U256 = U256::from(10_i64.pow(decimals as u32));

//         // Set up the execution manager and a user address.
//         let mut manager = SimulationManager::default();
//         // let admin = manager.agents.get("admin").unwrap();

//         // Create arbiter token general contract.
//         let arbiter_token = SimulationContract::new(
//             arbiter_token::ARBITERTOKEN_ABI.clone(),
//             arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
//         );

//         // Deploy token_x.
//         let name = "Token X";
//         let symbol = "TKNX";
//         let args = (name.to_string(), symbol.to_string(), decimals);
//         let token_x = arbiter_token.deploy(
//             &mut manager.environment,
//             manager.agents.get("admin").unwrap(),
//             args,
//         );

//         // Deploy token_y.
//         let name = "Token Y";
//         let symbol = "TKNY";
//         let args = (name.to_string(), symbol.to_string(), decimals);
//         let token_y = arbiter_token.deploy(
//             &mut manager.environment,
//             manager.agents.get("admin").unwrap(),
//             args,
//         );

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
//         let liquid_exchange_xy0 = liquid_exchange.deploy(
//             &mut manager.environment,
//             manager.agents.get("admin").unwrap(),
//             args0,
//         );
//         let price_to_check = 123;
//         let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
//         let args1 = (
//             recast_address(token_x.address),
//             recast_address(token_y.address),
//             initial_price,
//         );
//         let liquid_exchange_xy1 = liquid_exchange.deploy(
//             &mut manager.environment,
//             manager.agents.get("admin").unwrap(),
//             args1,
//         );

//         // Create a simple arbitrageur agent.
//         let event_filters = vec![
//             SimulationEventFilter::new(&liquid_exchange_xy0, "PriceChange"),
//             SimulationEventFilter::new(&liquid_exchange_xy1, "PriceChange"),
//         ];
//         let arbitrageur =
//             AgentType::SimpleArbitrageur(SimpleArbitrageur::new("arbitrageur", event_filters));
//         manager.activate_agent(arbitrageur, B160::from_low_u64_be(2))?;
//         let arbitrageur = manager.agents.get("arbitrageur").unwrap();

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
//         println!("Beginning arbitrage detection.");
//         let (_arbitrage_detection_handle, rx) = base_arbitrageur.detect_arbitrage();

//         let new_price0 = wad.checked_mul(U256::from(42069)).unwrap();
//         let call_data = liquid_exchange_xy0.encode_function("setPrice", new_price0)?;
//         manager.agents.get("admin").unwrap().call_contract(
//             &mut manager.environment,
//             &liquid_exchange_xy0,
//             call_data,
//             U256::zero().into(),
//         );

//         // Make a price change to the second exchange.
//         let new_price1 = wad.checked_mul(U256::from(69420)).unwrap();
//         let call_data = liquid_exchange_xy1.encode_function("setPrice", new_price1)?;
//         manager.agents.get("admin").unwrap().call_contract(
//             &mut manager.environment,
//             &liquid_exchange_xy1,
//             call_data,
//             U256::zero().into(),
//         );
//         while let Ok((next_tx, ..)) = rx.recv() {
//             println!("Received a new message.");
//             match next_tx {
//                 NextTx::None => {
//                     println!("None");
//                     continue;
//                 }
//                 NextTx::Swap => {
//                     println!("Swap");
//                     break;
//                 }
//                 NextTx::UpdatePrice => {
//                     println!("Update price");
//                     continue;
//                 }
//             }
//         }
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

//         manager.shut_down();

//         Ok(())
//     }
// }
