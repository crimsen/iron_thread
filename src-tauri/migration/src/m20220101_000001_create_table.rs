use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        //
        manager
            .create_table(
                Table::create()
                    .table(Fabric::Table)
                    .if_not_exists()
                    .col(pk_auto(Fabric::Id))
                    .col(string_null(Fabric::Name))
                    .col(float_null(Fabric::Length))
                    .col(float_null(Fabric::Width))
                    .col(float_null(Fabric::Costs))
                    .col(string_null(Fabric::FotoPath))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(pk_auto(Project::Id))
                    .col(string(Project::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(FabricXProject::Table)
                    .if_not_exists()
                    .col(integer(FabricXProject::FabricId))
                    .col(integer(FabricXProject::ProjectId))
                    .primary_key(
                        Index::create()
                            .name("pk-fabric_x_project")
                            .col(FabricXProject::FabricId)
                            .col(FabricXProject::ProjectId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-fabric-id")
                            .from(
                                FabricXProject::Table,
                                FabricXProject::FabricId,
                            )
                            .to(Fabric::Table, Fabric::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-project-id")
                            .from(
                                FabricXProject::Table,
                                FabricXProject::ProjectId,
                            )
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await?;
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Fabric::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(FabricXProject::Table).to_owned())
            .await

        // manager
        //     .drop_table(Table::drop().table(Post::Table).to_owned())
        //     .await
    }
}

#[derive(DeriveIden)]
enum Fabric {
    Table,
    Id,
    Name,
    Length,
    Width,
    Costs,
    FotoPath,
}

#[derive(DeriveIden)]
enum Project {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum FabricXProject {
    Table,
    FabricId,
    ProjectId,
}
