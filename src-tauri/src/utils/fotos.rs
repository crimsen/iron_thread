use std::{fmt::Display, fs, path::PathBuf};

use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue::Set, DbConn,
};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::{
    entities::{foto_path, prelude::FotoPath},
    error::MyError,
};

pub enum FotoType {
    Fabric,
    Project,
}

impl Display for FotoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FotoType::Fabric => write!(f, "fabric"),
            FotoType::Project => write!(f, "project"),
        }
    }
}

pub async fn create_file(
    _type: FotoType,
    data: &Vec<u8>,
    db: &DbConn,
    app_handle: &AppHandle,
) -> Result<foto_path::Model, MyError> {
    let app_dir = app_handle.path().app_data_dir()?;
    let _path = app_dir.join(_type.to_string());
    fs::create_dir_all(&_path);
    let file_path = _path.join(format!("{}.png", Uuid::new_v4()));
    fs::write(&file_path, data)?;
    let mut foto_active_model = foto_path::ActiveModel::new();
    foto_active_model.path = Set(file_path.to_string_lossy().into_owned());
    let foto_path = foto_active_model.insert(db).await?;
    Ok(foto_path)
}
