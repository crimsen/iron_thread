use sea_orm::entity::prelude::*;

use crate::entities::foto_path::ActiveModel;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_delete<C>(self, _db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let _ = std::fs::remove_file(std::path::Path::new(self.path.as_ref()));
        Ok(self)
    }
}
