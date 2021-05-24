//TODO Update makefile for default builds of steam

# Initialize
.PHONY: init
init: submodule toolchain build-full

# Clean
.PHONY: clean
clean:
	cargo clean; rm -rf bin/eave-dev/target

# Check
.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check

# Build Developer Instance Beast - #Currently cant do both
.PHONY: beast
beast: 
	SKIP_WASM_BUILD= cargo build --release --manifest-path bin/eave-dev/Cargo.toml

.PHONY: runbeast
runbeast: 
	./bin/eave-dev/target/release/eave-dev --dev -lruntime=debug --instant-sealing

.PHONY: rundebug
rundebug: 
	cargo run --manifest-path bin/eave-dev/Cargo.toml -- --dev -lruntime=debug --instant-sealing

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --all


# Build WindMill

# Build Steam
.PHONY: steam
steam: 
	./scripts/steam.sh

# Build ICE 

# Build EAVE

# Additonal commands 
# Need to check what this does
.PHONY: buildwasm
buildwasm: 
	SKIP_WASM_BUILD= cargo build 

.PHONY: buildrun
buildrun:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-06 cargo build --release; ./target/release/eave purge-chain -y --chain bin/eave/chain_spec/local.json; ./target/release/eave --alice --chain bin/eave/chain_spec/local.json

.PHONY: submodule
submodule:
	git submodule update --init --recursive

.PHONY: toolchain
toolchain:
	./scripts/init.sh

.PHONY: build-full
build-full: 
	cargo clean; cargo build --release;

## From Rob Yeah, all the changes are in the Polkadot repo. you'll want to 
## cargo update -p sp-io -p polkadot-primitives -p cumulus-primitives-core as a way to bump all 3 commit refs

.PHONY: update-core
update-core:
	cargo update -p sp-io -p polkadot-primitives -p cumulus-primitives-core

