# Transcend SDK

Rust SDK for `transcend2/transcend-node`.

Program and dataset uploads are workspace-scoped:

```rust
use autark_frontend::{Dataset, Query};
use transcend_sdk::client::Client;

let client = Client::new(base_url, api_key)?;
let workspace = client.workspace(workspace_id)?;

let query = Query::new("SELECT COUNT(*) AS row_count FROM input")
    .with_dataset(Dataset::transcend("input", dataset_id));

let program = workspace.upload_program(&query, Some("example".to_string())).await?;
let dataset = workspace.upload_dataset(batches, Some("input".to_string())).await?;
let run = client
    .submit_run(program.program_id, dataset.dataset_id, transcend_sdk::runs::HardwareKind::CpuBasic)
    .await?;
let output = client.read_dataset_ipc("dataset-id").await?;
```

Environment used by the examples:

- `TRANSCEND_BASE_URL`
- `TRANSCEND_API_KEY`
- `TRANSCEND_WORKSPACE_ID`
- `TRANSCEND_WORKSPACE_NAME`
- `TRANSCEND_DATASET_ID`
