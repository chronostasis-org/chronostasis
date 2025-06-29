// /src/routes/login.rs
use axum::{
    routing::post,
    Router, Json,
    response::IntoResponse,
    http::StatusCode,
};
use serde::Deserialize;

const USERNAME: &str = "admin";
const PASSWORD: &str = "password";

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn login_handler(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    if payload.username == USERNAME && payload.password == PASSWORD {
        (StatusCode::OK, "Login successful")
    } else {
        (StatusCode::UNAUTHORIZED, "Invalid credentials")
    }
}

pub fn login_router() -> Router {
    Router::new().route("/login", post(login_handler))
}