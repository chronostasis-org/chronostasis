use crate::modules::users::enums::user_status::UserStatus;
use sea_orm::entity::prelude::*;
use crate::modules::users::enums::UserRole;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub slug: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub status: UserStatus,
    pub role: UserRole,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
