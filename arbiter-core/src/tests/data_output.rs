use serde::Serialize;

use super::*;
use crate::data_collection::EventLogger;

#[derive(Serialize, Clone)]
struct MockMetadata {
    pub name: String,
}

#[tokio::test]
async fn data_capture() {
    let (env, client) = startup_user_controlled().unwrap();
    let (arbx, arby, lex) = deploy_liquid_exchange(client.clone()).await.unwrap();
    println!("Deployed contracts");

    let metadata = MockMetadata {
        name: "test".to_string(),
    };

    let listener = EventLogger::builder()
        .add(arbx.events(), "arbx")
        .add(arby.events(), "arby")
        .add(lex.events(), "lex")
        .metadata(metadata)
        .unwrap();

    listener.run().unwrap();

    for _ in 0..5 {
        arbx.approve(client.address(), U256::from(1))
            .send()
            .await
            .unwrap()
            .await
            .unwrap();
        arby.approve(client.address(), U256::from(1))
            .send()
            .await
            .unwrap()
            .await
            .unwrap();
        lex.set_price(U256::from(10u128.pow(18)))
            .send()
            .await
            .unwrap()
            .await
            .unwrap();
    }

    env.stop().unwrap();
    let path = std::env::current_dir().unwrap().join("data");
    // std::fs::remove_dir_all(path).unwrap();
}
