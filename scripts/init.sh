#!/usr/bin/env bash

set -e

echo "*** Initializing WASM build environment"

if [ -z $CI ] ; then
   rustup update nightly
fi

rustup target add wasm32-unknown-unknown --toolchain nightly
rustup default nightly

git submodule update --init --recursive;
cargo clean
rm -rf bin/eave-dev/target
