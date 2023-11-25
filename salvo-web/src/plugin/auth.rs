use salvo::prelude::*;
use common::util::jwt;
use common::util::result::fail;
use common::util::thread::set_user_id;
use crate::{APP_CONFIG, logic};

#[handler]
pub async fn plugin(_req: &mut Request, _depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    let path = _req.uri().path();
    //println!("request path: {}", path);
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
            println!("认证查询: {:?}", a);
            set_user_id(a.id.unwrap())
        }
    }
}
