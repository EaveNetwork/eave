.PHONY: run
run:
	cargo run --features with-steam-runtime -- --dev -lruntime=debug --instant-sealing

.PHONY: run-eth
run-eth:
	cargo run --features with-steam-runtime --features with-ethereum-compatibility -- --dev -lruntime=debug -levm=debug --instant-sealing

.PHONY: run-ice
run-ice:
	cargo run --features with-ice-runtime -- --chain=ice

.PHONY: toolchain
toolchain:
	./scripts/init.sh

.PHONY: build
build: githooks
	SKIP_WASM_BUILD= cargo build --features with-steam-runtime

.PHONY: build-full
build-full: githooks
	cargo build --features with-steam-runtime

.PHONY: build-steam
build-steam: githooks
	cargo build --features with-steam-runtime --release

.PHONY: build-all
build-all:
	cargo build --locked --features with-all-runtime

.PHONY: check
check: githooks
	SKIP_WASM_BUILD= cargo check --features with-steam-runtime

.PHONY: check
check-ice: githooks
	SKIP_WASM_BUILD= cargo check --features with-steam-runtime

.PHONY: check-tests
check-tests: githooks
	SKIP_WASM_BUILD= cargo check --features with-all-runtime --tests --all

.PHONY: check-all
check-all: check-runtimes check-benchmarks

.PHONY: check-runtimes
check-runtimes:
	SKIP_WASM_BUILD= cargo check --features with-all-runtime --tests --all

.PHONY: check-benchmarks
check-benchmarks:
	SKIP_WASM_BUILD= cargo check --features runtime-benchmarks --no-default-features --target=wasm32-unknown-unknown -p steam-runtime
	SKIP_WASM_BUILD= cargo check --features runtime-benchmarks --no-default-features --target=wasm32-unknown-unknown -p ice-runtime

.PHONY: check-debug
check-debug:
	RUSTFLAGS="-Z macro-backtrace" SKIP_WASM_BUILD= cargo +nightly check --features with-steam-runtime

.PHONY: check-try-runtime
check-try-runtime:
	SKIP_WASM_BUILD= cargo check --features try-runtime --features with-all-runtime

.PHONY: test
test: githooks
	SKIP_WASM_BUILD= cargo test --features with-mandala-runtime --all

.PHONY: test-eth
test-eth: githooks
	SKIP_WASM_BUILD= cargo test --features with-steam-runtime --features with-ethereum-compatibility test_evm_module
	SKIP_WASM_BUILD= cargo test --features with-steam-runtime --features with-ethereum-compatibility should_not_kill_contract_on_transfer_all
	SKIP_WASM_BUILD= cargo test --features with-steam-runtime --features with-ethereum-compatibility schedule_call_precompile_should_work
	SKIP_WASM_BUILD= cargo test --features with-steam-runtime --features with-ethereum-compatibility schedule_call_precompile_should_handle_invalid_input

.PHONY: test-runtimes
test-runtimes:
	SKIP_WASM_BUILD= cargo test --all --features with-all-runtime

.PHONY: test-benchmarking
test-benchmarking:
	cargo test --features runtime-benchmarks --features with-all-runtime --features --all benchmarking

.PHONY: test-all
test-all: test-runtimes test-eth test-benchmarking

.PHONY: purge
purge: target/debug/eave
	target/debug/eave purge-chain --dev -y

.PHONY: restart
restart: purge run

target/debug/eave:
	SKIP_WASM_BUILD= cargo build --features with-steam-runtime

GITHOOKS_SRC = $(wildcard githooks/*)
GITHOOKS_DEST = $(patsubst githooks/%, .git/hooks/%, $(GITHOOKS_SRC))

.git/hooks:
	mkdir .git/hooks

.git/hooks/%: githooks/%
	cp $^ $@

.PHONY: githooks
githooks: .git/hooks $(GITHOOKS_DEST)

.PHONY: init
init: toolchain submodule build-full

.PHONY: submodule
submodule:
	git submodule update --init --recursive

.PHONY: update-orml
update-orml:
	cd orml && git checkout master && git pull
	git add orml

.PHONY: update
update: update-orml cargo-update check-all

.PHONY: cargo-update
cargo-update:
	cargo update

.PHONY: build-wasm-steam
build-wasm-steam:
	./scripts/build-only-wasm.sh -p steam-runtime --features=with-ethereum-compatibility

.PHONY: build-wasm-ice
build-wasm-ice:
	./scripts/build-only-wasm.sh -p ice-runtime --features=on-chain-release-build

.PHONY: srtool-build-wasm-steam
srtool-build-wasm-steam:
	PACKAGE=steam-runtime BUILD_OPTS="--features with-ethereum-compatibility" ./scripts/srtool-build.sh

.PHONY: srtool-build-wasm-ice
srtool-build-wasm-ice:
	PACKAGE=ice-runtime BUILD_OPTS="--features on-chain-release-build" ./scripts/srtool-build.sh

.PHONY: generate-tokens
generate-tokens:
	./scripts/generate-tokens-and-predeploy-contracts.sh

## Prep chainspecs and genesis and wasm

## Run Steam Locally

## Run Aqua and Noria

## Run Steam on Westend

## Run ICE on Kusama

