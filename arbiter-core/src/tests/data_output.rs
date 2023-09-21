use super::*;
use crate::data_collection::EventCapture;
use ethers::{
    contract::{EthEvent, EthLogDecode, Event},
    providers::{Middleware, MiddlewareError, StreamExt},
    types::Filter,
};
use futures_util::SinkExt;

#[tokio::test]
async fn data_capture() {
    let (mut manager, client) = startup_user_controlled().unwrap();
    let arbx = deploy_arbx(client.clone()).await.unwrap();
    let event_capture = EventCapture::new(client.clone(), arbx.approval_filter());
    event_capture.run().await.unwrap();
    // let approval = arbx.approval_filter();
    // let mut stream = approval.stream().await.unwrap();

    for _ in 0..10 {
        arbx.approve(client.address(), U256::from(1))
            .send()
            .await
            .unwrap()
            .await
            .unwrap();
    }

    // println!("next approval: {:?}", stream.next().await);
    let environment = manager.environments.get(TEST_ENV_LABEL).unwrap();
    let mut guard = environment.captures.lock().unwrap();
    for (handle, mut sender) in guard.drain(..) {
        sender.send(()).await.unwrap();
        // handle.join();
    }
    println!("guard: {:?}", guard);
    // manager.stop_environment(TEST_ENV_LABEL).unwrap();
}
