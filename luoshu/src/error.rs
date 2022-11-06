use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum AppError {
    #[error("Parse: `{0}`")]
    Any(#[from] anyhow::Error),
    #[error("Io: `{0}`")]
    Io(#[from] std::io::Error),
}

#[allow(dead_code)]
pub(crate) type LuoshuResult<T> = Result<T, AppError>;
