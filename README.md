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

## Structure Documentation

This PR has four main participants:

- Aggregator: The aggregator does three things in parallel:
  - [Listens to operators responses](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/aggregator/src/lib.rs#L206-L210) for created tasks to [send them](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/aggregator/src/lib.rs#L261) to the BLS Aggregation service.
  - [Listens to `NewTaskCreated` events](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/aggregator/src/lib.rs#L290-L292), and if one is received, saves that task and [sends it](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/aggregator/src/lib.rs#L320-L323) to the BLS aggregation service initialize_task method.
  - [Receives aggregated responses](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/aggregator/src/lib.rs#L340-L352) from the BLS aggregation service, and, in case it has the associated task response, [sends the aggregated response](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/aggregator/src/lib.rs#L362-L369) to the `TaskManager` contract.
- Challenger: The challenger listens to [`NewTaskCreated`](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/challenger/src/lib.rs#L114-L127) and [`TaskResponded`](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/challenger/src/lib.rs#L107-L113) events. In the first case, [adds the task](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/challenger/src/lib.rs#L142-L143) to the tasks hashMap. In the second, if the task was registered [calculates the response and compares it](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/challenger/src/lib.rs#L151-L155) to the aggregator's response, [raising a challenge](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/challenger/src/lib.rs#L183C34-L183C49) (calling the `TaskManager` contract) if they are different.
- Operator: The operator listens to [`NewTaskCreated`](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/operator/src/builder.rs#L136-L138) events. If one is received, [processes the task](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/operator/src/builder.rs#L93) and [returns the response](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/operator/src/builder.rs#L106-L109). After that, [sends the signed response](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/operator/src/builder.rs#L156-L159) to the BLS aggregation service.
- Task Generator: the task generator [sends a new task](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/task_generator/src/lib.rs#L66-L77) to `TaskManager` contract [every 10 seconds](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/task_generator/src/lib.rs#L85).

Now, we are showing each one in a more detailed way:

### Aggregator

The aggregator logic is on this segment of code in [`start()` method](https://github.com/maximopalopoli/incredible-squaring-avs-rs/blob/484e6968da4a9a0ee4e22effb0da807306fe76b7/crates/aggregator/src/lib.rs#L146):

``` Rust
// Spawn three tasks: one for the server, one for processing tasks, and one for processing aggregator responses
// 1) Process signatures
let server_handle =
  Self::start_server(port_address, service_handle.clone(), task_responses.clone())
    .await?;
// 2) Process tasks
let process_handle = tokio::spawn(Self::process_tasks(
  ws_rpc_url.clone(),
  Arc::clone(&tasks),
  service_handle,
));
// 3) Process aggregator responses
let responses_handle = tokio::spawn(Self::process_aggregator_responses(
  Arc::clone(&tasks),
  Arc::clone(&task_responses),
  avs_writer,
  aggregate_receiver,
));

// Join of the three tasks
```

The first process starts a server with an async function that [receives parameters](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/aggregator/src/lib.rs#L206-L210) and [calls `process_signed_task_response()` method](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/aggregator/src/lib.rs#L214-L219), which code is the following:

``` Rust
async fn process_signed_task_response(
  signed_task_response: SignedTaskResponse,
  service_handle: &ServiceHandle,
  task_responses: &mut HashMap<u32, HashMap<TaskResponseDigest, TaskResponse>>,
) -> Result<(), AggregatorError> {
  // Get task_index, task_response_digest, signature and operator_id from signed_task_response

  let task_signature = TaskSignature::new(task_index, task_response_digest, signature, operator_id);
  let result = service_handle.process_signature(task_signature).await;
  // Handle error

  task_responses
    .entry(task_index)
    .or_default()
    .entry(task_response_digest)
    .or_insert(signed_task_response.task_response);

  Ok(())
}
```

[This code](https://github.com/maximopalopoli/incredible-squaring-avs-rs/blob/484e6968da4a9a0ee4e22effb0da807306fe76b7/crates/aggregator/src/lib.rs#L245-L276) obtains the task_signature, and sends it to the BLS Aggregation service to process it.

The second process runs `process_tasks()` method on a separate task:

``` Rust
async fn process_tasks(
  ws_rpc_url: String,
  tasks: Arc<tokio::sync::Mutex<HashMap<u32, Task>>>,
  service_handle: ServiceHandle,
) -> eyre::Result<()> {
  // Get the provider

  let filter = Filter::new().event_signature(NewTaskCreated::SIGNATURE_HASH);
  let sub = provider.subscribe_logs(&filter).await?;
  let mut stream = sub.into_stream();

  while let Some(log) = stream.next().await {
    let NewTaskCreated { taskIndex, task } = log.log_decode()?.inner.data;

    tasks.lock().await.insert(taskIndex, task.clone());

    // Get quorum_nums, quorum_threshold_percentages and time_to_expiry from the task

    let task_metadata = TaskMetadata::new(
      taskIndex,
      task.taskCreatedBlock.into(),
      quorum_nums.clone(),
      quorum_threshold_percentages.clone(),
      time_to_expiry,
    );

    let _ = service_handle
      .initialize_task(task_metadata)
      .await
      .map_err(|e: BlsAggregationServiceError| eyre::eyre!(e));
  }

  Ok(())
}
```

[This code](https://github.com/maximopalopoli/incredible-squaring-avs-rs/blob/484e6968da4a9a0ee4e22effb0da807306fe76b7/crates/aggregator/src/lib.rs#L279-L327) listens to new `NewTaskCreated` events, and in case it receives a new one:

1. Parses task metadata parameters from the task
2. Creates the task metadata from that parameters
3. Calls BLS Aggregation service `initialize_task()` method, with the metadata of the new task as a parameter

The third process runs a new task with this code:

``` Rust
loop {
  // Wait for the next aggregated response received from BLS aggregator service
  let Ok(service_response) = aggregate_receiver_channel
    .receive_aggregated_response()
    .await
    // Handle a possible error
  else {
    continue;
  };

  // Get task response from aggregated response

  if let Some(task_response) = task_response {
    let tasks_lock = tasks.lock().await;
    send_aggregated_response_to_contract(
      &tasks_lock,
      &avs_writer,
      task_response,
      service_response,
    )
    .await?;
  } else {
    // inform there was no task_response for task_index
  }
}
```

In a simple way, [listens to aggregated responses](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/aggregator/src/lib.rs#L340-L352) from BLS aggregation service, and when receives one sends it to the `TaskManager` contract with [`send_aggregated_response_to_contract()` method](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/aggregator/src/lib.rs#L381-L386).

### Challenger

The challenger logic is placed in [this loop](https://github.com/maximopalopoli/incredible-squaring-avs-rs/blob/484e6968da4a9a0ee4e22effb0da807306fe76b7/crates/challenger/src/lib.rs#L105-L135) on `start_challenger()` method:

``` Rust
loop {
  tokio::select! {
    Some(log) = task_responded_stream.next() => {
      let task_index = self.process_task_response_log(log).await?;
      if self.tasks.contains_key(&task_index) {
        self.call_challenge(task_index).await?;
      }
    },
    Some(log) = new_task_created_stream.next() => {
      let new_task_created_option = log.log_decode::<NewTaskCreated>().ok();

      if let Some(data) = new_task_created_option {
        let m = data.data();
        let new_task_cr = NewTaskCreated {
          taskIndex: m.taskIndex,
          task: m.task.clone(),
        };

        let _ = self.process_new_task_created_log(new_task_cr);
      }
    },
    else => {
      // If both streams are exhausted, break the loop.
      break;
    }
  };
}
```

First, we will cover the case where we receive a `NewTaskCreated` event. In that case, we create a `NewTaskCreated` struct and send it as a parameter of the [`process_new_task_created_log()` method](https://github.com/maximopalopoli/incredible-squaring-avs-rs/blob/484e6968da4a9a0ee4e22effb0da807306fe76b7/crates/challenger/src/lib.rs#L141-L146), that adds that task to the tasks HashMap, indexed by the task index. If we receive a `TaskResponded` event, then we process that event, obtaining the index of that task, [to verify that index matches](https://github.com/maximopalopoli/incredible-squaring-avs-rs/blob/484e6968da4a9a0ee4e22effb0da807306fe76b7/crates/challenger/src/lib.rs#L110) a task in the tasks HashMap. If matches a task, we will call to [`call_challenge()` method](https://github.com/maximopalopoli/incredible-squaring-avs-rs/blob/484e6968da4a9a0ee4e22effb0da807306fe76b7/crates/challenger/src/lib.rs#L149):

``` Rust
pub async fn call_challenge(&self, task_index: u32) -> Result<(), ChallengerError> {
  if let Some(task) = self.tasks.get(&task_index) {
    let num_to_square = task.numberToBeSquared;

    if let Some(answer_in_response) = self.task_responses.get(&task_index) {
      let answer = answer_in_response.task_response.numberSquared;
      if answer != (num_to_square * num_to_square) {
        let _ = self.raise_challenge(task_index).await;

        return Ok(());
      }
      Ok(())
    }
  }
}
```

This code is simplified to show here, but in a simple way [gets the task](https://github.com/maximopalopoli/incredible-squaring-avs-rs/blob/484e6968da4a9a0ee4e22effb0da807306fe76b7/crates/challenger/src/lib.rs#L150) from the tasks HashMap, and [if the response calculated by the challenger differs from the one from the Task](https://github.com/maximopalopoli/incredible-squaring-avs-rs/blob/484e6968da4a9a0ee4e22effb0da807306fe76b7/crates/challenger/src/lib.rs#L154-L155), then a challenge will be raise calling [`raise_challenge()` method](https://github.com/maximopalopoli/incredible-squaring-avs-rs/blob/484e6968da4a9a0ee4e22effb0da807306fe76b7/crates/challenger/src/lib.rs#L218-L221), that ends up calling [`raiseAndResolveChallenge()` method](https://github.com/maximopalopoli/incredible-squaring-avs-rs/blob/484e6968da4a9a0ee4e22effb0da807306fe76b7/contracts/src/IncredibleSquaringTaskManager.sol#L170-L175) from `TaskManager` contract.

### Operator

The operator logic is in this code:

```Rust
let filter = Filter::new().event_signature(NewTaskCreated::SIGNATURE_HASH);
let sub = provider.subscribe_logs(&filter).await?;
let mut stream = sub.into_stream();

while let Some(log) = stream.next().await {
  let task_option = log
    .log_decode::<IncredibleSquaringTaskManager::NewTaskCreated>()
    .ok();
  if let Some(task) = task_option {
    let data = task.data();
    let new_task_created = NewTaskCreated {
      task: data.task.clone(),
      taskIndex: data.taskIndex,
    };

    incredible_metrics::increment_num_tasks_received();
    let task_response = self.process_new_task(new_task_created);
    let signed_task_response = self.sign_task_response(task_response)?;
    let _ = arc_client
      .send_signed_task_response(signed_task_response)
      .await;
  }
}
```

Here, operator subscribes to [`NewTaskCreated` events](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/operator/src/builder.rs#L136-L138) and listens to them. If one is received, process the new task in process_new_task method:

``` Rust
pub fn process_new_task(&self, new_task_created: NewTaskCreated) -> TaskResponse {
  let mut number_to_be_squared = new_task_created.task.numberToBeSquared;

  // Random fail logic

  let num_squared = number_to_be_squared * number_to_be_squared;

  TaskResponse {
      referenceTaskIndex: new_task_created.taskIndex,
      numberSquared: num_squared,
  }
}
```

This method [processes the task](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/operator/src/builder.rs#L93) and [returns the response](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/operator/src/builder.rs#L106-L109).

After that, [signs the response](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/operator/src/builder.rs#L156C53-L156C71) and [sends it](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/operator/src/builder.rs#L157-L159) to the BLS aggregation service.

### Task Generator

Task Generator code is the following:

``` Rust
pub async fn start(&self) -> eyre::Result<()> {
  sleep(Duration::from_secs(10)).await; // wait for 10 seconds first

  // URL, signer, wallet, pr, task_manager_contract, and task_num definition

  loop {
    let number_to_be_squared = task_num;
    let quorum_threshold_percentage = 40;
    let quorum_numbers = Bytes::from_str(&self.quorum_numbers)?;

    let _ = task_manager_contract
      .createNewTask(
        number_to_be_squared,
        quorum_threshold_percentage,
        quorum_numbers.clone(),
      )
      .send()
      .await?;

    // // Increment the task number for the next iteration
    task_num += *TASK_NUMBER_INCREMENT_VALUE;

    // // Wait for 10 seconds before the next iteration
    sleep(Duration::from_secs(10)).await;
  }
}
```

This code [sends a new task](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/task_generator/src/lib.rs#L66-L77) to `TaskManager` [every 10 seconds](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/a9120b02d794076ea0d7dd643779c1ea590fd3b8/crates/task_generator/src/lib.rs#L85).

## Default Configuration

- Metrics http endpoint - `http://localhost:9001/metrics`
- Aggregator Rpc endpoint - `127.0.0.1:8080`
- Operator1 - `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266` (anvil's 0 index key)
- Operator2 - `0x0b065a0423f076a340f37e16e1ce22e23d66caf2`

## Related Projects

- [eigensdk-rs](https://github.com/Layr-Labs/eigensdk-rs) - Official EigenLayer Rust SDK
- [rust-bls-bn254](https://github.com/Layr-Labs/bn254-bls-keystore-rs) - EIP 2335 Compatible Keystore using BN254
