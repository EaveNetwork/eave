#!/usr/bin/env bash

set -e

# cargo clean
WASM_BUILD_TYPE=release cargo run --features=with-ice-runtime --features=on-chain-release-build -- build-spec --raw --chain ice-latest > ./resources/ice-dist.json
