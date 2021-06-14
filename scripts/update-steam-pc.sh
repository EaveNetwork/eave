#!/usr/bin/env bash

set -e

# cargo clean
WASM_BUILD_TYPE=release cargo run -- build-spec --chain steam-latest --raw > ./resources/steam-pc-dist.json