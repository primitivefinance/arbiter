// use ethers_providers::{Middleware, Provider, Http};
// use ethers_signers::LocalWallet;
// use ethers_middleware::SignerMiddleware;
// use ethers_core::types::{Address, TransactionRequest};
use std::{
    convert::TryFrom,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Result;
use arbiter_core::{
    bindings::{
        arbiter_math::ArbiterMath,
        arbiter_token::{self, ArbiterToken},
    },
    environment::EnvironmentParameters,
    manager::Manager,
    middleware::RevmMiddleware,
};
use ethers::{
    core::{k256::ecdsa::SigningKey, utils::Anvil},
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet},
    types::{Address, I256, U256},
    utils::AnvilInstance,
};

const NUM_STEPS: usize = 1000;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let args = args.get(1).unwrap().as_str();
    match args {
        label @ "anvil" => {
            let (client, _anvil_instance) = anvil_startup().await?;
            let (arbiter_math, arbiter_token) = deployments(client.clone(), label).await?;
            stateless_call_loop(arbiter_math, label).await?;
            stateful_call_loop(arbiter_token, client.default_sender().unwrap(), label).await?;
        }
        label @ "arbiter" => {
            let client = arbiter_startup().await?;
            let (arbiter_math, arbiter_token) = deployments(client.clone(), label).await?;
            stateless_call_loop(arbiter_math, label).await?;
            stateful_call_loop(arbiter_token, client.default_sender().unwrap(), label).await?;
        }
        _ => panic!("Invalid argument"),
    };

    Ok(())
}

#[cfg(bench)]
use test::Bencher;
#[cfg(bench)]
use std::process::Termination;

#[cfg(bench)]
fn anvil(b: &mut Bencher) -> impl Termination {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    b.iter(|| {
        runtime.block_on(async {
            let label = "anvil";
            let (client, _anvil) = anvil_startup().await.unwrap();
            let (arbiter_math, arbiter_token) = deployments(client.clone(), label).await.unwrap();
            stateless_call_loop(arbiter_math, label).await.unwrap();
            stateful_call_loop(arbiter_token, client.default_sender().unwrap(), label)
                .await
                .unwrap();
        })
    });
}

#[cfg(bench)]
fn arbiter(b: &mut Bencher) -> impl Termination {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    b.iter(|| {
        runtime.block_on(async {
            let label = "arbiter";
            let client = arbiter_startup().await.unwrap();
            let (arbiter_math, arbiter_token) = deployments(client.clone(), label).await.unwrap();
            stateless_call_loop(arbiter_math, label).await.unwrap();
            stateful_call_loop(arbiter_token, client.default_sender().unwrap(), label)
                .await
                .unwrap();
        })
    });
}

async fn anvil_startup() -> Result<(
    Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    AnvilInstance,
)> {
    // Create an anvil instance
    // No blocktime mines a new block for each tx.
    let anvil = Anvil::new().spawn();
    println!("Anvil endpoint: {}", anvil.endpoint());

    // Create a client
    let provider = Provider::<Http>::try_from(anvil.endpoint())
        .unwrap()
        .interval(Duration::ZERO);

    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(anvil.chain_id()),
    ));

    Ok((client, anvil))
}

async fn arbiter_startup() -> Result<Arc<RevmMiddleware>> {
    let mut manager = Manager::new();
    let params = EnvironmentParameters {
        block_rate: 10.0,
        seed: 0,
    };
    manager.add_environment("env", params)?;

    let client = Arc::new(RevmMiddleware::new(
        manager.environments.get("env").unwrap(),
        Some("name".to_string()),
    ));

    manager.start_environment("env")?;

    Ok(client)
}

async fn deployments<M: Middleware + 'static>(
    client: Arc<M>,
    label: &str,
) -> Result<(ArbiterMath<M>, ArbiterToken<M>)> {
    let start = Instant::now();
    let arbiter_math = ArbiterMath::deploy(client.clone(), ())?.send().await?;
    let arbiter_token = arbiter_token::ArbiterToken::deploy(
        client.clone(),
        ("Arbiter Token".to_string(), "ARBT".to_string(), 18_u8),
    )?
    .send()
    .await?;
    let duration = start.elapsed();
    println!("Time elapsed in {} deployment is: {:?}", label, duration);
    Ok((arbiter_math, arbiter_token))
}

async fn stateless_call_loop<M: Middleware + 'static>(
    arbiter_math: ArbiterMath<M>,
    label: &str,
) -> Result<()> {
    let iwad = I256::from(10_u128.pow(18));
    let start = Instant::now();
    for _ in 0..NUM_STEPS {
        arbiter_math.cdf(iwad).call().await?;
    }
    let duration = start.elapsed();

    println!("Time elapsed in {} cdf loop is: {:?}", label, duration);
    Ok(())
}

async fn stateful_call_loop<M: Middleware + 'static>(
    arbiter_token: arbiter_token::ArbiterToken<M>,
    mint_address: Address,
    label: &str,
) -> Result<()> {
    let wad = U256::from(10_u128.pow(18));
    let start = Instant::now();
    for _ in 0..NUM_STEPS {
        arbiter_token.mint(mint_address, wad).send().await?.await?;
    }
    let duration = start.elapsed();
    println!("Time elapsed in {} mint loop is: {:?}", label, duration);
    Ok(())
}

async fn _mixture_loop<M>(
    _arbiter_math: ArbiterMath<M>,
    _arbiter_token: arbiter_token::ArbiterToken<M>,
) -> Result<()> {
    Ok(())
}
