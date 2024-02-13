use arbiter_engine::machine::Behavior;

pub struct Incrementer {}

#[async_trait::async_trait]
impl Behavior<E> for Incrementer {
    async fn startup(
        &mut self,
        _client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<EventStream<E>> {
        Ok(messager.stream()?)
    }

    async fn process(&mut self, event: E) -> Result<ControlFlow> {
        Ok(ControlFlow::Continue)
    }
}
