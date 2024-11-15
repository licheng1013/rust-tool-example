use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::{from_fn, Next},
    App, Error, HttpServer,
};
use common::config::config::AppConfig;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::new();
    println!("{:?}", config);
    let host = "127.0.0.1:".to_string() + config.port.to_string().as_str();
    println!("http://{}", host);

    HttpServer::new(move || {
        App::new()
            .wrap(from_fn(my_middleware))
            .configure(api::user_api::admin_api)
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}

async fn my_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    println!("path: {:#?}", req.path());
    let start_time = std::time::Instant::now();
    // pre-processing
    let res = next.call(req).await;
    // post-processing
    let end_time = std::time::Instant::now();
    println!("Request time: {} ms", end_time.duration_since(start_time).as_millis());
    res
}
