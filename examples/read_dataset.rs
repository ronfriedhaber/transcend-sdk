mod common;

#[tokio::main]
async fn main() -> transcend_sdk::Result<()> {
    let dataset_id =
        std::env::var("TRANSCEND_DATASET_ID").expect("TRANSCEND_DATASET_ID must be set");

    let client = common::client_from_env_default()?;
    let batches = client.read_dataset_ipc(dataset_id).await?;
    println!("ipc batches: {}", batches.len());

    Ok(())
}
