use arrow_array::RecordBatch;

use crate::{Result, client::Client};

pub(crate) async fn read_dataset_ipc(
    client: &Client,
    dataset_id: &str,
) -> Result<Vec<RecordBatch>> {
    let payload = client
        .http_bytes_v1(
            reqwest::Method::GET,
            &format!("/api/v1/datasets/{dataset_id}/read"),
            |request| request,
        )
        .await?;

    crate::arrow::decode_record_batches_ipc(payload)
}
