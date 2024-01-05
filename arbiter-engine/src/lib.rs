#![warn(missing_docs)]

use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::{debug, trace, warn};

pub mod agent;
pub mod examples;
pub mod machine;
pub mod messager;
pub mod world;
