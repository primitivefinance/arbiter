use arbiter_bindings::bindings::arbiter_token::TransferFilter;
use arbiter_core::events::stream_event;
use arbiter_engine::machine::{Processing, Processor, State};
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

pub trait Config {
    fn max_count(&self) -> u64;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenRequesterConfig {
    pub max_count: u64,
}

impl Config for TokenRequesterConfig {
    fn max_count(&self) -> u64 {
        self.max_count
    }
}

pub struct TokenRequesterData {
    pub messager: Messager,
    pub client: Arc<ArbiterMiddleware>,
    pub count: u64,
    pub max_count: u64,
}

#[async_trait::async_trait]
impl Behavior<TransferFilter> for TokenRequester<Configuration<TokenRequesterConfig>> {
    type Processor = TokenRequester<Processing<TokenRequesterData>>;

    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        mut messager: Messager,
    ) -> Result<Option<(Self::Processor, EventStream<TransferFilter>)>> {
        messager
            .send(
                To::Agent(self.request_to.clone()),
                TokenAdminQuery::AddressOf(self.token_data.name.clone()),
            )
            .await?;
        let message = messager.get_next().await.unwrap();
        let token_address = serde_json::from_str::<eAddress>(&message.data).unwrap();
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

        let stream = stream_event(token.transfer_filter());
        let data = TokenRequesterData {
            messager: messager.clone(),
            client,
            count: 0,
            max_count: self.data.max_count(),
        };
        Ok(Some((
            TokenRequester {
                token_data: self.token_data.clone(),
                request_to: self.request_to.clone(),
                data,
            },
            stream,
        )))
    }
}

#[async_trait::async_trait]
impl Processor<TransferFilter> for TokenRequester<Processing<TokenRequesterData>> {
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
