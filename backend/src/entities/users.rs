use crate::models::{UserRole, UserStatus};
use sea_orm::entity::prelude::*;

/// Users entity model
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub slug: String,
  #[sea_orm(unique)]
  pub email: String,
  pub password: String,
  pub username: String,
  pub status: UserStatus,
  pub role: UserRole,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: DateTimeWithTimeZone,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_items::Entity")]
    UserItems,
}

impl ActiveModelBehavior for ActiveModel {}
