use std::fs;
use std::ops::Deref;

use crate::DbState;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{
    ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, TryIntoModel,
};
use tauri::{AppHandle, Manager, State};

use crate::entities::fabric;
use crate::entities::prelude::Fabric;
use log;

#[tauri::command]
pub async fn get_fabrics(
    db: State<'_, DbState>,
) -> Result<Vec<fabric::Model>, String> {
    log::debug!("get_all fabrics");
    Fabric::find().all(&db.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_fabric(
    state: State<'_, DbState>,
    fabric_data: fabric::Model,
) -> Result<fabric::Model, String> {
    log::debug!("fabric_data is: {:?}", &fabric_data);
    let db = &state.db;
    let existing_fabric = Fabric::find_by_id(fabric_data.id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;
    let active_model = if let Some(existing_fabric) = existing_fabric {
        let mut active_model = existing_fabric.into_active_model();
        active_model.name = Set(fabric_data.name.clone());
        active_model.producer = Set(fabric_data.producer.clone());
        active_model.length = Set(fabric_data.length);
        active_model.width = Set(fabric_data.width);
        active_model.costs = Set(fabric_data.costs);
        active_model.foto_path = Set(fabric_data.foto_path.clone());
        active_model.kind_of_fabric_id = Set(fabric_data.kind_of_fabric_id);
        active_model.date_of_purchase = Set(fabric_data.date_of_purchase);
        active_model
    } else {
        let mut active_model = fabric_data.into_active_model();
        active_model.id = NotSet;
        active_model
    };

    log::debug!("active_model is: {:?}", &active_model);

    let saved_model = active_model.save(db).await.map_err(|e| e.to_string())?;
    log::debug!("saved_model is: {:?}", &saved_model);
    saved_model.try_into_model().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn upload_fabric_image(
    app: AppHandle,
    db_state: State<'_, DbState>,
    file_name: String,
    file_data: Vec<u8>,
    fabric_id: i32,
) -> Result<String, String> {
    let app_dir = app.path().app_data_dir().unwrap();
    let upload_dir = app_dir.join("fabric_images");

    fs::create_dir_all(&upload_dir).map_err(|e| e.to_string())?;

    let file_path = upload_dir.join(format!("{}_{}", fabric_id, file_name));

    fs::write(&file_path, file_data).map_err(|e| e.to_string())?;
    let db = &db_state.db;
    let fabric = Fabric::find_by_id(fabric_id)
        .one(db)
        .await
        .map_err(|e| e.to_string());
    if let Ok(Some(fabric)) = fabric {
        let mut active_model = fabric.into_active_model();
        active_model.foto_path =
            Set(Some(file_path.to_string_lossy().to_string().clone()));
        let _ = active_model.save(db).await.map_err(|e| e.to_string());
    }
    Ok(file_path.to_string_lossy().into_owned())
}
