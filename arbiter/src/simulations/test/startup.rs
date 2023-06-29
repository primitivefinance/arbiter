pub(crate) fn run(manager: &mut SimulationManager) -> Result<(), Box<dyn Error>> {
        let weth_address = manager.deployed_contracts.get("weth").unwrap().address;
        deploy_contracts(manager, weth_address)?;
        let liquid_exchange_xy = manager
            .deployed_contracts
            .get("liquid_exchange_xy")
            .unwrap();
        let address = B160::from_low_u64_be(2);
        let event_filters = vec![SimulationEventFilter::new(
            liquid_exchange_xy,
            "PriceChange",
        )];
        let arbitrageur = SimpleArbitrageur::new(
            "arbitrageur",
            event_filters,
            U256::from(997_000_000_000_000_000u128).into(),
        );
        manager
            .activate_agent(AgentType::SimpleArbitrageur(arbitrageur), address)
            .unwrap();
    
        mint(
            &manager.deployed_contracts,
            manager.agents.get("admin").unwrap(),
            manager.agents.get("arbitrageur").unwrap(),
        )?;
        approve(
            manager.agents.get("admin").unwrap(),
            manager.agents.get("arbitrageur").unwrap(),
            &manager.deployed_contracts,
        )?;
    
        allocate(
            manager.agents.get("admin").unwrap(),
            &manager.deployed_contracts,
        )?;
    
        Ok()
    }
    pub fn deploy() {
    todo()
    }
    
    pub fn mint() {
    todo()
    }

    pub fn approve() {
    todo()
    }

    pub fn allocate() {
    todo()
    }
    