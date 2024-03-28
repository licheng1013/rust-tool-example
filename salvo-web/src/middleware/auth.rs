use salvo::prelude::*;

use common::util::jwt;

use crate::{APP_CONFIG, logic};
use crate::model::admin::Admin;
use crate::util::result::fail;

#[handler]
pub async fn plugin(_req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    let path = _req.uri().path();
    let method = _req.method().as_str();
    // 打印请求路径
    println!("[RustTool] {} - \"{}\"", method, path);

    if APP_CONFIG.is_exclude(&path) {
        return;
    }
    let token = _req.header(jwt::TOKEN);
    if token.is_none() {
        res.render(Json(fail("未认证".to_string())));
        ctrl.cease(); // 跳过所有处理
        return;
    }
    let token = token.unwrap();
    let id = jwt::JwtUtil::id(token);
    // 查询用户信息
    let user = logic::admin_logic::get(id).await;
    match user {
        None => {
            res.render(Json(fail("用户不存在".to_string())));
            ctrl.cease(); // 跳过所有处理
            return;
        }
        Some(a) => {
            println!("认证查询: {:?}", a.id);
            save_ctx(a, depot)
        }
    }
}

static SAVE_KEY: &str = "SAVE_KEY";

pub fn save_ctx(a: Admin, depot: &mut Depot) {
    // 插入数据
    depot.insert(SAVE_KEY, a);
}

pub fn get_ctx(depot: &mut Depot) -> Admin {
    depot.get::<Admin>(SAVE_KEY).unwrap().clone()
}

