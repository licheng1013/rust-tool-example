use std::sync::Arc;
use actix_web::*;
use rbatis::RBatis;
use rbdc_mysql::driver::MysqlDriver;
use common::config::config::AppConfig;

mod api;
mod model;
mod logic;
mod plugin;
mod util;


#[actix_web::main]
async fn main() -> std::io::Result<()> {


    let config = AppConfig::new();
    println!("{:?}", config);
    let host = "127.0.0.1:".to_string() + config.port.to_string().as_str();
    println!("http://{}", host);

    let rb = RBatis::new();
    rb.init(MysqlDriver {}, config.mysql_url.as_str()).unwrap();
    let rb = Arc::new(rb);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(rb.to_owned())) //mysql
            .configure(api::admin_api::admin_api)
    })
        .bind(("0.0.0.0", config.port))?
        .run()
        .await
}


