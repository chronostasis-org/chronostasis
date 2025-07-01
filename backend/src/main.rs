use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

#[tokio::main]
async fn main() {
    server().await;
}

async fn server() {
    let app: Router = Router::new()
        .route("/api/test", get(test));

    let listener = tokio::net::TcpListener::bind("localhost:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn test() -> impl IntoResponse {
    println!("Test Api");

    (StatusCode::ACCEPTED, "Works!")
}