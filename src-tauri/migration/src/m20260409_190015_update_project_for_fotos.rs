use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("PRAGMA foreign_keys = OFF").await?;
        manager
            .create_table(
                Table::create()
                    .table("project_new")
                    .if_not_exists()
                    .col(pk_auto(Project::Id))
                    .col(string(Project::Name))
                    .col(integer_null(Project::Size))
                    .col(integer_null(Project::FotoPathId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-project-foto-path-id")
                            .from(Project::Table, Project::FotoPathId)
                            .to(FotoPath::Table, FotoPath::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;
        db.execute_unprepared("INSERT INTO project_new (id, name, size) SELECT id, name, size FROM project").await?;
        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await?;
        manager
            .rename_table(
                Table::rename()
                    .table("project_new", Project::Table)
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
                    .table("project_old")
                    .if_not_exists()
                    .col(pk_auto(Project::Id))
                    .col(string(Project::Name))
                    .col(integer_null(Project::Size))
                    .to_owned(),
            )
            .await?;
        db.execute_unprepared("INSERT INTO project_old (id, name, size) SELECT id, name, size FROM project").await?;
        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await?;
        manager
            .rename_table(
                Table::rename()
                    .table("project_old", Project::Table)
                    .to_owned(),
            )
            .await?;
        db.execute_unprepared("PRAGMA foreign_keys = ON").await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Project {
    Table,
    Id,
    Name,
    Size,
    FotoPathId,
}
#[derive(DeriveIden)]
enum FotoPath {
    Table,
    Id,
}
