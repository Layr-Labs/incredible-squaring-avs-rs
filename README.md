# Incredible Squaring Avs 

Basic repo demoing a simple AVS middleware with full eigenlayer integration, in rust.

## Dependencies

- [Foundry](https://github.com/foundry-rs/foundry)
- [Docker](https://www.docker.com/) 


## Anvil 

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

- Single command AVS start using the following command (default values)
```sh
cargo run --bin incredible-squaring-avs  start
```

- To change the parameters, provide path to a toml config file 
```
cargo run --bin incredible-squaring-avs  start --config-path <PATH>
```
We have an example file [incredible_config.toml](https://github.com/Layr-Labs/incredible-squaring-avs-rust/tree/master/incredible_config.toml) for reference.

This command launches 5 services(crates) together:

- Operator1 : It listens for new tasks , responds them by signing with their bls key and send the signed response to the aggregator. Stake in strategy: 5000 tokens
- Operator2: Same task as operator 1. Stake in strategy: 7000 tokens
- Aggregator: Sets up an Rpc client to receive signed task responses from operators, aggregates the signatures, if quorums is met (i.e both operators sign the response), it calls the respondToTask function in the TaskManager contract.
- Challenger : It listens for new tasks , checks the operators response, if found wrong, it raises a challenge by calling the `raiseAndResolveChallenge` function in the task manager contract.
- Task Spam : It creates a new task every 10 seconds by calling the `createNewTask` function in the task manager contract.

## Mainnet 
We support mainnet deployment simulation of AVS contracts 

1)  git submodule and copy env 
    ```sh
    git submodule update --init --recursive
    cp contracts/.env.example contracts/.env
    ```

2) Paste your .env variables 
  - `MAINNET_DEPLOYER_KEY` , `MAINNET_RPC_URL` , `ETHERSCAN_API_KEY`

3) Run this to simulate
   ```sh
    make simulate-avs-mainnet-deployment
   ```
4) To actually deploy(Optional)
   Add --broadcast in the forge script [command]()
   Update the `mainnet_incredible_config.toml` file with correct addresses to run the avs 
   Then run 
   ```sh
   cargo run --bin incredible-squaring-avs  start --config-path <mainnet config path>
   ```
   
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
  - [TaskManager](contracts/src/IncredibleSquaringTaskManager.sol) which contains [task creation](contracts/src/IncredibleSquaringTaskManager.sol#L83) and [task response](contracts/src/IncredibleSquaringTaskManager.sol#L102) logic.
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



