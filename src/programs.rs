use serde::{Deserialize, Serialize};

pub mod upload;

pub use upload::ProgramResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramMetadata {
    pub id: String,
    pub workspace_id: String,
    pub owner: String,
    pub alias: String,
    pub byte_size: i64,
    pub object_key: String,
    pub created_at: String,
    pub updated_at: String,
}
