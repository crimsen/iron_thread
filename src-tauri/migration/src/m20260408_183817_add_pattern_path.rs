use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create()
        .table(PatternPath::Table)
            .col(pk_auto(PatternPath::Id))
            .col(string(PatternPath::Path))
            .col(integer(PatternPath::PattenId))
            .foreign_key(ForeignKey::create()
                .name("fk-pattern-path-pattern-id")
                .from(PatternPath::Table, PatternPath::PattenId)
                .to("pattern", "id").on_delete(ForeignKeyAction::Restrict)
                .on_update(ForeignKeyAction::Cascade)
            )
        .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(PatternPath::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum PatternPath {
    Table,
    Id,
    Path,
    PattenId,
}
