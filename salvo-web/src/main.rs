use std::ops::Deref;
use rbatis::RBatis;
use rbdc_mysql::driver::MysqlDriver;
use salvo::__private::once_cell::sync::Lazy;
use salvo::cors::Cors;
use salvo::catcher::Catcher;
use salvo::http::Method;
use salvo::prelude::*;
use common::config::config::AppConfig;

mod api;
pub(crate) mod logic;
mod model;
mod plugin;


pub static RB: Lazy<RBatis> = Lazy::new(RBatis::new);
pub static APP_CONFIG: Lazy<AppConfig> = Lazy::new(AppConfig::new);


#[tokio::main]
async fn main() {
    println!("{:?}", APP_CONFIG.deref());
    let host = "127.0.0.1:".to_string() + APP_CONFIG.port.to_string().as_str();
    // mysql connect info
    RB.init(MysqlDriver {}, APP_CONFIG.mysql_url.as_str()).unwrap();

    // 跨域
    let cors_handler = Cors::new()
        .allow_credentials(false)
        .allow_origin("*")
        .allow_headers("*")
        .allow_methods(vec![Method::GET, Method::POST,
                            Method::PUT, Method::DELETE, Method::OPTIONS])
        .max_age(3600)
        .into_handler();


    let acceptor = TcpListener::new(host.clone()).bind().await;
    let router = Router::new()
        .hoop(cors_handler.clone())
        .hoop(plugin::auth::plugin)
        .push(api::file_api::router())
        .push(api::admin_api::router())
        .options(handler::empty());

    println!("http://{}", host);
    let service = Service::new(router).catcher(Catcher::default().hoop(cors_handler));
    Server::new(acceptor).serve(service).await;
}