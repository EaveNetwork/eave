cargo build --release
./target/release/eave build-spec --disable-default-bootnode > ./resources/steam-local-plain.json
./target/release/eave build-spec --chain=./resources/steam-local-plain.json --raw --disable-default-bootnode > ./resources/steam-local.json
./target/release/eave export-genesis-state --parachain-id 7 > ./resources/para-7-genesis
./target/release/eave export-genesis-wasm > ./resources/para-7.wasm
rm -rf local-alioth
./target/release/eave -d local-alioth --collator --name Alioth --ws-port 9946 --chain ./resources/steam-local.json --parachain-id 7 -- --chain ../polkadot/rococo_local.json


