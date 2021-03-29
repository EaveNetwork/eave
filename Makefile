# Initialize
.PHONY: init
init:
	./scripts/init.sh

# Clean
.PHONY: clean
clean:
	cargo clean; rm -rf node/eave-dev/target

# Check
.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check

# Build Developer Instance Beast
.PHONY: build
build: 
	SKIP_WASM_BUILD= cargo build --manifest-path node/eave-dev/Cargo.toml

.PHONY: run
run: 
	cargo run --manifest-path node/eave-dev/Cargo.toml -- --dev -lruntime=debug --instant-sealing

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --all


# Build WindMill

# Build Dawn

# Build ICE 

# Build EAVE

# Additonal commands 
# Need to check what this does
.PHONY: buildwasm
buildwasm: 
	SKIP_WASM_BUILD= cargo build 

.PHONY: buildrun
buildrun:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-06 cargo build --release; ./target/release/eave purge-chain -y --chain node/eave/chain_spec/local.json; ./target/release/eave --alice --chain node/eave/chain_spec/local.json
