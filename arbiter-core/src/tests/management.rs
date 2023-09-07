use super::*;

#[test_log::test]
fn add_environment() {
    let mut manager = Manager::new();
    let params = EnvironmentParameters {
        label: TEST_ENV_LABEL.to_string(),
        block_type: BlockType::RandomlySampled {
            block_rate: TEST_BLOCK_RATE,
            block_time: TEST_BLOCK_TIME,
            seed: TEST_ENV_SEED,
        },
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
    let (manager, _client) = startup_randomly_sampled().unwrap();
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
    let (mut manager, _client) = startup_randomly_sampled().unwrap();

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
    let (mut manager, _client) = startup_randomly_sampled().unwrap();

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
async fn stop_environment_after_transactions() {
    let (mut manager, client) = startup_randomly_sampled().unwrap();
    ArbiterMath::deploy(client, ())
        .unwrap()
        .send()
        .await
        .unwrap();

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
