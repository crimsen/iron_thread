#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("database error: {0}")]
    DbError(#[from] sea_orm::DbErr),
    #[error("validation error: {0}")]
    Validation(String),
}

impl serde::Serialize for MyError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
