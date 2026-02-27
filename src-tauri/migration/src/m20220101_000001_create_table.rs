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
                    .col(string_null(Fabric::Producer))
                    .col(integer_null(Fabric::KindOfFabricId))
                    .col(date_null(Fabric::DateOfPurchase))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-fabric-kind-of-fabric-id")
                            .from(Fabric::Table, Fabric::KindOfFabricId)
                            .to(KindOfFabric::Table, KindOfFabric::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
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
                    .col(integer_null(Project::Size))
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
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(KindOfFabric::Table)
                    .if_not_exists()
                    .col(pk_auto(KindOfFabric::Id))
                    .col(string_uniq(KindOfFabric::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Pattern::Table)
                    .if_not_exists()
                    .col(pk_auto(Pattern::Id))
                    .col(string(Pattern::Name))
                    .col(string_null(Pattern::Designer))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PatternXProject::Table)
                    .if_not_exists()
                    .col(integer(PatternXProject::PatternId))
                    .col(integer(PatternXProject::ProjectId))
                    .primary_key(
                        Index::create()
                            .name("pk-pattern-x-project")
                            .col(PatternXProject::PatternId)
                            .col(PatternXProject::ProjectId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-pattern-id")
                            .from(
                                PatternXProject::Table,
                                PatternXProject::PatternId,
                            )
                            .to(Pattern::Table, Pattern::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-project-id")
                            .from(
                                PatternXProject::Table,
                                PatternXProject::ProjectId,
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
            .drop_table(Table::drop().table(FabricXProject::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PatternXProject::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Pattern::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(KindOfFabric::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await?;
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Fabric::Table).to_owned())
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
    Producer,
    DateOfPurchase,
    KindOfFabricId,
}

#[derive(DeriveIden)]
enum Project {
    Table,
    Id,
    Name,
    Size,
}

#[derive(DeriveIden)]
enum FabricXProject {
    Table,
    FabricId,
    ProjectId,
}

#[derive(DeriveIden)]
pub enum KindOfFabric {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum Pattern {
    Table,
    Id,
    Name,
    Designer,
}

#[derive(DeriveIden)]
enum PatternXProject {
    Table,
    PatternId,
    ProjectId,
}
