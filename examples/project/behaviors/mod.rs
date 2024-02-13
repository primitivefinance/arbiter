use arbiter_macros::Behaviors;
use serde::{Deserialize, Serialize};

mod incrementer;

use incrementer::Incrementer;

#[derive(Serialize, Deserialize, Behaviors)]
pub enum Behaviors {
    Incrementer(Incrementer),
}
