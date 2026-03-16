use transcend_sdk::datasets::{DatasetIpcReadOptions, DatasetJsonReadOptions};

mod common;

#[tokio::main]
async fn main() -> transcend_sdk::Result<()> {
    let dataset = std::env::var("TRANSCEND_DATASET")
        .expect("TRANSCEND_DATASET must be set to a dataset id or alias");

    let client = common::client_from_env_default()?;

    let metadata = client.dataset(&dataset).await?;
    println!("metadata:\n{metadata:#?}");

    let rows = client
        .dataset_json(
            &dataset,
            DatasetJsonReadOptions {
                max_rows: Some(5),
            },
        )
        .await?;
    println!("json rows:\n{rows:#?}");

    let batches = client
        .dataset_ipc(
            &dataset,
            DatasetIpcReadOptions {
                max_batches: Some(1),
                latest: Some(true),
            },
        )
        .await?;
    println!("ipc batches: {}", batches.len());

    Ok(())
}
