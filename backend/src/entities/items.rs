use sea_orm::entity::prelude::*;
use crate::models::item_rarity::ItemRarity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub rarity: ItemRarity,
    pub x_coordinate: i32,
    pub y_coordinate: i32,
    pub total: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_items::Entity")]
    UserItems,
}

impl ActiveModelBehavior for ActiveModel {}
