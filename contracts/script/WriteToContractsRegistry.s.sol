// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import {ContractsRegistry} from "../src/ContractsRegistry.sol";
import "forge-std/console.sol";
import "forge-std/console2.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {IncredibleSquaringDeploymentLib} from "./utils/IncredibleSquaringDeploymentLib.sol";
import {Vm} from "forge-std/Vm.sol";

// forge script script/DeployMockAvs.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --etherscan-api-key $ETHERSCAN_API_KEY --broadcast --verify
contract WriteToContractsRegistry {
    Vm internal constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));
    CoreDeploymentLib.DeploymentData internal coreConfigData;
    IncredibleSquaringDeploymentLib.DeploymentData internal avsConfigData;
    address public CONTRACT_REGISTRY;
    ContractsRegistry contractsRegistry;

    function run() public virtual {
        CONTRACT_REGISTRY = vm.envAddress("CONTRACTS_REGISTRY_ADDR");
        contractsRegistry = ContractsRegistry(CONTRACT_REGISTRY);
        coreConfigData = CoreDeploymentLib.readDeploymentJson("script/deployments/core/", "31337.json");
        avsConfigData =
            IncredibleSquaringDeploymentLib.readDeploymentJson("script/deployments/incredible-squaring/", block.chainid);
        vm.startBroadcast();
        if (block.chainid == 31337 || block.chainid == 1337) {
            _writeCoreContractsToRegistry(coreConfigData);
            _writeIncredibleSquaringContractsToRegistry(avsConfigData);
        }

        vm.stopBroadcast();
    }

    function _writeCoreContractsToRegistry(CoreDeploymentLib.DeploymentData memory deploymentdata) internal {
        contractsRegistry.registerContract("delegationManager", address(deploymentdata.delegationManager));
        contractsRegistry.registerContract("strategyManager", address(deploymentdata.strategyManager));
        contractsRegistry.registerContract("avsDirectory", address(deploymentdata.avsDirectory));
        contractsRegistry.registerContract("rewardsCoordinator", address(deploymentdata.rewardsCoordinator));
    }

    function _writeIncredibleSquaringContractsToRegistry(
        IncredibleSquaringDeploymentLib.DeploymentData memory deploymentdata
    ) internal {
        contractsRegistry.registerContract(
            "incredible_squaring_task_manager", address(deploymentdata.incredibleSquaringTaskManager)
        );
        contractsRegistry.registerContract("erc20MockStrategy", address(deploymentdata.strategy));
        contractsRegistry.registerContract(
            "incredible_squaring_registry_coordinator", address(deploymentdata.registryCoordinator)
        );
        contractsRegistry.registerContract(
            "incredible_squaring_operator_state_retriever", address(deploymentdata.operatorStateRetriever)
        );
    }
}
