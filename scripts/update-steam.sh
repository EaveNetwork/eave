#!/usr/bin/env bash

set -e

# cargo clean
WASM_BUILD_TYPE=release cargo run --features with-steam-runtime --features with-ethereum-compatibility -- build-spec --raw --chain steam-latest > ./resources/steam-dist.json
