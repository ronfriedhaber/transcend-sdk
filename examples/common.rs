use transcend_sdk::{Result, client::Client};

pub fn client_from_env_default() -> Result<Client> {
    let base_url =
        std::env::var("TRANSCEND_BASE_URL").expect("TRANSCEND_BASE_URL must be set");
    let api_key =
        std::env::var("TRANSCEND_API_KEY").expect("TRANSCEND_API_KEY must be set");

    Client::new(base_url, api_key)
}
