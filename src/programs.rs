use serde::{Deserialize, Serialize};

use crate::{Result, client::Client};

pub mod upload;

pub use upload::ProgramResponse;

impl Client {
    pub async fn programs(&self, limit: Option<usize>) -> Result<ListProgramsResponse> {
        self.http_json_v1(reqwest::Method::GET, "/programs", |request| {
            request.query(&ProgramsQuery { limit })
        })
        .await
    }
}

#[derive(Debug, Clone, Serialize)]
struct ProgramsQuery {
    limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListProgramsResponse {
    pub programs: Vec<ProgramMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramMetadata {
    pub id: String,
    pub created_at_ms: u64,
    pub alias: Option<String>,
    pub size_bytes: usize,
}
