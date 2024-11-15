use axum::{middleware, Router};
use axum::{
    response::Response,
    middleware::Next,
    extract::Request,
};
mod api;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(api::goods_api::router())
        .route_layer(middleware::from_fn(my_middleware));
    println!("Server start: http://localhost:8000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn my_middleware(
    req: Request,
    next: Next,
) -> Response {
    println!("path: {:#?}", req.uri().path());
    let start_time = std::time::Instant::now();
    // do something with `request`...
    let response = next.run(req).await;
    // do something with `response`...
    let end_time = std::time::Instant::now();
    println!("Request time: {} ms", end_time.duration_since(start_time).as_millis());
    response
}