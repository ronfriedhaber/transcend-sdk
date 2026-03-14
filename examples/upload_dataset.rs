use std::sync::Arc;

use arrow_array::{Int64Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};

mod common;

#[tokio::main]
async fn main() -> transcend_sdk::Result<()> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("name", DataType::Utf8, false),
    ]));

    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(Int64Array::from(vec![1, 2, 3])),
            Arc::new(StringArray::from(vec!["alpha", "beta", "gamma"])),
        ],
    )?;

    let client = common::client_from_env_default()?;
    let response = client
        .upload_dataset(
            vec![batch],
            Some("example-upload".to_string()),
            Some("transcend-sdk-example".to_string()),
        )
        .await?;

    println!("{response:#?}");

    Ok(())
}
