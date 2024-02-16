# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.17](https://github.com/primitivefinance/arbiter/compare/arbiter-v0.4.16...arbiter-v0.4.17) - 2024-02-16

### Added
- *(engine)* optional stream for behaviors ([#899](https://github.com/primitivefinance/arbiter/pull/899))

## [0.4.16](https://github.com/primitivefinance/arbiter/compare/arbiter-v0.4.15...arbiter-v0.4.16) - 2024-02-15

### Other
- release ([#890](https://github.com/primitivefinance/arbiter/pull/890))

## [0.4.15](https://github.com/primitivefinance/arbiter/compare/arbiter-v0.4.14...arbiter-v0.4.15) - 2024-02-13

### Added
- *(arbiter-macros)* `#[arbiter_macros::main]` and a project example ([#880](https://github.com/primitivefinance/arbiter/pull/880))

### Other
- Engine/world from config ([#882](https://github.com/primitivefinance/arbiter/pull/882))
- Docs/examples ([#881](https://github.com/primitivefinance/arbiter/pull/881))
- release ([#878](https://github.com/primitivefinance/arbiter/pull/878))

## [0.4.14](https://github.com/primitivefinance/arbiter/compare/arbiter-v0.4.13...arbiter-v0.4.14) - 2024-01-23

### Added
- `World::stop()` method
- double agent test
- stream into process
- world/agent/behavior state machine
- generic decoded event stream
- database disk handling
- run()
- messaging layer + example starts
- pubsub provider worlds

### Fixed
- fix error
- `arbiter-core` tests
- `Engine::run_state`
- tests and test workflow

### Other
- Merge pull request [#811](https://github.com/primitivefinance/arbiter/pull/811) from primitivefinance/dependabot/cargo/chrono-0.4.32
- Merge pull request [#804](https://github.com/primitivefinance/arbiter/pull/804) from primitivefinance/arbiter-core/impl-signer
- Update README.md
- rm `arbiter-derive`
- Merge pull request [#807](https://github.com/primitivefinance/arbiter/pull/807) from primitivefinance/dependabot/cargo/clap-4.4.18
- Merge pull request [#806](https://github.com/primitivefinance/arbiter/pull/806) from primitivefinance/dependabot/cargo/proc-macro2-1.0.78
- *(deps)* bump proc-macro2 from 1.0.76 to 1.0.78
- Merge branch 'main' into arbiter-engine/world-agent-behaviors
- pin revm versions
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
- version finagle
- chore spelling
- chore version specification
- Update Cargo.lock
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
- Merge pull request [#782](https://github.com/primitivefinance/arbiter/pull/782) from primitivefinance/dependabot/cargo/clap-4.4.14
- Merge pull request [#781](https://github.com/primitivefinance/arbiter/pull/781) from primitivefinance/dependabot/cargo/revm-30bbcdf
- Merge pull request [#780](https://github.com/primitivefinance/arbiter/pull/780) from primitivefinance/dependabot/cargo/proc-macro2-1.0.76
- *(deps)* bump proc-macro2 from 1.0.75 to 1.0.76
- *(deps)* bump revm from `67331de` to `aff028e`
- *(deps)* bump thiserror from 1.0.51 to 1.0.55
- Merge pull request [#775](https://github.com/primitivefinance/arbiter/pull/775) from primitivefinance/dependabot/cargo/proc-macro2-1.0.75
- Merge pull request [#774](https://github.com/primitivefinance/arbiter/pull/774) from primitivefinance/dependabot/cargo/clap-4.4.13
- Merge pull request [#773](https://github.com/primitivefinance/arbiter/pull/773) from primitivefinance/dependabot/cargo/syn-2.0.43
- *(deps)* bump syn from 2.0.42 to 2.0.43
- Merge pull request [#770](https://github.com/primitivefinance/arbiter/pull/770) from primitivefinance/dependabot/cargo/tokio-1.35.1
- Merge pull request [#771](https://github.com/primitivefinance/arbiter/pull/771) from primitivefinance/dependabot/cargo/futures-0.3.30
- *(deps)* bump crossbeam-channel from 0.5.9 to 0.5.10
- Merge branch 'main' into arbiter-core/db-backend
- mod database and `coprocessor`
- `arbiter-core` compiling
- Merge branch 'main' into arbiter-core/db-backend
- #[ignore] `test_agent()`
- #[ignore] on `base_simulation()`
- cleanup
- replace `artemis-core`
- cleanup
- messager echo
- *(deps)* bump tokio from 1.34.0 to 1.35.0
- docs + cleanup
- *(deps)* bump test-log from 0.2.13 to 0.2.14
- fmt
- Delete world.rs
- Update Cargo.lock
- Merge branch 'main' into arbiter-engine/init
- dependencies
- Update tests.rs
- Update lint.yaml
- Revert "Merge pull request [#725](https://github.com/primitivefinance/arbiter/pull/725) from quidproquoo/migrate-alloy"
- *(deps)* bump clap from 4.4.10 to 4.4.11
- minor
- Merge branch 'main' into release-plz-2023-11-21T13-00-44Z

## [0.4.13](https://github.com/primitivefinance/arbiter/compare/arbiter-v0.4.12...arbiter-v0.4.13) - 2023-11-18

### Other
- *(deps)* bump polars from 0.34.2 to 0.35.2
- codecov.yaml -> codecov.yml
- arbiter bind doesn't output nested bindings now
- Merge branch 'main' into arbiter_bindings
- solstat compiles
- forge install: solstat
- forge install: solmate
- contracts
- forge install: forge-std
- forge init
- arbiter-bindings

## [0.4.12](https://github.com/primitivefinance/arbiter/compare/arbiter-v0.4.11...arbiter-v0.4.12) - 2023-11-15

### Other
- improve error handling
- working as intended
- added a passing itterator test
- added a passing itterator test
- abstracted broken code into for_each_submodile() function

## [0.4.11](https://github.com/primitivefinance/arbiter/compare/arbiter-v0.4.10...arbiter-v0.4.11) - 2023-11-14

### Other
- Merge branch 'main' into dependabot/cargo/clap-4.4.8
- abiter -> arbiter

## [0.4.10](https://github.com/primitivefinance/arbiter/compare/arbiter-v0.4.9...arbiter-v0.4.10) - 2023-11-13

### Other
- Merge pull request [#700](https://github.com/primitivefinance/arbiter/pull/700) from primitivefinance/feat/client-label
- ignore changelog
- cargo lock

## [0.4.9](https://github.com/primitivefinance/arbiter/compare/arbiter-v0.4.8...arbiter-v0.4.9) - 2023-11-13

### Other
- tag allow unused on label
- Merge branch 'main' into feat/client-label
- wont fail tests in CI
- code spell
- check-lockfile
- Update Cargo.lock
- organize tests to colins suggestion
- Update release-plz.yml

## [0.4.8](https://github.com/primitivefinance/arbiter/compare/arbiter-v0.4.7...arbiter-v0.4.8) - 2023-11-10

### Other
- release-plz
- Update README.md
- Merge pull request [#693](https://github.com/primitivefinance/arbiter/pull/693) from primitivefinance/enhance/logging
- clippy
- Merge branch 'main' into enhance/logging
- Merge pull request [#691](https://github.com/primitivefinance/arbiter/pull/691) from primitivefinance/dependabot/cargo/toml-0.8.8
- Merge pull request [#672](https://github.com/primitivefinance/arbiter/pull/672) from primitivefinance/outputfile_optionality
- Merge branch 'main' into cargo_lock
- move to release-plz pr commits
- commit to head_ref
- auto commit lock file
- Fix broken link in README.md
- *(deps)* bump serde from 1.0.191 to 1.0.192
- Merge pull request [#665](https://github.com/primitivefinance/arbiter/pull/665) from primitivefinance/arbiter_config
- Merge pull request [#679](https://github.com/primitivefinance/arbiter/pull/679) from primitivefinance/dependabot/cargo/syn-2.0.39
- *(deps)* bump syn from 2.0.38 to 2.0.39
- *(deps)* bump serde from 1.0.190 to 1.0.191

## [0.4.7](https://github.com/primitivefinance/arbiter/compare/arbiter-v0.4.6...arbiter-v0.4.7) - 2023-11-06

### Other
- updated the following local packages: arbiter-core

## [0.4.6](https://github.com/primitivefinance/arbiter/compare/arbiter-v0.4.5...arbiter-v0.4.6) - 2023-10-30

### Other
- *(deps)* bump serde_json from 1.0.107 to 1.0.108
- Merge pull request [#658](https://github.com/primitivefinance/arbiter/pull/658) from primitivefinance/dependabot/cargo/tempfile-3.8.1
- Merge branch 'main' into dependabot/cargo/tempfile-3.8.1
- other test was broken too
- remove book deploy
- don't use macos in CI

## [0.4.5](https://github.com/primitivefinance/arbiter/compare/arbiter-v0.4.4...arbiter-v0.4.5) - 2023-10-27

### Fixed
- fix data output test to rmdir after completion
- fix clippy + fmt
- fix tests

### Other
- Merge pull request [#656](https://github.com/primitivefinance/arbiter/pull/656) from primitivefinance/colin/fix-event-logger
- rm printlns
- fmt
- create valid json
- Update data_collection.rs
- wip
- push json logs to vec
- it works!!!!!!!!!!!!
- wip
- unsafe transmute
- Update data_collection.rs
- broadcast enum
- create a channel for direct reads
- Merge pull request [#642](https://github.com/primitivefinance/arbiter/pull/642) from primitivefinance/dependabot/cargo/clap-4.4.7

## [0.4.4](https://github.com/primitivefinance/arbiter/compare/arbiter-v0.4.3...arbiter-v0.4.4) - 2023-10-23

### Other
- cargo lock
- *(deps)* update toml requirement from =0.8.2 to =0.8.4

## [0.4.3](https://github.com/primitivefinance/arbiter/compare/arbiter-v0.4.2...arbiter-v0.4.3) - 2023-10-22

### Fixed
- *(bind)* output directory outputs to src/bindings

### Other
- Merge pull request [#633](https://github.com/primitivefinance/arbiter/pull/633) from primitivefinance/md_book
- Merge pull request [#634](https://github.com/primitivefinance/arbiter/pull/634) from primitivefinance/fix/arbiter-bind-output

## [0.4.2](https://github.com/primitivefinance/arbiter/compare/arbiter-v0.4.1...arbiter-v0.4.2) - 2023-10-20

### Other
- code spell
- clippy
- tests pass
- 🧪
- Merge pull request [#618](https://github.com/primitivefinance/arbiter/pull/618) from primitivefinance/feat/release_pls
- bump version
- forking EOA
- *(deps)* update thiserror requirement from =1.0.49 to =1.0.50
- fmt
- Merge branch 'main' into feature/forking_eoa
- eoa fork
- Update tests.rs
- eoa forking
