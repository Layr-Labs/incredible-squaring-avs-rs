use std::process::Stdio;
use tokio::process::Command as TokioCommand;

/// Anvil RPC URL
const ANVIL_URL: &str = "http://localhost:8545";
/// Signer
const SIGNER: &str = "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";
/// Task Manager Address
const TASK_MANAGER_ADDRESS: &str = "0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3";

/// Spawns the aggregator binary
async fn spawn_aggregator() -> tokio::process::Child {
    TokioCommand::new("cargo")
        .args(["run", "--bin", "incredible-squaring-aggregator"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Could not spawn aggregator binary")
}

/// Spawns the challenger binary
async fn spawn_challenger() -> tokio::process::Child {
    TokioCommand::new("cargo")
        .args(["run", "--bin", "incredible-squaring-challenger"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Could not spawn challenger binary")
}

/// Spawns the operator binary
async fn spawn_operator(config_path: &str, failure_rate: u32) -> tokio::process::Child {
    // Wait 5 seconds to let the aggregator start
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    TokioCommand::new("cargo")
        .args([
            "run",
            "--bin",
            "incredible-squaring-operator",
            "--",
            "-c",
            config_path,
            "-f",
            &failure_rate.to_string(),
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Could not spawn operator binary")
}

/// Spawns the task spammer binary with the given number of tasks
async fn spawn_task_spammer(number_of_tasks: u32) -> tokio::process::Child {
    // Wait until operators are spawned
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    TokioCommand::new("cargo")
        .args([
            "run",
            "--bin",
            "incredible-squaring-task-spammer",
            "--",
            "-n",
            &number_of_tasks.to_string(),
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Could not spawn task spammer binary")
}
pub mod integration_test {
    use std::str::FromStr;

    use alloy::{
        network::EthereumWallet, primitives::{Address, FixedBytes}, providers::ProviderBuilder,
        signers::local::PrivateKeySigner, transports::http::reqwest::Url,
    };
    use incredible_squaring::bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::{IncredibleSquaringTaskManagerInstance};

    use super::*;

    // This test waits for the task spammer to complete executing 3 tasks
    // and then verifies that all 3 tasks have been responded to in the contract
    #[tokio::test]
    async fn test_integration_wait_for_completion() {
        let mut children = vec![
            spawn_aggregator().await,
            spawn_challenger().await,
            spawn_operator("src/config/squaring-operator.toml", 0).await,
        ];
        let mut task_spammer_handle = spawn_task_spammer(3).await;

        // Wait for the task spammer to complete
        task_spammer_handle.wait().await.unwrap();

        // Terminate the other processes
        for child in &mut children {
            let _ = child.kill().await;
        }

        // Verify the responses in IncredibleSquaringTaskManager
        let task_manager_address = Address::from_str(TASK_MANAGER_ADDRESS).unwrap();
        let url = Url::parse(ANVIL_URL).unwrap();
        let wallet = EthereumWallet::new(PrivateKeySigner::from_str(SIGNER).unwrap());
        let provider = ProviderBuilder::new().wallet(wallet).on_http(url);
        let task_manager_contract =
            IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider);

        let latest_task_num = task_manager_contract
            .latestTaskNum()
            .call()
            .await
            .unwrap()
            ._0;

        assert_eq!(latest_task_num, 3);

        // Verify that the 3 most recent tasks have valid responses
        for task_index in 0..latest_task_num {
            let response_hash = task_manager_contract
                .allTaskResponses(task_index)
                .call()
                .await
                .unwrap()
                ._0;

            assert_ne!(FixedBytes::<32>::default(), response_hash,);
        }
    }

    // This test spawns the aggregator, challenger, two operators, and a task spammer.
    // The operators are configured to fail 30% and 100% of the time, respectively.
    // So when checking `taskSuccesfullyChallenged`, we expect it to be true.
    #[tokio::test]
    async fn test_wrong_response_is_challenged() {
        let mut children = vec![
            spawn_aggregator().await,
            spawn_challenger().await,
            spawn_operator("src/config/squaring-operator.toml", 30).await,
            spawn_operator("src/config/squaring-operator-2.toml", 100).await,
            spawn_task_spammer(1).await,
        ];

        // Wait 10 seconds to let operators respond to tasks
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        for child in &mut children {
            let _ = child.kill().await;
        }

        // Check responses in IncredibleSquaringTaskManager
        let task_manager_address = Address::from_str(TASK_MANAGER_ADDRESS).unwrap();
        let url = Url::parse(ANVIL_URL).unwrap();
        let wallet = EthereumWallet::new(PrivateKeySigner::from_str(SIGNER).unwrap());
        let provider = ProviderBuilder::new().wallet(wallet).on_http(url);
        let task_manager_contract =
            IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider);

        let latest_task_num = task_manager_contract
            .latestTaskNum()
            .call()
            .await
            .unwrap()
            ._0;

        let task_hash = task_manager_contract
            .allTaskHashes(latest_task_num - 1)
            .call()
            .await
            .unwrap()
            ._0;

        assert_ne!(FixedBytes::<32>::default(), task_hash);

        let response_hash = task_manager_contract
            .allTaskResponses(latest_task_num - 1)
            .call()
            .await
            .unwrap()
            ._0;
        assert_ne!(FixedBytes::<32>::default(), response_hash);

        let is_challenge_success = task_manager_contract
            .taskSuccesfullyChallenged(latest_task_num - 1)
            .call()
            .await
            .unwrap()
            ._0;

        assert!(is_challenge_success);
    }
}
