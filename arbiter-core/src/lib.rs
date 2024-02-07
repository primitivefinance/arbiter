//! ```text
//!      _     _____  ____ _____ _______ ______ _____  
//!     / \   |  __ \|  _ \_   _|__   __|  ____|  __ \
//!    /   \  | |__) | |_) || |    | |  | |__  | |__) |
//!   / / \ \ |  _  /|  _ < | |    | |  |  __| |  _  /
//!  / _____ \| | \ \| |_) || |_   | |  | |____| | \ \
//! /_/     \_\_|  \_\____/_____|  |_|  |______|_|  \_\
//! ```
//!                                              
//! `arbiter-core` is designed to facilitate agent-based simulations of Ethereum
//! smart contracts in a local environment.
//!
//! With a primary emphasis on ease of use and performance, it employs the
//! [`revm`](https://crates.io/crates/revm) (Rust EVM) to provide a local
//! execution environment that closely simulates the Ethereum blockchain but
//! without associated overheads like networking latency.
//!
//! Key Features:
//! - **Manager Interface**: The main user entry-point that offers management of
//!   different environments and agents.
//! - **Environment Handling**: Detailed setup and control mechanisms for
//!   running the Ethereum-like blockchain environment.
//! - **Middleware Implementation**: Customized middleware to reduce overhead
//!   and provide optimal performance.
//!
//! For a detailed guide on getting started, check out the
//! [Arbiter Github page](https://github.com/primitivefinance/arbiter/).
//!
//! For specific module-level information and examples, navigate to the
//! respective module documentation below.

#![warn(missing_docs)]

pub mod console;
pub mod coprocessor;
pub mod data_collection;
pub mod database;
pub mod environment;
pub mod middleware;

use std::{collections::HashMap, fmt::Debug, sync::Arc};

use revm::db::{CacheDB, EmptyDB};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, trace, warn};
