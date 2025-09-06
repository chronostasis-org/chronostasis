use crate::database::Db;
use crate::dto::user_get_dto::UserGetDto;
use crate::entities::users;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn get_user_by_slug(
  conn: &DatabaseConnection,
  slug: String,
) -> Result<Option<UserGetDto>, sea_orm::DbErr> {
  match users::Entity::find()
    .filter(users::Column::Slug.eq(slug.clone()))
    .one(conn)
    .await
  {
    Ok(Some(user)) => Ok(Some(UserGetDto {
      id: user.id.to_string(),
      slug: user.slug,
      email: user.email,
      name: user.name,
    })),
    Ok(None) => Ok(None),
    Err(e) => Err(e),
  }
}

pub async fn get_user_by_id(db: Db, id: Uuid) -> Result<Option<UserGetDto>, sea_orm::DbErr> {
  match users::Entity::find_by_id(id).one(&db.conn).await {
    Ok(Some(user)) => Ok(Some(UserGetDto {
      id: user.id.to_string(),
      slug: user.slug,
      email: user.email,
      name: user.name,
    })),
    Ok(None) => Ok(None),
    Err(e) => Err(e),
  }
}
