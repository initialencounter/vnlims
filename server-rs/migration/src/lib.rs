pub use sea_orm_migration::prelude::*;

mod m20250413_095241_create;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250413_095241_create::Migration),
        ]
    }
}
