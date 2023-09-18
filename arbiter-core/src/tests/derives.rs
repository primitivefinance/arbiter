use arbiter_derive::Deploy;
use ethers::prelude::{ContractDeploymentTx, ContractError};

use super::*;

#[derive(Deploy)]
pub struct Contracts {
    arbx: ContractDeploymentTx<Arc<RevmMiddleware>, RevmMiddleware, ArbiterToken<RevmMiddleware>>,
    arby: ContractDeploymentTx<Arc<RevmMiddleware>, RevmMiddleware, ArbiterToken<RevmMiddleware>>,
}

#[tokio::test]
async fn deploy_macro() {
    let (_manager, client) = startup_user_controlled().unwrap();
    let constructor_args = (
        ARBITER_TOKEN_X_NAME.to_string(),
        ARBITER_TOKEN_X_SYMBOL.to_string(),
        ARBITER_TOKEN_X_DECIMALS,
    );
    let constructor_args2 = (
        ARBITER_TOKEN_Y_NAME.to_string(),
        ARBITER_TOKEN_Y_SYMBOL.to_string(),
        ARBITER_TOKEN_Y_DECIMALS,
    );
    let contracts = Contracts {
        arbx: ArbiterToken::deploy(client.clone(), constructor_args).unwrap(),
        arby: ArbiterToken::deploy(client.clone(), constructor_args2).unwrap(),
    };
    let deployed_contracts = contracts.deploy().await.unwrap();
    println!("Deployed contracts: {:#?}", deployed_contracts);
    let arbx = deployed_contracts.arbx;
    let receipt = arbx
        .mint(client.address(), U256::from(1))
        .send()
        .await
        .unwrap()
        .await
        .unwrap();
    println!("Receipt: {:#?}", receipt);
    assert!(receipt.is_some());

    let arby = deployed_contracts.arby;
    let receipt = arby
        .mint(client.address(), U256::from(1))
        .send()
        .await
        .unwrap()
        .await
        .unwrap();
    println!("Receipt: {:#?}", receipt);
    assert!(receipt.is_some());
}
