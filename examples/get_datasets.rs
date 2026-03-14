use transcend_sdk::client::Client;

#[tokio::main]
async fn main() -> transcend_sdk::Result<()> {
    let base_url = std::env::var("TRANSCEND_BASE_URL")
        .expect("TRANSCEND_BASE_URL must be set");
    let api_key =
        std::env::var("TRANSCEND_API_KEY").expect("TRANSCEND_API_KEY must be set");

    let client = Client::new(base_url, api_key)?;
    let datasets = client.get_datasets().await?;

    println!("{datasets:#?}");

    Ok(())
}
