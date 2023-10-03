use super::*;
use crate::data_collection::EventLogger;
use tokio::io::AsyncReadExt;
use tracing::info;
use tracing_test::traced_test;

#[traced_test]
#[tokio::test(flavor = "multi_thread")]
async fn data_capture() {
    let (mut _env, client) = startup_user_controlled().unwrap();
    let arbx = deploy_arbx(client.clone()).await.unwrap();
    let arby = deploy_arbx(client.clone()).await.unwrap();
    let listener = EventLogger::builder()
        .add(arbx.events(), "arbx")
        .add(arby.events(), "arby");

    listener.run().await.unwrap();

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
    }
    // check if dir exists
    tokio::fs::remove_dir_all("./events").await.unwrap();
}

#[traced_test]
#[tokio::test(flavor = "multi_thread")]
async fn data_capture_output_validation() {
    let (mut _env, client) = startup_user_controlled().unwrap();
    let arbx = deploy_arbx(client.clone()).await.unwrap();
    let arby = deploy_arbx(client.clone()).await.unwrap();
    let listener = EventLogger::builder()
        .add(arbx.events(), "arbx")
        .add(arby.events(), "arby")
        .path("./test_output/");

    listener.run().await.unwrap();

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
    }

    let mut file0 = tokio::fs::File::open("./test_output/arbx/ApprovalFilter.csv")
        .await
        .unwrap();
    let mut contents0 = vec![];
    file0.read_to_end(&mut contents0).await.unwrap();
    let contents0 = String::from_utf8(contents0).unwrap();

    let mut file1 = tokio::fs::File::open("./test_output/arby/ApprovalFilter.csv")
        .await
        .unwrap();
    let mut contents1 = vec![];
    file1.read_to_end(&mut contents1).await.unwrap();
    let contents1 = String::from_utf8(contents1).unwrap();

    assert_eq!(contents0, contents1);

    tokio::fs::remove_dir_all("./test_output").await.unwrap();
}
