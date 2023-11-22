use salvo::prelude::*;
use crate::logic;
use crate::model::admin::Admin;
use common::util::page::PageParam;
use common::util::result::{ok_data, ok_msg};

///
/// @Param(size=2,user_name=admin)
///
#[handler]
async fn list(_req: &mut Request, res: &mut Response) {
    let model: Admin = _req.parse_queries().unwrap();
    let page: PageParam = _req.parse_queries().unwrap();
    let data = logic::admin_logic::list(page, model).await;
    res.render(Json(ok_data(&data)));
}

/// @Param(aa=aa)
///
#[handler]
async fn update(_req: &mut Request, res: &mut Response) {
    let model: Admin = _req.parse_json().await.unwrap();
    logic::admin_logic::update(model).await;
    res.render(Json(ok_msg("修改成功!".to_string())));
}

#[handler]
async fn delete(_req: &mut Request, res: &mut Response) {
    let model: Admin = _req.parse_json().await.unwrap();
    logic::admin_logic::delete(model).await;
    res.render(Json(ok_msg("删除成功!".to_string())));
}

#[handler]
async fn insert(_req: &mut Request, res: &mut Response) {
    let model: Admin = _req.parse_json().await.unwrap();
    logic::admin_logic::insert(model).await;
    res.render(Json(ok_msg("插入成功!".to_string())));
}


/// @Param(user_name=admin,password=123456)
///
#[handler]
async fn login(_req: &mut Request, res: &mut Response) {
    let model: Admin = _req.parse_json().await.unwrap();
    let map = logic::admin_logic::login(model).await;
    res.render(Json(ok_data(&map)));
}


#[handler]
async fn userInfo(_req: &mut Request, res: &mut Response) {
    let map = logic::admin_logic::user_info().await;
    res.render(Json(ok_data(&map)));
}



pub fn router() -> Router {
    Router::with_path("/admin")
        .push(Router::with_path("/list").get(list))
        .push(Router::with_path("/update").post(update))
        .push(Router::with_path("/delete").post(delete))
        .push(Router::with_path("/insert").post(insert))
        .push(Router::with_path("/userInfo").get(userInfo)) // 获取用户信息
        .push(Router::with_path("/login").post(login)) // 登录
}