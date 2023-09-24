use super::*;
use crate::data_collection::EventCapture;
use ethers::{
    contract::{EthEvent, EthLogDecode, Event},
    providers::{Middleware, MiddlewareError, StreamExt},
    types::Filter,
};
use futures_util::SinkExt;
use tokio_util::sync::CancellationToken;
use tokio::sync::Barrier;

#[tokio::test]
async fn data_capture() {
    let (mut manager, client) = startup_user_controlled().unwrap();
    let arbx = deploy_arbx(client.clone()).await.unwrap();
    let event_capture = EventCapture::new(arbx.events());

    let token = CancellationToken::new();
    let token_clone = token.clone();

    let handle1 = tokio::spawn(async move {
        tokio::select! {
            _ = token_clone.cancelled() => {
                println!("Task 1 cancelled");
            }
            _ = event_capture.run() => {
                println!("Task 1 finished");
            }
        }
    });

    let handle2 = tokio::spawn(async move {
        for i in 0..1000 {
            println!("Task 2: {}", i);
            arbx.approve(client.address(), U256::from(1))
                .send()
                .await
                .unwrap()
                .await
                .unwrap();
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        println!("done sleeping!");
        token.cancel();
    });
    tokio::try_join!(handle1, handle2);


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
