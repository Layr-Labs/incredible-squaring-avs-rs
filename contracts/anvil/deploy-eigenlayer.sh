#!/bin/bash

RPC_URL=http://localhost:8545
PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
# cd to the directory of this script so that this can be run from anywhere
parent_path=$(
    cd "$(dirname "${BASH_SOURCE[0]}")"
    pwd -P
)
# At this point we are in tests/anvil
cd "$parent_path"

root_dir=$(realpath $parent_path/../..)

# DEPLOY CONTRACT REGISTRY
cd $root_dir/contracts
forge create ContractsRegistry --rpc-url $RPC_URL --private-key $PRIVATE_KEY
forge script script/DeployEigenlayerCore.s.sol --rpc-url http://localhost:8545 --broadcast --slow





