use crate::api::api_error::ApiError;
use crate::dto::user_dto::{slug_from_username, UserCreateDto, UserGetDto, UserUpdateDto};
use crate::entities::users::{
  ActiveModel as UserActiveModel, Column as UserColumn, Entity as UserEntity, Model as UserModel,
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
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

fn hash_with_pepper(password: &str) -> Result<String, ApiError> {
  let pepper = std::env::var("PASSWORD_PEPPER").map_err(|e| {
    ApiError::InternalError(anyhow::anyhow!(
      "Failed to read pepper during password hashing: {}",
      e
    ))
  })?;
  let new_pwd = format!("{}{}", password, pepper);
  hash(new_pwd.as_bytes(), DEFAULT_COST)
    .map_err(|e| ApiError::InternalError(anyhow::anyhow!("Failed to hash password: {}", e)))
}

pub async fn get_user_by_id(conn: &DatabaseConnection, id: Uuid) -> Result<UserGetDto, ApiError> {
  let user = UserEntity::find_by_id(id)
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

  // hide soft-deleted
  if user.deleted_at.is_some() {
    return Err(ApiError::NotFound("User not found".to_string()));
  }

  Ok(user.into())
}

pub async fn get_user_by_slug(
  conn: &DatabaseConnection,
  slug: &str,
) -> Result<UserGetDto, ApiError> {
  let user = UserEntity::find()
    .filter(UserColumn::Slug.eq(slug.to_string()))
    .filter(UserColumn::DeletedAt.is_null())
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

  Ok(user.into())
}

pub async fn create_user(
  conn: &DatabaseConnection,
  mut req: UserCreateDto,
) -> Result<UserGetDto, ApiError> {
  // Normalize in service to guarantee consistent uniqueness checks
  req = req.normalize();

  // Compute slug from username (lowercase only).
  let slug = slug_from_username(&req.username);
  let email = req.email.clone();

  // Uniqueness check across ALL rows (including soft-deleted)
  if UserEntity::find()
    .filter(UserColumn::Slug.eq(slug.clone()))
    .one(conn)
    .await?
    .is_some()
  {
    return Err(ApiError::InvalidRequest("Username is already taken".into()));
  }
  if UserEntity::find()
    .filter(UserColumn::Email.eq(email.clone()))
    .one(conn)
    .await?
    .is_some()
  {
    return Err(ApiError::InvalidRequest("Email is already in use".into()));
  }

  // Hash password
  let password_hash = hash_with_pepper(&req.password)?;

  // Persist user (created_at/updated_at set by DB defaults)
  let active = UserActiveModel {
    id: Set(Uuid::new_v4()),
    slug: Set(slug),
    email: Set(email),
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

pub async fn update_user(
  conn: &DatabaseConnection,
  id: Uuid,
  mut req: UserUpdateDto,
) -> Result<UserGetDto, ApiError> {
  // Normalize optional fields in service
  req = req.normalize();

  // Load existing user (including soft-deleted, but reject below if deleted)
  let user = UserEntity::find_by_id(id)
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;
  if user.deleted_at.is_some() {
    return Err(ApiError::NotFound("User not found".to_string()));
  }

  let mut active: UserActiveModel = user.into();

  if let Some(username) = req.username {
    let new_slug = slug_from_username(&username);

    // Uniqueness across ALL rows, excluding self
    let slug_taken = UserEntity::find()
      .filter(UserColumn::Slug.eq(new_slug.clone()))
      .filter(UserColumn::Id.ne(id))
      .one(conn)
      .await?
      .is_some();
    if slug_taken {
      return Err(ApiError::InvalidRequest("Username is already taken".into()));
    }

    active.username = Set(username);
    active.slug = Set(new_slug);
  }

  if let Some(mut email) = req.email {
    email = email.trim().to_lowercase();

    let email_taken = UserEntity::find()
      .filter(UserColumn::Email.eq(email.clone()))
      .filter(UserColumn::Id.ne(id))
      .one(conn)
      .await?
      .is_some();
    if email_taken {
      return Err(ApiError::InvalidRequest("Email is already in use".into()));
    }

    active.email = Set(email);
  }

  // Hash password
  if let Some(password) = req.password {
    let password_hash = hash_with_pepper(&password)?;
    active.password = Set(password_hash);
  }

  // update updated_at
  active.updated_at = Set(Utc::now().into());

  let updated = active
    .update(conn)
    .await
    .map_err(|e| ApiError::InternalError(anyhow::anyhow!(e)))?;

  Ok(updated.into())
}

pub async fn delete_user_soft(conn: &DatabaseConnection, id: Uuid) -> Result<(), ApiError> {
  let user = UserEntity::find_by_id(id)
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

  // Idempotent
  if user.deleted_at.is_some() {
    return Ok(());
  }

  let now = Utc::now().into();

  let mut active: UserActiveModel = user.into();
  active.deleted_at = Set(Some(now));
  active.updated_at = Set(now);
  active
    .update(conn)
    .await
    .map_err(|e| ApiError::InternalError(anyhow::anyhow!(e)))?;
  Ok(())
}
