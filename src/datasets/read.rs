use arrow_array::RecordBatch;
use serde::Serialize;
use serde_json::Value;

use crate::{Result, client::Client};

use super::DatasetMetadata;

impl Client {
    pub async fn dataset(&self, id_or_alias: &str) -> Result<DatasetMetadata> {
        self.http_json_v1(
            reqwest::Method::GET,
            &format!("/datasets/{id_or_alias}"),
            |request| request.query(&DatasetReadQuery::metadata()),
        )
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
            |request| request.query(&DatasetReadQuery::json(options)),
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
                |request| request.query(&DatasetReadQuery::ipc(options)),
            )
            .await?;

        crate::arrow::decode_record_batches_ipc(payload)
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum DatasetReadFormat {
    Metadata,
    Json,
    Ipc,
}

#[derive(Debug, Clone, Serialize)]
struct DatasetReadQuery {
    format: DatasetReadFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_rows: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_batches: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latest: Option<bool>,
}

impl DatasetReadQuery {
    fn metadata() -> Self {
        Self {
            format: DatasetReadFormat::Metadata,
            max_rows: None,
            max_batches: None,
            latest: None,
        }
    }

    fn json(options: DatasetJsonReadOptions) -> Self {
        Self {
            format: DatasetReadFormat::Json,
            max_rows: options.max_rows,
            max_batches: None,
            latest: None,
        }
    }

    fn ipc(options: DatasetIpcReadOptions) -> Self {
        Self {
            format: DatasetReadFormat::Ipc,
            max_rows: None,
            max_batches: options.max_batches,
            latest: options.latest,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DatasetJsonReadOptions {
    pub max_rows: Option<usize>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DatasetIpcReadOptions {
    pub max_batches: Option<usize>,
    pub latest: Option<bool>,
}
