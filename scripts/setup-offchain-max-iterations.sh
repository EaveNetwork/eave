# example for set eave/cdp-engine/max-iterations to 1000
#
# codec:
#   eave/cdp-engine/max-iterations/ = 0x0c2df85f943312fc853059336627d0b7a08669629ebd99b4debc6e58c1b35c2b,
#   eave/auction-manager/max-iterations/ = 0x0c2df85f943312fc853059336627d0b7a08669629ebd99b4debc6e58c1b35c2b,
# Litter-endian u32:
#   10 = 0x0a000000
#   100 = 0x64000000
#   1000 = 0xe803000000
#   10000 = 0x1027000000

curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "offchain_localStorageSet", "params":["PERSISTENT", "0x0c2df85f943312fc853059336627d0b7a08669629ebd99b4debc6e58c1b35c2b", "0xe803000000"]}' http://localhost:9933
