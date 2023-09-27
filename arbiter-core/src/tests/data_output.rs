use super::*;
use crate::data_collection::EventCapture;
use ethers::{
    contract::{EthEvent, EthLogDecode, Event},
    providers::{Middleware, MiddlewareError, StreamExt},
    types::Filter,
};
use futures_util::SinkExt;
use stream_cancel::{StreamExt as StreamCancelExt, Tripwire};
use tracing::info;
use tracing_test::traced_test;

#[traced_test]
#[tokio::test]
async fn data_capture() {
    let (mut _manager, client) = startup_user_controlled().unwrap();
    let arbx = deploy_arbx(client.clone()).await.unwrap();
    let event_capture = EventCapture::new(arbx.events(), client.clone());

    let (tx, handle) = event_capture.run().await.unwrap();
    
    let handle2 = tokio::spawn(async move {
        for i in 0..5 {
            info!("Task 1: {}", i);
            arbx.approve(client.address(), U256::from(1))
                .send()
                .await
                .unwrap()
                .await
                .unwrap();
        }
        tx.send(()).unwrap();
        info!("Task 2: done");
    });
    tokio::join!(handle, handle2);
}
