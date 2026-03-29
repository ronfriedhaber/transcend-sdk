use autark_frontend::Query;

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = Query::new("SELECT COUNT(*) AS row_count FROM input");

    let client = common::client_from_env_default()?;
    let workspace = common::workspace_from_env_default(&client)?;
    let response = workspace
        .upload_program(&query, Some("transcend-sdk-example-program".to_string()))
        .await?;

    println!("{response:#?}");

    Ok(())
}
