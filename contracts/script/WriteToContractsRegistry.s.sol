// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import {ContractsRegistry} from "../src/ContractsRegistry.sol";
import "forge-std/console.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {Vm} from "forge-std/Vm.sol";

// forge script script/DeployMockAvs.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --etherscan-api-key $ETHERSCAN_API_KEY --broadcast --verify
contract DeployMockAvs {
    Vm internal constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));
    CoreDeploymentLib.DeploymentData internal configData;

    ContractsRegistry contractsRegistry = ContractsRegistry(0x5FbDB2315678afecb367f032d93F642f64180aa3);

    function run() public virtual {
        // The ContractsRegistry contract should always be deployed at this address on anvil
        // it's the address of the contract created at nonce 0 by the first anvil account (0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266)
        configData = CoreDeploymentLib.readDeploymentJson("script/deployments/core/", "31337.json");

        vm.startBroadcast();
        if (block.chainid == 31337 || block.chainid == 1337) {
            _writeContractsToRegistry(configData);
        }

        vm.stopBroadcast();
    }

    function _writeContractsToRegistry(CoreDeploymentLib.DeploymentData memory deploymentdata) internal {
        contractsRegistry.registerContract("delegationManager", address(deploymentdata.delegationManager));
        contractsRegistry.registerContract("strategyManager", address(deploymentdata.strategyManager));
        contractsRegistry.registerContract("avsDirectory", address(deploymentdata.avsDirectory));
    }
}
