use axum::{extract::State, Json};
use validator::Validate;

use crate::api::api_error::ApiError;
use crate::api::router::AppState;
use crate::dto::user_dto::{UserCreateDto, UserGetDto};
use crate::services::user_service;

/// POST /users
/// Body: UserCreateDto
pub async fn create_user(
  State(state): State<AppState>,
  Json(payload): Json<UserCreateDto>,
) -> Result<Json<UserGetDto>, ApiError> {
  payload
    .validate()
    .map_err(|e| ApiError::InvalidRequest(format!("Validation error: {e}")))?;

  let payload = payload.normalize();

  let dto = user_service::create_user(&state.db.conn, payload).await?;
  Ok(Json(dto))
}
