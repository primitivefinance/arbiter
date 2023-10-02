use super::*;
use crate::data_collection::EventLogger;
use tracing::info;
use tracing_test::traced_test;

#[traced_test]
#[tokio::test(flavor = "multi_thread")]
async fn data_capture() {
    let (mut _manager, client) = startup_user_controlled().unwrap();
    let arbx = deploy_arbx(client.clone()).await.unwrap();
    let arby = deploy_arbx(client.clone()).await.unwrap();
    let listener = EventLogger::builder()
        .add(arbx.events(), "arbx")
        .add(arby.events(), "arby");

    listener.run().await.unwrap();

    for i in 0..5 {
        info!("Task 1: {}", i);
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
    }
}

#[traced_test]
#[tokio::test(flavor = "multi_thread")]
async fn data_capture_with_path() {
    let (mut _manager, client) = startup_user_controlled().unwrap();
    let arbx = deploy_arbx(client.clone()).await.unwrap();
    let arby = deploy_arbx(client.clone()).await.unwrap();
    let listener = EventLogger::builder()
        .add(arbx.events(), "arbx")
        .add(arby.events(), "arby")
        .path("./test_output/");

    listener.run().await.unwrap();

    for i in 0..5 {
        info!("Task 1: {}", i);
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
    }
}
