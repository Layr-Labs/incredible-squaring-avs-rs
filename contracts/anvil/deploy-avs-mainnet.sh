#!/bin/bash

RPC_URL=https://ethereum-holesky-rpc.publicnode.com
PRIVATE_KEY=0x5e47a5bb433969e0305547a1a8119fd2d00c1429ea95977aadd752a0e96599bc

# cd to the directory of this script so that this can be run from anywhere
parent_path=$(
    cd "$(dirname "${BASH_SOURCE[0]}")"
    pwd -P
)
cd "$parent_path"

cd ../
forge script script/mainnet/IncredibleSquaringDeployer.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast
