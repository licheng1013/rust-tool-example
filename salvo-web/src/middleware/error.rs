use salvo::{async_trait, Depot, Request, Response, Writer};
use salvo::writing::Text;


#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, depot: &mut Depot, res: &mut Response) {
        res.render(Text::Plain("I'm a error!"));
    }
}


use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{error_info:?}")]
    Service{error_info:String},
}

pub type AppResult<T> = Result<T, AppError>;