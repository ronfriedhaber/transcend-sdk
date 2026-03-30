use autark_frontend::{Dataset, Query};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Result, client::Client, error::Error};

pub(crate) async fn upload_program(
    client: &Client,
    workspace_id: &str,
    query: &Query,
    alias: Option<String>,
) -> Result<ProgramResponse> {
    let program = serde_json::to_value(SerializableQuery::try_from_query(query)?)?;

    client
        .http_json_v1(reqwest::Method::POST, "/api/v1/programs", |request| {
            request.json(&UploadProgramRequest {
                workspace_id,
                program: &program,
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
    program: &'a Value,
    alias: Option<String>,
}

#[derive(Serialize)]
struct SerializableQuery<'a> {
    datasets: Vec<SerializableDataset<'a>>,
    sql: &'a str,
}

impl<'a> SerializableQuery<'a> {
    fn try_from_query(query: &'a Query) -> Result<Self> {
        Ok(Self {
            datasets: query
                .datasets()
                .iter()
                .map(SerializableDataset::try_from_dataset)
                .collect::<Result<Vec<_>>>()?,
            sql: query.sql(),
        })
    }
}

#[derive(Serialize)]
enum SerializableDataset<'a> {
    Csv {
        name: &'a str,
        path: &'a std::path::Path,
        has_header: bool,
    },
    Parquet {
        name: &'a str,
        path: &'a std::path::Path,
    },
    Transcend {
        name: &'a str,
        dataset_id: &'a str,
    },
}

impl<'a> SerializableDataset<'a> {
    fn from_dataset(dataset: &'a Dataset) -> Self {
        match dataset {
            Dataset::Csv {
                name,
                path,
                has_header,
            } => Self::Csv {
                name,
                path,
                has_header: *has_header,
            },
            Dataset::Parquet { name, path } => Self::Parquet { name, path },
            Dataset::Transcend { name, dataset_id } => Self::Transcend { name, dataset_id },
            Dataset::InMemory { .. } => {
                unreachable!("in-memory datasets are rejected before serialization")
            }
        }
    }

    fn try_from_dataset(dataset: &'a Dataset) -> Result<Self> {
        match dataset {
            Dataset::InMemory { .. } => Err(Error::UnsupportedQueryDataset("InMemory")),
            _ => Ok(Self::from_dataset(dataset)),
        }
    }
}
