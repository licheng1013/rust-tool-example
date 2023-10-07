use salvo::prelude::*;

#[handler]
pub async fn plugin(_req: &mut Request, _depot: &mut Depot,res: &mut Response) {
    res.render("Hello world");
}