use std::sync::Arc;
use actix_web::*;
use actix_web::web::*;
use rbatis::RBatis;
use common::util::page::PageParam;

use crate::{get, logic, post};
use crate::model::admin::Admin;
use crate::util::result::{ok_data, ok_msg};

pub fn admin_api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/admin")
            .service(list)
            .service(update)
            .service(delete)
            .service(insert)
    );
}

/// 列表
#[get("/list")]
async fn list(model: Query<Admin>,page:Query<PageParam>, rb: Data<Arc<RBatis>>) -> Result<impl Responder> {
    println!("收到数据: {:#?}, {:#?}", model,page);
    Ok(Json(ok_data(logic::admin_logic::list(page.into_inner(),model.into_inner(),rb).await)))
}

/// 修改
#[post ("/update")]
async fn update(model: Json<Admin>, rb: Data<Arc<RBatis>>) -> Result<impl Responder> {
    logic::admin_logic::update(model.into_inner(),rb).await;
    Ok(Json(ok_msg("修改成功".to_string())))
}

/// 删除
#[post("/delete")]
async fn delete(model: Json<Admin>, rb: Data<Arc<RBatis>>) -> Result<impl Responder> {
    logic::admin_logic::delete(model.into_inner(),rb).await;
    Ok(Json(ok_msg("删除成功".to_string())))
}

/// 插入
#[post("/insert")]
async fn insert(model: Json<Admin>, rb: Data<Arc<RBatis>>) -> Result<impl Responder> {
    logic::admin_logic::insert(model.into_inner(),rb).await;
    Ok(Json(ok_msg("插入成功".to_string())))
}
