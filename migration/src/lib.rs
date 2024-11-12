pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20241111_052418_user;
mod m20241112_074049_fruit;
mod m20241112_074142_cake;
mod m20241112_074225_filling;
mod m20241112_074314_cake_filling;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20241111_052418_user::Migration),
            Box::new(m20241112_074049_fruit::Migration),
            Box::new(m20241112_074142_cake::Migration),
            Box::new(m20241112_074225_filling::Migration),
            Box::new(m20241112_074314_cake_filling::Migration),
        ]
    }
}
