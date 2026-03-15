use serde::{Deserialize, Serialize};

use crate::{Result, client::Client};

impl Client {
    pub async fn metadata(&self) -> Result<MetadataResponse> {
        self.http_json_v1(reqwest::Method::GET, "/metadata", |request| request)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataResponse {
    pub total_data_bytes: u64,
    pub total_programs: u64,
    pub total_runs: u64,
}
