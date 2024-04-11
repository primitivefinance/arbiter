pub mod token_admin;
pub mod token_requester;

use std::sync::Arc;

use anyhow::Result;
use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::{
    machine::{
        Behavior, Configuration, ControlFlow, CreateStateMachine, Engine, EventStream, StateMachine,
    },
    messager::{Message, Messager, To},
};
use arbiter_macros::Behaviors;
use ethers::types::{Address as eAddress, Create, U256 as eU256};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tracing::{debug, error, trace, warn};

use self::{token_admin::TokenAdminConfig, token_requester::Config};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Behaviors<C: Config> {
    TokenAdmin(token_admin::TokenAdmin<Configuration<TokenAdminConfig>>),
    TokenRequester(token_requester::TokenRequester<Configuration<C>>),
}

impl<C: Config + DeserializeOwned> CreateStateMachine for Behaviors<C> {
    fn create_state_machine(self) -> Box<dyn StateMachine> {
        match self {
            Behaviors::TokenAdmin(token_admin) => Box::new(Engine::new(token_admin)),
            Behaviors::TokenRequester(token_requester) => Box::new(Engine::new(token_requester)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub address: Option<eAddress>,
}
