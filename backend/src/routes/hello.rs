use axum::{
    routing::get,
    Router,
};

pub async fn hello_handler() -> &'static str {
    "Hello, world!"
}

pub fn hello_router() -> Router {
    Router::new().route("/hello", get(hello_handler))
}