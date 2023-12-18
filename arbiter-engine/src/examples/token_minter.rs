use artemis_core::executors::mempool_executor::MempoolExecutor;

use tracing::error;

use super::*;
use crate::agent::BehaviorBuilder;

const TOKEN_ADMIN_ID: &str = "token_admin";
const REQUESTER_ID: &str = "requester";
const TOKEN_NAME: &str = "Arbiter Token";
const TOKEN_SYMBOL: &str = "ARB";
const TOKEN_DECIMALS: u8 = 18;

/// The token admin is responsible for handling token minting requests.
#[derive(Clone, Debug)]
pub struct TokenAdmin {
    /// The identifier of the token admin.
    pub id: String, // TODO: The strategies should not really need an ID.

    pub token_data: HashMap<String, TokenData>,

    /// The tokens that the token admin has control over.
    /// These will be deployed when we call `sync_state()`
    pub tokens: Option<HashMap<String, ArbiterToken<RevmMiddleware>>>,

    pub client: Arc<RevmMiddleware>,
}

impl TokenAdmin {
    // TODO: I don't think we should pass in a client like this, probably, doing it
    // for testing purposes. Also using RevmMiddleware for testing purposes,
    // although this strategy should never be deployed
    /// Creates a new token admin with the given identifier.
    pub fn new(client: Arc<RevmMiddleware>) -> Self {
        Self {
            id: TOKEN_ADMIN_ID.to_owned(),
            token_data: HashMap::new(),
            tokens: None,
            client,
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
impl Strategy<Message, MessageOrTx> for TokenAdmin {
    #[tracing::instrument(skip(self), fields(id = %self.id))]
    async fn sync_state(&mut self) -> Result<()> {
        debug!("Syncing state for `TokenAdmin`.");
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
            trace!("Deployed token: {:?}", token);
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, event), fields(id = %self.id))]
    async fn process_event(&mut self, event: Message) -> Vec<MessageOrTx> {
        trace!("Processing event for `TokenAdmin` {:?}.", event);
        if self.tokens.is_none() {
            error!("There were no tokens to deploy! You must add tokens to the token admin before running the simulation.");
        }
        if event.to == self.id {
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
                        from: self.id.to_owned(),
                        to: event.from.clone(), // Reply back to sender
                        data: serde_json::to_string(token_data).unwrap(),
                    };
                    vec![MessageOrTx::Message(message)]
                }
                TokenAdminQuery::MintRequest(mint_request) => {
                    trace!("Minting tokens: {:?}", mint_request);
                    let token = self
                        .tokens
                        .as_ref()
                        .unwrap()
                        .get(&mint_request.token)
                        .unwrap();
                    let tx = SubmitTxToMempool {
                        tx: token
                            .mint(mint_request.mint_to, U256::from(mint_request.mint_amount))
                            .tx,
                        gas_bid_info: None,
                    };
                    vec![MessageOrTx::Tx(tx)]
                }
            }
        } else {
            vec![]
        }
    }
}

/// The token requester is responsible for requesting tokens from the token
/// admin. This agents is purely for testing purposes as far as I can tell.
#[derive(Clone, Debug)]
pub struct TokenRequester {
    /// The identifier of the token requester.
    pub id: String,

    /// The tokens that the token requester has requested.
    pub token_data: TokenData,

    /// The agent ID to request tokens to.
    pub request_to: String,

    /// Client to have an address to receive token mint to and check balance
    pub client: Arc<RevmMiddleware>,
}

impl TokenRequester {
    pub fn new(id: &str, client: Arc<RevmMiddleware>) -> Self {
        Self {
            id: id.to_owned(),
            token_data: TokenData {
                name: TOKEN_NAME.to_owned(),
                symbol: TOKEN_SYMBOL.to_owned(),
                decimals: TOKEN_DECIMALS,
                address: None,
            },
            request_to: TOKEN_ADMIN_ID.to_owned(),
            client,
        }
    }
}

#[async_trait::async_trait]
impl Strategy<Message, Message> for TokenRequester {
    async fn sync_state(&mut self) -> Result<()> {
        Ok(())
    }

    #[tracing::instrument(skip(self, event), fields(id = %self.id))]
    async fn process_event(&mut self, event: Message) -> Vec<Message> {
        trace!("Processing event for `TokenRequester` {:?}.", event);
        if event.to == self.id {
            if event.data == "Start" {
                trace!("Requesting address of token: {:?}", self.token_data.name);
                let message = Message {
                    from: self.id.to_owned(),
                    to: self.request_to.clone(),
                    data: serde_json::to_string(&TokenAdminQuery::AddressOf(
                        self.token_data.name.clone(),
                    ))
                    .unwrap(),
                };
                vec![message]
            } else if event.data == "Mint" {
                trace!("Requesting mint of token: {:?}", self.token_data.name);
                let message = Message {
                    from: self.id.to_owned(),
                    to: self.request_to.clone(),
                    data: serde_json::to_string(&TokenAdminQuery::MintRequest(MintRequest {
                        token: self.token_data.name.clone(),
                        mint_to: self.client.address(),
                        mint_amount: 1,
                    }))
                    .unwrap(),
                };
                vec![message]
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }
}

// #[async_trait::async_trait]
// impl Strategy<Log, Message> for TokenRequester {
//     async fn sync_state(&mut self) -> Result<()> {
//         Ok(())
//     }

//     async fn process_event(&mut self, event: Log) -> Vec<Message> {
//         println!("Got event: {:?}", event);
//         let message = Message {
//             from: "requester".to_owned(),
//             to: self.request_to.clone(),
//             data: serde_json::to_string(&MintRequest {
//                 token: self.token_data.name.clone(),
//                 mint_to: self.client.address(),
//                 mint_amount: 1,
//             })
//             .unwrap(),
//         };
//         vec![message]
//     }
// }

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn token_minter_simulation() {
    // TODO: Test outline, requester requests 1, 2, 3 tokens, and the test stops
    // when the balance is checked to be 6.
    std::env::set_var("RUST_LOG", "trace");
    tracing_subscriber::fmt::init();

    let environment = EnvironmentBuilder::new().build();
    let connection = Connection::from(&environment);
    let provider = Provider::new(connection);
    let mut world = World::new("test_world", provider);
    let messager = world.messager.clone();

    // Create the token admin agent
    let mut token_admin_agent = Agent::new(TOKEN_ADMIN_ID);
    let token_admin_client = RevmMiddleware::new(&environment, Some(TOKEN_ADMIN_ID)).unwrap();
    let mut token_admin_strategy = TokenAdmin::new(token_admin_client.clone());
    token_admin_strategy.add_token(TokenData {
        name: TOKEN_NAME.to_owned(),
        symbol: TOKEN_SYMBOL.to_owned(),
        decimals: TOKEN_DECIMALS,
        address: None,
    });

    let message_and_mempool_executor = MessageAndMempoolExecutor {
        messager: messager.clone(),
        mempool_executor: MempoolExecutor::new(token_admin_client.clone()),
    };

    let token_admin_behavior = BehaviorBuilder::new()
        .add_collector(messager.clone())
        .add_executor(message_and_mempool_executor)
        .add_strategy(token_admin_strategy.clone())
        .build();
    token_admin_agent.add_behavior(token_admin_behavior);

    // Create the token requester agent
    let mut requester_agent = Agent::new(REQUESTER_ID);
    let requester_client = RevmMiddleware::new(&environment, Some(REQUESTER_ID)).unwrap();
    let requester_behavior = TokenRequester::new(REQUESTER_ID, requester_client.clone());
    let requester_behavior = BehaviorBuilder::new()
        .add_collector(messager.clone())
        .add_executor(messager.clone())
        .add_strategy(requester_behavior)
        .build();
    requester_agent.add_behavior(requester_behavior);

    // Add agents to world
    world.add_agent(token_admin_agent);
    world.add_agent(requester_agent);

    // Run the world and send the start message
    let tasks = world.run().await;
    // std::thread::sleep(std::time::Duration::from_millis(100));
    let message = Message {
        from: "host".to_owned(),
        to: REQUESTER_ID.to_owned(),
        data: "Start".to_owned(),
    };
    let _send_result = messager.execute(message).await;

    for task in tasks {
        task.await.unwrap();
    }
}
