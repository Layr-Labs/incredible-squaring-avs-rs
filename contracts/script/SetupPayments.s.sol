// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Script} from "forge-std/Script.sol";
import {IncredibleSquaringDeploymentLib} from "./utils/IncredibleSquaringDeploymentLib.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {SetupPaymentsLib} from "./utils/SetupPaymentsLib.sol";
import {IRewardsCoordinator} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import {console2} from "forge-std/console2.sol";
import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";

interface Ownable {
    function owner() external view returns (address);
}

contract SetupPayments is Script {
    struct PaymentInfo {
        address[] earners;
        bytes32[] earnerTokenRoots;
        address recipient;
        uint256 numPayments;
        uint256 amountPerPayment;
        uint32 duration;
        uint32 startTimestamp;
        uint32 endTimestamp;
        uint256 indexToProve;
    }

    address private deployer;
    CoreDeploymentLib.DeploymentData coreDeployment;
    IncredibleSquaringDeploymentLib.DeploymentData incredibleSquaringDeployment;
    string internal constant filePath = "test/mockData/scratch/payments.json";

    uint256 constant NUM_TOKEN_EARNINGS = 1;
    uint256 constant DURATION = 1 weeks;

    function setUp() public {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");

        coreDeployment = CoreDeploymentLib.readDeploymentJson("script/deployments/core/", block.chainid);
        incredibleSquaringDeployment =
            IncredibleSquaringDeploymentLib.readDeploymentJson("script/deployments/incredible-squaring/", block.chainid);
    }

    function run() external {
        vm.startBroadcast(deployer);
        IRewardsCoordinator(coreDeployment.rewardsCoordinator).setRewardsUpdater(deployer);
        // console2.log("deployer",deployer);
        // console2.log("reward coordinator",coreDeployment.rewardsCoordinator);
        // address t =IRewardsCoordinator(coreDeployment.rewardsCoordinator).rewardsUpdater();
        // console2.log("rewards updater",t);
        // ProxyAdmin admin = UpgradeableProxyLib.getProxyAdmin(address(coreDeployment.rewardsCoordinator));
        // console2.log("proxy admin",address(admin));
        // PaymentInfo memory info = abi.decode(vm.parseJson(vm.readFile(filePath)), (PaymentInfo));

        // createAVSRewardsSubmissions(info.numPayments, info.amountPerPayment, info.duration, info.startTimestamp);
        // submitPaymentRoot(info.earners, info.endTimestamp, uint32(info.numPayments), uint32(info.amountPerPayment));

        // IRewardsCoordinator.EarnerTreeMerkleLeaf memory earnerLeaf = IRewardsCoordinator.EarnerTreeMerkleLeaf({
        //     earner: info.earners[info.indexToProve],
        //     earnerTokenRoot: info.earnerTokenRoots[info.indexToProve]
        // });

        // processClaim(filePath, info.indexToProve, info.recipient, earnerLeaf);

        vm.stopBroadcast();
    }

    function createAVSRewardsSubmissions(
        uint256 numPayments,
        uint256 amountPerPayment,
        uint32 duration,
        uint32 startTimestamp
    ) public {
        SetupPaymentsLib.createAVSRewardsSubmissions(
            IRewardsCoordinator(coreDeployment.rewardsCoordinator),
            incredibleSquaringDeployment.strategy,
            numPayments,
            amountPerPayment,
            duration,
            startTimestamp
        );
    }

    function processClaim(
        string memory filePath,
        uint256 indexToProve,
        address recipient,
        IRewardsCoordinator.EarnerTreeMerkleLeaf memory earnerLeaf
    ) public {
        SetupPaymentsLib.processClaim(
            IRewardsCoordinator(coreDeployment.rewardsCoordinator),
            filePath,
            indexToProve,
            recipient,
            earnerLeaf,
            NUM_TOKEN_EARNINGS,
            incredibleSquaringDeployment.strategy
        );
    }

    function submitPaymentRoot(
        address[] memory earners,
        uint32 endTimestamp,
        uint32 numPayments,
        uint32 amountPerPayment
    ) public {
        bytes32[] memory tokenLeaves = SetupPaymentsLib.createTokenLeaves(
            IRewardsCoordinator(coreDeployment.rewardsCoordinator),
            NUM_TOKEN_EARNINGS,
            amountPerPayment,
            incredibleSquaringDeployment.strategy
        );
        IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory earnerLeaves =
            SetupPaymentsLib.createEarnerLeaves(earners, tokenLeaves);

        SetupPaymentsLib.submitRoot(
            IRewardsCoordinator(coreDeployment.rewardsCoordinator),
            tokenLeaves,
            earnerLeaves,
            incredibleSquaringDeployment.strategy,
            endTimestamp,
            numPayments,
            NUM_TOKEN_EARNINGS,
            filePath
        );
    }
}
