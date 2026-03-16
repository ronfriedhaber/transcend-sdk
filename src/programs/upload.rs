use mpera_frontend::program::Program;
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
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProgramResponse {
    pub id: String,
}

#[derive(Serialize)]
struct UploadProgramRequest<'a> {
    program: &'a Program,
    alias: Option<String>,
}
