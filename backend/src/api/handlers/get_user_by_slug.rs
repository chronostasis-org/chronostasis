use std::sync::Arc;
use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use crate::database::Db;
use crate::services::user_service;

pub async fn get_user_by_slug(
    State(db): State<&Db>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match user_service::get_user_by_slug(db, slug).await {
        Ok(Some(user)) => Json(user).into_response(),
        Ok(None) => (axum::http::StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => {
            log::error!("Database error: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        },
    }
}
