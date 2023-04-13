#![warn(missing_docs)]
//! This module contains the `Exchange` and `Cfmm` traits that are used to describe the functionality of a contract that can be used to swap tokens.

use ethers::types::Address;

/// A trait that describes the functionality of any exchange.
pub trait Exchange {
    /// Returns the price listed on the exchange for a pair.
    fn get_price(&self, token_x: Address, token_y: Address) -> f64;
    /// Swaps a token for another token using the exchange's logic.
    fn swap(&self, token_in: Address, amount: f64);
}

/// Trait that uses the `Exchange` trait to describe the more detailed functionality of a CFMM.
pub trait Cfmm: Exchange {
    /// Returns the list of pools that the CFMM supports.
    fn get_pools(&self) -> Vec<String>;
    /// Lets a user add liquidity to a pool.
    fn add_liquidity(&self, token: &str, amount: f64);
    /// Lets a user remove liquidity from a pool.
    fn remove_liquidity(&self, token: &str, amount: f64);
}

#[cfg(test)]
mod tests {
    use bindings;
    use ethers::{
        abi::Tokenize,
        prelude::{BaseContract, U256},
    };
    use revm::primitives::{ruint::Uint, B160};

    use crate::{
        environment::SimulationContract, manager::SimulationManager, utils::recast_address,
    };
    #[test]
    fn test_swap_x_for_y_liquid_exchange() {
        // define the wad constant
        let decimals = 18_u8;
        let wad: U256 = U256::from(10_i64.pow(decimals as u32));

        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();

        // Set up a user named alice
        let user_name = "alice";
        let user_address = B160::from_low_u64_be(2); // TODO: Prevent address collisions
        manager.create_user(user_address, user_name);

        // Pull out the admin and alice
        let admin = manager.agents.get("admin").unwrap();
        let alice = manager.agents.get(user_name).unwrap();

        // Create arbiter token general contract.
        let arbiter_token = SimulationContract::new(
            BaseContract::from(bindings::arbiter_token::ARBITERTOKEN_ABI.clone()),
            bindings::arbiter_token::ARBITERTOKEN_BYTECODE
                .clone()
                .into_iter()
                .collect(),
        );

        // Deploy token_x.
        let name = "Token X";
        let symbol = "TKNX";
        let args = (name.to_string(), symbol.to_string(), decimals).into_tokens();
        let token_x = admin.deploy(&mut manager.environment, arbiter_token.clone(), args);

        // Deploy token_y.
        let name = "Token Y";
        let symbol = "TKNY";
        let args = (name.to_string(), symbol.to_string(), decimals).into_tokens();
        let token_y = admin.deploy(&mut manager.environment, arbiter_token, args);

        // Deploy LiquidExchange
        let price_to_check = 1000;
        let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
        let liquid_exchange = SimulationContract::new(
            BaseContract::from(bindings::liquid_exchange::LIQUIDEXCHANGE_ABI.clone()),
            bindings::liquid_exchange::LIQUIDEXCHANGE_BYTECODE
                .clone()
                .into_iter()
                .collect(),
        );
        let args = (
            recast_address(token_x.address.unwrap()),
            recast_address(token_y.address.unwrap()),
            U256::from(initial_price),
        )
            .into_tokens();
        let liquid_exchange_xy = admin.deploy(&mut manager.environment, liquid_exchange, args);

        // Mint token_x to alice.
        let mint_amount = wad.checked_mul(U256::from(20)).unwrap(); // in wei units
        let args = (recast_address(alice.address()), mint_amount);
        let call_data = token_x
            .base_contract
            .encode("mint", args)
            .unwrap()
            .into_iter()
            .collect();
        admin.call_contract(&mut manager.environment, &token_x, call_data, Uint::from(0));

        // Mint max token_y to the liquid_exchange contract.
        let args = (
            recast_address(liquid_exchange_xy.address.unwrap()),
            U256::MAX,
        );
        let call_data = token_y
            .base_contract
            .encode("mint", args)
            .unwrap()
            .into_iter()
            .collect();
        admin.call_contract(&mut manager.environment, &token_y, call_data, Uint::from(0));

        // Have alice's approval for token_x to be spent by the liquid_exchange.
        let args = (
            recast_address(liquid_exchange_xy.address.unwrap()),
            U256::MAX,
        );
        let call_data = token_x
            .base_contract
            .encode("approve", args)
            .unwrap()
            .into_iter()
            .collect();
        alice.call_contract(&mut manager.environment, &token_x, call_data, Uint::from(0));

        // Have alice call the swap function to trade token_x for token_y.
        let swap_amount = mint_amount / 2;
        let call_data = liquid_exchange_xy
            .base_contract
            .encode(
                "swap",
                (
                    recast_address(token_x.address.unwrap()),
                    U256::from(swap_amount),
                ),
            )
            .unwrap()
            .into_iter()
            .collect();
        alice.call_contract(
            &mut manager.environment,
            &liquid_exchange_xy,
            call_data,
            Uint::from(0),
        );

        // Let alice check they spent the right amount of token_x
        let call_data = token_x
            .base_contract
            .encode("balanceOf", recast_address(user_address))
            .unwrap()
            .into_iter()
            .collect();
        let execution_result =
            alice.call_contract(&mut manager.environment, &token_x, call_data, Uint::from(0)); // Call the 'balanceOf' function.
        let value = manager.unpack_execution(execution_result);
        let response: U256 = token_x
            .base_contract
            .decode_output("balanceOf", value)
            .unwrap();
        println!("alice has {} token_x after swap", response);
        assert_eq!(response, mint_amount - swap_amount);

        // Let alice check they received the right amount of token_y
        let call_data = token_y
            .base_contract
            .encode("balanceOf", recast_address(user_address))
            .unwrap()
            .into_iter()
            .collect();
        let execution_result =
            alice.call_contract(&mut manager.environment, &token_y, call_data, Uint::from(0)); // Call the 'balanceOf' function.
        let value = manager.unpack_execution(execution_result);
        let response: U256 = token_y
            .base_contract
            .decode_output("balanceOf", value)
            .unwrap();
        println!("alice has {} token_y after swap", response);
        assert_eq!(response, swap_amount * U256::from(price_to_check));
    }

    #[test]
    fn test_swap_y_for_x_liquid_exchange() {
        // define the wad constant
        let decimals = 18_u8;
        let wad: U256 = U256::from(10_i64.pow(decimals as u32));

        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();

        // Set up a user named alice
        let user_name = "alice";
        let user_address = B160::from_low_u64_be(2); // TODO: Prevent address collisions
        manager.create_user(user_address, user_name);

        // Pull out the admin and alice
        let admin = manager.agents.get("admin").unwrap();
        let alice = manager.agents.get(user_name).unwrap();

        // Create arbiter token general contract.
        let arbiter_token = SimulationContract::new(
            BaseContract::from(bindings::arbiter_token::ARBITERTOKEN_ABI.clone()),
            bindings::arbiter_token::ARBITERTOKEN_BYTECODE
                .clone()
                .into_iter()
                .collect(),
        );

        // Deploy token_x.
        let name = "Token X";
        let symbol = "TKNX";
        let args = (name.to_string(), symbol.to_string(), decimals).into_tokens();
        let token_x = admin.deploy(&mut manager.environment, arbiter_token.clone(), args);

        // Deploy token_y.
        let name = "Token Y";
        let symbol = "TKNY";
        let args = (name.to_string(), symbol.to_string(), decimals).into_tokens();
        let token_y = admin.deploy(&mut manager.environment, arbiter_token, args);

        // Deploy LiquidExchange
        let price_to_check = 1000;
        let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
        let liquid_exchange = SimulationContract::new(
            BaseContract::from(bindings::liquid_exchange::LIQUIDEXCHANGE_ABI.clone()),
            bindings::liquid_exchange::LIQUIDEXCHANGE_BYTECODE
                .clone()
                .into_iter()
                .collect(),
        );
        let args = (
            recast_address(token_x.address.unwrap()),
            recast_address(token_y.address.unwrap()),
            U256::from(initial_price),
        )
            .into_tokens();
        let liquid_exchange_xy = admin.deploy(&mut manager.environment, liquid_exchange, args);

        // Mint token_y to alice.
        let mint_amount = wad.checked_mul(U256::from(20)).unwrap(); // in wei units
        let args = (recast_address(alice.address()), mint_amount);
        let call_data = token_y
            .base_contract
            .encode("mint", args)
            .unwrap()
            .into_iter()
            .collect();
        admin.call_contract(&mut manager.environment, &token_y, call_data, Uint::from(0));

        // Mint max token_x to the liquid_exchange contract.
        let args = (
            recast_address(liquid_exchange_xy.address.unwrap()),
            U256::MAX,
        );
        let call_data = token_x
            .base_contract
            .encode("mint", args)
            .unwrap()
            .into_iter()
            .collect();
        admin.call_contract(&mut manager.environment, &token_x, call_data, Uint::from(0));

        // Have alice's approval for token_y to be spent by the liquid_exchange.
        let args = (
            recast_address(liquid_exchange_xy.address.unwrap()),
            U256::MAX,
        );
        let call_data = token_y
            .base_contract
            .encode("approve", args)
            .unwrap()
            .into_iter()
            .collect();
        alice.call_contract(&mut manager.environment, &token_y, call_data, Uint::from(0));

        // Have alice call the swap function to trade token_y for token_x.
        let swap_amount = mint_amount / 2;
        let call_data = liquid_exchange_xy
            .base_contract
            .encode(
                "swap",
                (
                    recast_address(token_y.address.unwrap()),
                    U256::from(swap_amount),
                ),
            )
            .unwrap()
            .into_iter()
            .collect();
        alice.call_contract(
            &mut manager.environment,
            &liquid_exchange_xy,
            call_data,
            Uint::from(0),
        );

        // Let alice check they spent the right amount of token_y
        let call_data = token_y
            .base_contract
            .encode("balanceOf", recast_address(user_address))
            .unwrap()
            .into_iter()
            .collect();
        let execution_result =
            alice.call_contract(&mut manager.environment, &token_y, call_data, Uint::from(0)); // Call the 'balanceOf' function.
        let value = manager.unpack_execution(execution_result);
        let response: U256 = token_y
            .base_contract
            .decode_output("balanceOf", value)
            .unwrap();
        println!("alice has {} token_y after swap", response);
        assert_eq!(response, mint_amount - swap_amount);

        // Let alice check they received the right amount of token_x
        let call_data = token_x
            .base_contract
            .encode("balanceOf", recast_address(user_address))
            .unwrap()
            .into_iter()
            .collect();
        let execution_result =
            alice.call_contract(&mut manager.environment, &token_x, call_data, Uint::from(0)); // Call the 'balanceOf' function.
        let value = manager.unpack_execution(execution_result);
        let response: U256 = token_x
            .base_contract
            .decode_output("balanceOf", value)
            .unwrap();
        println!("alice has {} token_x after swap", response);
        assert_eq!(response, U256::from(10_i64.pow(16)));
    }

    #[test]
    fn test_price_simulation_oracle() {
        // Get a price path from the oracle.
        let timestep = 1.0;
        let timescale = String::from("day");
        let num_steps = 7;
        let initial_price = 100.0;
        let drift = 0.5;
        let volatility = 0.1;
        let seed = 123;
        let ou_mean_reversion_speed = 0.1;
        let ou_mean_price = 1.0;
        let gbm = crate::price_simulation::PriceSimulation::new(
            timestep,
            timescale,
            num_steps,
            initial_price,
            drift,
            volatility,
            ou_mean_reversion_speed,
            ou_mean_price,
            seed,
        );
        let (_time, price_path) = gbm.gbm();

        // Set up the liquid exchange
        // define the wad constant
        let decimals = 18_u8;
        let wad: U256 = U256::from(10_i64.pow(decimals as u32));

        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();
        let admin = manager.agents.get("admin").unwrap();

        // Create arbiter token general contract.
        let arbiter_token = SimulationContract::new(
            BaseContract::from(bindings::arbiter_token::ARBITERTOKEN_ABI.clone()),
            bindings::arbiter_token::ARBITERTOKEN_BYTECODE
                .clone()
                .into_iter()
                .collect(),
        );

        // Deploy token_x.
        let name = "Token X";
        let symbol = "TKNX";
        let args = (name.to_string(), symbol.to_string(), decimals).into_tokens();
        let token_x = admin.deploy(&mut manager.environment, arbiter_token.clone(), args);

        // Deploy token_y.
        let name = "Token Y";
        let symbol = "TKNY";
        let args = (name.to_string(), symbol.to_string(), decimals).into_tokens();
        let token_y = admin.deploy(&mut manager.environment, arbiter_token, args);

        // Deploy LiquidExchange
        let price_to_check = 1000;
        let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
        let liquid_exchange = SimulationContract::new(
            BaseContract::from(bindings::liquid_exchange::LIQUIDEXCHANGE_ABI.clone()),
            bindings::liquid_exchange::LIQUIDEXCHANGE_BYTECODE
                .clone()
                .into_iter()
                .collect(),
        );
        let args = (
            recast_address(token_x.address.unwrap()),
            recast_address(token_y.address.unwrap()),
            U256::from(initial_price),
        )
            .into_tokens();
        let liquid_exchange_xy = admin.deploy(&mut manager.environment, liquid_exchange, args);

        // Loop over and set prices on the liquid exchange from the oracle.
        for price in price_path {
            println!("Price from price path: {}", price);
            let wad_price = crate::price_simulation::float_to_wad(price);
            println!("WAD price: {}", wad_price);
            let call_data = liquid_exchange_xy
                .base_contract
                .encode("setPrice", wad_price)
                .unwrap()
                .into_iter()
                .collect();
            admin.call_contract(
                &mut manager.environment,
                &liquid_exchange_xy,
                call_data,
                Uint::from(0),
            );
            // Check that the price is set correctly
            let call_data = liquid_exchange_xy
                .base_contract
                .encode("price", ())
                .unwrap()
                .into_iter()
                .collect();
            let execution_result = admin.call_contract(
                &mut manager.environment,
                &liquid_exchange_xy,
                call_data,
                Uint::from(0),
            );
            let value = manager.unpack_execution(execution_result);
            let response: U256 = liquid_exchange_xy
                .base_contract
                .decode_output("price", value)
                .unwrap();
            println!("Price from the exchange: {}", response);
            assert_eq!(response, wad_price);
        }
    }
}
