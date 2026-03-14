mod common;

#[tokio::main]
async fn main() -> transcend_sdk::Result<()> {
    let client = common::client_from_env_default()?;
    let datasets = client.datasets().await?;

    println!("{datasets:#?}");

    Ok(())
}
