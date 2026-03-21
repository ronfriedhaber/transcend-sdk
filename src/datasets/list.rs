use crate::{Result, client::Client, datasets::DatasetMetadata};

pub(crate) async fn list_datasets(
    client: &Client,
    workspace_id: Option<&str>,
) -> Result<Vec<DatasetMetadata>> {
    let mut query = Vec::new();
    if let Some(workspace_id) = workspace_id {
        query.push(("workspace_id", workspace_id.to_string()));
    }

    client
        .http_json_v1(reqwest::Method::GET, "/api/v1/datasets", |request| {
            if query.is_empty() {
                request
            } else {
                request.query(&query)
            }
        })
        .await
}
