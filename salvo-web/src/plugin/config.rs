use salvo::prelude::*;

#[handler]
pub async fn plugin(_req: &mut Request, _depot: &mut Depot,res: &mut Response,ctrl: &mut FlowCtrl) {
    let path = _req.uri().path();
    println!("plugin path: {}", path);
    if path == "/admin/list" {
        res.render("Hello world");
        ctrl.cease(); // 跳过所有处理
    }
}