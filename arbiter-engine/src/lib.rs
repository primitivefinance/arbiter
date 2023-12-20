use std::collections::HashMap;

use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream, Executor};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::{debug, trace, warn};

pub mod agent;
#[cfg(test)]
pub mod examples;
pub mod messager;
pub mod transactor;
pub mod world;
