use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("api key must not be empty")]
    EmptyApiKey,
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("api request failed with status {status}: {body}")]
    Api { status: StatusCode, body: String },
}
