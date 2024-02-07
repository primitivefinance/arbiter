#![warn(missing_docs)]
#![allow(unused)]
//! The examples module contains example strategies.
use std::{collections::HashMap, sync::Arc};

use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
use arbiter_core::middleware::RevmMiddleware;
use ethers::types::{transaction::eip2718::TypedTransaction, Address, Log, U256};
use futures_util::{stream, StreamExt};

use super::*;
use crate::{
    agent::Agent,
    errors::ArbiterEngineError,
    machine::{
        Behavior, ControlFlow, CreateStateMachine, Engine, EventStream, State, StateMachine,
    },
    messager::{Message, Messager, To},
    world::World,
};
#[cfg(test)]
pub(crate) mod minter;
#[cfg(test)]
pub(crate) mod timed_message;
