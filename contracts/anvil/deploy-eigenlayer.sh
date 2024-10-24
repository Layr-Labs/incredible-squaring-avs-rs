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
forge create src/ContractsRegistry.sol:ContractsRegistry --rpc-url $RPC_URL --private-key $PRIVATE_KEY

# cd ./lib/eigenlayer-middleware/lib/eigenlayer-contracts/
# forge script script/deploy/devnet/M2_Deploy_From_Scratch.s.sol --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --broadcast --sig "run(string memory configFile)" -- M2_deploy_from_scratch.anvil.config.json

# cp script/output/devnet/M2_from_scratch_deployment_data.json ../../../../../contracts/script/output/31337/eigenlayer_deployment_output.json

forge script script/DeployEigenlayerCore.s.sol --rpc-url http://localhost:8545 --broadcast  --via-ir

# cd $root_dir/contracts
forge script script/WriteToContractsRegistry.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast

# # DEPLOY TOKENS AND STRATEGIES
# cd $root_dir/contracts

# forge script script/DeployTokensStrategiesCreateQuorums.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast --slow


