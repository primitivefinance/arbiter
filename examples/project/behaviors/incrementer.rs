use std::sync::Arc;

use anyhow::Result;
use arbiter_core::{events::stream_event, middleware::ArbiterMiddleware};
use arbiter_engine::{
    machine::{Behavior, ControlFlow, EventStream},
    messager::Messager,
};
use tracing::{debug, info};

use super::*;
use crate::bindings::modified_counter::{IncrementedFilter, ModifiedCounter};

#[derive(Debug, Serialize, Deserialize)]
pub struct Incrementer {
    #[serde(default)]
    curr_number_of_times: u64,
    max_number_of_times: u64,
    #[serde(skip)]
    counter: Option<ModifiedCounter<ArbiterMiddleware>>,
}

#[async_trait::async_trait]
impl Behavior<IncrementedFilter> for Incrementer {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        _messager: Messager,
    ) -> Result<EventStream<IncrementedFilter>> {
        debug!("Incrementer starting up");
        let counter = ModifiedCounter::deploy(client.clone(), ())?.send().await?;
        let stream = stream_event(counter.incremented_filter());
        counter.increment().send().await?.await?;
        self.curr_number_of_times += 1;
        let curr_number = counter.number().call().await?;
        debug!("Incremented to: {}", curr_number);
        self.counter = Some(counter);
        Ok(stream)
    }

    async fn process(&mut self, _event: IncrementedFilter) -> Result<ControlFlow> {
        debug!("Incrementer processing event");
        let counter = self.counter.as_ref().unwrap();
        if self.curr_number_of_times < self.max_number_of_times {
            counter.increment().send().await?.await?;
            self.curr_number_of_times += 1;
            let curr_number = counter.number().call().await?;
            debug!("Incremented to: {}", curr_number);
            Ok(ControlFlow::Continue)
        } else {
            info!("Incrementer done");
            return Ok(ControlFlow::Halt);
        }
    }
}
