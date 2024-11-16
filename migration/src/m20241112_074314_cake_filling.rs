use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20241112_074142_cake::Cake, m20241112_074225_filling::Filling};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CakeFilling::Table)
                    .if_not_exists()
                    .col(integer(CakeFilling::CakeId))
                    .col(integer(CakeFilling::FillingId))
                    .primary_key(
                        Index::create()
                            .col(CakeFilling::CakeId)
                            .col(CakeFilling::FillingId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cake_filling_cakeId")
                            .from(CakeFilling::Table, CakeFilling::CakeId)
                            .to(Cake::Table, Cake::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cake_fillilng_filling_id")
                            .from(CakeFilling::Table, CakeFilling::FillingId)
                            .to(Filling::Table, Filling::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CakeFilling::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CakeFilling {
    Table,
    CakeId,
    FillingId,
}
