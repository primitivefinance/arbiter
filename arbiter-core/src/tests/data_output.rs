use std::path::Path;

use serde::Serialize;

use super::*;
use crate::{
    data_collection::{EventLogger, OutputFileType},
    middleware::errors::RevmMiddlewareError,
};

#[derive(Serialize, Clone)]
struct MockMetadata {
    pub name: String,
}

#[tokio::test]
async fn data_capture() {
    let (env, client) = startup_user_controlled().unwrap();
    let (arbx, arby, lex) = deploy_liquid_exchange(client.clone()).await.unwrap();
    println!("Deployed contracts");

    // default_listener
    EventLogger::builder()
        .add(arbx.events(), "arbx")
        .add(arby.events(), "arby")
        .add(lex.events(), "lex")
        .run()
        .unwrap();

    let metadata = MockMetadata {
        name: "test".to_string(),
    };

    let listener = EventLogger::builder()
        .add(arbx.events(), "arbx")
        .add(arby.events(), "arby")
        .add(lex.events(), "lex")
        .metadata(metadata)
        .unwrap();

    EventLogger::builder()
        .add(arbx.events(), "arbx")
        .add(arby.events(), "arby")
        .add(lex.events(), "lex")
        .file_type(OutputFileType::CSV)
        .run()
        .unwrap();

    EventLogger::builder()
        .add(arbx.events(), "arbx")
        .add(arby.events(), "arby")
        .add(lex.events(), "lex")
        .file_type(OutputFileType::Parquet)
        .run()
        .unwrap();

    generate_events(arbx, arby, lex, client.clone())
        .await
        .unwrap_or_else(|e| {
            panic!("Error generating events: {}", e);
        });

    let _ = env.stop();

    std::thread::sleep(std::time::Duration::from_secs(1));
    assert!(Path::new("./data/output.csv").exists());
    assert!(Path::new("./data/output.parquet").exists());
    assert!(Path::new("./data/output.json").exists());
    // std::fs::remove_dir_all("./data").unwrap();
}

async fn generate_events(
    arbx: ArbiterToken<RevmMiddleware>,
    arby: ArbiterToken<RevmMiddleware>,
    lex: LiquidExchange<RevmMiddleware>,
    client: Arc<RevmMiddleware>,
) -> Result<(), RevmMiddlewareError> {
    for _ in 0..5 {
        arbx.approve(client.address(), U256::from(1))
            .send()
            .await
            .unwrap()
            .await?;
        arby.approve(client.address(), U256::from(1))
            .send()
            .await
            .unwrap()
            .await?;
        lex.set_price(U256::from(10u128.pow(18)))
            .send()
            .await
            .unwrap()
            .await?;
    }
    Ok(())
}
