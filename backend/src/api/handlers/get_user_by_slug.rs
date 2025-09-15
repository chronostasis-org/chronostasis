use crate::api::api_error::ApiError;
use crate::api::router::AppState;
use crate::dto::user_dto::UserGetDto;
use crate::services::user_service;
use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

pub async fn get_user_by_slug(
  State(state): State<AppState>,
  Path(id): Path<Uuid>,
) -> Result<Json<UserGetDto>, ApiError> {
  let dto = user_service::get_user_by_id(&state.db.conn, id).await?;
  Ok(Json(dto))
}
