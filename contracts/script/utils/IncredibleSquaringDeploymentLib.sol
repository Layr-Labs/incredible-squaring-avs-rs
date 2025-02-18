// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {IAVSDirectory} from "@eigenlayer/contracts/interfaces/IAVSDirectory.sol";
import {ISocketRegistry} from "@eigenlayer-middleware/src/interfaces/ISocketRegistry.sol";
import {SocketRegistry} from "@eigenlayer-middleware/src/SocketRegistry.sol";
import {
    IncredibleSquaringServiceManager,
    IServiceManager,
    IIncredibleSquaringTaskManager
} from "../../src/IncredibleSquaringServiceManager.sol";
import {IncredibleSquaringTaskManager} from "../../src/IncredibleSquaringTaskManager.sol";
import {IDelegationManager} from "@eigenlayer/contracts/interfaces/IDelegationManager.sol";
import {Quorum} from "@eigenlayer-middleware/src/interfaces/IECDSAStakeRegistryEventsAndErrors.sol";
import {UpgradeableProxyLib} from "./UpgradeableProxyLib.sol";
import {CoreDeploymentLib} from "./CoreDeploymentLib.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {BLSApkRegistry} from "@eigenlayer-middleware/src/BLSApkRegistry.sol";
import {IndexRegistry} from "@eigenlayer-middleware/src/IndexRegistry.sol";
import {StakeRegistry} from "@eigenlayer-middleware/src/StakeRegistry.sol";
import {IRegistryCoordinator} from "@eigenlayer-middleware/src/interfaces/IRegistryCoordinator.sol";
import {IStrategy} from "@eigenlayer/contracts/interfaces/IStrategyManager.sol";
import {CoreDeploymentLib} from "./CoreDeploymentLib.sol";

import {
    RegistryCoordinator,
    IBLSApkRegistry,
    IIndexRegistry,
    IStakeRegistry
} from "@eigenlayer-middleware/src/RegistryCoordinator.sol";
import {PauserRegistry, IPauserRegistry} from "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import {OperatorStateRetriever} from "@eigenlayer-middleware/src/OperatorStateRetriever.sol";

library IncredibleSquaringDeploymentLib {
    using stdJson for *;
    using Strings for *;
    using UpgradeableProxyLib for address;

    Vm internal constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    struct DeploymentData {
        address incredibleSquaringServiceManager;
        address incredibleSquaringTaskManager;
        address registryCoordinator;
        address operatorStateRetriever;
        address blsapkRegistry;
        address indexRegistry;
        address stakeRegistry;
        address socketRegistry;
        address strategy;
        address token;
    }

    struct IncredibleSquaringSetupConfig {
        uint256 numQuorums;
        uint256[] operatorParams;
        address operator_addr;
        address operator_2_addr;
        address contracts_registry_addr;
        address task_generator_addr;
        address aggregator_addr;
    }

    function deployContracts(
        address proxyAdmin,
        CoreDeploymentLib.DeploymentData memory core,
        address strategy,
        IncredibleSquaringSetupConfig memory isConfig,
        address admin
    ) internal returns (DeploymentData memory) {
        /// read EL deployment address
        CoreDeploymentLib.DeploymentData memory coredata =
            readCoreDeploymentJson("script/deployments/core/", block.chainid);
        address avsdirectory = coredata.avsDirectory;

        DeploymentData memory result;

        // First, deploy upgradeable proxy contracts that will point to the implementations.
        result.incredibleSquaringServiceManager = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.stakeRegistry = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.incredibleSquaringTaskManager = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.registryCoordinator = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.blsapkRegistry = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.indexRegistry = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.strategy = strategy;
        result.socketRegistry = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.operatorStateRetriever = address(new OperatorStateRetriever());
        // Deploy the implementation contracts, using the proxy contracts as inputs
        address stakeRegistryImpl = address(
            new StakeRegistry(
                IRegistryCoordinator(result.registryCoordinator), IDelegationManager(core.delegationManager)
            )
        );
        UpgradeableProxyLib.upgrade(result.stakeRegistry, stakeRegistryImpl);

        address blsApkRegistryImpl = address(new BLSApkRegistry(IRegistryCoordinator(result.registryCoordinator)));
        UpgradeableProxyLib.upgrade(result.blsapkRegistry, blsApkRegistryImpl);

        address indexRegistryimpl = address(new IndexRegistry(IRegistryCoordinator(result.registryCoordinator)));
        UpgradeableProxyLib.upgrade(result.indexRegistry, indexRegistryimpl);

        address[] memory pausers = new address[](2);
        pausers[0] = admin;
        pausers[1] = admin;
        PauserRegistry pausercontract = new PauserRegistry(pausers, admin);

        IStrategy[1] memory deployedStrategyArray = [IStrategy(strategy)];
        uint256 numStrategies = deployedStrategyArray.length;

        uint256 numQuorums = isConfig.numQuorums;
        IRegistryCoordinator.OperatorSetParam[] memory quorumsOperatorSetParams =
            new IRegistryCoordinator.OperatorSetParam[](numQuorums);
        uint256[] memory operator_params = isConfig.operatorParams;

        for (uint256 i = 0; i < numQuorums; i++) {
            quorumsOperatorSetParams[i] = IRegistryCoordinator.OperatorSetParam({
                maxOperatorCount: uint32(operator_params[i]),
                kickBIPsOfOperatorStake: uint16(operator_params[i + 1]),
                kickBIPsOfTotalStake: uint16(operator_params[i + 2])
            });
        }
        // set to 0 for every quorum
        uint96[] memory quorumsMinimumStake = new uint96[](numQuorums);
        IStakeRegistry.StrategyParams[][] memory quorumsStrategyParams =
            new IStakeRegistry.StrategyParams[][](numQuorums);
        for (uint256 i = 0; i < numQuorums; i++) {
            quorumsStrategyParams[i] = new IStakeRegistry.StrategyParams[](numStrategies);
            for (uint256 j = 0; j < numStrategies; j++) {
                quorumsStrategyParams[i][j] = IStakeRegistry.StrategyParams({
                    strategy: deployedStrategyArray[j],
                    // setting this to 1 ether since the divisor is also 1 ether
                    // therefore this allows an operator to register with even just 1 token
                    // see https://github.com/Layr-Labs/eigenlayer-middleware/blob/m2-mainnet/src/StakeRegistry.sol#L484
                    //    weight += uint96(sharesAmount * strategyAndMultiplier.multiplier / WEIGHTING_DIVISOR);
                    multiplier: 1 ether
                });
            }
        }

        bytes memory upgradeCall = abi.encodeCall(
            RegistryCoordinator.initialize,
            (
                admin,
                admin,
                admin,
                pausercontract,
                0,
                quorumsOperatorSetParams,
                quorumsMinimumStake,
                quorumsStrategyParams
            )
        );

        address registryCoordinatorImpl = address(
            new RegistryCoordinator(
                IServiceManager(result.incredibleSquaringServiceManager),
                IStakeRegistry(result.stakeRegistry),
                IBLSApkRegistry(result.blsapkRegistry),
                IIndexRegistry(result.indexRegistry),
                ISocketRegistry(result.socketRegistry)
            )
        );
        address socketRegistryImpl = address(new SocketRegistry(IRegistryCoordinator(result.registryCoordinator)));
        UpgradeableProxyLib.upgrade(result.socketRegistry, socketRegistryImpl);
        UpgradeableProxyLib.upgradeAndCall(result.registryCoordinator, registryCoordinatorImpl, upgradeCall);
        IncredibleSquaringServiceManager incredibleSquaringServiceManagerImpl = new IncredibleSquaringServiceManager(
            (IAVSDirectory(avsdirectory)),
            IRegistryCoordinator(result.registryCoordinator),
            IStakeRegistry(result.stakeRegistry),
            core.rewardsCoordinator,
            IIncredibleSquaringTaskManager(result.incredibleSquaringTaskManager)
        );
        UpgradeableProxyLib.upgrade(
            result.incredibleSquaringServiceManager, address(incredibleSquaringServiceManagerImpl)
        );
        IncredibleSquaringServiceManager(result.incredibleSquaringServiceManager).initialize(admin, admin);
        bytes memory taskmanagerupgradecall = abi.encodeCall(
            IncredibleSquaringTaskManager.initialize, (IPauserRegistry(address(pausercontract)), admin, admin)
        );
        UpgradeableProxyLib.upgradeAndCall(
            result.incredibleSquaringTaskManager,
            address(new IncredibleSquaringTaskManager(IRegistryCoordinator(result.registryCoordinator), 30)),
            (taskmanagerupgradecall)
        );

        verify_deployment(result);

        return result;
    }

    function readDeploymentJson(uint256 chainId) internal returns (DeploymentData memory) {
        return readDeploymentJson("script/deployments/incredible-squaring/", chainId);
    }

    function readIncredibleSquaringConfigJson(string memory directoryPath)
        internal
        returns (IncredibleSquaringSetupConfig memory)
    {
        string memory fileName = string.concat(directoryPath, ".json");
        require(vm.exists(fileName), "Deployment file does not exist");
        string memory json = vm.readFile(fileName);

        IncredibleSquaringSetupConfig memory data;
        data.numQuorums = json.readUint(".num_quorums");
        data.operatorParams = json.readUintArray(".operator_params");
        data.aggregator_addr = json.readAddress(".aggregator_addr");
        data.contracts_registry_addr = json.readAddress(".contracts_registry_addr");
        data.operator_addr = json.readAddress(".operator_addr");
        data.task_generator_addr = json.readAddress(".task_generator_addr");
        data.operator_2_addr = json.readAddress(".operator_2_addr");
        return data;
    }

    function readDeploymentJson(string memory directoryPath, uint256 chainId)
        internal
        returns (DeploymentData memory)
    {
        string memory fileName = string.concat(directoryPath, vm.toString(chainId), ".json");

        require(vm.exists(fileName), "Deployment file does not exist");

        string memory json = vm.readFile(fileName);

        DeploymentData memory data;
        data.incredibleSquaringServiceManager = json.readAddress(".addresses.IncredibleSquaringServiceManager");
        data.incredibleSquaringTaskManager = json.readAddress(".addresses.IncredibleSquaringTaskManager");
        data.registryCoordinator = json.readAddress(".addresses.registryCoordinator");
        data.operatorStateRetriever = json.readAddress(".addresses.operatorStateRetriever");
        data.stakeRegistry = json.readAddress(".addresses.stakeRegistry");
        data.strategy = json.readAddress(".addresses.strategy");
        data.token = json.readAddress(".addresses.token");

        return data;
    }

    /// write to default output path
    function writeDeploymentJson(DeploymentData memory data) internal {
        writeDeploymentJson("script/deployments/incredible-squaring/", block.chainid, data);
    }

    function writeDeploymentJson(string memory outputPath, uint256 chainId, DeploymentData memory data) internal {
        address proxyAdmin = address(UpgradeableProxyLib.getProxyAdmin(data.incredibleSquaringServiceManager));

        string memory deploymentData = _generateDeploymentJson(data, proxyAdmin);

        string memory fileName = string.concat(outputPath, vm.toString(chainId), ".json");
        if (!vm.exists(outputPath)) {
            vm.createDir(outputPath, true);
        }

        vm.writeFile(fileName, deploymentData);
        console2.log("Deployment artifacts written to:", fileName);
    }

    function _generateDeploymentJson(DeploymentData memory data, address proxyAdmin)
        private
        view
        returns (string memory)
    {
        return string.concat(
            '{"lastUpdate":{"timestamp":"',
            vm.toString(block.timestamp),
            '","block_number":"',
            vm.toString(block.number),
            '"},"addresses":',
            _generateContractsJson(data, proxyAdmin),
            "}"
        );
    }

    function _generateContractsJson(DeploymentData memory data, address proxyAdmin)
        private
        view
        returns (string memory)
    {
        return string.concat(
            '{"proxyAdmin":"',
            proxyAdmin.toHexString(),
            '","IncredibleSquaringServiceManager":"',
            data.incredibleSquaringServiceManager.toHexString(),
            '","incredibleSquaringServiceManagerImpl":"',
            data.incredibleSquaringServiceManager.getImplementation().toHexString(),
            '","IncredibleSquaringTaskManager":"',
            data.incredibleSquaringTaskManager.toHexString(),
            '","registryCoordinator":"',
            data.registryCoordinator.toHexString(),
            '","blsapkRegistry":"',
            data.blsapkRegistry.toHexString(),
            '","indexRegistry":"',
            data.indexRegistry.toHexString(),
            '","stakeRegistry":"',
            data.stakeRegistry.toHexString(),
            '","operatorStateRetriever":"',
            data.operatorStateRetriever.toHexString(),
            '","strategy":"',
            data.strategy.toHexString(),
            '","token":"',
            data.token.toHexString(),
            '"}'
        );
    }

    function readCoreDeploymentJson(string memory directoryPath, uint256 chainId)
        internal
        returns (CoreDeploymentLib.DeploymentData memory)
    {
        return readCoreDeploymentJson(directoryPath, string.concat(vm.toString(chainId), ".json"));
    }

    function readCoreDeploymentJson(string memory path, string memory fileName)
        internal
        returns (CoreDeploymentLib.DeploymentData memory)
    {
        string memory pathToFile = string.concat(path, fileName);

        require(vm.exists(pathToFile), "Deployment file does not exist");

        string memory json = vm.readFile(pathToFile);

        CoreDeploymentLib.DeploymentData memory data;
        data.strategyFactory = json.readAddress(".addresses.strategyFactory");
        data.strategyManager = json.readAddress(".addresses.strategyManager");
        data.eigenPodManager = json.readAddress(".addresses.eigenPodManager");
        data.delegationManager = json.readAddress(".addresses.delegation");
        data.avsDirectory = json.readAddress(".addresses.avsDirectory");

        return data;
    }

    function verify_deployment(DeploymentData memory result) internal view {
        IBLSApkRegistry blsapkregistry = IRegistryCoordinator(result.registryCoordinator).blsApkRegistry();
        require(address(blsapkregistry) != address(0));
        IStakeRegistry stakeregistry = IRegistryCoordinator(result.registryCoordinator).stakeRegistry();
        require(address(stakeregistry) != address(0));
        IDelegationManager delegationmanager = IStakeRegistry(address(stakeregistry)).delegation();
        require(address(delegationmanager) != address(0));
    }
}
