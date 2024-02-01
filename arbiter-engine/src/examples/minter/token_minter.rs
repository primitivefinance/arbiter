use super::*;
use crate::world::World;
use agents::{token_admin::TokenAdmin, token_requester::TokenRequester};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn token_minter_simulation() {
    std::env::set_var("RUST_LOG", "trace");
    tracing_subscriber::fmt::init();
    // 3. have a method on world to update mutable map of addresses
    let mut world = World::new("test_world");
    // self.contracts: HashMap<String, ContrantInstance>

    let token_requester = Agent::builder(REQUESTER_ID).unwrap();
    let mut token_requester_behavior = TokenRequester::new(Some(4));
    world.add_agent(token_requester.with_behavior(token_requester_behavior));

    // Create the token admin agent
    let token_admin = Agent::builder(TOKEN_ADMIN_ID).unwrap();
    let mut token_admin_behavior = TokenAdmin::new(Some(4));
    token_admin_behavior.add_token(TokenData {
        name: TOKEN_NAME.to_owned(),
        symbol: TOKEN_SYMBOL.to_owned(),
        decimals: TOKEN_DECIMALS,
        address: None,
    });
    world.add_agent(token_admin.with_behavior(token_admin_behavior));

    world.run().await;
    // 2. appropriately handle event driven behaviors
    // let arb = ArbiterToken::new(
    // Address::from_str("0x240a76d4c8a7dafc6286db5fa6b589e8b21fc00f").unwrap(),
    // // token_requester.client.clone(), );
    // let transfer_event = arb.transfer_filter();
    //
    // let token_requester_behavior_again = TokenRequester::new(0, Some(4));
    // world.add_agent(
    // token_requester
    // .with_behavior::<Message>(token_requester_behavior)
    // .with_behavior::<arbiter_token::TransferFilter>(token_requester_behavior_again)
    // .with_event(transfer_event),
    // );
    //
    // let transfer_stream = EventLogger::builder()
    // .add_stream(arb.transfer_filter())
    // .stream()
    // .unwrap();
    // let mut stream = Box::pin(transfer_stream);
    // let mut idx = 0;
    //
    // world.run().await;
    //
    // loop {
    // match timeout(Duration::from_secs(1), stream.next()).await {
    // Ok(Some(event)) => {
    // println!("Event received in outside world: {:?}", event);
    // idx += 1;
    // if idx == 4 {
    // break;
    // }
    // }
    // _ => {
    // panic!("Timeout reached. Test failed.");
    // }
    // }
    // }
}
