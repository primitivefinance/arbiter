use self::examples::minter::agents::token_requester::TokenRequester;
use super::*;
use arbiter_bindings::bindings::arbiter_token::{ArbiterToken, TransferFilter};
use arbiter_core::data_collection::EventLogger;
use std::pin::Pin;
use token_admin::{MintRequest, TokenAdminQuery};

#[async_trait::async_trait]
impl Behavior<TransferFilter> for TokenRequester {
    #[tracing::instrument(skip(self), fields(id = messager.id.as_deref()))]
    async fn startup(
        &mut self,
        client: Arc<RevmMiddleware>,
        mut messager: Messager,
    ) -> Pin<Box<dyn Stream<Item = TransferFilter> + Send + Sync>> {
        debug!("inside of token requester startup!");
        let message = Message {
            from: messager.id.clone().unwrap(),
            to: To::Agent(self.request_to.clone()),
            data: serde_json::to_string(&TokenAdminQuery::AddressOf(self.token_data.name.clone()))
                .unwrap(),
        };
        messager.send(message).await;
        let message = messager.get_next().await;
        let token_address = serde_json::from_str::<Address>(&message.data).unwrap();
        let token = ArbiterToken::new(token_address, client.clone());
        self.token_data.address = Some(token_address);
        debug!("token addr: {:?}", token_address);

        let mint_data = serde_json::to_string(&TokenAdminQuery::MintRequest(MintRequest {
            token: token.name().call().await.unwrap(),
            mint_to: client.address(),
            mint_amount: 1,
        }))
        .unwrap();

        let mint_request = Message {
            from: messager.id.clone().unwrap(),
            to: To::Agent(self.request_to.clone()),
            data: mint_data,
        };

        messager.send(mint_request).await;
        /*

        let mint_request = Message {
            from: messager.id.clone().unwrap(),
            to: To::Agent(self.request_to.clone()),
            data: serde_json::to_string(&TokenAdminQuery::MintRequest(MintRequest {
                token: self.token_data.name.clone(),
                mint_to: self.client.as_ref().unwrap().address(),
                mint_amount: 1,
            }))
            .unwrap(),
        };
        messager.send(mint_request).await;
        */
        debug!("message sent successfully");

        self.messager = Some(messager.clone());
        self.client = Some(client.clone());
        return Box::pin(
            EventLogger::builder()
                .add_stream(token.transfer_filter())
                .stream()
                .unwrap()
                .map(|value| serde_json::from_str(&value).unwrap()),
        );
    }

    #[tracing::instrument(skip(self), fields(id =
 self.messager.as_ref().unwrap().id.as_deref()))]
    async fn process(&mut self, event: TransferFilter) -> Option<MachineHalt> {
        let messager = self.messager.as_ref().unwrap();
        while (self.count < self.max_count.unwrap()) {
            debug!("sending message from requester");
            let message = Message {
                from: messager.id.clone().unwrap(),
                to: To::Agent(self.request_to.clone()),
                data: serde_json::to_string(&TokenAdminQuery::MintRequest(MintRequest {
                    token: self.token_data.name.clone(),
                    mint_to: self.client.as_ref().unwrap().address(),
                    mint_amount: 1,
                }))
                .unwrap(),
            };
            messager.send(message).await;
            self.count += 1;
        }
        Some(MachineHalt)
    }
}
