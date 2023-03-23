#!/bin/bash

forge bind -C lib/arbmod/contracts -b crates/bindings/ --crate-name bindings --overwrite --single-file --skip-cargo-toml
echo "Generated bindings for arbmod" 
forge bind -C lib/portfolio/contracts -b crates/bindings/ --crate-name bindings --overwrite --single-file --skip-cargo-toml
echo "Generated bindings for portfolio"