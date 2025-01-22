#!/bin/bash

# Source the .env file
ENV_FILE_PATH="contracts/.env"

if [ -f "$ENV_FILE_PATH" ]; then
    source "$ENV_FILE_PATH"
fi

# cd to the directory of this script so that this can be run from anywhere
parent_path=$(
    cd "$(dirname "${BASH_SOURCE[0]}")"
    pwd -P
)
cd "$parent_path"

cd ../
forge script script/mainnet/IncredibleSquaringDeployer.s.sol --private-key $MAINNET_DEPLOYER_KEY --verifier etherscan --verifier-api-key ETHERSCAN_API_KEY 
