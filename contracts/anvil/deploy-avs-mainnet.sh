#!/bin/bash

RPC_URL=https://eth.llamarpc.com
PRIVATE_KEY=

# cd to the directory of this script so that this can be run from anywhere
parent_path=$(
    cd "$(dirname "${BASH_SOURCE[0]}")"
    pwd -P
)
cd "$parent_path"

cd ../
forge script script/mainnet/IncredibleSquaringDeployer.s.sol --private-key $PRIVATE_KEY
