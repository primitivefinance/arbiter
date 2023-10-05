use tokio::io::AsyncReadExt;
use tracing_test::traced_test;

use super::*;
use crate::data_collection::EventLogger;

#[traced_test]
#[tokio::test(flavor = "multi_thread")]
async fn data_capture() {
    for _ in 0..10 {
        let (mut _env, client) = startup_user_controlled().unwrap();
        let (arbx, arby, lex) = deploy_liquid_exchange(client.clone()).await.unwrap();

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
    }
}

#[traced_test]
#[tokio::test(flavor = "multi_thread")]
async fn data_capture_output_validation() {
    for i in 0..10 {
        println!("Test iteration {}", i);
        let (mut _env, client) = startup_user_controlled().unwrap();
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

        let mut file0 = tokio::fs::File::open("./test_output2/arbx/ApprovalFilter.csv")
            .await
            .unwrap();
        let mut contents0 = vec![];
        file0.read_to_end(&mut contents0).await.unwrap();
        let contents0 = String::from_utf8(contents0).unwrap();

        let mut file1 = tokio::fs::File::open("./test_output2/arby/ApprovalFilter.csv")
            .await
            .unwrap();
        let mut contents1 = vec![];
        file1.read_to_end(&mut contents1).await.unwrap();
        let contents1 = String::from_utf8(contents1).unwrap();

        assert_eq!(contents0, contents1);
    }
}
