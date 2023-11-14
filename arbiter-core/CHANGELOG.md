# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.0](https://github.com/primitivefinance/arbiter/compare/arbiter-core-v0.7.6...arbiter-core-v0.8.0) - 2023-11-14

### Fixed
- fix hanging test
- fix shutdown receiver logic

### Other
- add missing shutdown signal for json filetype
- rm sleep in data capture test
- clippy + fmt
- revert back to old json style
- add shutdown sender and receiver oneshot for data collection/environment interaction
- log when file is done being written
- use polars JsonWriter
- rm useless warns
- try w warn and moving to singleton break
- break event collector loop

## [0.7.6](https://github.com/primitivefinance/arbiter/compare/arbiter-core-v0.7.5...arbiter-core-v0.7.6) - 2023-11-13

### Other
- Merge branch 'main' into feat/client-label
- Update mod.rs

## [0.7.5](https://github.com/primitivefinance/arbiter/compare/arbiter-core-v0.7.4...arbiter-core-v0.7.5) - 2023-11-13

### Added
- provide `RevmMiddleware` with label field

### Other
- tag allow unused on label
- Merge branch 'main' into feat/client-label

## [0.7.4](https://github.com/primitivefinance/arbiter/compare/arbiter-core-v0.7.3...arbiter-core-v0.7.4) - 2023-11-10

### Other
- release-plz
- clippy
- Merge branch 'main' into enhance/logging
- Merge pull request [#672](https://github.com/primitivefinance/arbiter/pull/672) from primitivefinance/outputfile_optionality
- abstract data_frame flattening
- Merge branch 'main' into outputfile_optionality
- *(deps)* update serde requirement in /arbiter-core
- *(deps)* bump serde from 1.0.190 to 1.0.191

## [0.7.2](https://github.com/primitivefinance/arbiter/compare/arbiter-core-v0.7.1...arbiter-core-v0.7.2) - 2023-11-06

### Added
- metadata
- pubsubclient test

### Fixed
- fix broken test

### Other
- remove generic
- Merge branch 'main' into feat/pubsubclient
- cleanup
- yo it work
- pubsub implemented

## [0.7.1](https://github.com/primitivefinance/arbiter/compare/arbiter-core-v0.7.0...arbiter-core-v0.7.1) - 2023-10-30

### Fixed
- fix test and change output directory

### Other
- *(deps)* bump serde_json from 1.0.107 to 1.0.108
- other test was broken too
- fmt
- remove other redundant file
- remove test
- fmt

## [0.7.0](https://github.com/primitivefinance/arbiter/compare/arbiter-core-v0.6.3...arbiter-core-v0.7.0) - 2023-10-27

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
- *(deps)* bump tracing from 0.1.39 to 0.1.40
