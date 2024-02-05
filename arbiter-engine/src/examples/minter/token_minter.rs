use std::{str::FromStr, time::Duration};

use agents::{token_admin::TokenAdmin, token_requester::TokenRequester};
use arbiter_core::data_collection::EventLogger;
use arbiter_macros::Behaviors;
use ethers::types::Address;
use tokio::time::timeout;

use super::*;
use crate::world::World;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn token_minter_simulation() {
    let mut world = World::new("test_world");
    let client = RevmMiddleware::new(&world.environment, None).unwrap();

    // Create the token admin agent
    let token_admin = Agent::builder(TOKEN_ADMIN_ID);
    let mut token_admin_behavior = TokenAdmin::new(Some(4));
    token_admin_behavior.add_token(TokenData {
        name: TOKEN_NAME.to_owned(),
        symbol: TOKEN_SYMBOL.to_owned(),
        decimals: TOKEN_DECIMALS,
        address: None,
    });
    // Create the token requester agent
    let token_requester = Agent::builder(REQUESTER_ID);
    let mut token_requester_behavior = TokenRequester::new(Some(4));
    world.add_agent(token_requester.with_behavior(token_requester_behavior));

    world.add_agent(token_admin.with_behavior(token_admin_behavior));

    let arb = ArbiterToken::new(
        Address::from_str("0x240a76d4c8a7dafc6286db5fa6b589e8b21fc00f").unwrap(),
        client.clone(),
    );
    let transfer_event = arb.transfer_filter();

    let transfer_stream = EventLogger::builder()
        .add_stream(arb.transfer_filter())
        .stream()
        .unwrap();
    let mut stream = Box::pin(transfer_stream);
    world.run().await;
    let mut idx = 0;

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

#[derive(Serialize, Deserialize, Debug, Behaviors)]
enum Behaviors {
    TokenAdmin(TokenAdmin),
    TokenRequester(TokenRequester),
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn config_test() {
    std::env::set_var("RUST_LOG", "trace");
    tracing_subscriber::fmt::init();
    tracing::info!("Starting config_test");
    let mut world = World::new("world");
    world.build_with_config::<Behaviors>("src/examples/minter/config.toml");

    world.run().await;
}
