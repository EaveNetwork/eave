.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check

.PHONY: build
build: 
	SKIP_WASM_BUILD= cargo build 

.PHONY: builddev
build: 
	SKIP_WASM_BUILD= cargo build --manifest-path node/eave-dev/Cargo.toml

.PHONY: run
run: 
	cargo run --manifest-path node/eave-dev/Cargo.toml -- --dev -lruntime=debug --instant-sealing

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --all


# BuildDaybreak

# BuildDawn

# Build Beam

# Build Aurora


.PHONY: buildrun
buildrun:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-06 cargo build --release; ./target/release/eave purge-chain -y --chain node/eave/chain_spec/local.json; ./target/release/eave --alice --chain node/eave/chain_spec/local.json
