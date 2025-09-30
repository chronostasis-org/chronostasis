use sea_orm::{ActiveEnum, DbBackend, Schema, Statement};
use sea_orm_migration::prelude::*;
use crate::models::item_rarity::ItemRarity;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(DbBackend::Postgres);
        let db = manager.get_connection();
        let enum_name = ItemRarity::name().to_string();
        let check_type = format!(
            "SELECT EXISTS (SELECT 1 FROM pg_type WHERE typname = '{}')",
            enum_name
        );
        let type_exists: bool = db
            .query_one(Statement::from_string(DbBackend::Postgres, check_type))
            .await?
            .map(|row| row.try_get::<bool>("", "exists").unwrap_or(false))
            .unwrap_or(false);
        if !type_exists {
            manager.create_type(schema.create_enum_from_active_enum::<ItemRarity>()).await?;
        }
        manager
            .create_table(
                Table::create()
                    .table(Items::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Items::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Items::Name).string().not_null())
                    .col(ColumnDef::new(Items::Rarity).custom(ItemRarity::name()).not_null())
                    .col(ColumnDef::new(Items::XCoordinate).integer().not_null().default(0))
                    .col(ColumnDef::new(Items::YCoordinate).integer().not_null().default(0))
                    .col(ColumnDef::new(Items::Total).integer().not_null().default(0))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Items::Table).to_owned()).await?;
        let db = manager.get_connection();
        let enum_name = ItemRarity::name().to_string();
        let check_type = format!(
            "SELECT EXISTS (SELECT 1 FROM pg_type WHERE typname = '{}')",
            enum_name
        );
        let type_exists: bool = db
            .query_one(Statement::from_string(DbBackend::Postgres, check_type))
            .await?
            .map(|row| row.try_get::<bool>("", "exists").unwrap_or(false))
            .unwrap_or(false);
        if type_exists {
            let drop_type = format!("DROP TYPE IF EXISTS {}", enum_name);
            manager.get_connection().execute(Statement::from_string(DbBackend::Postgres, drop_type)).await?;
        }
        Ok(())
    }
}

#[derive(Iden)]
enum Items {
    Table,
    Id,
    Name,
    Rarity,
    XCoordinate,
    YCoordinate,
    Total,
}
