use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Fruit::Table)
                    .if_not_exists()
                    .col(pk_auto(Fruit::Id))
                    .col(string(Fruit::Name))
                    .col(integer(Fruit::CakeId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Fruit::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Fruit {
    Table,
    Id,
    Name,
    CakeId,
}
