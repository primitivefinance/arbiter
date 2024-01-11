#![warn(missing_docs)]

//! Arbiter Engine is a library for creating and running automations or
//! simulations of multi-agent systems. It is designed to be used in a
//! distributed fashion where each agent is running in its own process and
//! communicating with other agents via a messaging layer.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
#[allow(unused)]
use tracing::{debug, trace, warn};

pub mod agent;
pub mod examples;
pub mod machine;
pub mod messager;
pub mod world;
