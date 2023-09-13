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
    environment::{BlockSettings, EnvironmentParameters, GasSettings},
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

const NUM_BENCH_ITERATIONS: usize = 1000;
const NUM_LOOP_STEPS: usize = 100;

#[derive(Debug)]
struct BenchDurations {
    deploy: Duration,
    lookup: Duration,
    stateless_call: Duration,
    stateful_call: Duration,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Choose the benchmark group items by label.
    let group = ["anvil", "arbiter"];

    // Set up for showing percentage done.
    let ten_percent = NUM_BENCH_ITERATIONS / 10;

    for item in group {
        // Count up total durations for each part of the benchmark.
        let mut durations = Vec::with_capacity(NUM_BENCH_ITERATIONS);
        println!("Running {item} benchmark");

        for index in 0..NUM_BENCH_ITERATIONS {
            durations.push(match item {
                label @ "anvil" => {
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
                println!("{index} out of {NUM_BENCH_ITERATIONS} complete");
            }
        }
        let sum_durations = durations.iter().fold(
            BenchDurations {
                deploy: Duration::default(),
                lookup: Duration::default(),
                stateless_call: Duration::default(),
                stateful_call: Duration::default(),
            },
            |acc, duration| BenchDurations {
                deploy: acc.deploy + duration.deploy,
                lookup: acc.lookup + duration.lookup,
                stateless_call: acc.stateless_call + duration.stateless_call,
                stateful_call: acc.stateful_call + duration.stateful_call,
            },
        );

        let average_durations = BenchDurations {
            deploy: sum_durations.deploy / NUM_BENCH_ITERATIONS as u32,
            lookup: sum_durations.lookup / NUM_BENCH_ITERATIONS as u32,
            stateless_call: sum_durations.stateless_call / NUM_BENCH_ITERATIONS as u32,
            stateful_call: sum_durations.stateful_call / NUM_BENCH_ITERATIONS as u32,
        };

        println!("Average durations for {item}: {:?}", average_durations);
    }

    Ok(())
}

async fn bencher<M: Middleware + 'static>(client: Arc<M>, label: &str) -> Result<BenchDurations> {
    // Track the duration for each part of the benchmark.
    let mut total_deploy_duration = 0;
    let mut total_lookup_duration = 0;
    let mut total_stateless_call_duration = 0;
    let mut total_stateful_call_duration = 0;

    // Deploy `ArbiterMath` and `ArbiterToken` contracts and tally up how long this
    // takes.
    let (arbiter_math, arbiter_token, deploy_duration) = deployments(client.clone(), label).await?;
    total_deploy_duration += deploy_duration.as_micros();

    // Call `balance_of` `NUM_LOOP_STEPS` times on `ArbiterToken` and tally up how
    // long basic lookups take.
    let lookup_duration = lookup(arbiter_token.clone(), label).await?;
    total_lookup_duration += lookup_duration.as_micros();

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
        lookup: Duration::from_micros(total_lookup_duration as u64),
        stateless_call: Duration::from_micros(total_stateless_call_duration as u64),
        stateful_call: Duration::from_micros(total_stateful_call_duration as u64),
    })
}

async fn anvil_startup() -> Result<(
    Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    AnvilInstance,
)> {
    // Create an Anvil instance
    // No blocktime mines a new block for each tx, which is fastest.
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
        label: ENV_LABEL.to_string(),
        block_settings: BlockSettings::UserControlled,
        gas_settings: GasSettings::UserControlled,
    };
    manager.add_environment(params)?;
    manager.start_environment(ENV_LABEL)?;

    let client = Arc::new(RevmMiddleware::new(
        manager.environments.get(ENV_LABEL).unwrap(),
        Some("name".to_string()),
    )?);

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

async fn lookup<M: Middleware + 'static>(
    arbiter_token: ArbiterToken<M>,
    label: &str,
) -> Result<Duration> {
    let address = arbiter_token.client().default_sender().unwrap();
    let start = Instant::now();
    for _ in 0..NUM_LOOP_STEPS {
        arbiter_token.balance_of(address).call().await?;
    }
    let duration = start.elapsed();
    info!("Time elapsed in {} cdf loop is: {:?}", label, duration);

    Ok(duration)
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
