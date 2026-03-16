pub mod arrow;
pub mod client;
pub mod datasets;
pub mod error;
pub mod metadata;
pub mod programs;

pub type Result<T> = std::result::Result<T, crate::error::Error>;
