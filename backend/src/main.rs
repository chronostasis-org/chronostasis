// src/main.rs
mod routes;

use axum::{
    routing::get,
    Router,
};

async fn hello_handler() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello_handler))
        .merge(routes::echo::echo_router())
        .merge(routes::login::login_router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}