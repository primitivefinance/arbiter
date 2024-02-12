# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/primitivefinance/arbiter/compare/arbiter-engine-v0.1.0...arbiter-engine-v0.1.1) - 2024-02-12

### Other
- remove `PhantomData` ([#868](https://github.com/primitivefinance/arbiter/pull/868))

## [0.1.0](https://github.com/primitivefinance/arbiter/releases/tag/arbiter-engine-v0.1.0) - 2024-01-23

### Added
- `World::stop()` method
- double agent test
- stream into process
- world/agent/behavior state machine
- generic decoded event stream
- multi agent simulation
- run()
- messaging layer + example starts
- pubsub provider worlds

### Fixed
- fix error
- `Engine::run_state`
- messaging channels for multithread
- leak private type

### Other
- Merge branch 'main' into arbiter-engine/world-agent-behaviors
- calling it here
- push up: test adjustment
- make clippy happy
- last push
- push up
- save
- helper functions for `StateMachine`s
- `StateMachine` trait
- agent::streaming() passes
- removing eth event leads to different behavior
- test gets stuck
- example ping pong
- timed message
- chore spelling
- chore version specification
- clean up
- mvp combo stream
- compile errors gone
- I am gutting things
- lock the versions wtf
- Update token_minter.rs
- channel sometimes closes too soon
- messager + transactor
- leaving off here,
- some ideas laid out
- attempting messager refactor
- *(deps)* bump futures from 0.3.29 to 0.3.30
- Merge branch 'main' into arbiter-core/db-backend
- *(deps)* bump anyhow from 1.0.77 to 1.0.78
- *(deps)* bump anyhow from 1.0.76 to 1.0.77
- *(deps)* bump anyhow from 1.0.75 to 1.0.76
- fmt
- cleanup
- agent behaviors
- fmt, lint, agent `HashMap`
- remove E, A from World
- #[ignore] `test_agent()`
- #[ignore] on `base_simulation()`
- cleanup
- replace `artemis-core`
- cleanup
- messager echo
- docs + cleanup
- fmt
- Delete world.rs
- impl `test_agent()`
- save state
- package creation
