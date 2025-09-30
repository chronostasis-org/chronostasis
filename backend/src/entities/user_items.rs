use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;
use super::items;
use super::users;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub item_id: i32,
    pub acquired_at: DateTimeWithTimeZone,
    pub count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::users::Entity", from = "Column::UserId", to = "super::users::Column::Id")]
    User,
    #[sea_orm(belongs_to = "super::items::Entity", from = "Column::ItemId", to = "super::items::Column::Id")]
    Item,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Item.def()
    }
}

impl Related<users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
