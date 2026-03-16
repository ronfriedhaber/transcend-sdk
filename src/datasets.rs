pub mod upload;

use serde::{Deserialize, Serialize};

use crate::{Result, client::Client};

pub use upload::DatasetResponse;

impl Client {
    pub async fn datasets(&self) -> Result<ListDatasetsResponse> {
        self.http_json_v1(reqwest::Method::GET, "/datasets", |request| request)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDatasetsResponse {
    pub datasets: Vec<DatasetMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetMetadata {
    pub id: String,
    pub author: String,
    pub is_output: bool,
    pub created_at_ms: u64,
    pub last_updated_ms: u64,
    pub size_bytes: u64,
    pub chunk_count: u64,
    pub alias: Option<String>,
    pub schema: Option<DatasetSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetSchema {
    pub fields: Vec<DatasetSchemaField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetSchemaField {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
}
