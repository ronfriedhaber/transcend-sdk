use transcend_sdk::{Result, client::Client, workspaces::WorkspaceClient};

pub fn client_from_env_default() -> Result<Client> {
    let base_url = std::env::var("TRANSCEND_BASE_URL").expect("TRANSCEND_BASE_URL must be set");
    let api_key = std::env::var("TRANSCEND_API_KEY").expect("TRANSCEND_API_KEY must be set");

    Client::new(base_url, api_key)
}

#[allow(dead_code)]
pub fn workspace_from_env_default(client: &Client) -> Result<WorkspaceClient<'_>> {
    let workspace_id =
        std::env::var("TRANSCEND_WORKSPACE_ID").expect("TRANSCEND_WORKSPACE_ID must be set");
    client.workspace(workspace_id)
}
