use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        manager
            .create_table(
                Table::create()
                    .table(FotoPath::Table)
                    .if_not_exists()
                    .col(pk_auto(FotoPath::Id))
                    .col(integer_null(FotoPath::ProjectId))
                    .col(string(FotoPath::Path))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-foto-path-of-project-id")
                            .from(FotoPath::Table, FotoPath::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        db.execute_unprepared("PRAGMA foreign_keys = OFF").await?;
        db.execute_unprepared("INSERT INTO foto_path (path) SELECT foto_path from fabric WHERE foto_path IS NOT NULL").await?;
        manager
            .create_table(
                Table::create()
                    .table("fabric_new")
                    .if_not_exists()
                    .col(pk_auto(Fabric::Id))
                    .col(string_null(Fabric::Name))
                    .col(float_null(Fabric::Length))
                    .col(float_null(Fabric::Width))
                    .col(float_null(Fabric::Costs))
                    .col(string_null(Fabric::Producer))
                    .col(integer_null(Fabric::KindOfFabricId))
                    .col(date_null(Fabric::DateOfPurchase))
                    .col(integer_null(Fabric::FotoPathId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-fabric-foto-path-id")
                            .from(Fabric::Table, Fabric::FotoPathId)
                            .to(FotoPath::Table, FotoPath::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
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
        // 1. Erstelle eine Backup-Tabelle nur für die Pfade
        db.execute_unprepared(
            "INSERT INTO fabric_new (id, name, length, width, costs, producer, kind_of_fabric_id, date_of_purchase, foto_path_id) SELECT f.id, f.name, f.length, f.width, f.costs, f.producer, f.kind_of_fabric_id, f.date_of_purchase, p.id FROM fabric f LEFT JOIN foto_path p ON f.foto_path=p.path",
        )
        .await?;
        manager
            .drop_table(Table::drop().table(Fabric::Table).to_owned())
            .await?;
        manager
            .rename_table(
                Table::rename()
                    .table("fabric_new", Fabric::Table)
                    .to_owned(),
            )
            .await?;
        db.execute_unprepared("PRAGMA foreign_keys = ON").await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("PRAGMA foreign_keys = OFF").await?;
        manager
            .create_table(
                Table::create()
                    .table("fabric_old")
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
        db.execute_unprepared("INSERT INTO fabric_old (id, name, length, width, costs, producer, kind_of_fabric_id, date_of_purchase, foto_path) SELECT f.id, f.name, f.length, f.width, f.costs, f.producer, f.kind_of_fabric_id, f.date_of_purchase, p.path FROM fabric f LEFT JOIN foto_path p on f.foto_path_id=p.id").await?;
        manager
            .drop_table(Table::drop().table(FotoPath::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Fabric::Table).to_owned())
            .await?;
        manager
            .rename_table(
                Table::rename()
                    .table("fabric_old", Fabric::Table)
                    .to_owned(),
            )
            .await?;
        db.execute_unprepared("PRAGMA foreign_keys = ON").await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum FotoPath {
    Table,
    Id,
    ProjectId,
    Path,
}
#[derive(DeriveIden)]
enum Fabric {
    Table,
    Id,
    Name,
    Length,
    Width,
    Costs,
    Producer,
    DateOfPurchase,
    KindOfFabricId,
    FotoPath,
    FotoPathId,
}

#[derive(DeriveIden)]
enum Project {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum KindOfFabric {
    Table,
    Id,
}
