pub use sea_orm_migration::prelude::*;

mod m20221025_000001_create_bakery_table;
mod m20221025_000002_create_chef_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221025_000001_create_bakery_table::Migration),
            Box::new(m20221025_000002_create_chef_table::Migration),
        ]
    }
}
