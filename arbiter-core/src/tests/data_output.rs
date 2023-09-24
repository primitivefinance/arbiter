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
    let (mut manager, client) = startup_user_controlled().unwrap();
    let arbx = deploy_arbx(client.clone()).await.unwrap();
    let event_capture = EventCapture::new(arbx.events());

    let (trigger, tripwire) = Tripwire::new();


    let handle2 = tokio::spawn(async move {
        for i in 0..10 {
            println!("Task 2: {}", i);
            arbx.approve(client.address(), U256::from(1))
                .send()
                .await
                .unwrap()
                .await
                .unwrap();
        }
        info!("Task 2: done");
        drop(trigger);
    });
    tokio::join!(event_capture.run(tripwire), handle2);


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
