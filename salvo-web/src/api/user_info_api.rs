use rbatis::sql::PageRequest;
use salvo::prelude::*;
use crate::logic;
use crate::model::user_info::{UserInfo, UserInfoDto};
use common::util::page::PageParam;
use common::util::result::{ok_data, ok_msg};

#[handler]
async fn list(_req: &mut Request, res: &mut Response) {
    let model: UserInfoDto = _req.parse_queries().unwrap();
    let page: PageParam = _req.parse_queries().unwrap();
    let data = logic::user_info_logic::list(page, UserInfo::from(model)).await;
    res.render(Json(ok_data(&data)));
}

#[handler]
async fn update(_req: &mut Request, res: &mut Response) {
    let model: UserInfoDto = _req.parse_json().await.unwrap();
    logic::user_info_logic::update(UserInfo::from(model)).await;
    res.render(Json(ok_data("修改成功!")));
}

#[handler]
async fn delete(_req: &mut Request, res: &mut Response) {
    let model: Vec<i64> = _req.parse_json().await.unwrap();
    logic::user_info_logic::delete(model).await;
    res.render(Json(ok_data("删除成功!")));
}

#[handler]
async fn insert(_req: &mut Request, res: &mut Response) {
    let model: UserInfoDto = _req.parse_json().await.unwrap();
    logic::user_info_logic::insert(UserInfo::from(model)).await;
    res.render(Json(ok_data("插入成功!")));
}

pub fn router() -> Router {
    Router::with_path("/user/info")
        .push(Router::with_path("/list").get(list))
        .push(Router::with_path("/update").post(update))
        .push(Router::with_path("/delete").post(delete))
        .push(Router::with_path("/insert").post(insert))
}