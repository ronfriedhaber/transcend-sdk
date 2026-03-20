use arrow_array::RecordBatch;
use mpera_frontend::program::Program;
use serde::{Deserialize, Serialize};

use crate::{
    Result,
    client::Client,
    datasets::{self, DatasetResponse},
    error::Error,
    programs::{ProgramResponse, upload},
};

impl Client {
    pub fn workspace(&self, workspace_id: impl Into<String>) -> Result<WorkspaceClient<'_>> {
        let workspace_id = workspace_id.into();
        if workspace_id.trim().is_empty() {
            return Err(Error::EmptyWorkspaceId);
        }

        Ok(WorkspaceClient {
            client: self,
            workspace_id,
        })
    }

    pub async fn list_workspaces(&self) -> Result<Vec<Workspace>> {
        self.http_json_v1(reqwest::Method::GET, "/api/v1/workspaces", |request| {
            request
        })
        .await
    }

    pub async fn create_workspace(&self, name: impl Into<String>) -> Result<Workspace> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(Error::EmptyName);
        }

        self.http_json_v1(reqwest::Method::POST, "/api/v1/workspaces", |request| {
            request.json(&CreateWorkspaceRequest { name })
        })
        .await
    }

    pub async fn read_dataset_ipc(&self, dataset_id: impl AsRef<str>) -> Result<Vec<RecordBatch>> {
        datasets::read::read_dataset_ipc(self, dataset_id.as_ref()).await
    }
}

pub struct WorkspaceClient<'a> {
    client: &'a Client,
    workspace_id: String,
}

impl<'a> WorkspaceClient<'a> {
    pub fn id(&self) -> &str {
        &self.workspace_id
    }

    pub async fn upload_program(
        &self,
        program: &Program,
        alias: Option<String>,
    ) -> Result<ProgramResponse> {
        upload::upload_program(self.client, &self.workspace_id, program, alias).await
    }

    pub async fn upload_dataset(
        &self,
        batches: Vec<RecordBatch>,
        alias: Option<String>,
    ) -> Result<DatasetResponse> {
        datasets::upload::upload_dataset(self.client, &self.workspace_id, batches, alias).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
struct CreateWorkspaceRequest {
    name: String,
}
