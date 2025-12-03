use thiserror::Error;

#[derive(Error, Debug)]
pub enum AiError {
    #[error("Model load failed: {0}")]
    ModelLoadFailed(String),
    #[error("Inference failed: {0}")]
    InferenceFailed(String),
    #[error("Tokenization failed: {0}")]
    TokenizationFailed(String),
    #[error("Context window exceeded")]
    ContextWindowExceeded,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Candle error: {0}")]
    CandleError(#[from] candle_core::Error),
    #[error("Unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, AiError>;
