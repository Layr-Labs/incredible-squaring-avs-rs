// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.12;

import {Script} from "forge-std/Script.sol";
import {CoreDeploymentLib} from "../utils/CoreDeploymentLib.sol";
import {IncredibleSquaringDeploymentLib} from "../utils/IncredibleSquaringDeploymentLib.sol";
import "../../src/MockERC20.sol";
import {FundOperator} from "../utils/FundOperator.sol";
import {IStrategyManager, IStrategy} from "@eigenlayer/contracts/interfaces/IStrategyManager.sol";
import {StrategyFactory} from "@eigenlayer/contracts/strategies/StrategyFactory.sol";
import {UpgradeableProxyLib} from "../utils/UpgradeableProxyLib.sol";
import {TestnetISDeploymentLib} from "../utils/holesky/TestnetISDeploymentLib.sol";
import {TestnetCoreLib} from "../utils/holesky/TestnetCoreLib.sol";

contract HoleskyIncredibleSquaringDeployer is Script {
    // DEPLOYMENT CONSTANTS
    uint256 public constant QUORUM_THRESHOLD_PERCENTAGE = 100;
    uint32 public constant TASK_RESPONSE_WINDOW_BLOCK = 30;
    uint32 public constant TASK_DURATION_BLOCKS = 0;
    address public AGGREGATOR_ADDR;
    address public TASK_GENERATOR_ADDR;
    address public CONTRACTS_REGISTRY_ADDR;
    address public OPERATOR_ADDR;
    address public OPERATOR_2_ADDR;
    // ContractsRegistry contractsRegistry;

    // StrategyBaseTVLLimits public erc20MockStrategy;

    // address public rewardscoordinator;

    // ProxyAdmin public incredibleSquaringProxyAdmin;
    // PauserRegistry public incredibleSquaringPauserReg;

    // regcoord.RegistryCoordinator public registryCoordinator;
    // regcoord.IRegistryCoordinator public registryCoordinatorImplementation;

    // IBLSApkRegistry public blsApkRegistry;
    // IBLSApkRegistry public blsApkRegistryImplementation;

    // IIndexRegistry public indexRegistry;
    // IIndexRegistry public indexRegistryImplementation;

    // IStakeRegistry public stakeRegistry;
    // IStakeRegistry public stakeRegistryImplementation;

    // OperatorStateRetriever public operatorStateRetriever;

    // IncredibleSquaringServiceManager public incredibleSquaringServiceManager;
    // IServiceManager public incredibleSquaringServiceManagerImplementation;

    // IncredibleSquaringTaskManager public incredibleSquaringTaskManager;
    // IIncredibleSquaringTaskManager public incredibleSquaringTaskManagerImplementation;
    CoreDeploymentLib.DeploymentData internal configData;
    IStrategy incredibleSquaringStrategy;
    address private deployer;
    MockERC20 public erc20Mock;
    TestnetISDeploymentLib.DeploymentData incrediblSquaringDeployment;

    using UpgradeableProxyLib for address;

    address proxyAdmin;

    function setUp() public virtual {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");
    }

    function run() external {
        // Eigenlayer contracts
        vm.startBroadcast(vm.rememberKey(vm.envUint("HOLESKY_DEPLOYER_KEY"))); // 0x08ebac0c47e1afbc8355816ed68ecd97796797c3
        TestnetISDeploymentLib.IncredibleSquaringSetupConfig memory isConfig =
            TestnetISDeploymentLib.readIncredibleSquaringConfigJson("holesky_config");

        configData = CoreDeploymentLib.DeploymentData({
            delegationManager: TestnetCoreLib.DELEGATION_MANAGER_ADDRESS,
            avsDirectory: TestnetCoreLib.AVS_DIRECTORY_ADDRESS,
            strategyManager: TestnetCoreLib.STRATEGY_MANAGER_ADDRESS,
            eigenPodManager: TestnetCoreLib.EIGEN_POD_MANAGER_ADDRESS,
            rewardsCoordinator: TestnetCoreLib.REWARDS_COORDINATOR_ADDRESS,
            eigenPodBeacon: TestnetCoreLib.EIGEN_POD_BEACON_ADDRESS,
            pauserRegistry: TestnetCoreLib.PAUSER_REGISTRY_ADDRESS,
            strategyFactory: TestnetCoreLib.STRATEGY_FACTORY_ADDRESS,
            strategyBeacon: TestnetCoreLib.STRATEGY_BEACON_ADDRESS
        });
        erc20Mock = new MockERC20();
        FundOperator.fund_operator(address(erc20Mock), isConfig.operator_addr, 10e18);
        FundOperator.fund_operator(address(erc20Mock), isConfig.operator_2_addr, 10e18);
        (bool s,) = isConfig.operator_2_addr.call{value: 0.02 ether}("");
        require(s);
        (bool ss,) = isConfig.operator_addr.call{value: 0.02 ether}("");
        require(ss);
        incredibleSquaringStrategy = IStrategy(StrategyFactory(configData.strategyFactory).deployNewStrategy(erc20Mock));

        proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();
        incrediblSquaringDeployment = TestnetISDeploymentLib.deployContracts(
            proxyAdmin, configData, address(incredibleSquaringStrategy), isConfig, msg.sender
        );
        // FundOperator.fund_operator(
        //     address(erc20Mock), incrediblSquaringDeployment.incredibleSquaringServiceManager, 1e18
        // );

        TestnetISDeploymentLib.writeDeploymentJson(incrediblSquaringDeployment);

        vm.stopBroadcast();
    }
}
