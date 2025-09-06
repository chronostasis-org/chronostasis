use crate::api::router::AppState;
use crate::services::asset_service;
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};

pub async fn get_spritesheet(State(state): State<AppState>) -> impl IntoResponse {
  match asset_service::get_spritesheet(&state.cfg).await {
    Ok(bytes) => Response::builder()
      .status(StatusCode::OK)
      .header(header::CONTENT_TYPE, "image/png")
      .header(header::CACHE_CONTROL, "public, max-age=31536000") // Cache for 1 year
      .body(bytes.into())
      .unwrap_or_else(|_| {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          "Failed to build response",
        )
          .into_response()
      }),
    Err(e) => {
      log::error!("Error serving spritesheet: {:?}", e);
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Failed to serve spritesheet",
      )
        .into_response()
    }
  }
}
