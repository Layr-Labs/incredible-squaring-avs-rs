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

cd ./lib/eigenlayer-middleware.git/lib/eigenlayer-contracts
# deployment overwrites this file, so we save it as backup, because we want that output in our local files, and not in the eigenlayer-contracts submodule files
mv script/output/devnet/M2_from_scratch_deployment_data.json script/output/devnet/M2_from_scratch_deployment_data.json.bak
# M2_Deploy_From_Scratch.s.sol prepends "script/testing/" to the configFile passed as input (M2_deploy_from_scratch.anvil.config.json)
forge script script/deploy/devnet/M2_Deploy_From_Scratch.s.sol --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --broadcast --sig "run(string memory configFile)" -- M2_deploy_from_scratch.anvil.config.json
mv script/output/devnet/M2_from_scratch_deployment_data.json ../../../../script/output/31337/eigenlayer_deployment_output.json
mv script/output/devnet/M2_from_scratch_deployment_data.json.bak script/output/devnet/M2_from_scratch_deployment_data.json

# DEPLOY MOCKAVS
cd $root_dir/contracts
forge script script/DeployMockAvs.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast

# DEPLOY TOKENS AND STRATEGIES
cd $root_dir/contracts
# DO NOT REMOVE THE SLOW DIRECTIVE FROM THIS SCRIPT INVOCATION
# slow ensures that the transaction reciept is successful and recieved before sending the next transaction
# this should prevent the strategies deploying/registering in a flakey manner,
forge script script/DeployTokensStrategiesCreateQuorums.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast --slow

# REGISTER OPERATORS WITH EIGENLAYER
cd $root_dir/contracts
# DO NOT REMOVE THE SLOW DIRECTIVE FROM THIS SCRIPT INVOCATION
# slow ensures that the transaction receipt is successful and recieved before sending the next transaction
# this should prevent the operators registering in a flakey manner, the operators registered will change from run to run without this
forge script script/RegisterOperatorsWithEigenlayer.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast --slow

forge script script/UpdateOperators.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast --slow
