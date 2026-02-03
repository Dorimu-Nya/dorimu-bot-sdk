use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("signature verification failed")]
    InvalidSignature,
    #[error("missing header: {0}")]
    MissingHeader(&'static str),
    #[error("invalid header: {0}")]
    InvalidHeader(String),
    #[error("event name not found")]
    EventNameNotFound,
    #[error("token error: {0}")]
    Token(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("other error: {0}")]
    Other(String),
}
