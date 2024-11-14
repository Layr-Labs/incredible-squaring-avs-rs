// // SPDX-License-Identifier: UNLICENSED
// pragma solidity ^0.8.12;


// import {ContractsRegistry} from "../../src/ContractsRegistry.sol";
// import "forge-std/console.sol";
// import "forge-std/console2.sol";
// import {CoreDeploymentLib} from "./CoreDeploymentLib.sol";
// import {IncredibleSquaringDeploymentLib} from "./IncredibleSquaringDeploymentLib.sol";
// import {Vm} from "forge-std/Vm.sol";
// import {WriteToContractsRegistryLib} from "./WriteToContractsRegistryLib.sol";


// library WriteToContractsRegistryLib {

//   Vm internal constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));
//    CoreDeploymentLib.DeploymentData internal coreConfigData;
//     IncredibleSquaringDeploymentLib.DeploymentData internal avsConfigData;
    
//   struct ContractsRegistryConfig{

//     address contractsRegistry;

//   }
//     function writeContractsToRegistry() internal {
//         address CONTRACT_REGISTRY = vm.envAddress("CONTRACTS_REGISTRY_ADDR");
//         ContractsRegistryConfig memory registryConfig = ContractsRegistryConfig({contractsRegistry:CONTRACT_REGISTRY});
//            coreConfigData = CoreDeploymentLib.readDeploymentJson("script/deployments/core/", "31337.json");
//         avsConfigData =
//             IncredibleSquaringDeploymentLib.readDeploymentJson("script/deployments/incredible-squaring/", block.chainid);
//     }


// }