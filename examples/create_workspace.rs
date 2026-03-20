mod common;

#[tokio::main]
async fn main() -> transcend_sdk::Result<()> {
    let client = common::client_from_env_default()?;
    let workspace_name = std::env::var("TRANSCEND_WORKSPACE_NAME")
        .unwrap_or_else(|_| "transcend-sdk-example".to_string());
    let workspace = client.create_workspace(workspace_name).await?;

    println!("{workspace:#?}");

    Ok(())
}
