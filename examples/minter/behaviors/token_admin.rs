use std::collections::HashMap;

use arbiter_engine::machine::{Processing, Processor, State};

use super::*;

#[derive(Debug, Clone)]
pub struct TokenAdminData {
    pub messager: Messager,
    pub client: Arc<ArbiterMiddleware>,
    pub tokens: HashMap<String, ArbiterToken<ArbiterMiddleware>>,
}

pub struct TokenAdminConfig {
    pub token_data: HashMap<String, TokenData>,
    pub count: u64,
    pub max_count: Option<u64>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct TokenAdmin<S: State> {
    /// The identifier of the token admin.
    pub token_data: HashMap<String, TokenData>,
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

/// Used as an action to ask what tokens are available.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TokenAdminQuery {
    /// Get the address of the token.
    AddressOf(String),

    /// Mint tokens.
    MintRequest(MintRequest),
}

/// Used as an action to mint tokens.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MintRequest {
    /// The token to mint.
    pub token: String,

    /// The address to mint to.
    pub mint_to: eAddress,

    /// The amount to mint.
    pub mint_amount: u64,
}

#[async_trait::async_trait]
impl Behavior<Message> for TokenAdmin<Configuration> {
    type Processor = TokenAdmin<Processing<TokenAdminData>>;

    #[tracing::instrument(skip(self), fields(id = messager.id.as_deref()))]
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<(Self::Processor, EventStream<Message>)>> {
        let mut processor = TokenAdmin::<Processing<TokenAdminData>> {
            token_data: self.token_data.clone(),
            count: self.count,
            max_count: self.max_count,
            data: TokenAdminData {
                messager: messager.clone(),
                client: client.clone(),
                tokens: HashMap::new(),
            },
        };
        for token_data in self.token_data.values_mut() {
            let token = ArbiterToken::deploy(
                client.clone(),
                (
                    token_data.name.clone(),
                    token_data.symbol.clone(),
                    token_data.decimals,
                ),
            )
            .unwrap()
            .send()
            .await
            .unwrap();

            token_data.address = Some(token.address());
            processor
                .data
                .tokens
                .insert(token_data.name.clone(), token.clone());
            processor
                .token_data
                .get_mut(&token_data.name)
                .unwrap()
                .address = Some(token.address());
        }
        Ok(Some((processor, messager.stream()?)))
    }
}

#[async_trait::async_trait]
impl Processor<Message> for TokenAdmin<Processing<TokenAdminData>> {
    #[tracing::instrument(skip(self), fields(id = self.data.messager.id.as_deref()))]
    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        let query: TokenAdminQuery = serde_json::from_str(&event.data).unwrap();
        trace!("Got query: {:?}", query);
        match query {
            TokenAdminQuery::AddressOf(token_name) => {
                trace!(
                    "Getting address of token with name: {:?}",
                    token_name.clone()
                );
                let token_data = self.token_data.get(&token_name).unwrap();
                println!("Got the token data: {:?}", token_data);
                self.data
                    .messager
                    .send(To::Agent(event.from.clone()), token_data.address.unwrap())
                    .await?;
            }
            TokenAdminQuery::MintRequest(mint_request) => {
                trace!("Minting tokens: {:?}", mint_request);
                let token = self.data.tokens.get(&mint_request.token).unwrap();
                token
                    .mint(mint_request.mint_to, eU256::from(mint_request.mint_amount))
                    .send()
                    .await
                    .unwrap()
                    .await
                    .unwrap();
                self.count += 1;
                if self.count == self.max_count.unwrap_or(u64::MAX) {
                    warn!("Reached max count. Halting behavior.");
                    return Ok(ControlFlow::Halt);
                }
            }
        }
        Ok(ControlFlow::Continue)
    }
}
