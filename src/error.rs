use thiserror::Error as ThisError;
use reqwest::Error as ReqError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Request error: {0}")]
    RequestError(#[from] ReqError),
    #[error("Unknown api error")]
    Unknown,
}

