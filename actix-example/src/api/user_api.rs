use actix_web::web::*;
use actix_web::*;
use serde::{Deserialize, Serialize};

pub fn admin_api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/user")
            .service(list)
            .service(update)
            .service(delete)
            .service(insert),
    );
}
#[get("/list")]
async fn list(model: Query<Admin>) -> Result<impl Responder> {
    Ok(Json(model.into_inner()))
}
#[post("/update")]
async fn update(model: Json<Admin>) -> Result<impl Responder> {
    Ok(Json(model))
}
#[post("/delete")]
async fn delete(model: Json<Admin>) -> Result<impl Responder> {
    Ok(Json(model))
}
#[post("/insert")]
async fn insert(model: Json<Admin>) -> Result<impl Responder> {
    Ok(Json(model))
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Admin {
    /// 账号
    pub user_name: Option<String>,
    /// 密码
    pub password: Option<String>,
}
