use arrow_array::RecordBatch;
use serde::Serialize;
use serde_json::Value;

use crate::{Result, client::Client};

use super::DatasetMetadata;

impl Client {
    pub async fn dataset(&self, id_or_alias: &str) -> Result<DatasetMetadata> {
        self.http_json_v1(reqwest::Method::GET, &format!("/datasets/{id_or_alias}"), |request| {
            request
        })
        .await
    }

    pub async fn dataset_json(
        &self,
        id_or_alias: &str,
        options: DatasetJsonReadOptions,
    ) -> Result<Vec<Value>> {
        self.http_json_v1(
            reqwest::Method::GET,
            &format!("/datasets/{id_or_alias}"),
            |request| request.query(&DatasetJsonReadQuery::from(options)),
        )
        .await
    }

    pub async fn dataset_ipc(
        &self,
        id_or_alias: &str,
        options: DatasetIpcReadOptions,
    ) -> Result<Vec<RecordBatch>> {
        let payload = self
            .http_bytes_v1(
                reqwest::Method::GET,
                &format!("/datasets/{id_or_alias}"),
                |request| request.query(&DatasetIpcReadQuery::from(options)),
            )
            .await?;

        crate::arrow::decode_record_batches_ipc(payload)
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
struct DatasetJsonReadQuery {
    format: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_rows: Option<usize>,
}

impl From<DatasetJsonReadOptions> for DatasetJsonReadQuery {
    fn from(options: DatasetJsonReadOptions) -> Self {
        Self {
            format: "json",
            max_rows: options.max_rows,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
struct DatasetIpcReadQuery {
    format: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_batches: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latest: Option<bool>,
}

impl From<DatasetIpcReadOptions> for DatasetIpcReadQuery {
    fn from(options: DatasetIpcReadOptions) -> Self {
        Self {
            format: "ipc",
            max_batches: options.max_batches,
            latest: options.latest,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DatasetJsonReadOptions {
    pub max_rows: Option<usize>,
}

impl DatasetJsonReadOptions {
    pub fn new(max_rows: Option<usize>) -> Self {
        Self { max_rows }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DatasetIpcReadOptions {
    pub max_batches: Option<usize>,
    pub latest: Option<bool>,
}

impl DatasetIpcReadOptions {
    pub fn new(max_batches: Option<usize>, latest: Option<bool>) -> Self {
        Self {
            max_batches,
            latest,
        }
    }
}
