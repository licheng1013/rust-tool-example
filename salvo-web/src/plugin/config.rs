use salvo::prelude::*;

#[handler]
pub async fn plugin(_req: &mut Request, _depot: &mut Depot) {
    // 打印
    println!("Hello Plugin");

}