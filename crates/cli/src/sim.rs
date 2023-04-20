#![warn(missing_docs)]
use std::{error::Error};

use bindings::{
    arbiter_token, encoder_target, i_portfolio, liquid_exchange, rmm01_portfolio, simple_registry,
    weth9,
};
use primitive_types::H160 as PH160;
use bytes::Bytes;
use ethers::{prelude::U256, types::H256, abi::Token};
use eyre::Result;
use revm::primitives::{ruint::Uint, B160};
use simulate::{
    agent::Agent,
    contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    utils::recast_address,
};
use compiler::{codegen::Codegen, assembler::{Expression}, opcode::Opcode};


pub fn sim() -> Result<(), Box<dyn Error>> {
    // define the wad constant
    let decimals = 18_u8;
    let wad: U256 = U256::from(10_i64.pow(decimals as u32));
    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    let mut manager = SimulationManager::new();

    let user_name = "arbitrageur";
    let user_address = B160::from_low_u64_be(2);
    manager.create_user(user_address, user_name)?;
    

    // Deploying Contracts
    let contracts =
        deploy_sim_contracts(&mut manager, wad)?;

    intitalization_calls(&mut manager, contracts)?;

    Ok(())
}

fn deploy_sim_contracts(
    manager: &mut SimulationManager,
    wad: U256,
) -> Result<
    (
        SimulationContract<IsDeployed>,
        SimulationContract<IsDeployed>,
        SimulationContract<IsDeployed>,
        SimulationContract<IsDeployed>,
        SimulationContract<IsDeployed>,
    ),
    Box<dyn Error>,
> {
    let decimals = 18_u8;
    let admin = manager.agents.get("admin").unwrap();
    // Deploy Weth
    let weth = SimulationContract::new(weth9::WETH9_ABI.clone(), weth9::WETH9_BYTECODE.clone());
    let weth = admin.deploy(&mut manager.environment, weth, ());
    println!("WETH deployed at: {}", weth.address);

    // Deploy the registry contract.
    let registry = SimulationContract::new(
        simple_registry::SIMPLEREGISTRY_ABI.clone(),
        simple_registry::SIMPLEREGISTRY_BYTECODE.clone(),
    );
    let registry = admin.deploy(&mut manager.environment, registry, ());
    println!("Simple registry deployed at: {}", registry.address);

    // Deploy the portfolio contract.
    let portfolio = SimulationContract::new(
        rmm01_portfolio::RMM01PORTFOLIO_ABI.clone(),
        rmm01_portfolio::RMM01PORTFOLIO_BYTECODE.clone(),
    );

    let portfolio_args = (
        recast_address(weth.address),
        recast_address(registry.address),
    );
    let portfolio = admin.deploy(&mut manager.environment, portfolio, portfolio_args);
    println!("Portfolio deployed at: {}", portfolio.address);

    let arbiter_token = SimulationContract::new(
        arbiter_token::ARBITERTOKEN_ABI.clone(),
        arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
    );

    // Choose name and symbol and combine into the constructor args required by ERC-20 contracts.
    let name = "ArbiterToken";
    let symbol = "ARBX";
    let args = (name.to_string(), symbol.to_string(), decimals);

    // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
    let arbiter_token_x = admin.deploy(&mut manager.environment, arbiter_token.clone(), args);
    println!("Arbiter Token x deployed at: {}", arbiter_token_x.address);

    let name = "ArbiterTokenY";
    let symbol = "ARBY";
    let args = (name.to_string(), symbol.to_string(), decimals);

    // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
    let arbiter_token_y = admin.deploy(&mut manager.environment, arbiter_token, args);
    println!("Arbiter Token Y deployed at: {}", arbiter_token_y.address);

    // Deploy LiquidExchange
    let price_to_check = 1000;
    let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
    let liquid_exchange = SimulationContract::new(
        liquid_exchange::LIQUIDEXCHANGE_ABI.clone(),
        liquid_exchange::LIQUIDEXCHANGE_BYTECODE.clone(),
    );
    let args = (
        recast_address(arbiter_token_x.address),
        recast_address(arbiter_token_y.address),
        initial_price,
    );
    let liquid_exchange_xy = admin.deploy(&mut manager.environment, liquid_exchange, args);

    // Deploy encoder target
    let encoder_contract = SimulationContract::new(
        encoder_target::ENCODERTARGET_ABI.clone(),
        encoder_target::ENCODERTARGET_BYTECODE.clone(),
    );
    let encoder_target = admin.deploy(&mut manager.environment, encoder_contract, ());

    println!("encoder target deployed at: {}", encoder_target.address);
    Ok((
        arbiter_token_x,
        arbiter_token_y,
        portfolio,
        liquid_exchange_xy,
        encoder_target,
    ))
}
fn intitalization_calls(manager: &mut SimulationManager, contracts: (SimulationContract<IsDeployed>,SimulationContract<IsDeployed>, SimulationContract<IsDeployed>, SimulationContract<IsDeployed>, SimulationContract<IsDeployed>)) ->  Result<(),Box<dyn Error>> {

    let admin = manager.agents.get("admin").unwrap();
    // Get all the necessary users.
    let (arbiter_token_x, arbiter_token_y, portfolio, liquid_exchange_xy, encoder_target) = contracts;

    let arbitrageur = manager.agents.get("arbitrageur").unwrap();

    // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
    let mint_amount = U256::from(1000);
    let input_arguments = (recast_address(arbitrageur.address()), mint_amount);
    let call_data = arbiter_token_x.encode_function("mint", input_arguments)?;

    // Call the 'mint' function to the arber. for token x
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_token_x,
        call_data.clone(),
        Uint::from(0),
    ); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
    println!(
        "Minted token_x to arber {:#?}",
        execution_result.is_success()
    );
    // Call the `mint` function to the arber for token y.
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        call_data,
        Uint::from(0),
    ); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
    println!(
        "Minted token_y to arber: {:#?}",
        execution_result.is_success()
    );

    // Mint max token_y to the liquid_exchange contract.
    let args = (recast_address(liquid_exchange_xy.address), U256::MAX);
    let call_data = arbiter_token_y.encode_function("mint", args)?;
    let result = admin.call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        call_data,
        Uint::from(0),
    );
    println!(
        "Minted token_y to liquid_excahnge: {:#?}",
        result.is_success()
    );

    // APROVALS
    // --------------------------------------------------------------------------------------------
    // aprove the liquid_exchange to spend the arbitrageur's token_x
    let approve_liquid_excahnge_args = (recast_address(liquid_exchange_xy.address), U256::MAX);
    let call_data = arbiter_token_x.encode_function("approve", approve_liquid_excahnge_args)?;

    let result = arbitrageur.call_contract(
        &mut manager.environment,
        &arbiter_token_x,
        call_data,
        Uint::from(0),
    );
    println!(
        "Aproved token_x to liquid_excahnge for arber: {:#?}",
        result.is_success()
    );

    // aprove the liquid_exchange to spend the arbitrageur's token_y
    let call_data = arbiter_token_y.encode_function("approve", approve_liquid_excahnge_args)?;
    let result = arbitrageur.call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        call_data,
        Uint::from(0),
    );
    println!(
        "Aproved token_y to liquid_excahnge for arber: {:#?}",
        result.is_success()
    );

    // aprove tokens on portfolio for arbitrageur
    let approve_portfolio_args = (recast_address(portfolio.address), U256::MAX);
    // Approve token_y
    let call_data = arbiter_token_y.encode_function("approve", approve_portfolio_args)?;
    let result = arbitrageur.call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        call_data,
        Uint::from(0),
    );
    println!(
        "Aproved token_y to portfolio for arber: {:#?}",
        result.is_success()
    );
    // approve token_x
    let call_data = arbiter_token_x.encode_function("approve", approve_portfolio_args)?;
    let result = arbitrageur.call_contract(
        &mut manager.environment,
        &arbiter_token_x,
        call_data,
        Uint::from(0),
    );
    println!(
        "Aproved token_y to portfolio for arber: {:#?}",
        result.is_success()
    );

    let encoder_args = (
        recast_address(arbiter_token_x.address),
        recast_address(arbiter_token_y.address),
    );
    let encoder_create_pair_call_data =
        encoder_target.encode_function("createPair", encoder_args)?;
    let encoded_create_pair_result = admin.call_contract(
        &mut manager.environment,
        &encoder_target,
        encoder_create_pair_call_data,
        Uint::from(0),
    );
    // Encoder Target encoding
    // TODO migrate this to a test late on
    let encoded_data = manager.unpack_execution(encoded_create_pair_result)?;
    let decoded_encoded_data: Bytes = encoder_target.decode_output("createPair", encoded_data)?;
    let portfolio_create_pair_call_data: Bytes =
        portfolio.encode_function("multiprocess", decoded_encoded_data.clone())?;

    // Folio encoding
    let token_x_ad = PH160::from(arbiter_token_x.address.as_fixed_bytes());
    let token_y_ad = PH160::from(arbiter_token_y.address.as_fixed_bytes());
    let codegen = Codegen::new(vec![Expression::Opcode(Opcode::CreatePair { token_0: (token_x_ad), token_1: (token_y_ad) })]);
    let create_pair = codegen.encode()[0].clone();
    let create_pair: Bytes = hex::decode(create_pair).unwrap().into_iter().collect();
    // make sure encodings have parity
    assert_eq!(create_pair, decoded_encoded_data);

    // Use the folio encoding
    let call_data: Bytes =
        portfolio.encode_function("multiprocess", create_pair)?;
    assert_eq!(portfolio_create_pair_call_data, call_data);
    let encoded_create_pair_result = admin.call_contract(
&mut manager.environment,
        &portfolio,
        portfolio_create_pair_call_data,
        Uint::from(0),
    );
    println!(
        "Encoded create pair with Folio encoding: {:#?}",
        encoded_create_pair_result.is_success()
    );
    let topics = encoded_create_pair_result.logs()[0].topics.clone();

    let h256_vec: Vec<H256> = topics
        .iter()
        .map(|b256| H256::from_slice(b256.as_bytes()))
        .collect();
    let i_portfolio = SimulationContract::new(
        i_portfolio::IPORTFOLIO_ABI.clone(),
        bindings::rmm01_portfolio::RMM01PORTFOLIO_BYTECODE.clone(),
    );
    let data = encoded_create_pair_result.logs()[0].data.clone();
    let (pair_id, _token_1, _token_2, _dec_1, _dec_2): (Token, Token, Token, Token, Token) =
        i_portfolio
            .base_contract
            .decode_event("CreatePair", h256_vec, ethers::types::Bytes(data))
            .unwrap();
    println!("Decoded pairID: {:#?}", pair_id.to_string());
    let pair_id: u32 = pair_id.to_string().parse::<u32>().unwrap();    
    let encoded_pair = portfolio.encode_function("pairs", pair_id.clone())?;
    let pairs = admin.call_contract(&mut manager.environment, &portfolio, encoded_pair, Uint::from(0));
    let result = manager.unpack_execution(pairs.clone())?;
    let decoded_result = i_portfolio.base_contract.decode_output("pairs", result)?;
    println!("got pairs: {:#?}", decoded_result);

    // // Create a new pool parameters
    // // --------------------------------------------------------------------------------------------
    // // uint24 pairId,
    // // address controller,
    // // uint16 priorityFee,
    // // uint16 fee,
    // // uint16 vol,
    // // uint16 dur,
    // // uint16 jit,
    // // uint128 maxPrice,
    // // uint128 price
    Ok(())
}
mod test {

    use std::str::FromStr;
    use primitive_types::H160 as PH160;

    use ethers::{types::H160, prelude::BaseContract, abi::Address, utils::parse_ether};
    use super::*;

    #[test]
    fn test_create_pair_target_encoding() -> Result<(), Box<dyn std::error::Error>> {
        // define the wad constant
        let decimals = 18_u8;
        let wad: U256 = U256::from(10_i64.pow(decimals as u32));
        // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
        let mut manager = SimulationManager::new();

        // Deploy the contracts
        let (arbiter_token_x,
        arbiter_token_y,
        _portfolio,
        _liquid_exchange_xy,
        encoder_target) = deploy_sim_contracts(&mut manager, wad)?;

        let admin = manager.agents.get("admin").unwrap();
        let encoder_args = (
            recast_address(arbiter_token_x.address),
            recast_address(arbiter_token_y.address),
        );
        assert_eq!(encoder_args.0, H160::from_str("0x2c1de3b4dbb4adebebb5dcecae825be2a9fc6eb6").unwrap());
        assert_eq!(encoder_args.1, H160::from_str("0x83769beeb7e5405ef0b7dc3c66c43e3a51a6d27f").unwrap());

        let encoder_create_pair_call_data =
            encoder_target.encode_function("createPair", encoder_args)?;

        let encoded_create_pair_result = admin.call_contract(
            &mut manager.environment,
            &encoder_target,
            encoder_create_pair_call_data,
            Uint::from(0),
        );
        assert_eq!(encoded_create_pair_result.is_success(), true);

        let encoded_data = manager.unpack_execution(encoded_create_pair_result)?;
        let decoded_encoded_data: Bytes = encoder_target.decode_output("createPair", encoded_data)?;
 
        assert_eq!(
            hex::encode(decoded_encoded_data.clone()),
            "0c2c1de3b4dbb4adebebb5dcecae825be2a9fc6eb683769beeb7e5405ef0b7dc3c66c43e3a51a6d27f");
        Ok(())
    }

    #[test]
    fn test_folio_encoding_for_create_pair() -> Result<(), Box<dyn std::error::Error>>{
        let decimals = 18_u8;
        let wad: U256 = U256::from(10_i64.pow(decimals as u32));
        // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
        let mut manager = SimulationManager::new();

        // Deploy the contracts
        let (arbiter_token_x,
        arbiter_token_y,
        _portfolio,
        _liquid_exchange_xy,
        _encoder_target) = deploy_sim_contracts(&mut manager, wad)?;

        // Folio encoding
        let token_x_ad = PH160::from(arbiter_token_x.address.as_fixed_bytes());
        let token_y_ad = PH160::from(arbiter_token_y.address.as_fixed_bytes());

        assert_eq!(token_x_ad, PH160::from_str("0x2c1de3b4dbb4adebebb5dcecae825be2a9fc6eb6").unwrap());
        assert_eq!(token_y_ad, PH160::from_str("0x83769beeb7e5405ef0b7dc3c66c43e3a51a6d27f").unwrap());

        let codegen = Codegen::new(vec![Expression::Opcode(Opcode::CreatePair { token_0: (token_x_ad), token_1: (token_y_ad) })]);
        let create_pair = codegen.encode()[0].clone();
        let create_pair: Bytes = hex::decode(create_pair).unwrap().into_iter().collect();
 
        assert_eq!(
            hex::encode(create_pair),
            "0c2c1de3b4dbb4adebebb5dcecae825be2a9fc6eb683769beeb7e5405ef0b7dc3c66c43e3a51a6d27f");
        Ok(())
    }

    #[test]
    fn test_create_pair_call() -> Result<(), Box<dyn std::error::Error>> {
        let decimals = 18_u8;
        let wad: U256 = U256::from(10_i64.pow(decimals as u32));
        // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
        let mut manager = SimulationManager::new();
        // Deploy the contracts
        let (arbiter_token_x,
        arbiter_token_y,
        portfolio,
        _liquid_exchange_xy,
        _encoder_target) = deploy_sim_contracts(&mut manager, wad)?;

        let admin = manager.agents.get("admin").unwrap();
        // Folio encoding // get args
        let token_x_ad = PH160::from(arbiter_token_x.address.as_fixed_bytes());
        let token_y_ad = PH160::from(arbiter_token_y.address.as_fixed_bytes());

        let codegen = Codegen::new(vec![Expression::Opcode(Opcode::CreatePair { token_0: (token_x_ad), token_1: (token_y_ad) })]);
        let create_pair = codegen.encode()[0].clone();
        let create_pair: Bytes = hex::decode(create_pair).unwrap().into_iter().collect();
        // println!("create_pair: {:?}", create_pair);

        // Portfolio encoding call data
        let create_pair_call_data = portfolio.encode_function("multiprocess", create_pair).unwrap();
        let encoded_create_pair_result = admin.call_contract(
            &mut manager.environment,
            &portfolio,
            create_pair_call_data,
            Uint::from(0),
        );
        assert_eq!(encoded_create_pair_result.is_success(), true);

        // This wont return anything, so we now have to call the i_portfolio pairs
        // to get the pair id we need to get it from the event here
        let topics = encoded_create_pair_result.logs()[0].topics.clone();

        let h256_vec: Vec<H256> = topics
            .iter()
            .map(|b256| H256::from_slice(b256.as_bytes()))
            .collect();
        let i_portfolio = SimulationContract::new(
            i_portfolio::IPORTFOLIO_ABI.clone(),
            bindings::rmm01_portfolio::RMM01PORTFOLIO_BYTECODE.clone(),
        );
        let data = encoded_create_pair_result.logs()[0].data.clone();
        let (pair_id, _token_1, _token_2, _dec_1, _dec_2): (Token, Token, Token, Token, Token) =
            i_portfolio
                .base_contract
                .decode_event("CreatePair", h256_vec, ethers::types::Bytes(data))
                .unwrap();
        println!("Decoded pairID: {:#?}", pair_id.to_string());
        let pair_id: u32 = pair_id.to_string().parse::<u32>().unwrap();    
        let encoded_pair: Bytes = i_portfolio.base_contract.encode("pairs", pair_id.clone())?.into_iter().collect();
        let request = admin.call_contract(
            &mut manager.environment,
            &portfolio,
            encoded_pair,
            Uint::from(0),
        );
        assert_eq!(request.is_success(), true);
        let unpacked = manager.unpack_execution(request)?;

        // Mental Picture of the data
        //     000000000000000000000000|2c1de3b4dbb4adebebb5dcecae825be2a9fc6eb6|0000000000000000000000000000000000000000000000000000000000000012...
        // token_x: ^ 24 bits:8bytes |0x2c1de3b4dbb4adebebb5dcecae825be2a9fc6eb6| ^ 64 bits, 12 = 18 decimals
        // ...|000000000000000000000000|83769beeb7e5405ef0b7dc3c66c43e3a51a6d27f|0000000000000000000000000000000000000000000000000000000000000012
        // token_y: ^ 24 bits:8bytes |0x83769beeb7e5405ef0b7dc3c66c43e3a51a6d27f| ^ 64 bits, 12 = 18 decimals

        let decoded_pairs_response: (H160, u8, H160, u8) = i_portfolio.base_contract.decode_output("pairs", unpacked)?;
        assert!(decoded_pairs_response.0 == arbiter_token_x.address.into());
        assert!(decoded_pairs_response.2 == arbiter_token_y.address.into());
        assert!(decoded_pairs_response.1 == decimals);
        assert!(decoded_pairs_response.3 == decimals);

        // test create_pool_encoding
        // let pair_id_bytes: Bytes = pair_id.to_string().parse::<Bytes>().unwrap();
        let pair_id: usize = pair_id.to_string().parse::<usize>().unwrap();    
        // send create pool call data to portfolio
        // check with the i_portfolio that the pair was created and state was changed
        // data encoding should look like this
        // let max_price = parse_ether("1").unwrap();
        // let max_price = hex::encode(max_price.to_string()).parse::<usize>().unwrap();
        let codegen = Codegen::new(vec![Expression::Opcode(Opcode::CreatePool {
            pair_id: pair_id, 
            controller: PH160::from(admin.address().as_fixed_bytes()), 
            priority_fee: 100 as usize, // 1%
            fee: 100 as usize, // 1%
            vol: 1 as usize,  // .1%
            dur: 65535 as usize, // magic perp number
            jit: 0 as usize, // 
            max_price: usize::MAX, // Strike
            price: usize::MAX, //
        })]);
        let code_gen = codegen.encode()[0].clone();
        assert_eq!(code_gen, "0b0000010000000000000000000000000000000000000001006400640001ffff000034000000000000000000ffffffffffffffff000000000000000000ffffffffffffffff");

        let create_pool_call_data: Bytes = hex::decode(code_gen).unwrap().into_iter().collect();

        // 0b|000001|0000000000000000000000000000000000000001|0064|0064|0001|ffff|0000|34|00|00000000000000000000000000000001|00|00000000000000000000000000000001
        // 0x0b|000001|0000000000000000000000000000000000000000|0001|0001|0001|ffff|0001|34|00|00000000000000000de0b6b3a7640000|00|00000000000000000de0b6b3a7640000
        // Prefixed:0x|a0fdf413|00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000045
        //|0b|000001|0000000000000000000000000000000000000000|0001|0001|007b|ffff|0001|34|00|00000000000000000de0b6b3a7640000|00|00000000000000000de0b6b3a7640000|000000000000000000000000000000000000000000000000000000
        // 0x|a0fdf413|
        // `0x | CREATE_POOL (1 byte) | pairId (3 bytes) | controller (20 bytes) | priorityFee (2 bytes) | fee (2 bytes) | vol (2 bytes) | dur (2 bytes) | jit (2 bytes) | pointerPrice (1 byte) | powerMaxPrice (1 byte) | baseMaxPrice (? bytes) | powerPrice (1 byte) | basePrice (? bytes)`\
        let encoded_from_string: Bytes = "0b0000010000000000000000000000000000000000000000000100010001ffff0001340000000000000000000de0b6b3a76400000000000000000000000de0b6b3a7640000".into();
        println!("encoded_from_string: {:#?}", encoded_from_string.clone());
        let abi_encoded_create_pool = portfolio.encode_function("multiprocess", encoded_from_string).unwrap();
        
        println!("abi_encoded_create_pool: {:#?}", hex::encode(abi_encoded_create_pool.clone()));
        // 0x|a0fdf413|00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000045|0b|000001|0000000000000000000000000000000000000000|0001|0001|007b|ffff|0001|34|00|00000000000000000de0b6b3a7640000|00|00000000000000000de0b6b3a7640000|000000000000000000000000000000000000000000000000000000
        // ..|a0fdf413|00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000045|0b|000001|0000000000000000000000000000000000000001|0064|0064|0001|ffff|0000|34|00|00000000000000000000000000000001|00|00000000000000000000000000000001|000000000000000000000000000000000000000000000000000000
        let encoded_create_pool_result = admin.call_contract(
            &mut manager.environment,
            &portfolio,
            abi_encoded_create_pool,
            Uint::from(0),
        );
        println!("encoded_create_pool_result: {:#?}", encoded_create_pool_result);
        assert_eq!(encoded_create_pool_result.is_success(), true);



        Ok(())
    }
}