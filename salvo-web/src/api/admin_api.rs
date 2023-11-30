use salvo::prelude::*;
use serde_json::{Map, Value};
use validator::{Validate, ValidationErrors};
use crate::logic;
use crate::model::admin::{Admin, AdminDto};
use common::util::page::{PageParam, PageResult};
use common::util::result::{fail, ok_data, ok_msg};
use crate::middleware::auth::get_ctx;
use crate::middleware::error::AppResult;
use crate::util::result::JsonResult;

///
/// @Param(size=2,user_name=admin)
///
#[handler]
async fn list(_req: &mut Request, res: &mut Response) -> AppResult<JsonResult<PageResult<Vec<AdminDto>>>> {
    let model: AdminDto = _req.parse_queries().unwrap();
    let page: PageParam = _req.parse_queries().unwrap();
    return logic::admin_logic::list(page, Admin::from(model)).await;
}


#[handler]
async fn update(_req: &mut Request, res: &mut Response) -> AppResult<JsonResult<()>> {
    let model: AdminDto = _req.parse_json().await.unwrap();
    return logic::admin_logic::update(Admin::from(model)).await;
}

#[handler]
async fn delete(_req: &mut Request, res: &mut Response) -> AppResult<JsonResult<()>> {
    let model: AdminDto = _req.parse_json().await.unwrap();
    return logic::admin_logic::delete(model).await;
}

#[handler]
async fn insert(_req: &mut Request, res: &mut Response) -> AppResult<JsonResult<()>> {
    let model: AdminDto = _req.parse_json().await.unwrap();
    return logic::admin_logic::insert(Admin::from(model)).await;
}


/// @Param(user_name=admin,password=123456)
#[handler]
async fn login(_req: &mut Request, res: &mut Response) -> AppResult<JsonResult<Map<String, Value>>> {
    let model: AdminDto = _req.parse_json().await.unwrap();
    let map = logic::admin_logic::login(Admin::from(model)).await;
    return map;
}


#[handler]
async fn user_info(_req: &mut Request, res: &mut Response, depot: &mut Depot) -> AppResult<JsonResult<Map<String, Value>>> {
    let admin = get_ctx(depot);
    let map = logic::admin_logic::user_info(admin).await;
    return map;
}

pub fn router() -> Router {
    Router::with_path("/admin")
        .push(Router::with_path("/list").get(list))
        .push(Router::with_path("/update").post(update))
        .push(Router::with_path("/delete").post(delete))
        .push(Router::with_path("/insert").post(insert))
        .push(Router::with_path("/userInfo").get(user_info)) // 获取用户信息
        .push(Router::with_path("/login").post(login)) // 登录
}