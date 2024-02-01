use super::*;
pub mod agents;
pub mod behaviors;
pub mod token_minter;

use crate::{
    agent::Agent,
    machine::{Behavior, MachineHalt, MachineInstruction, StateMachine},
    messager::To,
    world::World,
};
use futures_util::Stream;
use std::pin::Pin;
use tracing::error;

const TOKEN_ADMIN_ID: &str = "token_admin";
const REQUESTER_ID: &str = "requester";
const TOKEN_NAME: &str = "Arbiter Token";
const TOKEN_SYMBOL: &str = "ARB";
const TOKEN_DECIMALS: u8 = 18;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub address: Option<Address>,
}
