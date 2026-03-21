pub mod list;
pub mod read;
pub mod upload;

use serde::{Deserialize, Serialize};

pub use upload::DatasetResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetMetadata {
    pub id: String,
    pub workspace_id: String,
    pub owner: String,
    pub alias: String,
    pub byte_size: i64,
    pub object_key: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetSchema {
    pub fields: Vec<DatasetSchemaField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetSchemaField {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
}
