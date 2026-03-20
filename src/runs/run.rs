use serde::{Deserialize, Serialize};

use crate::{Result, client::Client};

impl Client {
    pub async fn submit_run(
        &self,
        program_id: impl Into<String>,
        dataset_id: impl Into<String>,
    ) -> Result<RunResponse> {
        submit_run(self, RunRequest::new(program_id, dataset_id)).await
    }
}

pub(crate) async fn submit_run(client: &Client, request: RunRequest) -> Result<RunResponse> {
    client
        .http_json_v1(reqwest::Method::POST, "/api/v1/run/submit", |builder| {
            builder.json(&request)
        })
        .await
}

#[derive(Debug, Clone, Serialize)]
pub struct RunRequest {
    program_id: String,
    dataset_id: String,
}

impl RunRequest {
    pub fn new(program_id: impl Into<String>, dataset_id: impl Into<String>) -> Self {
        Self {
            program_id: program_id.into(),
            dataset_id: dataset_id.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RunResponse {
    pub run_id: String,
    pub queue: String,
    pub workspace_id: String,
    pub program_id: String,
    pub dataset_id: String,
    pub status: String,
}
