use serde::{Serialize};

#[derive(Serialize)]
pub(crate) struct Resp<T>
    where T: Serialize {
    is_success: bool,
    code: u8,
    data: Option<T>,
    message: String,
}

impl<T> Default for Resp<T>
    where T: Serialize {
    fn default() -> Self {
        Self {
            is_success: true,
            code: 0,
            data: None,
            message: String::from("ok"),
        }
    }
}

pub(crate) enum ErrorArgs {
    Arg(String, u8)
}

impl From<String> for ErrorArgs {
    fn from(message: String) -> Self {
        Self::Arg(message, 1)
    }
}

impl From<(String, u8)> for ErrorArgs {
    fn from((message, code): (String, u8)) -> Self {
        Self::Arg(message, code)
    }
}

impl<T> Resp<T>
    where T: Serialize {
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            ..Self::default()
        }
    }
    pub fn error(args: ErrorArgs) -> Self {
        let (message, code) = match args { ErrorArgs::Arg(message, code) => { (message, code) } };
        Self {
            is_success: false,
            code,
            message,
            ..Self::default()
        }
    }
}