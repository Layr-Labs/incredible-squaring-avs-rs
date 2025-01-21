// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.12;

import {Script} from "forge-std/Script.sol";
import {CoreDeploymentLib} from "../utils/CoreDeploymentLib.sol";
import {IncredibleSquaringDeploymentLib} from "../utils/IncredibleSquaringDeploymentLib.sol";
import "../../src/MockERC20.sol";
import {console2} from "forge-std/console2.sol";
import {FundOperator} from "../utils/FundOperator.sol";
import {IStrategyManager, IStrategy} from "@eigenlayer/contracts/interfaces/IStrategyManager.sol";
import {Ownable} from "@eigenlayer-middleware/lib/openzeppelin-contracts/contracts/access/Ownable.sol";
import {StrategyFactory} from "@eigenlayer/contracts/strategies/StrategyFactory.sol";
import {UpgradeableProxyLib} from "../utils/UpgradeableProxyLib.sol";
import {MainnetISDeploymentLib} from "../utils/mainnet/MainnetISDeploymentLib.sol";
import {MainnetCoreLib} from "../utils/mainnet/MainnetCoreLib.sol";

contract MainnetIncredibleSquaringDeployer is Script {
    // DEPLOYMENT CONSTANTS
    uint256 public constant QUORUM_THRESHOLD_PERCENTAGE = 100;
    uint32 public constant TASK_RESPONSE_WINDOW_BLOCK = 30;
    uint32 public constant TASK_DURATION_BLOCKS = 0;
    address public AGGREGATOR_ADDR;
    address public TASK_GENERATOR_ADDR;
    address public CONTRACTS_REGISTRY_ADDR;
    address public OPERATOR_ADDR;
    address public OPERATOR_2_ADDR;
 
    CoreDeploymentLib.DeploymentData internal configData;
    IStrategy incredibleSquaringStrategy;
    address private deployer;
    MockERC20 public erc20Mock;
    MainnetISDeploymentLib.DeploymentData incrediblSquaringDeployment;

    using UpgradeableProxyLib for address;

    address proxyAdmin;

    function setUp() public virtual {
        require(block.number > 0);
        vm.createSelectFork(vm.envString("MAINNET_RPC_URL"),block.number);

        deployer = vm.rememberKey(vm.envUint("MAINNET_DEPLOYER_KEY"));
        vm.label(deployer, "Deployer");
    }

    function run() external {
        // Eigenlayer contracts
        vm.startBroadcast(vm.rememberKey(vm.envUint("MAINNET_DEPLOYER_KEY"))); // 0x08ebac0c47e1afbc8355816ed68ecd97796797c3
        MainnetISDeploymentLib.IncredibleSquaringSetupConfig memory isConfig =
            MainnetISDeploymentLib.readIncredibleSquaringConfigJson("mainnet_config");

        configData = CoreDeploymentLib.DeploymentData({
            delegationManager: MainnetCoreLib.DELEGATION_MANAGER_ADDRESS,
            avsDirectory: MainnetCoreLib.AVS_DIRECTORY_ADDRESS,
            strategyManager: MainnetCoreLib.STRATEGY_MANAGER_ADDRESS,
            eigenPodManager: MainnetCoreLib.EIGEN_POD_MANAGER_ADDRESS,
            rewardsCoordinator: MainnetCoreLib.REWARDS_COORDINATOR_ADDRESS,
            eigenPodBeacon: MainnetCoreLib.EIGEN_POD_BEACON_ADDRESS,
            pauserRegistry: MainnetCoreLib.PAUSER_REGISTRY_ADDRESS,
            strategyFactory: MainnetCoreLib.STRATEGY_FACTORY_ADDRESS,
            strategyBeacon: MainnetCoreLib.STRATEGY_BEACON_ADDRESS
        });

        incredibleSquaringStrategy  = IStrategy(MainnetCoreLib.ST_ETH_STRATEGY_ADDRESS);
        proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();

        incrediblSquaringDeployment = MainnetISDeploymentLib.deployContracts(
            proxyAdmin, configData, address(incredibleSquaringStrategy), isConfig, msg.sender
        );

        MainnetISDeploymentLib.writeDeploymentJson(incrediblSquaringDeployment);

        vm.stopBroadcast();
    }
}

