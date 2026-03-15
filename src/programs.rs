use mpera::program::Program;
use serde::{Deserialize, Serialize};

use crate::{Result, client::Client};

impl Client {
    pub async fn upload_program(
        &self,
        program: &Program,
        alias: Option<String>,
    ) -> Result<ProgramResponse> {
        self.http_json_v1(reqwest::Method::POST, "/programs", |request| {
            request.json(&UploadProgramRequest { program, alias })
        })
        .await
    }

    pub async fn programs(
        &self,
        limit: Option<usize>,
    ) -> Result<ListProgramsResponse> {
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

#[derive(Debug, Clone, Deserialize)]
pub struct ProgramResponse {
    pub id: String,
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

#[derive(Serialize)]
struct UploadProgramRequest<'a> {
    program: &'a Program,
    alias: Option<String>,
}
