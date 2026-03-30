use arrow_array::RecordBatch;
use reqwest::header;
use serde::{Deserialize, Serialize};

use crate::{Result, arrow::encode_record_batches_ipc, client::Client};

pub(crate) async fn upload_dataset(
    client: &Client,
    workspace_id: &str,
    batches: Vec<RecordBatch>,
    alias: Option<String>,
) -> Result<DatasetResponse> {
    let payload = encode_record_batches_ipc(&batches)?;
    upload_dataset_ipc_bytes(client, workspace_id, payload, alias).await
}

pub(crate) async fn upload_dataset_ipc_bytes(
    client: &Client,
    workspace_id: &str,
    payload: Vec<u8>,
    alias: Option<String>,
) -> Result<DatasetResponse> {
    let query = dataset_upload_query(workspace_id, alias);

    client
        .http_json_v1(reqwest::Method::POST, "/api/v1/dataset/upload", |request| {
            request
                .query(&query)
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .body(payload)
        })
        .await
}

fn dataset_upload_query(workspace_id: &str, alias: Option<String>) -> Vec<(&'static str, String)> {
    let mut query = vec![("workspace_id", workspace_id.to_string())];

    if let Some(alias) = alias {
        query.push(("alias", alias));
    }

    query
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetResponse {
    pub dataset_id: String,
    pub workspace_id: String,
    pub alias: String,
    pub bytes_received: usize,
    pub owner: String,
    pub object_key: String,
    pub created_at: String,
}
