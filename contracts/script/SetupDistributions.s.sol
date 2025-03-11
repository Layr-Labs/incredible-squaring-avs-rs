// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Script} from "forge-std/Script.sol";
import {IncredibleSquaringDeploymentLib} from "./utils/IncredibleSquaringDeploymentLib.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {SetupDistributionsLib} from "./utils/SetupDistributionsLib.sol";
import {IRewardsCoordinator, IRewardsCoordinatorTypes} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import {console2} from "forge-std/console2.sol";
import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";
import {RewardsCoordinator} from "@eigenlayer/contracts/core/RewardsCoordinator.sol";
import {ERC20Mock} from "../test/ERC20Mock.sol";
import "forge-std/Test.sol";

import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {stdJson} from "forge-std/StdJson.sol";

interface Ownable {
    function owner() external view returns (address);
}

contract SetupDistributions is Script, Test {
    using stdJson for string;

    struct PaymentInfo {
        address recipient;
        uint256 numPayments;
        uint256 amountPerPayment;
        uint32 duration;
        uint32 startTimestamp;
        uint32 endTimestamp;
        uint256 indexToProve;
    }

    address private deployer;
    RewardsCoordinator rewardsCoordinator;
    CoreDeploymentLib.DeploymentData coreDeployment;
    IncredibleSquaringDeploymentLib.DeploymentData incredibleSquaringDeployment;
    IncredibleSquaringDeploymentLib.DeploymentConfigData isConfig;
    string internal constant paymentfilepath = "test/mockData/scratch/payments.json";
    string internal constant filePath = "test/mockData/scratch/payment_info.json";
    IRewardsCoordinator.EarnerTreeMerkleLeaf[] public earnerLeaves;
    address[] public earners;
    uint32 startTimestamp;
    uint32 endTimestamp;
    uint256 cumumlativePaymentMultiplier;
    address nonceSender = 0x998abeb3E57409262aE5b751f60747921B33613E;
    uint256 constant NUM_TOKEN_EARNINGS = 1;
    uint32 constant DURATION = 1;
    uint256 constant NUM_EARNERS = 8;
    uint32 constant CALCULATION_INTERVAL_SECONDS = 1 days;
    uint32 numPayments = 8;
    uint32 indexToProve = 0;
    uint32 amountPerPayment = 100;
    address recipient = address(1);
    address operator1 = address(1);
    address operator2 = address(2);

    function setUp() public {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");
        isConfig =
            IncredibleSquaringDeploymentLib.readDeploymentConfigValues("config/incredible-squaring/", block.chainid);
        coreDeployment = CoreDeploymentLib.readDeploymentJson("script/deployments/core/", block.chainid);
        incredibleSquaringDeployment =
            IncredibleSquaringDeploymentLib.readDeploymentJson("script/deployments/incredible-squaring/", block.chainid);
        rewardsCoordinator = RewardsCoordinator(coreDeployment.rewardsCoordinator);
    }

    function run() external {
        vm.startBroadcast(isConfig.rewardsInitiatorKey);

        if (rewardsCoordinator.currRewardsCalculationEndTimestamp() == 0) {
            startTimestamp = uint32(block.timestamp) - (uint32(block.timestamp) % CALCULATION_INTERVAL_SECONDS);
            console2.log("start_time");
            console2.log(startTimestamp);
        } else {
            startTimestamp =
                rewardsCoordinator.currRewardsCalculationEndTimestamp() - DURATION + CALCULATION_INTERVAL_SECONDS;
            console2.log("start_timestamp");
            console2.log(startTimestamp);
        }

        endTimestamp = startTimestamp + 1;

        if (endTimestamp > block.timestamp) {
            revert("End timestamp must be in the future.  Please wait to generate new payments.");
        }

        // sets a multiplier based on block number such that cumulativeEarnings increase accordingly for multiple runs of this script in the same session
        uint256 nonce = rewardsCoordinator.getDistributionRootsLength();
        amountPerPayment = uint32(amountPerPayment * (nonce + 1));

        createAVSRewardsSubmissions(numPayments, amountPerPayment, startTimestamp);
        vm.stopBroadcast();
        vm.startBroadcast(deployer);
        earners = _getEarners(deployer);
        submitPaymentRoot(earners, endTimestamp, numPayments, amountPerPayment);
        vm.stopBroadcast();
    }

    function runOperatorDirected() external {
        vm.startBroadcast(isConfig.rewardsInitiatorKey);
        if (rewardsCoordinator.currRewardsCalculationEndTimestamp() == 0) {
            startTimestamp = uint32(block.timestamp) - (uint32(block.timestamp) % CALCULATION_INTERVAL_SECONDS);
            console2.log("start_time");
            console2.log(startTimestamp);
        } else {
            startTimestamp =
                rewardsCoordinator.currRewardsCalculationEndTimestamp() - DURATION + CALCULATION_INTERVAL_SECONDS;
            console2.log("start_timestamp");
            console2.log(startTimestamp);
        }

        endTimestamp = startTimestamp + 1;

        if (endTimestamp > block.timestamp) {
            console2.log("current_time");
            console2.log(block.timestamp);
            revert("End timestamp must be in the future.  Please wait to generate new payments.");
        }

        // sets a multiplier based on block number such that cumulativeEarnings increase accordingly for multiple runs of this script in the same session
        uint256 nonce = rewardsCoordinator.getDistributionRootsLength();
        amountPerPayment = uint32(amountPerPayment * (nonce + 1));

        createOperatorDirectedAVSRewardsSubmissions(numPayments, amountPerPayment, startTimestamp);
        vm.stopBroadcast();
        vm.startBroadcast(deployer);
        earners = _getEarners(deployer);
        submitPaymentRoot(earners, endTimestamp, numPayments, amountPerPayment);
        vm.stopBroadcast();
    }

    function executeProcessClaim() public {
        uint256 nonce = rewardsCoordinator.getDistributionRootsLength();
        amountPerPayment = uint32(amountPerPayment * nonce);

        vm.startBroadcast(deployer);
        earnerLeaves = _getEarnerLeaves(_getEarners(deployer), amountPerPayment, incredibleSquaringDeployment.strategy);
        processClaim(filePath, indexToProve, recipient, earnerLeaves[indexToProve], amountPerPayment);
        vm.stopBroadcast();
    }

    function createAVSRewardsSubmissions(uint256 numPayments, uint256 amountPerPayment, uint32 startTimestamp) public {
        ERC20Mock(incredibleSquaringDeployment.token).mint(isConfig.rewardsInitiator, amountPerPayment * numPayments);
        ERC20Mock(incredibleSquaringDeployment.token).increaseAllowance(
            incredibleSquaringDeployment.incredibleSquaringServiceManager, amountPerPayment * numPayments
        );
        uint32 duration = rewardsCoordinator.MAX_REWARDS_DURATION();
        SetupDistributionsLib.createAVSRewardsSubmissions(
            incredibleSquaringDeployment.incredibleSquaringServiceManager,
            incredibleSquaringDeployment.strategy,
            numPayments,
            amountPerPayment,
            duration,
            startTimestamp
        );
    }

    function createOperatorDirectedAVSRewardsSubmissions(
        uint256 numPayments,
        uint256 amountPerPayment,
        uint32 startTimestamp
    ) public {
        ERC20Mock(incredibleSquaringDeployment.token).mint(isConfig.rewardsInitiator, amountPerPayment * numPayments);
        ERC20Mock(incredibleSquaringDeployment.token).increaseAllowance(
            incredibleSquaringDeployment.incredibleSquaringServiceManager, amountPerPayment * numPayments
        );
        uint32 duration = 0;
        address[] memory operators = new address[](2);
        operators[0] = operator1;
        operators[1] = operator2;

        uint256 numOperators = operators.length;

        SetupDistributionsLib.createOperatorDirectedAVSRewardsSubmissions(
            incredibleSquaringDeployment.incredibleSquaringServiceManager,
            operators,
            numOperators,
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
        IRewardsCoordinator.EarnerTreeMerkleLeaf memory earnerLeaf,
        uint32 amountPerPayment
    ) public {
        SetupDistributionsLib.processClaim(
            IRewardsCoordinator(coreDeployment.rewardsCoordinator),
            filePath,
            indexToProve,
            recipient,
            earnerLeaf,
            NUM_TOKEN_EARNINGS,
            incredibleSquaringDeployment.strategy,
            amountPerPayment
        );
    }

    function submitPaymentRoot(
        address[] memory earners,
        uint32 endTimestamp,
        uint32 numPayments,
        uint32 amountPerPayment
    ) public {
        emit log_named_uint("cumumlativePaymentMultiplier", cumumlativePaymentMultiplier);
        bytes32[] memory tokenLeaves = SetupDistributionsLib.createTokenLeaves(
            IRewardsCoordinator(coreDeployment.rewardsCoordinator),
            NUM_TOKEN_EARNINGS,
            amountPerPayment,
            incredibleSquaringDeployment.strategy
        );
        IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory earnerLeaves =
            SetupDistributionsLib.createEarnerLeaves(earners, tokenLeaves);
        emit log_named_uint("Earner Leaves Length", earnerLeaves.length);
        emit log_named_uint("numPayments", numPayments);

        SetupDistributionsLib.submitRoot(
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

    function _getEarnerLeaves(address[] memory earners, uint32 amountPerPayment, address strategy)
        internal
        view
        returns (IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory)
    {
        bytes32[] memory tokenLeaves = SetupDistributionsLib.createTokenLeaves(
            IRewardsCoordinator(coreDeployment.rewardsCoordinator), NUM_TOKEN_EARNINGS, amountPerPayment, strategy
        );

        IRewardsCoordinator.EarnerTreeMerkleLeaf[] memory earnerLeaves =
            SetupDistributionsLib.createEarnerLeaves(earners, tokenLeaves);

        return earnerLeaves;
    }

    function _getEarners(address deployer) internal pure returns (address[] memory) {
        address[] memory earners = new address[](NUM_EARNERS);
        for (uint256 i = 0; i < earners.length; i++) {
            earners[i] = deployer;
        }
        return earners;
    }
}
