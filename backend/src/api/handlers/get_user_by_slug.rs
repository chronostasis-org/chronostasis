use crate::api::router::AppState;
use crate::services::user_service;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;

pub async fn get_user_by_slug(
  State(state): State<AppState>,
  Path(slug): Path<String>,
) -> impl IntoResponse {
  match user_service::get_user_by_slug(&state.db.conn, slug).await {
    Ok(Some(user)) => Json(user).into_response(),
    Ok(None) => (axum::http::StatusCode::NOT_FOUND, "User not found").into_response(),
    Err(e) => {
      log::error!("Database error: {:?}", e);
      (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        "Database error",
      )
        .into_response()
    }
  }
}
