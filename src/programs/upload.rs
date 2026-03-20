use mpera_frontend::program::Program;
use serde::{Deserialize, Serialize};

use crate::{Result, client::Client};

pub(crate) async fn upload_program(
    client: &Client,
    workspace_id: &str,
    program: &Program,
    alias: Option<String>,
) -> Result<ProgramResponse> {
    client
        .http_json_v1(reqwest::Method::POST, "/api/v1/programs", |request| {
            request.json(&UploadProgramRequest {
                workspace_id,
                program,
                alias,
            })
        })
        .await
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProgramResponse {
    pub program_id: String,
    pub workspace_id: String,
    pub alias: String,
    pub owner: String,
    pub object_key: String,
    pub created_at: String,
}

#[derive(Serialize)]
struct UploadProgramRequest<'a> {
    workspace_id: &'a str,
    program: &'a Program,
    alias: Option<String>,
}
