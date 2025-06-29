use axum::{Json, routing::post, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EchoRequest {
    pub message: String,
}

pub async fn echo_handler(Json(payload): Json<EchoRequest>) -> Json<EchoRequest> {
    Json(payload)
}

pub fn echo_router() -> Router {
    Router::new().route("/echo", post(echo_handler))
}