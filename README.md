# Incredible Squaring Avs 

Basic repo demoing a simple AVS middleware with full eigenlayer integration, in rust.

## Dependencies

- [Foundry](https://github.com/foundry-rs/foundry)
- [Docker](https://www.docker.com/) 

## Tools

- [jq]
  * To install, follow the instructions [here](https://jqlang.org/download/)

## To run 

- Start anvil in a separate terminal 
```sh
anvil
```

- git submodule and copy env 
```sh
git submodule update --init --recursive
cp contracts/.env.example contracts/.env
```


- Deploy eigenlayer and avs contracts and setup payments
```sh
make deploy-el-and-avs-contracts
```

- Single command AVS start using the following command (default values)(without simulating slashing)
```sh
cargo run --bin incredible-squaring-avs  start
```

- To change the parameters, provide path to a toml config file 
```
cargo run --bin incredible-squaring-avs  start --config-path <PATH>
```
- Simulate slashing
```
cargo run --bin incredible-squaring-avs  start --slash-simulate
```

We have an example file [incredible_config.toml](https://github.com/Layr-Labs/incredible-squaring-avs-rust/tree/master/incredible_config.toml) for reference.

This command launches 5 services(crates) together:

- Operator1 : It listens for new tasks , responds them by signing with their bls key and send the signed response to the aggregator. Stake in strategy: 5000 tokens
- Operator2: Same task as operator 1. Stake in strategy: 7000 tokens
- Aggregator: Sets up an Rpc client to receive signed task responses from operators, aggregates the signatures, if quorums is met (i.e both operators sign the response), it calls the respondToTask function in the TaskManager contract.
- Challenger : It listens for new tasks , checks the operators response, if found wrong, it raises a challenge by calling the `raiseAndResolveChallenge` function in the task manager contract.
- Task Spam : It creates a new task every 10 seconds by calling the `createNewTask` function in the task manager contract.

## Creating and Claiming Distributions

The example exposes 3 scripts in the Makefile interface:
- Creating a distribution root, that implies creating an AVS rewards submission and submitting a payment root.
- Creating an operator directed distribution root, similar to previous one but with rewards to operators involved in the claim generation. Note: operators in this case are hardcoded in the script file.
- Claiming the created distribution, giving the rewards to an specific receiver account. Note: The receiver in this case is harcoded in the script file (address 0x01).

This leads to 2 possible workflows, distributing equally across all operators and using custom distribution for each operator.

### Distributing equally across all operators

First, start anvil in a separate terminal and deploy the contracts. To do that follow the instructions in [To run section](#to-run)

Then, run the command:

``` bash
make create-avs-distributions-root
```

This creates a claimable root, a root of the merkle tree that stores cumulative earnings per ERC20 reward token for each earner.

To claim against the root, use:
``` bash
make claim-distributions
```

If you want to check the balance of the claimer, you can run the following command:
``` bash
make claimer-account-token-balance
```
Note that the claimer address is not passed by parameter, because in the script that address is hardcoded.

### Using custom distribution for each operator

First, start anvil in a separate terminal and deploy the contracts. To do that follow the instructions in [To run section](#to-run)

Then, run the command:

``` bash
make create-operator-directed-distributions-root
```

This creates a claimable root, that differs from the previous one in the fact that also distributes the claim to the directed operators established in the script (currently hardcoded).

The payment leaves are available in `contracts/payments.json`. The payment leaves are the keccak256 hash of each earner leaf. An earner leaf is composed by the earner and the token root of the token leaves, and each token leaf is the result of hashing the token address with the token earnings.

To claim against the root, use:

``` bash
make claim-distributions
```

If you want to check the balance of the claimer, you can run the following command:
``` bash
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

- [Eigenlayer core](https://github.com/Layr-Labs/eigenlayer-contracts/tree/master) contracts
- AVS contracts
  - [ServiceManager](contracts/src/IncredibleSquaringServiceManager.sol) which will eventually contain slashing logic but for M2 is just a placeholder.
  - [TaskManager](contracts/src/IncredibleSquaringTaskManager.sol) which contains [task creation](contracts/src/IncredibleSquaringTaskManager.sol#L83) and [task response](contracts/src/IncredibleSquaringTaskManager.sol#L102) logic. Calls `fulfillSlashingRequest` to the [Slasher] contract using the `raiseAndResolveChallenge` function .
  - The [challenge](contracts/src/IncredibleSquaringTaskManager.sol#L176) logic could be separated into its own contract, but we have decided to include it in the TaskManager for this simple task.
  - Set of [registry contracts](https://github.com/Layr-Labs/eigenlayer-middleware) to manage operators opted in to this avs
- Task Generator
  - This is a separate entity .
- Aggregator
  - aggregates BLS signatures from operators and posts the aggregated response to the task manager
  - For this simple demo, the aggregator is not an operator, and thus does not need to register with eigenlayer or the AVS contract. It's IP address is simply hardcoded into the operators' config.
- Operators
  - Square the number sent to the task manager by the task generator, sign it, and send it to the aggregator



## Default Configuration
- Metrics http endpoint - `http://localhost:9001/metrics`
- Aggregator Rpc endpoint - `127.0.0.1:8080`
- Operator1 - `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266` (anvil's 0 index key)
- Operator2 - `0x0b065a0423f076a340f37e16e1ce22e23d66caf2` 


## Dependencies 
- [eigensdk-rs](https://github.com/Layr-Labs/eigensdk-rs)
- [rust-bls-bn254](https://github.com/Layr-Labs/rust-bls-bn254/tree/main) 



