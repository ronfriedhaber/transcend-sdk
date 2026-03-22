use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Result, client::Client, error::Error};

impl Client {
    pub async fn run_status(&self, run_id: impl Into<String>) -> Result<GetRunStatusResponse> {
        run_status(self, run_id.into()).await
    }
}

pub(crate) async fn run_status(client: &Client, run_id: String) -> Result<GetRunStatusResponse> {
    if run_id.trim().is_empty() {
        return Err(Error::EmptyRunId);
    }

    client
        .http_json_v1(reqwest::Method::GET, "/api/v1/run/status", |builder| {
            builder.query(&[("run_id", run_id)])
        })
        .await
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RunStatus {
    Queued,
    Leased,
    Done,
    Dead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunScalar {
    Number(serde_json::Number),
    String(String),
    Boolean(bool),
}

pub type RunOutputTable = BTreeMap<String, Vec<Option<RunScalar>>>;
pub type RunOutput = BTreeMap<String, RunOutputTable>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRunStatusResponse {
    pub run_id: String,
    pub queue: String,
    pub workspace_id: String,
    pub program_id: String,
    pub dataset_id: String,
    pub submitted_by: String,
    pub status: RunStatus,
    pub attempts: u32,
    pub max_attempts: u32,
    pub last_error: Option<String>,
    pub output: Option<RunOutput>,
    pub created_at: String,
    pub updated_at: String,
}
