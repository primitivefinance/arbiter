use super::*;
use crate::data_collection::EventCapture;

#[tokio::test]
async fn data_capture() {
    let (_manager, client) = startup_user_controlled().unwrap();
    let arbx = deploy_arbx(client.clone()).await.unwrap();
    let event_capture = EventCapture::new(arbx.approval_filter());
    event_capture.run().await.unwrap();

    for _ in 0..10 {
        arbx.approve(client.address(), U256::from(1))
            .send()
            .await
            .unwrap()
            .await
            .unwrap();
    }
}
