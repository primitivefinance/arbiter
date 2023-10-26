use tokio::io::AsyncReadExt;
use tracing_test::traced_test;

use super::*;
use crate::data_collection::EventLogger;

#[traced_test]
#[tokio::test(flavor = "multi_thread")]
async fn data_capture() {
    let (mut env, client) = startup_user_controlled().unwrap();
    let (arbx, arby, lex) = deploy_liquid_exchange(client.clone()).await.unwrap();
    println!("Deployed contracts");

    let listener = EventLogger::builder()
        .path("./test_output1")
        .add(arbx.events(), "arbx")
        .add(arby.events(), "arby")
        .add(lex.events(), "lex");

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

    env.stop();
}

#[traced_test]
#[tokio::test(flavor = "multi_thread")]
async fn data_capture_output_validation() {
    let (mut env, client) = startup_user_controlled().unwrap();
    let (arbx, arby, lex) = deploy_liquid_exchange(client.clone()).await.unwrap();
    let listener = EventLogger::builder()
        .path("./test_output2/")
        .add(arbx.events(), "arbx")
        .add(arby.events(), "arby")
        .add(lex.events(), "lex");

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

    env.stop();
}
