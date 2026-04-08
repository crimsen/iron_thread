use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
        manager
            .alter_table(
                Table::alter()
                    .table(Fabric::Table)
                    .add_column(
                        ColumnDef::new(Fabric::FotoPathId).integer().null(),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-fabric-foto-path-id")
                            .from_tbl(Fabric::Table)
                            .from_col(Fabric::Id)
                            .to_tbl(FotoPath::Table)
                            .to_col(Fabric::FotoPathId)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        db.execute_unprepared("INSERT INTO foto_path (path) SELECT FROM fabric WHERE foto_path IS NOT NULL").await?;
        db.execute_unprepared("UPDATE fabric SET foto_path_id = (SELECT id FROM foto_path WHERE foto_path.path=fabric.foto_path) WHERE foto_path IS NOT NULL").await?;
        
        manager.alter_table(Table::alter().table(Fabric::Table).drop_column(Fabric::FotoPath).to_owned()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.alter_table(Table::alter().table(Fabric::Table).add_column(ColumnDef::new(Fabric::FotoPath).string().null()).to_owned()).await?;
        let db = manager.get_connection();

        db.execute_unprepared("UPDATE fabric SET foto_path = (SELECT foto_path.path from foto_path WHERE foto_path.id=fabric.foto_path_id) WHERE foto_path_id IS NOT NULL").await?;

        manager.alter_table(Table::alter().table(Fabric::Table).drop_foreign_key("fk-fabric-foto-path-id").drop_column(Fabric::FotoPathId).to_owned()).await?;

        manager.drop_table(Table::drop().table(FotoPath::Table).to_owned()).await?;
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
    FotoPath,
    FotoPathId,
}

#[derive(DeriveIden)]
enum Project {
    Table,
    Id,
}
