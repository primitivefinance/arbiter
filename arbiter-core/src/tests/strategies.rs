use super::*;

#[test]
fn attach_agent() {
    let environment = &mut Environment::new(TEST_ENV_LABEL, 1.0, 1);
    let agent = Agent::new(TEST_AGENT_NAME);
    agent.attach_to_environment(environment);
    assert_eq!(environment.agents[0].name, TEST_AGENT_NAME);
}

#[test]
fn simulation_agent_wallet() {
    let environment = &mut Environment::new(TEST_ENV_LABEL, 1.0, 1);
    let agent = Agent::new(TEST_AGENT_NAME);
    agent.attach_to_environment(environment);
    assert_eq!(
        environment.agents[0].client.default_sender().unwrap(),
        Address::from_str("0x09e12ce98726acd515b68f87f49dc2e5558f6a72").unwrap()
    );
}

#[test]
fn multiple_agent_addresses() {
    let environment = &mut Environment::new(TEST_ENV_LABEL, 1.0, 1);
    let agent = Agent::new(TEST_AGENT_NAME);
    agent.attach_to_environment(environment);
    let agent2 = Agent::new(format!("new_{}", TEST_AGENT_NAME));
    agent2.attach_to_environment(environment);
    assert_ne!(
        environment.agents[0].client.default_sender(),
        environment.agents[1].client.default_sender()
    );
}

// TODO: Test to see that we prvent agents with the same name from being added.
#[test]
fn agent_name_collision() {
    todo!();
}