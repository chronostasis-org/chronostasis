pub use sea_orm_migration::prelude::*;

mod m20240126114845_create_users_table;
mod m20250915_131425_create_items_table;
mod m20250915_140000_create_user_items_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
            Box::new(m20240126114845_create_users_table::Migration),
            Box::new(m20250915_131425_create_items_table::Migration),
            Box::new(m20250915_140000_create_user_items_table::Migration),
        ]
  }
}

