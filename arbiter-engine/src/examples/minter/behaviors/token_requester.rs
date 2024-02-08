use arbiter_bindings::bindings::arbiter_token::TransferFilter;
use arbiter_core::events::Logger;
use token_admin::{MintRequest, TokenAdminQuery};

use self::{
    errors::ArbiterEngineError, examples::minter::agents::token_requester::TokenRequester,
    machine::EventStream,
};
use super::*;
use futures::StreamExt;

#[async_trait::async_trait]
impl Behavior<TransferFilter> for TokenRequester {
    #[tracing::instrument(skip(self), fields(id = messager.id.as_deref()))]
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        mut messager: Messager,
    ) -> Result<EventStream<TransferFilter>> {
        messager
            .send(
                To::Agent(self.request_to.clone()),
                &TokenAdminQuery::AddressOf(self.token_data.name.clone()),
            )
            .await;
        let message = messager.get_next().await.unwrap();
        let token_address = serde_json::from_str::<Address>(&message.data).unwrap();
        let token = ArbiterToken::new(token_address, client.clone());
        self.token_data.address = Some(token_address);

        let mint_data = TokenAdminQuery::MintRequest(MintRequest {
            token: self.token_data.name.clone(),
            mint_to: client.address(),
            mint_amount: 1,
        });
        messager
            .send(To::Agent(self.request_to.clone()), mint_data)
            .await;

        self.messager = Some(messager.clone());
        self.client = Some(client.clone());
        let stream = token.transfer_filter().stream();
        Err(anyhow::anyhow!("Not implemented"))
    }

    #[tracing::instrument(skip(self), fields(id =
 self.messager.as_ref().unwrap().id.as_deref()))]
    async fn process(&mut self, event: TransferFilter) -> Result<ControlFlow> {
        let messager = self.messager.as_ref().unwrap();
        while (self.count < self.max_count.unwrap()) {
            debug!("sending message from requester");
            let mint_data = TokenAdminQuery::MintRequest(MintRequest {
                token: self.token_data.name.clone(),
                mint_to: self.client.as_ref().unwrap().address(),
                mint_amount: 1,
            });
            messager
                .send(To::Agent(self.request_to.clone()), mint_data)
                .await;
            self.count += 1;
        }
        Ok(ControlFlow::Halt)
    }
}
