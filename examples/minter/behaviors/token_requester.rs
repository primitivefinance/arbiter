use std::fmt::Debug;

use arbiter_bindings::bindings::arbiter_token::TransferFilter;
use arbiter_core::events::stream_event;
use arbiter_engine::machine::{Processor, State};
use token_admin::{MintRequest, TokenAdminQuery};

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct TokenRequester<S: State> {
    /// The tokens that the token requester has requested.
    pub token_data: TokenData,
    /// The agent ID to request tokens to.
    pub request_to: String,
    pub data: S::Data,
}

#[derive(Debug, Serialize, Deserialize, Clone, State)]
pub struct Config {
    pub max_count: u64,
}

#[derive(State)]
pub struct Processing {
    pub messager: Messager,
    pub client: Arc<ArbiterMiddleware>,
    pub token: ArbiterToken<ArbiterMiddleware>,
    pub count: u64,
    pub max_count: u64,
}

#[async_trait::async_trait]
impl Behavior<TransferFilter> for TokenRequester<Config> {
    type Processor = TokenRequester<Processing>;

    async fn startup(
        mut self,
        client: Arc<ArbiterMiddleware>,
        mut messager: Messager,
    ) -> Result<Self::Processor> {
        messager
            .send(
                To::Agent(self.request_to.clone()),
                TokenAdminQuery::AddressOf(self.token_data.name.clone()),
            )
            .await?;

        let token_address = messager.get_next::<eAddress>().await.unwrap().data;
        let token = ArbiterToken::new(token_address, client.clone());
        self.token_data.address = Some(token_address);

        let mint_data = TokenAdminQuery::MintRequest(MintRequest {
            token: self.token_data.name.clone(),
            mint_to: client.address(),
            mint_amount: 1,
        });
        messager
            .send(To::Agent(self.request_to.clone()), mint_data)
            .await?;

        let data = Processing {
            messager,
            token,
            client,
            count: 0,
            max_count: self.data.max_count,
        };
        Ok(TokenRequester::<Processing> {
            token_data: self.token_data,
            request_to: self.request_to,
            data,
        })
    }
}

#[async_trait::async_trait]
impl Processor<TransferFilter> for TokenRequester<Processing> {
    async fn get_stream(&mut self) -> Result<Option<EventStream<TransferFilter>>> {
        Ok(Some(stream_event(self.data.token.transfer_filter())))
    }

    async fn process(&mut self, _event: TransferFilter) -> Result<ControlFlow> {
        while self.data.count < self.data.max_count {
            debug!("sending message from requester");
            let mint_data = TokenAdminQuery::MintRequest(MintRequest {
                token: self.token_data.name.clone(),
                mint_to: self.data.client.address(),
                mint_amount: 1,
            });
            self.data
                .messager
                .send(To::Agent(self.request_to.clone()), mint_data)
                .await?;
            self.data.count += 1;
        }
        warn!("Reached max count. Halting behavior.");
        Ok(ControlFlow::Halt)
    }
}
