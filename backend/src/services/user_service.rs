use crate::api::api_error::ApiError;
use crate::dto::user_dto::UserGetDto;
use crate::entities::users::{Column as UserColumn, Entity as UserEntity, Model as UserModel};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
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
