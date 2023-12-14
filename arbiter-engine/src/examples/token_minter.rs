// use super::*;
// use artemis_core::{
//     collectors::log_collector::LogCollector, executors::mempool_executor::MempoolExecutor,
// };
// use ethers::{
//     providers::Middleware,
//     types::{Filter, Log},
// };
// use futures_util::StreamExt;
// use tracing::error;

// /// The token admin is responsible for handling token minting requests.
// pub struct TokenAdmin {
//     /// The identifier of the token admin.
//     pub id: String, // TODO: The strategies should not really need an ID.

//     pub token_data: HashMap<String, TokenData>,

//     /// The tokens that the token admin has control over.
//     /// These will be deployed when we call `sync_state()`
//     pub tokens: Option<HashMap<String, ArbiterToken<RevmMiddleware>>>,

//     pub client: Arc<RevmMiddleware>,
// }

// impl TokenAdmin {
//     // TODO: I don't think we should pass in a client like this, probably, doing it for testing purposes. Also using RevmMiddleware for testing purposes, although this strategy should never be deployed
//     /// Creates a new token admin with the given identifier.
//     pub fn new(id: &str, client: Arc<RevmMiddleware>) -> Self {
//         Self {
//             id: id.to_owned(),
//             token_data: HashMap::new(),
//             tokens: None,
//             client,
//         }
//     }

//     /// Adds a token to the token admin.
//     pub fn add_token(&mut self, token_data: TokenData) {
//         self.token_data.insert(token_data.name.clone(), token_data);
//     }
// }

// pub struct TokenData {
//     pub name: String,
//     pub symbol: String,
//     pub decimals: u8,
//     pub address: Option<Address>,
// }

// /// Used as an action to mint tokens.
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct MintRequest {
//     /// The token to mint.
//     pub token: String,

//     /// The address to mint to.
//     pub mint_to: Address,

//     /// The amount to mint.
//     pub mint_amount: u64,
// }

// /// Used as an action to ask what tokens are available.
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub enum TokenQuery {
//     /// Ask for address of specific token by name.
//     AddressOf(String),

//     /// Ask to be minted some a token.
//     Mint(MintRequest),
// }

// #[async_trait::async_trait]
// impl Strategy<Message, SubmitTxToMempool> for TokenAdmin {
//     #[tracing::instrument(skip(self), fields(id = %self.id))]
//     async fn sync_state(&mut self) -> Result<()> {
//         debug!("Syncing state for `TokenAdmin`.");
//         for token_data in self.token_data.values_mut() {
//             let token = ArbiterToken::deploy(
//                 self.client.clone(),
//                 (
//                     token_data.name.clone(),
//                     token_data.symbol.clone(),
//                     token_data.decimals,
//                 ),
//             )
//             .unwrap()
//             .send()
//             .await
//             .unwrap();
//             token_data.address = Some(token.address());
//             self.tokens
//                 .get_or_insert_with(HashMap::new)
//                 .insert(token_data.name.clone(), token.clone());
//             trace!("Deployed token: {:?}", token);
//         }
//         Ok(())
//     }

//     #[tracing::instrument(skip(self, event), fields(id = %self.id))]
//     async fn process_event(&mut self, event: Message) -> Vec<SubmitTxToMempool> {
//         if self.tokens.is_none() {
//             error!("There were no tokens to deploy! You must add tokens to the token admin before running the simulation.");
//         }

//         if event.to == self.id {
//             let token_query = serde_json::from_str::<TokenQuery>(&event.data).unwrap();

//             match token_query {
//                 TokenQuery::AddressOf(token_name) => {
//                     let token = self.tokens.as_ref().unwrap().get(&token_name).unwrap();
//                     let tx = SubmitTxToMempool {
//                         tx: token.address().into(),
//                         gas_bid_info: None,
//                     };
//                     vec![tx]
//                 }
//                 TokenQuery::Mint(mint_request) => {
//                     trace!("Received a request to mint tokens.");
//                     let token = self
//                         .tokens
//                         .as_ref()
//                         .unwrap()
//                         .get(&mint_request.token)
//                         .unwrap();
//                     let tx = SubmitTxToMempool {
//                         tx: token
//                             .mint(mint_request.mint_to, U256::from(mint_request.mint_amount))
//                             .tx,
//                         gas_bid_info: None,
//                     };
//                     vec![tx]
//                 }
//             }
//         } else {
//             vec![]
//         }
//     }
// }

// /// The token requester is responsible for requesting tokens from the token admin.
// /// This agents is purely for testing purposes as far as I can tell.
// pub struct TokenRequester {
//     /// The tokens that the token requester has requested.
//     pub token_data: TokenData,

//     /// The agent ID to request tokens to.
//     pub request_to: String,

//     /// Client to have an address to receive token mint to and check balance
//     pub client: Arc<RevmMiddleware>,
// }

// #[async_trait::async_trait]
// impl Strategy<Log, Message> for TokenRequester {
//     async fn sync_state(&mut self) -> Result<()> {
//         Ok(())
//     }

//     async fn process_event(&mut self, event: Log) -> Vec<Message> {
//         println!("Got event: {:?}", event);
//         let message = Message {
//             from: "token_requester".to_owned(),
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
// #[ignore]
// #[tracing_test::traced_test]
// #[tokio::test]
// async fn token_minter_simulation() {
//     // TODO: Test outline, requester requests 1, 2, 3 tokens, and the test stops when the balance is checked to be 6.

//     let environment = EnvironmentBuilder::new().build();
//     let connection = Connection::from(&environment);
//     let provider = Provider::new(connection);
//     let mut world = World::new("test_world", provider);
//     let messager = world.messager.clone();

//     // Create the token admin agent
//     let mut token_admin = Agent::new("token_admin");
//     let token_admin_client = RevmMiddleware::new(&environment, Some("token_admin")).unwrap();
//     token_admin.add_collector(messager.clone());
//     token_admin.add_executor(MempoolExecutor::new(token_admin_client.clone()));
//     let mut token_admin_strategy = TokenAdmin::new("token_admin", token_admin_client);
//     token_admin_strategy.add_token(TokenData {
//         name: "Arbiter Token".to_owned(),
//         symbol: "ARB".to_owned(),
//         decimals: 18,
//         address: None,
//     });
//     token_admin.add_behavior(token_admin_strategy);

//     // Create the token requester agent
//     let mut requester = Agent::new("requester");
//     let requester_client = RevmMiddleware::new(&environment, Some("requester")).unwrap();
//     requester.add_collector(LogCollector::new(
//         requester_client.clone(),
//         Filter::default(),
//     ));
//     requester.add_executor(messager.clone());
//     let requester_strategy = TokenRequester {
//         token_data: TokenData {
//             name: "Arbiter Token".to_owned(),
//             symbol: "ARB".to_owned(),
//             decimals: 18,
//             address: None,
//         },
//         request_to: "token_admin".to_owned(),
//         client: requester_client,
//     };
//     requester.add_behavior(requester_strategy);

//     // Add agents to world
//     world.add_agent(token_admin);
//     world.add_agent(requester);

//     let world_task = tokio::spawn(async move { world.run().await });
//     world_task.await.unwrap();
// }
