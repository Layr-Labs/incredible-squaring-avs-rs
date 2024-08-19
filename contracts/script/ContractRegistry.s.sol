// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;
import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "forge-std/Script.sol";
import {EigenlayerContracts, EigenlayerContractsParser} from "./parsers/EigenlayerContractsParser.sol";
import "forge-std/console.sol";
import {ContractsRegistry} from "../src/ContractsRegistry.sol";
import {MockAvsServiceManager} from "../src/MockAvsServiceManager.sol";
import "eigenlayer-contracts/src/test/mocks/EmptyContract.sol";

import {RegistryCoordinator} from "@eigenlayer-middleware/src/RegistryCoordinator.sol";
import {OperatorStateRetriever} from "@eigenlayer-middleware/src/OperatorStateRetriever.sol";
// forge script script/DeployMockAvs.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --etherscan-api-key $ETHERSCAN_API_KEY --broadcast --verify
contract UpdateContractRegistry is Script{

struct MockAvsContracts {
    MockAvsServiceManager mockAvsServiceManager;
    RegistryCoordinator registryCoordinator;
    OperatorStateRetriever operatorStateRetriever;
}
  struct MockAvsOpsAddresses {
        address communityMultisig;
        address pauser;
        address churner;
        address ejector;
    }

EmptyContract emptycontract;
ProxyAdmin mockavsproxyadmin;
MockAvsServiceManager mockavsservicemanager;
    function run() public virtual {
        // The ContractsRegistry contract should always be deployed at this address on anvil
        // it's the address of the contract created at nonce 0 by the first anvil account (0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266)
        ContractsRegistry contractsRegistry = ContractsRegistry(0x5FbDB2315678afecb367f032d93F642f64180aa3);
        EigenlayerContracts memory eigenlayerContracts = EigenlayerContractsParser._loadEigenlayerDeployedContracts();
        MockAvsOpsAddresses memory addressConfig = _loadAvsOpsAddresses("ops_addresses");

        vm.startBroadcast();

        // Deploy proxy admin for ability to upgrade proxy contracts
        // Note: can't deploy ProxyAdmin in setUp function, b/c its owner is not set correctly if so.
        //       not sure why...
        emptycontract = new EmptyContract();
        mockavsproxyadmin = new ProxyAdmin();
        mockavsservicemanager = MockAvsServiceManager(
            address(new TransparentUpgradeableProxy(address(emptycontract), address(mockavsproxyadmin), ""))
        );
        MockAvsContracts memory mockAvsContracts = _deploymockAvsRegistryContracts(
            eigenlayerContracts, addressConfig, mockAvsServiceManager, mockAvsServiceManagerImplementation
        );
        mockAvsServiceManagerImplementation = new MockAvsServiceManager(
            registryCoordinator, eigenlayerContracts.avsDirectory, eigenlayerContracts.rewardsCoordinator
        );

        mockAvsProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(mockAvsServiceManager))),
            address(mockAvsServiceManagerImplementation),
            abi.encodeWithSelector(mockAvsServiceManager.initialize.selector, addressConfig.communityMultisig)
        );
        require(Ownable(address(mockAvsServiceManager)).owner() != address(0), "Owner uninitialized");

        if (block.chainid == 31337 || block.chainid == 1337) {
            contractsRegistry.registerContract("ProxyAdmin",address(mockAvsProxyAdmin));
            contractsRegistry.registerContract("AvsDirectory",address(eigenlayerContracts.avsDirectory));
            contractsRegistry.registerContract("eigenlayerPauserReg", address(eigenlayerContracts.eigenlayerPauserReg));
            _writeContractsToRegistry(contractsRegistry, eigenlayerContracts, mockAvsContracts);
        }
        vm.stopBroadcast();
    }

     function _writeContractsToRegistry(
        ContractsRegistry contractsRegistry,
        EigenlayerContracts memory eigenlayerContracts,
        MockAvsContracts memory mockAvsContracts
    ) internal {
        contractsRegistry.registerContract("mockAvsServiceManager", address(mockAvsContracts.mockAvsServiceManager));
        contractsRegistry.registerContract("mockAvsRegistryCoordinator", address(mockAvsContracts.registryCoordinator));
        contractsRegistry.registerContract(
            "mockAvsOperatorStateRetriever", address(mockAvsContracts.operatorStateRetriever)
        );
        contractsRegistry.registerContract("delegationManager", address(eigenlayerContracts.delegationManager));
        contractsRegistry.registerContract("strategyManager", address(eigenlayerContracts.strategyManager));
    }

     function _loadAvsOpsAddresses(string memory opsAddressesFileName)
        internal
        view
        returns (MockAvsOpsAddresses memory)
    {
        string memory opsAddresses = readInput(opsAddressesFileName);
        MockAvsOpsAddresses memory addressConfig;
        addressConfig.communityMultisig = stdJson.readAddress(opsAddresses, ".communityMultisig");
        addressConfig.pauser = stdJson.readAddress(opsAddresses, ".pauser");
        addressConfig.churner = stdJson.readAddress(opsAddresses, ".churner");
        addressConfig.ejector = stdJson.readAddress(opsAddresses, ".ejector");
        return addressConfig;
    }
}
