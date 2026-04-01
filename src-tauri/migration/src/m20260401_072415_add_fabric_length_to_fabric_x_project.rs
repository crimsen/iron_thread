use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(FabricXProject::Table)
                    .add_column(
                        ColumnDef::new(FabricXProject::FabricLength)
                            .float()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(FabricXProject::Table)
                    .drop_column(FabricXProject::FabricLength)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum FabricXProject {
    Table,
    FabricLength,
}
