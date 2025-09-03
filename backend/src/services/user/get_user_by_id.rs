use std::sync::Arc;
use sea_orm::EntityTrait;
use crate::database::Db;
use crate::entities::users;
use uuid::Uuid;
use crate::dto::user_get_dto::UserGetDto;

pub async fn get_user_by_id_service(db: Arc<Db>, id: Uuid) -> Result<Option<UserGetDto>, sea_orm::DbErr> {
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

