use salvo::prelude::*;
use crate::logic;
use crate::model::user_info::{UserInfo, UserInfoDto};
use common::util::page::{PageParam, PageResult};
use crate::middleware::error::AppResult;
use crate::util::result::JsonResult;

#[handler]
async fn list(_req: &mut Request) -> AppResult<JsonResult<PageResult<Vec<UserInfoDto>>>> {
    let model: UserInfoDto = _req.parse_queries().unwrap();
    let page: PageParam = _req.parse_queries().unwrap();
    return logic::user_info_logic::list(page, UserInfo::from(model)).await;
}

#[handler]
async fn update(_req: &mut Request) -> AppResult<JsonResult<()>> {
    let model: UserInfoDto = _req.parse_json().await.unwrap();
    return logic::user_info_logic::update(UserInfo::from(model)).await;
}

#[handler]
async fn delete(_req: &mut Request) -> AppResult<JsonResult<()>> {
    let model: UserInfoDto = _req.parse_json().await.unwrap();
    return logic::user_info_logic::delete(UserInfo::from(model)).await;
}

#[handler]
async fn insert(_req: &mut Request) -> AppResult<JsonResult<()>> {
    let model: UserInfoDto = _req.parse_json().await.unwrap();
    return logic::user_info_logic::insert(UserInfo::from(model)).await;
}

pub fn router() -> Router {
    Router::with_path("/user/info")
        .push(Router::with_path("/list").get(list))
        .push(Router::with_path("/update").post(update))
        .push(Router::with_path("/delete").post(delete))
        .push(Router::with_path("/insert").post(insert))
}