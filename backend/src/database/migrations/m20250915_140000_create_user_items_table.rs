use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserItems::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserItems::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserItems::ItemId).integer().not_null())
                    .col(ColumnDef::new(UserItems::AcquiredAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(UserItems::Count).integer().not_null().default(1))
                    .primary_key(Index::create().col(UserItems::UserId).col(UserItems::ItemId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_items_user")
                            .from_tbl(UserItems::Table)
                            .from_col(UserItems::UserId)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_items_item")
                            .from_tbl(UserItems::Table)
                            .from_col(UserItems::ItemId)
                            .to_tbl(Items::Table)
                            .to_col(Items::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(UserItems::Table).to_owned()).await
    }
}

#[derive(Iden)]
enum UserItems {
    Table,
    UserId,
    ItemId,
    AcquiredAt,
    Count,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}

#[derive(Iden)]
enum Items {
    Table,
    Id,
}
