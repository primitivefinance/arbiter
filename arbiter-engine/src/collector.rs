use std::pin::Pin;

use futures_util::Stream;

use super::*;

pub type DataStream<'a, E> = Pin<Box<dyn Stream<Item = E> + Send + 'a>>;

pub trait DataFeed<E> {
    fn get_stream(&self) -> Result<DataStream<'_, E>>;
}
