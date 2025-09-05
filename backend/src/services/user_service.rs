use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use crate::database::Db;
use crate::entities::users;
use crate::dto::user_get_dto::UserGetDto;
use uuid::Uuid;


pub async fn get_user_by_slug(db: &Db, slug: String) -> Result<Option<UserGetDto>, sea_orm::DbErr> {
    match users::Entity::find()
        .filter(users::Column::Slug.eq(slug.clone()))
        .one(&db.conn)
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

pub async fn get_user_by_id(db: &Db, id: Uuid) -> Result<Option<UserGetDto>, sea_orm::DbErr> {
    match users::Entity::find_by_id(id)
        .one(&db.conn)
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

