## TODO uodate this to build the correct paracahin files 
## TODO the parachain ID is now reserved via the polkadot explorer before building these files.

cargo build --release
./target/release/eave build-spec --disable-default-bootnode > ./resources/steam-local-plain.json
./target/release/eave build-spec --chain=./resources/steam-local-plain.json --raw --disable-default-bootnode > ./resources/steam-local.json
./target/release/eave export-genesis-state --parachain-id 77 > ./resources/para-77-genesis
./target/release/eave export-genesis-wasm > ./resources/para-77.wasm
rm -rf local-alioth
./target/release/eave -d local-alioth --collator --name Alioth --ws-port 9946 --chain ./resources/steam-local.json --parachain-id 77 -- --chain ../polkadot/rococo_local.json


