use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config parse error: {0}")]
    ConfigParse(#[from] serde_json::Error),

    #[error("Config file missing username")]
    MissingUsername,

    #[error("Decode error: {0}")]
    Decode(#[from] base64::DecodeError),

    #[error("Home directory not found")]
    HomeNotFound
}