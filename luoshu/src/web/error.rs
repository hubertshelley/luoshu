use salvo::{Depot, Request, Response, Writer};
use salvo::prelude::Json;
use thiserror::Error;
use async_trait::async_trait;

use crate::web::resp::Resp;

#[derive(Error, Debug)]
pub(crate) enum AppError {
    #[error("web: `{0}`")]
    Salvo(#[from] salvo::Error),
    #[error("Parse: `{0}`")]
    Parse(#[from] salvo::http::ParseError),
    #[error("Parse: `{0}`")]
    Any(#[from] anyhow::Error),
}

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render(Json(Resp::<String>::error(format!("{:?}", self).into())));
    }
}

pub(crate) type WebResult<T> = Result<T, AppError>;