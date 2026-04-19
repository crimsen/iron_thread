use std::path::Path;

use crate::dtos::fabric::FabricDTO;
use crate::error::MyError;
use crate::DbState;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait, TryIntoModel,
};
use tauri::{AppHandle, Manager, State};

use crate::entities::fabric;
use crate::entities::prelude::Fabric;
use log;

#[tauri::command]
pub async fn get_fabrics(
    db: State<'_, DbState>,
) -> Result<Vec<FabricDTO>, MyError> {
    log::debug!("get_all fabrics");
    FabricDTO::find_all(&db.db).await
}

#[tauri::command]
pub async fn save_fabric(
    state: State<'_, DbState>,
    fabric_data: FabricDTO,
    app: AppHandle,
) -> Result<FabricDTO, MyError> {
    log::debug!("fabric_data is: {:?}", &fabric_data);
    let db = &state.db;
    fabric_data.save(db, &app).await
}

// #[tauri::command]
// pub async fn upload_fabric_image(
//     app: AppHandle,
//     db_state: State<'_, DbState>,
//     file_name: String,
//     file_data: Vec<u8>,
//     fabric_id: i32,
// ) -> Result<String, MyError> {
//     // TODO: rewrite logic for fotouploads
//     let app_dir = app.path().app_data_dir().unwrap();
//     let upload_dir = app_dir.join("fabric_images");
//
//     f::create_dir_all(&upload_dir)?;
//
//     let file_path = upload_dir.join(format!("{}_{}", fabric_id, file_name));
//
//     fs::write(&file_path, file_data)?;
//     let db = &db_state.db;
//     let fabric = Fabric::find_by_id(fabric_id).one(db).await?;
//     if let Some(fabric) = fabric {
//         if let Some(to_delete_path) = &fabric.foto_path {
//             let _ = fs::remove_file(to_delete_path);
//         }
//         let mut active_model = fabric.into_active_model();
//         active_model.foto_path =
//             Set(Some(file_path.to_string_lossy().to_string().clone()));
//         let _ = active_model.save(db).await?;
//     }
//     Ok(file_path.to_string_lossy().into_owned())
// }

#[tauri::command]
pub async fn delete_fabric(
    db_state: State<'_, DbState>,
    fabric_id: i32,
) -> Result<i32, String> {
    let db = &db_state.db;
    let fabric = Fabric::find_by_id(fabric_id)
        .one(db)
        .await
        .map_err(|e| e.to_string());
    if let Ok(Some(fabric)) = fabric {
        let _ = fabric.delete(db).await.map_err(|e| e.to_string());
    }
    Ok(fabric_id)
}
