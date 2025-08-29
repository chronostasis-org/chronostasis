pub mod enums;

use std::sync::Arc;
use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use serde::Serialize;
use crate::database::Db;
use crate::entities::users;
use uuid::Uuid;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub slug: String,
    pub email: String,
    pub name: String,
    // Add other fields as needed
}

//TODO delete email from response
pub async fn get_user_by_slug(
    State(db): State<Arc<Db>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match users::Entity::find()
        .filter(users::Column::Slug.eq(slug.clone()))
        .one(&db.conn)
        .await
    {
        Ok(Some(user)) => {
            let response = UserResponse {
                id: user.id.to_string(),
                slug: user.slug,
                email: user.email,
                name: user.name,
                // Add other fields as needed
            };
            Json(response).into_response()
        }
        Ok(None) => (axum::http::StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => {

            eprintln!("Database error: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        },
    }
}
//TODO delete email from response
pub async fn get_user_by_id(
    State(db): State<Arc<Db>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match users::Entity::find_by_id(id)
        .one(&db.conn)
        .await
    {
        Ok(Some(user)) => {
            let response = UserResponse {
                id: user.id.to_string(),
                slug: user.slug,
                email: user.email,
                name: user.name,
                // Add other fields as needed
            };
            Json(response).into_response()
        }
        Ok(None) => (axum::http::StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        },
    }
}
