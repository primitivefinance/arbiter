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
use log::info;

const ENV_LABEL: &str = "env";

const NUM_BENCH_ITERATIONS: usize = 100;
const NUM_LOOP_STEPS: usize = 100;

#[derive(Debug)]
struct BenchDurations {
    deploy: Duration,
    stateless_call: Duration,
    stateful_call: Duration,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set up logging.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "error");
    }
    env_logger::init();

    // Get input argument.
    let args: Vec<String> = std::env::args().collect();
    let args = args.get(1).unwrap().as_str();
    let mut durations = vec![];

    let ten_percent = NUM_BENCH_ITERATIONS / 10;

    for index in 0..NUM_BENCH_ITERATIONS {
        durations.push(match args {
            label @ "anvil" => {
                // Start up Anvil with a client.
                let (client, _anvil_instance) = anvil_startup().await?;
                let duration = bencher(client, label).await?;
                drop(_anvil_instance);
                duration
            }
            label @ "arbiter" => {
                let (client, mut manager) = arbiter_startup().await?;
                let duration = bencher(client, label).await?;
                manager.stop_environment(ENV_LABEL)?;
                duration
            }
            _ => panic!("Invalid argument"),
        });
        if index % ten_percent == 0 {
            println!("{index}% complete");
        }
    }

    let sum_durations = durations.iter().fold(
        BenchDurations {
            deploy: Duration::default(),
            stateless_call: Duration::default(),
            stateful_call: Duration::default(),
        },
        |acc, duration| BenchDurations {
            deploy: acc.deploy + duration.deploy,
            stateless_call: acc.stateless_call + duration.stateless_call,
            stateful_call: acc.stateful_call + duration.stateful_call,
        },
    );

    // let len = durations.len() as u32;
    let average_durations = BenchDurations {
        deploy: sum_durations.deploy / NUM_LOOP_STEPS as u32,
        stateless_call: sum_durations.stateless_call / NUM_LOOP_STEPS as u32,
        stateful_call: sum_durations.stateful_call / NUM_LOOP_STEPS as u32,
    };

    println!("Average durations: {:?}", average_durations);

    Ok(())
}

async fn bencher<M: Middleware + 'static>(client: Arc<M>, label: &str) -> Result<BenchDurations> {
    // Track the duration for each part of the benchmark.
    let mut total_deploy_duration = 0;
    let mut total_stateless_call_duration = 0;
    let mut total_stateful_call_duration = 0;

    // Deploy `ArbiterMath` and `ArbiterToken` contracts and tally up how long this
    // takes.
    let (arbiter_math, arbiter_token, deploy_duration) = deployments(client.clone(), label).await?;
    total_deploy_duration += deploy_duration.as_micros();

    // Call `cdf` `NUM_LOOP_STEPS` times on `ArbiterMath` and tally up how long this
    // takes.
    let stateless_call_duration = stateless_call_loop(arbiter_math, label).await?;
    total_stateless_call_duration += stateless_call_duration.as_micros();

    // Call `mint` `NUM_LOOP_STEPS` times on `ArbiterToken` and tally up how long
    // this takes.
    let statefull_call_duration =
        stateful_call_loop(arbiter_token, client.default_sender().unwrap(), label).await?;
    total_stateful_call_duration += statefull_call_duration.as_micros();

    Ok(BenchDurations {
        deploy: Duration::from_micros(total_deploy_duration as u64),
        stateless_call: Duration::from_micros(total_stateless_call_duration as u64),
        stateful_call: Duration::from_micros(total_stateful_call_duration as u64),
    })
}

async fn anvil_startup() -> Result<(
    Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    AnvilInstance,
)> {
    // Create an anvil instance
    // No blocktime mines a new block for each tx.
    let anvil = Anvil::new().spawn();

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

async fn arbiter_startup() -> Result<(Arc<RevmMiddleware>, Manager)> {
    let mut manager = Manager::new();
    let params = EnvironmentParameters {
        block_rate: 10.0,
        seed: 0,
    };
    manager.add_environment(ENV_LABEL, params)?;

    let client = Arc::new(RevmMiddleware::new(
        manager.environments.get(ENV_LABEL).unwrap(),
        Some("name".to_string()),
    ));

    manager.start_environment(ENV_LABEL)?;

    Ok((client, manager))
}

async fn deployments<M: Middleware + 'static>(
    client: Arc<M>,
    label: &str,
) -> Result<(ArbiterMath<M>, ArbiterToken<M>, Duration)> {
    let start = Instant::now();
    let arbiter_math = ArbiterMath::deploy(client.clone(), ())?.send().await?;
    let arbiter_token = arbiter_token::ArbiterToken::deploy(
        client.clone(),
        ("Arbiter Token".to_string(), "ARBT".to_string(), 18_u8),
    )?
    .send()
    .await?;
    let duration = start.elapsed();
    info!("Time elapsed in {} deployment is: {:?}", label, duration);

    Ok((arbiter_math, arbiter_token, duration))
}

async fn stateless_call_loop<M: Middleware + 'static>(
    arbiter_math: ArbiterMath<M>,
    label: &str,
) -> Result<Duration> {
    let iwad = I256::from(10_u128.pow(18));
    let start = Instant::now();
    for _ in 0..NUM_LOOP_STEPS {
        arbiter_math.cdf(iwad).call().await?;
    }
    let duration = start.elapsed();
    info!("Time elapsed in {} cdf loop is: {:?}", label, duration);

    Ok(duration)
}

async fn stateful_call_loop<M: Middleware + 'static>(
    arbiter_token: arbiter_token::ArbiterToken<M>,
    mint_address: Address,
    label: &str,
) -> Result<Duration> {
    let wad = U256::from(10_u128.pow(18));
    let start = Instant::now();
    for _ in 0..NUM_LOOP_STEPS {
        arbiter_token.mint(mint_address, wad).send().await?.await?;
    }
    let duration = start.elapsed();
    info!("Time elapsed in {} mint loop is: {:?}", label, duration);

    Ok(duration)
}

async fn _mixture_loop<M>(
    _arbiter_math: ArbiterMath<M>,
    _arbiter_token: arbiter_token::ArbiterToken<M>,
) -> Result<()> {
    todo!("Have a loop here that takes a few different types of calls at once.")
}

#[cfg(bench)]
use std::process::Termination;

#[cfg(bench)]
use test::Bencher;

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
