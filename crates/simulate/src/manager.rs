/// Manages an EVM simulation.
pub struct SimulationManager {
    /// `SimulationEnvironment` that the`SimulationManager` controls.
    environment: SimulationEnvironment,

    /// Contains the default transaction options for revm such as gas limit and gas price.
    pub transact_settings: TransactSettings,
}

impl SimulationManager {
    // TODO: This should call `recast_address` when a B160 is passed as an arg. Not sure how to handle this yet.
    /// Deploy a contract.
    pub fn deploy<T: Tokenize>(
        &mut self,
        contract: SimulationContract<NotDeployed>,
        agent: dyn Agent,
        args: T,
    ) -> SimulationContract<IsDeployed> {
        // Append constructor args (if available) to generate the deploy bytecode;
        let constructor = contract.base_contract.abi().constructor();
        
        let bytecode = match constructor {
            Some(constructor) => Bytes::from(
                constructor
                    .encode_input(contract.bytecode.clone(), &args.into_tokens())
                    .unwrap(),
            ),
            None => Bytes::from(contract.bytecode.clone()),
        };

        // Take the execution result and extract the contract address.
        // Manager address will always be the sender for contract deployments.

        let deploy_transaction = self.build_deploy_transaction(bytecode);
        let execution_result = self.environment.execute(deploy_transaction);
        let output = match execution_result {
            ExecutionResult::Success { output, .. } => output,
            ExecutionResult::Revert { output, .. } => panic!("Failed due to revert: {:?}", output),
            ExecutionResult::Halt { reason, .. } => panic!("Failed due to halt: {:?}", reason),
        };
        let contract_address = match output {
            Output::Create(_, address) => address.unwrap(),
            _ => panic!("failed"),
        };

        contract.to_deployed(contract_address)
    }

    /// Create a user account.
    pub fn create_user(&mut self, address: B160) {
        self.environment
            .evm
            .db()
            .unwrap()
            .insert_account_info(address, AccountInfo::default());
    }

    /// Give an address a specified amount of raw ether.
    pub fn allocate(&mut self, address: B160, amount: U256) {
        let account = self
            .environment
            .evm
            .db()
            .unwrap()
            .load_account(address)
            .unwrap();

        account.info.balance += amount;
    }
}