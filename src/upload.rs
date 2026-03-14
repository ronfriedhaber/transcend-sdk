use arrow_array::RecordBatch;
use arrow_ipc::writer::StreamWriter;
use reqwest::header;
use serde::{Deserialize, Serialize};

use crate::{Result, client::Client, error::Error};

impl Client {
    pub async fn upload_dataset(
        &self,
        batches: Vec<RecordBatch>,
        alias: Option<String>,
        author: Option<String>,
    ) -> Result<DatasetResponse> {
        let payload = encode_record_batches_ipc(&batches)?;
        let query = dataset_upload_query(alias, author);

        self.http_json_v1(reqwest::Method::POST, "/datasets", |request| {
            request
                .query(&query)
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .body(payload)
        })
        .await
    }
}

fn dataset_upload_query(
    alias: Option<String>,
    author: Option<String>,
) -> Vec<(&'static str, String)> {
    let mut query = vec![("format", "ipc".to_string())];

    if let Some(alias) = alias {
        query.push(("alias", alias));
    }

    if let Some(author) = author {
        query.push(("author", author));
    }

    query
}

fn encode_record_batches_ipc(batches: &[RecordBatch]) -> Result<Vec<u8>> {
    let schema = batches
        .first()
        .ok_or(Error::EmptyRecordBatches)?
        .schema();
    let mut payload = Vec::new();
    let mut writer = StreamWriter::try_new(&mut payload, &schema)?;

    for batch in batches {
        writer.write(batch)?;
    }

    writer.finish()?;

    Ok(payload)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetResponse {
    pub id: String,
    pub size_bytes: u64,
    pub chunk_count: u64,
    pub alias: Option<String>,
}
