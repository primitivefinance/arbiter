use super::*;

#[test_log::test]
fn add_environment() {
    let mut manager = Manager::new();
    let params = EnvironmentParameters {
        label: TEST_ENV_LABEL.to_string(),
        block_rate: 1.0,
        seed: 1,
    };
    manager.add_environment(params).unwrap();
    assert!(manager
        .environments
        .contains_key(&TEST_ENV_LABEL.to_string()));
    assert_eq!(
        manager
            .environments
            .get(&TEST_ENV_LABEL.to_string())
            .unwrap()
            .state
            .load(std::sync::atomic::Ordering::Relaxed),
        State::Initialization
    );
}

#[test_log::test]
fn run_environment() {
    let mut manager = Manager::new();
    let params = EnvironmentParameters {
        label: TEST_ENV_LABEL.to_string(),
        block_rate: 1.0,
        seed: 1,
    };
    manager.add_environment(params).unwrap();
    manager.start_environment(TEST_ENV_LABEL).unwrap();
    assert_eq!(
        manager
            .environments
            .get(&TEST_ENV_LABEL.to_string())
            .unwrap()
            .state
            .load(std::sync::atomic::Ordering::Relaxed),
        State::Running
    );
}

#[test_log::test]
fn pause_environment() {
    let mut manager = Manager::new();
    let params = EnvironmentParameters {
        label: TEST_ENV_LABEL.to_string(),
        block_rate: 1.0,
        seed: 1,
    };
    manager.add_environment(params).unwrap();
    manager.start_environment(TEST_ENV_LABEL).unwrap();
    manager.pause_environment(TEST_ENV_LABEL).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    assert_eq!(
        manager
            .environments
            .get(&TEST_ENV_LABEL.to_string())
            .unwrap()
            .state
            .load(std::sync::atomic::Ordering::Relaxed),
        State::Paused
    );
    manager.start_environment(TEST_ENV_LABEL).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    assert_eq!(
        manager
            .environments
            .get(&TEST_ENV_LABEL.to_string())
            .unwrap()
            .state
            .load(std::sync::atomic::Ordering::Relaxed),
        State::Running
    );
}

#[test_log::test]
fn stop_environment() {
    let mut manager = Manager::new();
    let params = EnvironmentParameters {
        label: TEST_ENV_LABEL.to_string(),
        block_rate: 1.0,
        seed: 1,
    };
    manager.add_environment(params).unwrap();
    manager.start_environment(TEST_ENV_LABEL).unwrap();
    manager.stop_environment(TEST_ENV_LABEL).unwrap();
    assert_eq!(
        manager
            .environments
            .get(&TEST_ENV_LABEL.to_string())
            .unwrap()
            .state
            .load(std::sync::atomic::Ordering::Relaxed),
        State::Stopped
    );
}

#[tokio::test]
async fn stop_environment_after_transactions() -> Result<()> {
    let mut manager = Manager::new();
    let params = EnvironmentParameters {
        label: TEST_ENV_LABEL.to_string(),
        block_rate: 1.0,
        seed: 1,
    };
    manager.add_environment(params).unwrap();
    manager.start_environment(TEST_ENV_LABEL).unwrap();

    // Send some transactions (e.g., deploy `ArbiterMath` which is easy and has no
    // args)
    let client = Arc::new(RevmMiddleware::new(
        manager.environments.get(TEST_ENV_LABEL).unwrap(),
        Some(TEST_SIGNER_SEED_AND_LABEL.to_string()),
    ));
    ArbiterMath::deploy(client, ())?.send().await?;

    manager.stop_environment(TEST_ENV_LABEL).unwrap();
    assert_eq!(
        manager
            .environments
            .get(&TEST_ENV_LABEL.to_string())
            .unwrap()
            .state
            .load(std::sync::atomic::Ordering::Relaxed),
        State::Stopped
    );
    Ok(())
}
