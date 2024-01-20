use std::{str::FromStr, time::Duration};

use anyhow::Context;
use arbiter_bindings::bindings::arbiter_token;
use arbiter_core::data_collection::EventLogger;
use ethers::{
    abi::token,
    types::{transaction::request, Filter},
};
use tokio::time::timeout;
use tracing::error;

use self::machine::MachineHalt;
use super::*;
use crate::{
    agent::Agent,
    machine::{Behavior, MachineInstruction, StateMachine},
    messager::To,
    world::World,
};

const TOKEN_ADMIN_ID: &str = "token_admin";
const REQUESTER_ID: &str = "requester";
const TOKEN_NAME: &str = "Arbiter Token";
const TOKEN_SYMBOL: &str = "ARB";
const TOKEN_DECIMALS: u8 = 18;

/// The token admin is responsible for handling token minting requests.
#[derive(Debug)]
pub struct TokenAdmin {
    /// The identifier of the token admin.
    pub token_data: HashMap<String, TokenData>,

    pub tokens: Option<HashMap<String, ArbiterToken<RevmMiddleware>>>,

    // TODO: We should not have to have a client or a messager put here
    // explicitly, they should come from the Agent the behavior is given to.
    pub client: Arc<RevmMiddleware>,
    pub messager: Messager,

    count: u64,

    max_count: Option<u64>,
}

impl TokenAdmin {
    pub fn new(
        client: Arc<RevmMiddleware>,
        messager: Messager,
        count: u64,
        max_count: Option<u64>,
    ) -> Self {
        Self {
            token_data: HashMap::new(),
            tokens: None,
            client,
            messager,
            count,
            max_count,
        }
    }

    /// Adds a token to the token admin.
    pub fn add_token(&mut self, token_data: TokenData) {
        self.token_data.insert(token_data.name.clone(), token_data);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub address: Option<Address>,
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
    pub mint_to: Address,

    /// The amount to mint.
    pub mint_amount: u64,
}

#[async_trait::async_trait]
impl Behavior<Message> for TokenAdmin {
    #[tracing::instrument(skip(self), fields(id =
self.messager.id.as_deref()))]
    async fn sync(&mut self) {
        for token_data in self.token_data.values_mut() {
            let token = ArbiterToken::deploy(
                self.client.clone(),
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
            self.tokens
                .get_or_insert_with(HashMap::new)
                .insert(token_data.name.clone(), token.clone());
            debug!("Deployed token: {:?}", token);
        }
    }

    #[tracing::instrument(skip(self), fields(id =
self.messager.id.as_deref()))]
    async fn process(&mut self, event: Message) -> Option<MachineHalt> {
        if self.tokens.is_none() {
            error!(
                "There were no tokens to deploy! You must add tokens to
the token admin before running the simulation."
            );
        }

        let query: TokenAdminQuery = serde_json::from_str(&event.data).unwrap();
        trace!("Got query: {:?}", query);
        match query {
            TokenAdminQuery::AddressOf(token_name) => {
                trace!(
                    "Getting address of token with name: {:?}",
                    token_name.clone()
                );
                let token_data = self.token_data.get(&token_name).unwrap();
                let message = Message {
                    from: self.messager.id.clone().unwrap(),
                    to: To::Agent(event.from.clone()), // Reply back to sender
                    data: serde_json::to_string(token_data).unwrap(),
                };
                self.messager.send(message).await;
            }
            TokenAdminQuery::MintRequest(mint_request) => {
                trace!("Minting tokens: {:?}", mint_request);
                let token = self
                    .tokens
                    .as_ref()
                    .unwrap()
                    .get(&mint_request.token)
                    .unwrap();
                token
                    .mint(mint_request.mint_to, U256::from(mint_request.mint_amount))
                    .send()
                    .await
                    .unwrap()
                    .await
                    .unwrap();
                self.count += 1;
                if self.count == self.max_count.unwrap_or(u64::MAX) {
                    warn!("Reached max count. Halting behavior.");
                    return Some(MachineHalt);
                }
            }
        }
        None
    }
}

/// The token requester is responsible for requesting tokens from the token
/// admin. This agents is purely for testing purposes as far as I can tell.
#[derive(Debug)]
pub struct TokenRequester {
    /// The tokens that the token requester has requested.
    pub token_data: TokenData,

    /// The agent ID to request tokens to.
    pub request_to: String,

    /// Client to have an address to receive token mint to and check balance
    pub client: Arc<RevmMiddleware>,

    /// The messaging layer for the token requester.
    pub messager: Messager,

    pub count: u64,

    pub max_count: Option<u64>,
}

impl TokenRequester {
    pub fn new(
        client: Arc<RevmMiddleware>,
        messager: Messager,
        count: u64,
        max_count: Option<u64>,
    ) -> Self {
        Self {
            token_data: TokenData {
                name: TOKEN_NAME.to_owned(),
                symbol: TOKEN_SYMBOL.to_owned(),
                decimals: TOKEN_DECIMALS,
                address: None,
            },
            request_to: TOKEN_ADMIN_ID.to_owned(),
            client,
            messager,
            count,
            max_count,
        }
    }
}

#[async_trait::async_trait]
impl Behavior<Message> for TokenRequester {
    #[tracing::instrument(skip(self), fields(id =
self.messager.id.as_deref()))]
    async fn startup(&mut self) {
        trace!("Requesting address of token: {:?}", self.token_data.name);
        let message = Message {
            from: self.messager.id.clone().unwrap(),
            to: To::Agent(self.request_to.clone()),
            data: serde_json::to_string(&TokenAdminQuery::AddressOf(self.token_data.name.clone()))
                .unwrap(),
        };
        self.messager.send(message).await;
    }

    #[tracing::instrument(skip(self), fields(id =
self.messager.id.as_deref()))]
    async fn process(&mut self, event: Message) -> Option<MachineHalt> {
        if let Ok(token_data) = serde_json::from_str::<TokenData>(&event.data) {
            trace!(
                "Got
token data: {:?}",
                token_data
            );
            trace!(
                "Requesting first mint of
token: {:?}",
                self.token_data.name
            );
            let message = Message {
                from: self.messager.id.clone().unwrap(),
                to: To::Agent(self.request_to.clone()),
                data: serde_json::to_string(&TokenAdminQuery::MintRequest(MintRequest {
                    token: self.token_data.name.clone(),
                    mint_to: self.client.address(),
                    mint_amount: 1,
                }))
                .unwrap(),
            };
            self.messager.send(message).await;
        }
        Some(MachineHalt)
    }
}

#[async_trait::async_trait]
impl Behavior<arbiter_token::TransferFilter> for TokenRequester {
    #[tracing::instrument(skip(self), fields(id =
self.messager.id.as_deref()))]
    async fn process(&mut self, event: arbiter_token::TransferFilter) -> Option<MachineHalt> {
        trace!(
            "Got event for
`TokenRequester` logger: {:?}",
            event
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
        let message = Message {
            from: self.messager.id.clone().unwrap(),
            to: To::Agent(self.request_to.clone()),
            data: serde_json::to_string(&TokenAdminQuery::MintRequest(MintRequest {
                token: self.token_data.name.clone(),
                mint_to: self.client.address(),
                mint_amount: 1,
            }))
            .unwrap(),
        };
        self.messager.send(message).await;
        self.count += 1;
        if self.count == self.max_count.unwrap_or(u64::MAX) {
            warn!("Reached max count. Halting behavior.");
            return Some(MachineHalt);
        }
        None
    }
}

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn token_minter_simulation() {
    // std::env::set_var("RUST_LOG", "trace");
    // tracing_subscriber::fmt::init();

    let mut world = World::new("test_world");

    // Create the token admin agent
    let token_admin = Agent::new(TOKEN_ADMIN_ID, &world);
    let mut token_admin_behavior = TokenAdmin::new(
        token_admin.client.clone(),
        token_admin
            .messager
            .as_ref()
            .unwrap()
            .join_with_id(Some(TOKEN_ADMIN_ID.to_owned())),
        0,
        Some(4),
    );
    token_admin_behavior.add_token(TokenData {
        name: TOKEN_NAME.to_owned(),
        symbol: TOKEN_SYMBOL.to_owned(),
        decimals: TOKEN_DECIMALS,
        address: None,
    });
    world.add_agent(token_admin.with_behavior(token_admin_behavior));

    // Create the token requester agent
    let token_requester = Agent::new(REQUESTER_ID, &world);
    let token_requester_behavior = TokenRequester::new(
        token_requester.client.clone(),
        token_requester
            .messager
            .as_ref()
            .unwrap()
            .join_with_id(Some(REQUESTER_ID.to_owned())),
        0,
        Some(4),
    );
    let arb = ArbiterToken::new(
        Address::from_str("0x240a76d4c8a7dafc6286db5fa6b589e8b21fc00f").unwrap(),
        token_requester.client.clone(),
    );
    let transfer_event = arb.transfer_filter();

    let token_requester_behavior_again = TokenRequester::new(
        token_requester.client.clone(),
        token_requester
            .messager
            .as_ref()
            .unwrap()
            .join_with_id(Some(REQUESTER_ID.to_owned())),
        0,
        Some(4),
    );
    world.add_agent(
        token_requester
            .with_behavior::<Message>(token_requester_behavior)
            .with_behavior::<arbiter_token::TransferFilter>(token_requester_behavior_again)
            .with_event(transfer_event),
    );

    let transfer_stream = EventLogger::builder()
        .add_stream(arb.transfer_filter())
        .stream()
        .unwrap();
    let mut stream = Box::pin(transfer_stream);
    let mut idx = 0;

    world.run().await;

    loop {
        match timeout(Duration::from_secs(1), stream.next()).await {
            Ok(Some(event)) => {
                println!("Event received in outside world: {:?}", event);
                idx += 1;
                if idx == 4 {
                    break;
                }
            }
            _ => {
                panic!("Timeout reached. Test failed.");
            }
        }
    }
}
