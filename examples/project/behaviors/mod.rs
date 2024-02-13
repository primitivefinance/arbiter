use arbiter_macros::Behaviors;
use serde::{Deserialize, Serialize};

pub mod incrementer;

use incrementer::Incrementer;

#[derive(Debug, Serialize, Deserialize, Behaviors)]
pub enum Behaviors {
    Incrementer(Incrementer),
}
