use crate::m20220101_000001_create_table::KindOfFabric;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let kind_of_fabric = vec![
            ["Baumwollwebware".into()],
            ["Webware Jacquard".into()],
            ["Viskose Webware".into()],
            ["(Stretch-)Satin".into()],
            ["Leinen".into()],
            ["Jeans/Denim".into()],
            ["Canvas/Segeltuch".into()],
            ["Cord".into()],
            ["Velvet/Samt".into()],
            ["Seide".into()],
            ["Voile".into()],
            ["Batist".into()],
            ["Chiffon".into()],
            ["Krepp".into()],
            ["Chambray".into()],
            ["Tüll".into()],
            ["Frottee".into()],
            ["Musselin/Spucktuch Stoff".into()],
            ["Molton".into()],
            ["Flanell".into()],
            ["Pique/Pikee".into()],
            ["Seersucker".into()],
            ["Tweed".into()],
        ];
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(KindOfFabric::Table)
                    .columns([KindOfFabric::Name])
                    .values_from_panic(kind_of_fabric.into_iter())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .exec_stmt(
                Query::delete()
                    .from_table(KindOfFabric::Table)
                    .and_where(Expr::col(KindOfFabric::Id).is_not_null())
                    .to_owned(),
            )
            .await
    }
}
