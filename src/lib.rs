pub mod client;
pub mod datasets;
pub mod error;

pub type Result<T> = std::result::Result<T, crate::error::Error>;
