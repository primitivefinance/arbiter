use arbiter_bindings::bindings::arbiter_token::TransferFilter;
use arbiter_core::events::stream_event;
use arbiter_engine::machine::{Processing, Processor, State};
use token_admin::{MintRequest, TokenAdminQuery};

use super::*;

pub struct TokenRequesterData {
    pub messager: Messager,
    pub client: Arc<ArbiterMiddleware>,
}

/// The token requester is responsible for requesting tokens from the token
/// admin. This agents is purely for testing purposes as far as I can tell.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct TokenRequester<S: State> {
    /// The tokens that the token requester has requested.
    pub token_data: TokenData,
    /// The agent ID to request tokens to.
    pub request_to: String,
    #[serde(default)]
    pub count: u64,
    #[serde(default = "default_max_count")]
    pub max_count: Option<u64>,
    #[serde(default)]
    pub data: S::Data,
}

pub fn default_max_count() -> Option<u64> {
    Some(3)
}

#[async_trait::async_trait]
impl Behavior<TransferFilter> for TokenRequester<Configuration<()>> {
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
        println!("Sent message to {}", self.request_to.clone());
        let message = messager.get_next().await.unwrap();
        println!("{:?}", message.data);
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
        };
        Ok(Some((
            TokenRequester {
                token_data: self.token_data.clone(),
                request_to: self.request_to.clone(),
                count: 0,
                max_count: self.max_count,
                data,
            },
            stream,
        )))
    }
}

#[async_trait::async_trait]
impl Processor<TransferFilter> for TokenRequester<Processing<TokenRequesterData>> {
    async fn process(&mut self, _event: TransferFilter) -> Result<ControlFlow> {
        while self.count < self.max_count.unwrap() {
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
            self.count += 1;
        }
        warn!("Reached max count. Halting behavior.");
        Ok(ControlFlow::Halt)
    }
}

// #[async_trait::async_trait]
// impl Behavior<TransferFilter> for TokenRequester<Configuration> {
//     #[tracing::instrument(skip(self), fields(id = messager.id.as_deref()))]
//     async fn startup(
//         &mut self,
//         client: Arc<ArbiterMiddleware>,
//         mut messager: Messager,
//     ) -> Result<Option<EventStream<TransferFilter>>> {
//         messager
//             .send(
//                 To::Agent(self.request_to.clone()),
//                 &TokenAdminQuery::AddressOf(self.token_data.name.clone()),
//             )
//             .await?;
//         let message = messager.get_next().await.unwrap();
//         let token_address = serde_json::from_str::<eAddress>(&message.data).unwrap();
//         let token = ArbiterToken::new(token_address, client.clone());
//         self.token_data.address = Some(token_address);

//         let mint_data = TokenAdminQuery::MintRequest(MintRequest {
//             token: self.token_data.name.clone(),
//             mint_to: client.address(),
//             mint_amount: 1,
//         });
//         messager
//             .send(To::Agent(self.request_to.clone()), mint_data)
//             .await?;

//         self.messager = Some(messager.clone());
//         self.client = Some(client.clone());
//         let transfer_stream = stream_event(token.transfer_filter());
//         Ok(Some(transfer_stream))
//     }

//     #[tracing::instrument(skip(self), fields(id =
//      self.messager.as_ref().unwrap().id.as_deref()))]
//     async fn process(&mut self, _event: TransferFilter) -> Result<ControlFlow> {
//
//     }
// }
