# Incredible Squaring Avs

This example is a basic proposal of AVS, where the input and output type are `U256` values, representing the number to be squared and the number squared. In this sense, the task for the operators to complete is squaring the received number, and returning the result of the operation as the response value submitted to the Task Manager on-chain contract.

## Dependencies

- [Foundry](https://github.com/foundry-rs/foundry) - to compile and deploy the contracts
- [Docker](https://www.docker.com/) - for tests
- [jq](https://jqlang.org/download/) - for rewards examples

## Structure

### Types

The task type of the solidity contract is the following:

``` solidity
  struct Task {
      uint256 numberToBeSquared;
      uint32 taskCreatedBlock;
      bytes quorumNumbers;
      uint32 quorumThresholdPercentage;
  }
```

The input is an `uint256` representing the number to be squared.

The task response type is:

``` solidity
  struct TaskResponse {
      uint32 referenceTaskIndex;
      uint256 numberSquared;
  }
```

The `numberSquared` field represents the result of the squaring operation with the received number to square.

## Architecture

The architecture of the AVS contains:

- [EigenLayer core](https://github.com/Layr-Labs/eigenlayer-contracts/tree/master) contracts
- AVS contracts
  - [ServiceManager](contracts/src/IncredibleSquaringServiceManager.sol) which will eventually contain slashing logic but for M2 is just a placeholder.
  - [TaskManager](contracts/src/IncredibleSquaringTaskManager.sol) which contains [task creation](contracts/src/IncredibleSquaringTaskManager.sol#L83) and [task response](contracts/src/IncredibleSquaringTaskManager.sol#L102) logic. Calls `fulfillSlashingRequest` to the [Slasher] contract using the `raiseAndResolveChallenge` function .
  - Set of [registry contracts](https://github.com/Layr-Labs/eigenlayer-middleware) to manage operators opted in to this avs
- [Aggregator](./src/bin/aggregator.rs): The aggregator does not have much business logic, as the core functionality is delegated to the `IndexingTaskProcessor`, which is the standard implementation that the SDK provides.If you want to implement a custom task processor, you need to implement the `TaskProcessor` trait.
- [Challenger](./src/bin/challenger.rs): The challenger business logic lies in the task response validation. To validate the response, the challenger first calculates the response with the same function as the operator and then compares it with the received response, raising a challenge if they differ. The example is based on the `IndexingChallengerProcessor` implementation. If you want to implement a custom challenger processor, you need to implement the `ChallengerTaskProcessor` trait.
- [Operator](./src/bin/operator.rs): The operator responds to tasks using the `FunctionResponseCalculator` struct that implements `ResponseCalculator` trait. This struct must define a `compute_response` method to generate the task output. In thix example, we have two operators responding to tasks.
- [Task spammer](./src/bin/task-spammer.rs): The task spammer logic lies in an iterator that generates the inputs for the spammer to dispatch at the SDK level.

## Running the example

This simple session illustrates the basic flow of the AVS. We would have:

Initialize the EigenLayer Middleware and Forge submodules:

```bash
cp contracts/.env.example contracts/.env
git submodule update --init --recursive
```

Start anvil in a separate terminal:

```bash
anvil
```

Deploy contracts, set UAM permissions, and create a quorum in a single command in a separate terminal:

```bash
make deploy-el-and-avs-contracts
```

Start the aggregator:

```bash
make start-aggregator
```

For operators, you can set the failure rate to simulate a failing operator. The default failure rate is 30%.

Start the first operator with the default configuration in a separate terminal:

```bash
make start-operator
```

Start the second operator with a different configuration in a separate terminal:

```bash
make start-operator CONFIG=src/config/squaring-operator-2.toml FAILURE_RATE=60
```

The Operator will first check whether it is already registered on EigenLayer. If not, it will attempt to register automatically. To enable registration, create an [OperatorRegistrationConfig] and include it in the [OperatorConfig] struct.

The operator will produce invalid results often because it use `failing_response_calculator` method, which has a failure rate of 60% and returns `U256::from(42)` as the response.

These failures result in slashing once they're challenged. To see this in action, start the challenger in a separate terminal with:

```bash
make start-challenger
```

To start the cycle, start the task spammer in a separate terminal:

``` bash
make start-task-spammer
```

If you want to spawn more operators, create a new configuration file based on the existing one and use the following command passing the path to the new configuration file and the failure rate:

```bash
make start-operator CONFIG=OPERATOR_CONFIG.toml FAILURE_RATE=100
```

## Creating and Claiming Distributions

The example exposes 3 scripts in the Makefile interface:

- Creating a distribution root, that implies creating an AVS rewards submission and submitting a payment root.
- Creating an operator directed distribution root, similar to previous one but with rewards to operators involved in the claim generation. Note: operators in this case are hardcoded in the script file.
- Claiming the created distribution, giving the rewards to an specific receiver account. Note: The receiver in this case is harcoded in the script file (address 0x01).

This leads to 2 possible workflows, distributing equally across all operators and using custom distribution for each operator.

### Distributing equally across all operators

First, start anvil in a separate terminal and deploy the contracts following the instructions in ["Deploy the contracts"](#deploy-the-contracts).

Then, run the command:

``` sh
make create-avs-distributions-root
```

This creates a claimable root, a root of the merkle tree that stores cumulative earnings per ERC20 reward token for each earner.

To claim against the root, use:

```sh
make claim-distributions
```

If you want to check the balance of the claimer, you can run the following command:

```sh
make claimer-account-token-balance
```

Note that the claimer address is not passed by parameter, because in the script that address is hardcoded.

### Using custom distribution for each operator

First, start anvil in a separate terminal and deploy the contracts following the instructions in ["Deploy the contracts"](#deploy-the-contracts).

Then, run the command:

```sh
make create-operator-directed-distributions-root
```

This creates a claimable root, that differs from the previous one in the fact that also distributes the claim to the directed operators established in the script (currently hardcoded).

The payment leaves are available in `contracts/payments.json`. The payment leaves are the keccak256 hash of each earner leaf. An earner leaf is composed by the earner and the token root of the token leaves, and each token leaf is the result of hashing the token address with the token earnings.

To claim against the root, use:

```sh
make claim-distributions
```

If you want to check the balance of the claimer, you can run the following command:

```sh
make claimer-account-token-balance
```

Note that the claimer address is not passed by parameter, because in the script that address is hardcoded.

## Testing

- To run the integration tests, run the following command:

```sh
make integration-tests
```

## Default Configuration

- Metrics http endpoint - `http://localhost:9001/metrics`
- Aggregator Rpc endpoint - `127.0.0.1:8080`
- Operator1 - `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266` (anvil's 0 index key)
- Operator2 - `0x70997970C51812dc3A010C7d01b50e0d17dc79C8`

## Related Projects

- [eigensdk-rs](https://github.com/Layr-Labs/eigensdk-rs) - Official EigenLayer Rust SDK
- [rust-bls-bn254](https://github.com/Layr-Labs/bn254-bls-keystore-rs) - EIP 2335 Compatible Keystore using BN254
