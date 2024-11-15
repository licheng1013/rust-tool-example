use salvo::{async_trait, Depot, Request, Response, Writer};
use salvo::prelude::Text::Json;

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        match self {
            AppError::Service { error_info } => {
                res.render(Json(fail(error_info).to_string()));
            },
        }
    }
}


use thiserror::Error;
use crate::util::result::fail;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{error_info:?}")]
    Service { error_info: String },
}

pub type AppResult<T> = Result<T, AppError>;
