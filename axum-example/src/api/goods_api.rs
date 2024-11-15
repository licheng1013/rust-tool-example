use axum::routing::{get, post};
use axum::Router;

async fn list() -> &'static str {
    "list!"
}

async fn update() -> &'static str {
    "update!"
}

async fn delete() -> &'static str {
    "delete!"
}

async fn insert() -> &'static str {
    "insert!"
}

pub fn router() -> Router {
    Router::new()
        .route("/goods/list", get(list))
        .route("/goods/update", post(update))
        .route("/goods/delete", post(delete))
        .route("/goods/insert", post(insert))
}