use axum::Router;
use axum::routing::{get, post};

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
        .route("/user/list",get(list))
        .route("/user/update",post(update))
        .route("/user/delete",post(delete))
        .route("/user/insert",post(insert))
}


