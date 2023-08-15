use super::*;
use crate::bindings::liquid_exchange::LiquidExchange;

pub const ARBITER_TOKEN_X_NAME: &str = "Arbiter Token X";
pub const ARBITER_TOKEN_X_SYMBOL: &str = "ARBX";
pub const ARBITER_TOKEN_X_DECIMALS: u8 = 18;

pub const ARBITER_TOKEN_Y_NAME: &str = "Arbiter Token Y";
pub const ARBITER_TOKEN_Y_SYMBOL: &str = "ARBY";
pub const ARBITER_TOKEN_Y_DECIMALS: u8 = 18;

fn startup() -> Result<(Manager, Arc<RevmMiddleware>)> {
    let mut manager = Manager::new();
    manager.add_environment(TEST_ENV_LABEL, TEST_BLOCK_RATE, TEST_ENV_SEED)?;
    let environment = manager.environments.get(TEST_ENV_LABEL).unwrap();
    let client = Arc::new(RevmMiddleware::new(
        environment,
        Some(TEST_SIGNER_SEED_AND_LABEL.to_string()),
    ));
    manager.start_environment(TEST_ENV_LABEL)?;
    Ok((manager, client))
}

async fn deploy_arbx(client: Arc<RevmMiddleware>) -> Result<ArbiterToken<RevmMiddleware>> {
    Ok(ArbiterToken::deploy(
        client,
        (
            ARBITER_TOKEN_X_NAME.to_string(),
            ARBITER_TOKEN_X_SYMBOL.to_string(),
            ARBITER_TOKEN_X_DECIMALS,
        ),
    )?
    .send()
    .await?)
}

async fn deploy_arby(client: Arc<RevmMiddleware>) -> Result<ArbiterToken<RevmMiddleware>> {
    Ok(ArbiterToken::deploy(
        client,
        (
            ARBITER_TOKEN_Y_NAME.to_string(),
            ARBITER_TOKEN_Y_SYMBOL.to_string(),
            ARBITER_TOKEN_Y_DECIMALS,
        ),
    )?
    .send()
    .await?)
}

async fn deploy_liquid_exchange(
    client: Arc<RevmMiddleware>,
) -> Result<LiquidExchange<RevmMiddleware>> {
    let arbx = deploy_arbx(client.clone()).await?;
    let arby = deploy_arby(client.clone()).await?;
    let price = float_to_wad(420.69);
    let liquid_exchange = LiquidExchange::deploy(client, (arbx.address(), arby.address(), price))?
        .send()
        .await?;
    Ok(liquid_exchange)
}

async fn deploy_arbiter_math(client: Arc<RevmMiddleware>) -> Result<ArbiterMath<RevmMiddleware>> {
    Ok(ArbiterMath::deploy(client, ())?.send().await?)
}

// TODO: Hit all the contract bindings.
#[test]
fn token_mint_and_balance() -> Result<()> {
    todo!()
}

#[tokio::test]
async fn arbiter_math() -> Result<()> {
    let (_manager, client) = startup()?;
    let arbiter_math = deploy_arbiter_math(client).await?;

    // Test the cdf function
    let cdf_output = arbiter_math
        .cdf(ethers::types::I256::from(1))
        .call()
        .await?;
    println!("cdf(1) = {}", cdf_output);
    assert_eq!(cdf_output, ethers::types::I256::from(500000000000000000u64));

    // Test the pdf function
    let pdf_output = arbiter_math.pdf(ethers::types::I256::from(1)).call().await?;
    println!("pdf(1) = {}", pdf_output);
    assert_eq!(pdf_output, ethers::types::I256::from(398942280401432678u64));

    // Test the ppf function.
    let ppf_output = arbiter_math.ppf(ethers::types::I256::from(1)).call().await?;
    println!("ppf(1) = {}", ppf_output);
    assert_eq!(ppf_output, ethers::types::I256::from(-8710427241990476442_i128));

    // Test the mulWadDown function.
    let mulwaddown_output = arbiter_math.mul_wad_down(ethers::types::U256::from(1_000_000_000_000_000_000_u128), ethers::types::U256::from(2)).call().await?;
    println!("mulWadDown(1, 2) = {}", mulwaddown_output);
    assert_eq!(mulwaddown_output, ethers::types::U256::from(2));

    // Test the mulWadUp function.
    let mulwadup_output = arbiter_math.mul_wad_up(ethers::types::U256::from(1_000_000_000_000_000_000_u128), ethers::types::U256::from(2)).call().await?;
    println!("mulWadUp(1, 2) = {}", mulwadup_output);
    assert_eq!(mulwadup_output, ethers::types::U256::from(2));

    // Test the divWadDown function.
    let divwaddown_output = arbiter_math.div_wad_down(ethers::types::U256::from(1_000_000_000_000_000_000_u128), ethers::types::U256::from(2)).call().await?;
    println!("divWadDown(1, 2) = {}", divwaddown_output);
    assert_eq!(
        divwaddown_output,
        ethers::types::U256::from(500000000000000000000000000000000000_u128)
    );

    // Test the divWadUp function.
    let divwadup_output = arbiter_math.div_wad_up(ethers::types::U256::from(1_000_000_000_000_000_000_u128), ethers::types::U256::from(2)).call().await?;
    println!("divWadUp(1, 2) = {}", divwadup_output);
    assert_eq!(
        divwadup_output,
        ethers::types::U256::from(500000000000000000000000000000000000_u128)
    );

    // Test the lnWad function.
    let lnwad_output = arbiter_math.log(ethers::types::I256::from(1_000_000_000_000_000_000_u128)).call().await?;
    println!("ln(1) = {}", lnwad_output);
    assert_eq!(lnwad_output, ethers::types::I256::from(0));
    
    // Test the sqrt function
    let sqrt_output = arbiter_math.sqrt(ethers::types::U256::from(1_000_000_000_000_000_000_u128)).call().await?;
    println!("sqrt(1) = {}", sqrt_output);
    assert_eq!(sqrt_output, ethers::types::U256::from(1_000_000_000));
    Ok(())
}

// OLD TESTS DOWN HERE
// #[cfg(test)]
// mod tests {
//     use std::error::Error;

//     use bindings::{arbiter_token, liquid_exchange};
//     use ethers::prelude::U256;
//     use revm::primitives::{ruint::Uint, B160};

//     use crate::{
//         agent::{user::User, Agent, AgentType},
//         environment::contract::SimulationContract,
//         manager::SimulationManager,
//         stochastic::price_process::{PriceProcess, PriceProcessType, GBM},
//         utils::{recast_address, unpack_execution},
//     };

//     #[test]
//     fn swap_x_for_y_liquid_exchange() -> Result<(), Box<dyn Error>> {
//         // define the wad constant
//         let decimals = 18_u8;
//         let wad: U256 = U256::from(10_i64.pow(decimals as u32));

//         // Set up the execution manager and a user address.
//         let mut manager = SimulationManager::default();

//         // Set up a user named alice
//         let user_name = "alice";
//         let user_address = B160::from_low_u64_be(2);
//         let alice = User::new(user_name, None);
//         manager.activate_agent(AgentType::User(alice), user_address)?;
//         let admin = manager.agents.get("admin").unwrap();
//         let alice = manager.agents.get("alice").unwrap();

//         // Create arbiter token general contract.
//         let arbiter_token = SimulationContract::new(
//             arbiter_token::ARBITERTOKEN_ABI.clone(),
//             arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
//         );

//         // Deploy token_x.
//         let name = "Token X";
//         let symbol = "TKNX";
//         let args = (name.to_string(), symbol.to_string(), decimals);
//         let token_x = arbiter_token.deploy(&mut manager.environment, admin,
// args);

//         // Deploy token_y.
//         let name = "Token Y";
//         let symbol = "TKNY";
//         let args = (name.to_string(), symbol.to_string(), decimals);
//         let token_y = arbiter_token.deploy(&mut manager.environment, admin,
// args);

//         // Deploy LiquidExchange
//         let price_to_check = 1000;
//         let initial_price =
// wad.checked_mul(U256::from(price_to_check)).unwrap();         let
// liquid_exchange = SimulationContract::new(
// liquid_exchange::LIQUIDEXCHANGE_ABI.clone(),
// bindings::liquid_exchange::LIQUIDEXCHANGE_BYTECODE.clone(),         );
//         let args = (
//             recast_address(token_x.address),
//             recast_address(token_y.address),
//             initial_price,
//         );
//         let liquid_exchange_xy = liquid_exchange.deploy(&mut
// manager.environment, admin, args);

//         // Mint token_x to alice.
//         let mint_amount = wad.checked_mul(U256::from(20)).unwrap(); // in wei
// units         let args = (recast_address(alice.address()), mint_amount);
//         let call_data = token_x.encode_function("mint", args)?;
//         admin.call_contract(&mut manager.environment, &token_x, call_data,
// Uint::from(0));

//         // Mint max token_y to the liquid_exchange contract.
//         let args = (recast_address(liquid_exchange_xy.address), U256::MAX);
//         let call_data = token_y.encode_function("mint", args)?;
//         admin.call_contract(&mut manager.environment, &token_y, call_data,
// Uint::from(0));

//         // Have alice's approval for token_x to be spent by the
// liquid_exchange.         let args =
// (recast_address(liquid_exchange_xy.address), U256::MAX);         let
// call_data = token_x.encode_function("approve", args)?;         alice.
// call_contract(&mut manager.environment, &token_x, call_data, Uint::from(0));

//         // Have alice call the swap function to trade token_x for token_y.
//         let swap_amount = mint_amount / 2;
//         let call_data = liquid_exchange_xy
//             .encode_function("swap", (recast_address(token_x.address),
// swap_amount))?;         alice.call_contract(
//             &mut manager.environment,
//             &liquid_exchange_xy,
//             call_data,
//             Uint::from(0),
//         );

//         // Let alice check they spent the right amount of token_x
//         let call_data = token_x.encode_function("balanceOf",
// recast_address(user_address))?;         let execution_result =
//             alice.call_contract(&mut manager.environment, &token_x,
// call_data, Uint::from(0)); // Call the 'balanceOf' function.         let
// value = unpack_execution(execution_result)?;         let response: U256 =
// token_x.decode_output("balanceOf", value)?;         println!("alice has {}
// token_x after swap", response);         assert_eq!(response, mint_amount -
// swap_amount);

//         // Let alice check they received the right amount of token_y
//         let call_data = token_y.encode_function("balanceOf",
// recast_address(user_address))?;         let execution_result =
//             alice.call_contract(&mut manager.environment, &token_y,
// call_data, Uint::from(0)); // Call the 'balanceOf' function.         let
// value = unpack_execution(execution_result)?;         let response: U256 =
// token_y.decode_output("balanceOf", value)?;         println!("alice has {}
// token_y after swap", response);         assert_eq!(response, swap_amount *
// U256::from(price_to_check));         Ok(())
//     }

//     #[test]
//     fn swap_y_for_x_liquid_exchange() -> Result<(), Box<dyn Error>> {
//         // define the wad constant
//         let decimals = 18_u8;
//         let wad: U256 = U256::from(10_i64.pow(decimals as u32));

//         // Set up the execution manager and a user address.
//         let mut manager = SimulationManager::default();

//         // Set up a user named alice
//         let user_name = "alice";
//         let user_address = B160::from_low_u64_be(2);
//         let alice = User::new(user_name, None);
//         manager.activate_agent(AgentType::User(alice), user_address)?;
//         let admin = manager.agents.get("admin").unwrap();
//         let alice = manager.agents.get("alice").unwrap();

//         // Create arbiter token general contract.
//         let arbiter_token = SimulationContract::new(
//             arbiter_token::ARBITERTOKEN_ABI.clone(),
//             arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
//         );

//         // Deploy token_x.
//         let name = "Token X";
//         let symbol = "TKNX";
//         let args = (name.to_string(), symbol.to_string(), decimals);
//         let token_x = arbiter_token.deploy(&mut manager.environment, admin,
// args);

//         // Deploy token_y.
//         let name = "Token Y";
//         let symbol = "TKNY";
//         let args = (name.to_string(), symbol.to_string(), decimals);
//         let token_y = arbiter_token.deploy(&mut manager.environment, admin,
// args);

//         // Deploy LiquidExchange
//         let price_to_check = 1000;
//         let initial_price =
// wad.checked_mul(U256::from(price_to_check)).unwrap();         let
// liquid_exchange = SimulationContract::new(
// liquid_exchange::LIQUIDEXCHANGE_ABI.clone(),
// liquid_exchange::LIQUIDEXCHANGE_BYTECODE.clone(),         );
//         let args = (
//             recast_address(token_x.address),
//             recast_address(token_y.address),
//             initial_price,
//         );
//         let liquid_exchange_xy = liquid_exchange.deploy(&mut
// manager.environment, admin, args);

//         // Mint token_y to alice.
//         let mint_amount = wad.checked_mul(U256::from(20)).unwrap(); // in wei
// units         let args = (recast_address(alice.address()), mint_amount);
//         let call_data = token_y.encode_function("mint", args)?;
//         admin.call_contract(&mut manager.environment, &token_y, call_data,
// Uint::from(0));

//         // Mint max token_x to the liquid_exchange contract.
//         let args = (recast_address(liquid_exchange_xy.address), U256::MAX);
//         let call_data = token_x.encode_function("mint", args)?;
//         admin.call_contract(&mut manager.environment, &token_x, call_data,
// Uint::from(0));

//         // Have alice's approval for token_y to be spent by the
// liquid_exchange.         let args =
// (recast_address(liquid_exchange_xy.address), U256::MAX);         let
// call_data = token_y.encode_function("approve", args)?;         alice.
// call_contract(&mut manager.environment, &token_y, call_data, Uint::from(0));

//         // Have alice call the swap function to trade token_y for token_x.
//         let swap_amount = mint_amount / 2;
//         let call_data = liquid_exchange_xy
//             .encode_function("swap", (recast_address(token_y.address),
// swap_amount))?;         alice.call_contract(
//             &mut manager.environment,
//             &liquid_exchange_xy,
//             call_data,
//             Uint::from(0),
//         );

//         // Let alice check they spent the right amount of token_y
//         let call_data = token_y.encode_function("balanceOf",
// recast_address(user_address))?;         let execution_result =
//             alice.call_contract(&mut manager.environment, &token_y,
// call_data, Uint::from(0)); // Call the 'balanceOf' function.         let
// value = unpack_execution(execution_result)?;         let response: U256 =
// token_y.decode_output("balanceOf", value)?;         println!("alice has {}
// token_y after swap", response);         assert_eq!(response, mint_amount -
// swap_amount);

//         // Let alice check they received the right amount of token_x
//         let call_data = token_x.encode_function("balanceOf",
// recast_address(user_address))?;         let execution_result =
//             alice.call_contract(&mut manager.environment, &token_x,
// call_data, Uint::from(0)); // Call the 'balanceOf' function.         let
// value = unpack_execution(execution_result)?;         let response: U256 =
// token_x.decode_output("balanceOf", value)?;         println!("alice has {}
// token_x after swap", response);         assert_eq!(response,
// U256::from(10_i64.pow(16)));         Ok(())
//     }

//     #[test]
//     fn price_simulation_oracle() -> Result<(), Box<dyn Error>> {
//         // Get a price path from the oracle.
//         let timestep = 1.0;
//         let timescale = String::from("day");
//         let num_steps = 7;
//         let initial_price = 100.0;
//         let drift = 0.5;
//         let volatility = 0.1;
//         let seed = 123;
//         let price = PriceProcess::new(
//             PriceProcessType::GBM(GBM::new(drift, volatility)),
//             timestep,
//             timescale,
//             num_steps,
//             initial_price,
//             seed,
//         );
//         let price_path = price.generate_price_path();

//         // Set up the liquid exchange
//         // define the wad constant
//         let decimals = 18_u8;
//         let wad: U256 = U256::from(10_i64.pow(decimals as u32));

//         // Set up the execution manager and a user address.
//         let mut manager = SimulationManager::default();
//         let admin = manager.agents.get("admin").unwrap();

//         // Create arbiter token general contract.
//         let arbiter_token = SimulationContract::new(
//             arbiter_token::ARBITERTOKEN_ABI.clone(),
//             bindings::arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
//         );

//         // Deploy token_x.
//         let name = "Token X";
//         let symbol = "TKNX";
//         let args = (name.to_string(), symbol.to_string(), decimals);
//         let token_x = arbiter_token.deploy(&mut manager.environment, admin,
// args);

//         // Deploy token_y.
//         let name = "Token Y";
//         let symbol = "TKNY";
//         let args = (name.to_string(), symbol.to_string(), decimals);
//         let token_y = arbiter_token.deploy(&mut manager.environment, admin,
// args);

//         // Deploy LiquidExchange
//         let price_to_check = 1000; // Initial price is this for sake of ease.
//         let initial_price =
// wad.checked_mul(U256::from(price_to_check)).unwrap();         let
// liquid_exchange = SimulationContract::new(
// liquid_exchange::LIQUIDEXCHANGE_ABI.clone(),
// bindings::liquid_exchange::LIQUIDEXCHANGE_BYTECODE.clone(),         );
//         let args = (
//             recast_address(token_x.address),
//             recast_address(token_y.address),
//             initial_price,
//         );
//         let liquid_exchange_xy = liquid_exchange.deploy(&mut
// manager.environment, admin, args);

//         // Loop over and set prices on the liquid exchange from the oracle.
//         for price in price_path.1 {
//             println!("Price from price path: {}", price);
//             let wad_price = crate::utils::float_to_wad(price);
//             println!("WAD price: {}", wad_price);
//             let call_data = liquid_exchange_xy.encode_function("setPrice",
// wad_price)?;             admin.call_contract(
//                 &mut manager.environment,
//                 &liquid_exchange_xy,
//                 call_data,
//                 Uint::from(0),
//             );
//             // Check that the price is set correctly
//             let call_data = liquid_exchange_xy.encode_function("price", ())?;
//             let execution_result = admin.call_contract(
//                 &mut manager.environment,
//                 &liquid_exchange_xy,
//                 call_data,
//                 Uint::from(0),
//             );
//             let value = unpack_execution(execution_result)?;
//             let response: U256 = liquid_exchange_xy.decode_output("price",
// value)?;             println!("Price from the exchange: {}", response);
//             assert_eq!(response, wad_price);
//         }
//         Ok(())
//     }
// }
