pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20260227_115403_fill_kind_of_fabric;
mod m20260401_072415_add_fabric_length_to_fabric_x_project;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20260227_115403_fill_kind_of_fabric::Migration),
            Box::new(m20260401_072415_add_fabric_length_to_fabric_x_project::Migration),
        ]
    }
}
