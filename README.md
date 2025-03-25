# Incredible Squaring Avs

Basic repo demoing a simple AVS middleware with full EigenLayer integration, in Rust.

## Dependencies

- [Foundry](https://github.com/foundry-rs/foundry) - to compile and deploy the contracts
- [Docker](https://www.docker.com/) - for tests
- [jq](https://jqlang.org/download/) - for rewards examples

## Running the example

### Deploy the contracts

First, start anvil in a separate terminal

```sh
anvil
```

Second, update git submodules and copy `.env` file

```sh
git submodule update --init --recursive
cp contracts/.env.example contracts/.env
```

Finally, deploy EigenLayer and the AVS contracts

```sh
make deploy-el-and-avs-contracts
```

### Start the example

To start the whole example, run the following command

```sh
cargo run --bin incredible-squaring-avs start
```

This command launches 5 services:

- Aggregator: receives signed task responses from operators via a JSON-RPC server, aggregates the signatures, and calls the `TaskManager` contract's `respondToTask` function once quorum is reached.
- 2 operators: they wait for new tasks, respond to them and sign with their BLS keys, and then send the signed response to the aggregator.
- 1 challenger: it listens for task creations and responses, verifies the responses are correct and, if wrong, raises a challenge by calling the `raiseAndResolveChallenge` function in the `TaskManager` contract.
- 1 task generator: it periodically creates new tasks by calling the `createNewTask` function of the `TaskManager` contract.

> [!NOTE]
> All services are started with the default parameters.
> To specify custom values, provide a path to a toml config file with the `--config-path` flag like so:
>
> ```sh
> cargo run --bin incredible-squaring-avs start --config-path <PATH>
> ```
>
> We have an example file [incredible_config.toml](./incredible_config.toml) for reference.

### Simulating Slashing

The `operator_1_times_failing` and `operator_2_times_failing` config fields specify the probability percentage for the respective operator to produce an incorrect result.
Each of these failures will result in a slashing once a challenge is raised by the challenger.

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

- To run unit tests(start anvil in a separate terminal)

```sh
make pr
```

- To run integration tests(start anvil in a separate terminal)

```sh
make integration-tests
```

## Architecture

The architecture of the AVS contains:

- [EigenLayer core](https://github.com/Layr-Labs/eigenlayer-contracts/tree/master) contracts
- AVS contracts
  - [ServiceManager](contracts/src/IncredibleSquaringServiceManager.sol) which will eventually contain slashing logic but for M2 is just a placeholder.
  - [TaskManager](contracts/src/IncredibleSquaringTaskManager.sol) which contains [task creation](contracts/src/IncredibleSquaringTaskManager.sol#L83) and [task response](contracts/src/IncredibleSquaringTaskManager.sol#L102) logic. Calls `fulfillSlashingRequest` to the [Slasher] contract using the `raiseAndResolveChallenge` function .
  - The [challenge](contracts/src/IncredibleSquaringTaskManager.sol#L176) logic could be separated into its own contract, but we have decided to include it in the TaskManager for this simple task.
  - Set of [registry contracts](https://github.com/Layr-Labs/eigenlayer-middleware) to manage operators opted in to this avs
- Task Generator
  - This is a separate entity .
- Aggregator
  - aggregates BLS signatures from operators and posts the aggregated response to the task manager
  - For this simple demo, the aggregator is not an operator, and thus does not need to register with EigenLayer or the AVS contract. It's IP address is simply hardcoded into the operators' config.
- Operators
  - Square the number sent to the task manager by the task generator, sign it, and send it to the aggregator

## Default Configuration

- Metrics http endpoint - `http://localhost:9001/metrics`
- Aggregator Rpc endpoint - `127.0.0.1:8080`
- Operator1 - `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266` (anvil's 0 index key)
- Operator2 - `0x0b065a0423f076a340f37e16e1ce22e23d66caf2`

## Related Projects

- [eigensdk-rs](https://github.com/Layr-Labs/eigensdk-rs) - Official EigenLayer Rust SDK
- [rust-bls-bn254](https://github.com/Layr-Labs/bn254-bls-keystore-rs) - EIP 2335 Compatible Keystore using BN254
