use thiserror::Error;

/// 洛书错误枚举
#[derive(Error, Debug)]
pub enum LuoshuError {
    /// 任意类型错误
    #[error("Any: `{0}`")]
    Any(#[from] anyhow::Error),
    #[error("Parse: `{0}`")]
    /// 数据分析错误
    Parse(#[from] serde_json::error::Error),
    /// Io错误
    #[error("Io: `{0}`")]
    Io(#[from] std::io::Error),
}

/// 洛书Result
pub type LuoshuResult<T> = Result<T, LuoshuError>;
