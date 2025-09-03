use std::sync::Arc;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use crate::database::Db;
use crate::entities::users;
use crate::dto::user_get_dto::UserGetDto;

pub async fn get_user_by_slug_service(db: Arc<Db>, slug: String) -> Result<Option<UserGetDto>, sea_orm::DbErr> {
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

