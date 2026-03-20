mod common;

#[tokio::main]
async fn main() -> transcend_sdk::Result<()> {
    let client = common::client_from_env_default()?;
    let workspace = common::workspace_from_env_default(&client)?;
    println!("workspace_id={}", workspace.id());

    Ok(())
}
