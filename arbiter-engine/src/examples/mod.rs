#![warn(missing_docs)]
#![allow(unused)]
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TODO: Notes ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Create a BlockAdmin and a TokenAdmin.
// Potentially create an `Orchestrator`` that sends instructions to both
// BlockAdmin and TokenAdmin.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! The examples module contains example strategies.

use std::{collections::HashMap, sync::Arc};

use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
use arbiter_core::middleware::RevmMiddleware;
use ethers::types::{transaction::eip2718::TypedTransaction, Address, Log, U256};
use futures_util::{stream, StreamExt};

use super::*;
use crate::messager::{Message, Messager};
mod minter;
mod timed_message;
