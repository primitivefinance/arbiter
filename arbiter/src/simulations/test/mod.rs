pub async fn run(
    price_process: PriceProcess,
    output_storage: OutputStorage,
    label: usize,
) -> Result<(), Box<dyn Error>> {
    let _start = Instant::now();

    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    let mut manager = SimulationManager::new();

    // Run the startup script
    startup::run(&mut manager)?;
    let arbitrageur = manager.agents.get("arbitrageur").unwrap();
    let admin = manager.agents.get("admin").unwrap();

    // Intialize the arbitrageur with the prices from the two exchanges.
    let arbitrageur = match arbitrageur {
        AgentType::SimpleArbitrageur(base_arbitrageur) => base_arbitrageur,
        _ => panic!(),
    };
    let liquid_exchange = manager
    .deployed_contracts
    .get("liquid_exchange_xy")
    .unwrap();
let result = arbitrageur.call(liquid_exchange, "price", vec![])?;
assert!(result.is_success());

let liquid_exchange_xy_price: U256 =
    liquid_exchange.decode_output("price", unpack_execution(result)?)?;
    // etc
    
    todo()
}