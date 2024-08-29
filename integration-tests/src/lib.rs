

#[cfg(test)]
mod tests{

    
    #[tokio::test]
    async fn test_call_challenge() {
        let mut config: IncredibleConfig = toml::from_str(INCREDIBLE_CONFIG_FILE).unwrap();
        
        let mut challenger = build_challenger().await;
        let task_manager = TaskManager::new(get_incredible_squaring_task_manager().await,config.http_rpc_url(),config.task_manager_signer());
        let create_new_task = task_manager.create_new_task(U256::from(0)).await.unwrap();
        let create_new_task = task_manager.create_new_task(U256::from(1)).await.unwrap();
        let create_new_task = task_manager.create_new_task(U256::from(2)).await.unwrap();
        
        let task_manager_contract = IncredibleSquaringTaskManager::new(
            get_incredible_squaring_task_manager().await,
            get_signer(config.task_manager_signer(), &config.http_rpc_url()),
        );
        
        let third_task_data = create_new_task.inner.logs().get(0).unwrap();
        let decoded_task = third_task_data.log_decode::<NewTaskCreated>().unwrap();
        println!("decoded task {:?}",decoded_task);
        
        let NewTaskCreated { taskIndex, task } = decoded_task.data();
        
        println!("task index {:?}",taskIndex);
        println!("task {:?}",task);
        let new_task_created = NewTaskCreated {
            taskIndex:taskIndex.clone(),
            task:task.clone()
        };
        
        challenger.process_new_task_created_log(new_task_created.clone());
        
        let task  = Task{numberToBeSquared: U256::from(2), taskCreatedBlock:  task.taskCreatedBlock, quorumNumbers: Bytes::from_str("0x00").unwrap(), quorumThresholdPercentage: 100};
        let task_response = TaskResponse { referenceTaskIndex: 2, numberSquared: U256::from(4) };
        let non_signer_stakes_and_signature = NonSignerStakesAndSignature{
            nonSignerQuorumBitmapIndices: vec![],
            nonSignerPubkeys: vec![],
            quorumApks: vec![G1Point{X: U256::from_str("277950648056014144722774518899051149098728246263316284984520891067822832300").unwrap(),Y: U256::from_str("16927236637669640540790285431111034664564710839671197540688155537113438534238").unwrap()} ],
            apkG2: G2Point{X: ["15529400123788596166111036611862227541174221446291015207340396747864347375335".parse().unwrap(),"6834287759893774453556191528501556195232162436167606874229072410417955767882".parse().unwrap()],Y:["19775028091101520702581412350510183088819198056772055625089714355379667714558".parse().unwrap(),"7616309349481520605447660298084926776417001188005125143383153219707218450524".parse().unwrap()]},
            sigma: G1Point{X: "756457839249151979683908384765832254090586977920318235048918288232035503865".parse().unwrap(),Y:"5933935470696330953588600072761732001080108730780531718814482403231396239923".parse().unwrap()},
            quorumApkIndices: vec![1],
            totalStakeIndices: vec![1],
            nonSignerStakeIndices: vec![vec![]]
        };
        let task_manager_different_signer = TaskManager::new(get_incredible_squaring_task_manager().await,config.http_rpc_url(),"0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6".to_string());
        
        let s = task_manager_different_signer.respond_to_task(task, task_response, non_signer_stakes_and_signature).await.unwrap();
        println!("respond to task {:?}",s.transaction_hash);
        
        
        let operator = OperatorBuilder::build()
        // let topics = [B256::from_hex("0x1695b8d06ec800b4615e745cfb5bd00c1f2875615d42925c3b5afa543bb24c48").unwrap(),B256::from_hex("0x0000000000000000000000000000000000000000000000000000000000000003").unwrap()];
        // let data = Bytes::from_hex("0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000760000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000").unwrap();
        // let log = Log { inner: alloy::primitives::Log { address: address!("22753e4264fddc6181dc7cce468904a80a363e44"), data: LogData::new(topics.to_vec(), data).unwrap() }, block_hash: Some(B256::from_hex("0xad6fc0684c312f9174321093fa55973170b1fadcfea73c3035b4cdd975b382fe").unwrap()), block_number: Some(118), block_timestamp: None, transaction_hash: Some(B256::from_hex("0xeec060e7a2a4c74b3bd168aef249d0f5d64bb5925917056bfc45ecf420d3384e").unwrap()), transaction_index: Some(0), log_index: Some(0), removed: false };
        // let log = create_new_task.inner.logs().get(0).unwrap();
        // let _ = challenger.process_task_response_log(log.clone()).await.unwrap();
        
        
        // challenger.call_challenge(6).await.unwrap();
        
        
        
        
    }
}