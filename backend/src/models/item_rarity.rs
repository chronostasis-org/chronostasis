use sea_orm::entity::prelude::*;
use sea_orm::EnumIter;
use sea_orm::DeriveActiveEnum;

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "item_rarity")]
pub enum ItemRarity {
    #[sea_orm(string_value = "common")]
    Common,
    #[sea_orm(string_value = "uncommon")]
    Uncommon,
    #[sea_orm(string_value = "rare")]
    Rare,
    #[sea_orm(string_value = "epic")]
    Epic,
    #[sea_orm(string_value = "legendary")]
    Legendary,
}
impl Default for ItemRarity {
    fn default() -> Self {
        Self::Common
    }
}

