use super::*;
use crate::data_collection::EventCapture;
use futures_util::SinkExt;

#[tokio::test]
async fn data_capture() {
    let (_manager, client) = startup_user_controlled().unwrap();
    let arbx = deploy_arbx(client.clone()).await.unwrap();
    let event_capture = EventCapture::new(client.clone(), arbx.approval_filter());
    let (handle, mut sender) = event_capture.run().await.unwrap();
    for index in 0..5 {
        arbx.approve(client.address(), U256::from(index + 1))
            .send()
            .await
            .unwrap()
            .await
            .unwrap();
        println!("Number of transactions sent: {:?}", index + 1);
    }
    sender.send(()).await.unwrap();
    handle.join();
}
