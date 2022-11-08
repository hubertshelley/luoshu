use thiserror::Error;

#[derive(Error, Debug)]
pub enum LuoshuError {
    #[error("Any: `{0}`")]
    Any(#[from] anyhow::Error),
    #[error("Parse: `{0}`")]
    Parse(#[from] serde_json::error::Error),
    #[error("Io: `{0}`")]
    Io(#[from] std::io::Error),
}

pub type LuoshuResult<T> = Result<T, LuoshuError>;
