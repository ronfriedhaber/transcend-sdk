use serde::{Deserialize, Serialize};

use crate::{Result, client::Client};

impl Client {
    pub async fn submit_run(
        &self,
        program_id: impl Into<String>,
        dataset_id: impl Into<String>,
        hardware: HardwareKind,
    ) -> Result<RunResponse> {
        submit_run(self, RunRequest::new(program_id, dataset_id, hardware)).await
    }
}

pub(crate) async fn submit_run(client: &Client, request: RunRequest) -> Result<RunResponse> {
    client
        .http_json_v1(reqwest::Method::POST, "/api/v1/run/submit", |builder| {
            builder.json(&request)
        })
        .await
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HardwareKind {
    CpuBasic,
    GpuBasic,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunRequest {
    program_id: String,
    dataset_id: String,
    hardware: HardwareKind,
}

impl RunRequest {
    pub fn new(
        program_id: impl Into<String>,
        dataset_id: impl Into<String>,
        hardware: HardwareKind,
    ) -> Self {
        Self {
            program_id: program_id.into(),
            dataset_id: dataset_id.into(),
            hardware,
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
