# Incredible Squaring Avs 

Basic repo demoing a simple AVS middleware with full eigenlayer integration, in rust.

## Dependencies

- [Foundry](https://github.com/foundry-rs/foundry)
- [Docker](https://www.docker.com/) 


## To run 

- Start anvil in a separate terminal 
```
anvil
```

- Deploy eigenlayer and avs contracts
```
deploy-el-and-avs-contracts
```

- Single command AVS start using the following command (default values)
```bash
cargo run --bin incredible-squaring-avs  start
```

- To change the parameters, provide path to a toml config file 
```
cargo run --bin incredible-squaring-avs  start --config-path <PATH>
```
We have an example file [incredible_config.toml](https://github.com/Layr-Labs/incredible-squaring-avs-rust/tree/master/incredible_config.toml) for reference.

This command launches 4 crates together 
- Operator : It listens for new tasks , responds them by signing with their bls key and send the signed response to the aggregator.
- Aggregator : Sets up an Rpc client to receive signed task responses from operator, aggregates the signatures , calls the respondToTask function in the TaskManager contract.
- Challenger : It listens for new tasks , checks the operators response, if found wrong, it raises a challenge by calling the `raiseAndResolveChallenge` function in the task manager contract.
- Task Spam : It creates a new task every 10 seconds by calling the `createNewTask` function in the task manager contract.


## Testing 

- To run unit tests(start anvil in a separate terminal)
```
make pr
```

- To run integration tests(start anvil in a separate terminal) 
```
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
  - in a real world scenario, this could be a separate entity, but for this simple demo, the aggregator also acts as the task generator
- Aggregator
  - aggregates BLS signatures from operators and posts the aggregated response to the task manager
  - For this simple demo, the aggregator is not an operator, and thus does not need to register with eigenlayer or the AVS contract. It's IP address is simply hardcoded into the operators' config.
- Operators
  - Square the number sent to the task manager by the task generator, sign it, and send it to the aggregator

 ![architecture (1)](https://github.com/user-attachments/assets/389349cd-931f-448c-bf2c-ea49af133542)


## Default Configuration
- Metrics http endpoint - `http://localhost:9001/metrics`
- Aggregator Rpc endpoint - `127.0.0.1:8080`
- Operator - `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266` (anvil's 0 index key)


## Dependencies 
- [eigensdk-rs](https://github.com/Layr-Labs/eigensdk-rs)
- [rust-bls-bn254](https://github.com/Layr-Labs/rust-bls-bn254/tree/main) 



