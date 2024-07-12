use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenAiClientError {
    #[error("HTTP request failed")]
    HttpRequestFailed(#[from] http::Error),

    #[error("sending request failed")]
    SendRequestFailed(#[from] ReqwestError),

    #[error("parsing response failed")]
    ParseResponseFailed(#[from] SerdeError),

    #[error("invalid model id")]
    InvalidModelId,

    #[error("custom error: {0}")]
    Custom(String),
}
