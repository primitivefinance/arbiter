use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use tracing::{debug, trace, warn};

pub mod agent;
pub mod examples;
pub mod messager;
pub mod world;
