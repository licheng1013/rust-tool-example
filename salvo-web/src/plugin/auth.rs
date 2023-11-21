use salvo::prelude::*;
use common::util::jwt;
use crate::{APP_CONFIG, logic};

#[handler]
pub async fn plugin(_req: &mut Request, _depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    let path = _req.uri().path();
    println!("request path: {}", path);
    if APP_CONFIG.is_exclude(&path) {
        return;
    }
    let token = _req.header(jwt::TOKEN);
    if token.is_none() {
        res.render("需要认证");
        ctrl.cease(); // 跳过所有处理
        return;
    }
    let token = token.unwrap();
    let id = jwt::JwtUtil::id(token);
    // 查询用户信息
    let user = logic::admin_logic::get(id).await;
    println!("{:?}", user);
}
