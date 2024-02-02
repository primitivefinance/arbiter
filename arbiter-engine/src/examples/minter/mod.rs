use super::*;
pub(crate) mod agents;
pub(crate) mod behaviors;
pub(crate) mod token_minter;

use std::pin::Pin;

use futures_util::Stream;
use tracing::error;

use crate::{
    agent::Agent,
    machine::{Behavior, MachineHalt, MachineInstruction, StateMachine},
    messager::To,
    world::World,
};

const TOKEN_ADMIN_ID: &str = "token_admin";
const REQUESTER_ID: &str = "requester";
const TOKEN_NAME: &str = "Arbiter Token";
const TOKEN_SYMBOL: &str = "ARB";
const TOKEN_DECIMALS: u8 = 18;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub address: Option<Address>,
}
