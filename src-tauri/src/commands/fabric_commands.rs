use std::fs;
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
) -> Result<FabricDTO, MyError> {
    log::debug!("fabric_data is: {:?}", &fabric_data);
    let db = &state.db;
    // let existing_fabric = Fabric::find_by_id(fabric_data.id)
    //     .one(db)
    //     .await
    //     .map_err(|e| e.to_string())?;
    // let mut active_model: fabric::ActiveModel = fabric_data.clone().into();
    // if let Some(existing_fabric) = existing_fabric {
    //     let existing_active_model = existing_fabric.clone().into_active_model();
    //     active_model.id = existing_active_model.id;
    //     match (fabric_data.foto_path, existing_fabric.foto_path) {
    //         (None, Some(path)) => {
    //             let _ = fs::remove_file(Path::new(&path));
    //         }
    //         (Some(path_a), Some(path_b)) => {
    //             if path_a != path_b {
    //                 let _ = fs::remove_file(Path::new(&path_b));
    //             }
    //         }
    //         (_, _) => {}
    //     }
    // }
    // log::debug!("active_model is: {:?}", &active_model);
    //
    // let saved_model = active_model.save(db).await.map_err(|e| e.to_string())?;
    // log::debug!("saved_model is: {:?}", &saved_model);
    // saved_model.try_into_model().map_err(|e| e.to_string())
    fabric_data.save(db).await
}

#[tauri::command]
pub async fn upload_fabric_image(
    app: AppHandle,
    db_state: State<'_, DbState>,
    file_name: String,
    file_data: Vec<u8>,
    fabric_id: i32,
) -> Result<String, MyError> {
    // TODO: rewrite logic for fotouploads
    let app_dir = app.path().app_data_dir().unwrap();
    let upload_dir = app_dir.join("fabric_images");

    fs::create_dir_all(&upload_dir)?;

    let file_path = upload_dir.join(format!("{}_{}", fabric_id, file_name));

    fs::write(&file_path, file_data)?;
    let db = &db_state.db;
    let fabric = Fabric::find_by_id(fabric_id).one(db).await?;
    if let Some(fabric) = fabric {
        if let Some(to_delete_path) = &fabric.foto_path {
            let _ = fs::remove_file(to_delete_path);
        }
        let mut active_model = fabric.into_active_model();
        active_model.foto_path =
            Set(Some(file_path.to_string_lossy().to_string().clone()));
        let _ = active_model.save(db).await?;
    }
    Ok(file_path.to_string_lossy().into_owned())
}

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
        if let Some(file_path) = &fabric.foto_path {
            let _ = fs::remove_file(file_path);
        }
        let _ = fabric.delete(db).await.map_err(|e| e.to_string());
    }
    Ok(fabric_id)
}
