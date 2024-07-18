use alloy::sol;
use serde::{Deserialize, Serialize};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    IncredibleSquaringTaskManager,
    "IncredibleSquaringTaskManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    IncredibleSquaringServiceManager,
    "IncredibleSquaringServiceManager.json"
);
