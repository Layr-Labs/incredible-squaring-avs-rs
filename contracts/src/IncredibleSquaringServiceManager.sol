// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@eigenlayer/contracts/libraries/BytesLib.sol";
import "./IIncredibleSquaringTaskManager.sol";
import "@eigenlayer-middleware/src/ServiceManagerBase.sol";
import {IAllocationManager,IAllocationManagerTypes} from "@eigenlayer/contracts/interfaces/IAllocationManager.sol";
import {IAVSRegistrar} from "@eigenlayer/contracts/interfaces/IAVSRegistrar.sol";
import {IRewardsCoordinator} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";

/**
 * @title Primary entrypoint for procuring services from IncredibleSquaring.
 * @author Layr Labs, Inc.
 */
contract IncredibleSquaringServiceManager is ServiceManagerBase {
    using BytesLib for bytes;

    IIncredibleSquaringTaskManager public immutable incredibleSquaringTaskManager;

    /// @notice when applied to a function, ensures that the function is only callable by the `registryCoordinator`.
    modifier onlyIncredibleSquaringTaskManager() {
        require(
            msg.sender == address(incredibleSquaringTaskManager),
            "onlyIncredibleSquaringTaskManager: not from credible squaring task manager"
        );
        _;
    }
            
    constructor(
        IAVSDirectory _avsDirectory,
        IRegistryCoordinator _registryCoordinator,
        IStakeRegistry _stakeRegistry,
        address rewards_coordinator,
        IAllocationManager allocationManager,
        IIncredibleSquaringTaskManager _incredibleSquaringTaskManager
    )
        ServiceManagerBase(_avsDirectory, IRewardsCoordinator(rewards_coordinator), _registryCoordinator, _stakeRegistry,allocationManager)
    {
        incredibleSquaringTaskManager = _incredibleSquaringTaskManager;
    }

    function setAvsRegistrar() external {
        IAllocationManager(_allocationManager).setAVSRegistrar(address(this),IAVSRegistrar(address(_registryCoordinator)));
    }

    function createOperatorSet(IAllocationManagerTypes.CreateSetParams[] calldata params) external {
        IAllocationManager(_allocationManager).createOperatorSets(address(this), params);
    }

    // function createOperatorDirectedAVSRewardsSubmission(
    //     address avs,
    //     IRewardsCoordinator.OperatorDirectedRewardsSubmission[] memory operatorDirectedRewardsSubmission
    // ) public {
    //     _rewardsCoordinator.createOperatorDirectedAVSRewardsSubmission(avs, operatorDirectedRewardsSubmission);
    // }


}
