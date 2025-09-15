use crate::api::api_error::ApiError;
use crate::dto::user_dto::{UserCreateDto, UserGetDto};
use crate::entities::users::{
  ActiveModel as UserActiveModel, Column as UserColumn, Entity as UserEntity, Model as UserModel,
};
use bcrypt::{hash, DEFAULT_COST};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

impl From<UserModel> for UserGetDto {
  fn from(user: UserModel) -> Self {
    Self {
      id: user.id.to_string(),
      slug: user.slug,
      email: user.email,
      username: user.username,
    }
  }
}

pub async fn get_user_by_id(conn: &DatabaseConnection, id: Uuid) -> Result<UserGetDto, ApiError> {
  let user = UserEntity::find_by_id(id)
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

  Ok(user.into())
}

pub async fn get_user_by_slug(
  conn: &DatabaseConnection,
  slug: &str,
) -> Result<UserGetDto, ApiError> {
  let user = UserEntity::find()
    .filter(UserColumn::Slug.eq(slug.to_string()))
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

  Ok(user.into())
}

pub async fn create_user(
  conn: &DatabaseConnection,
  req: UserCreateDto,
) -> Result<UserGetDto, ApiError> {
  // validate fields

  // check uniqueness

  // Hash password
  let pepper = std::env::var("PASSWORD_SUFFIX").map_err(|e| {
    ApiError::InternalError(anyhow::anyhow!(
      "Failed to read pepper during password hashing: {}",
      e
    ))
  })?;
  let new_pwd = format!("{}{}", req.password, pepper);
  let password_hash = hash(new_pwd.as_bytes(), DEFAULT_COST)
    .map_err(|e| ApiError::InternalError(anyhow::anyhow!("Failed to hash password: {}", e)))?;

  // generate slug

  let active = UserActiveModel {
    id: Set(Uuid::new_v4()),
    slug: Set(req.username.clone()), // For now, just copy username
    email: Set(req.email),
    password: Set(password_hash),
    username: Set(req.username),
    ..Default::default()
  };

  let inserted = active
    .insert(conn)
    .await
    .map_err(|e| ApiError::InternalError(anyhow::anyhow!(e)))?;

  Ok(inserted.into())
}
