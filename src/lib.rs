pub mod arrow;
pub mod client;
pub mod datasets;
pub mod error;
pub mod programs;
pub mod runs;
pub mod workspaces;

pub type Result<T> = std::result::Result<T, crate::error::Error>;
