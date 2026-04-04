use sea_orm::{ActiveModelTrait, EntityTrait, TryIntoModel};
use tauri::{command, State};

use crate::{
    dtos::fabric_x_project::FabricXProjectDTO,
    entities::{
        fabric_x_project::{self},
        prelude::FabricXProject,
    },
    DbState,
};
use log;

#[command]
pub async fn get_fabric_x_projects(
    db: State<'_, DbState>,
) -> Result<Vec<fabric_x_project::Model>, String> {
    log::debug!("get_all fabric_x_projects:");
    FabricXProject::find()
        .all(&db.db)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn save_fabric_x_project(
    state: State<'_, DbState>,
    fabric_x_project_data: FabricXProjectDTO,
) -> Result<fabric_x_project::Model, String> {
    log::info!("fabric_x_project_data is {:?}", &fabric_x_project_data);
    let db = &state.db;
    let active_model: fabric_x_project::ActiveModel =
        fabric_x_project_data.clone().into();
    log::info!("active_model is: {:?}", &active_model);
    let existing = FabricXProject::find_by_id((
        fabric_x_project_data.fabric_id,
        fabric_x_project_data.project_id,
    ))
    .one(db)
    .await
    .map_err(|e| e.to_string())?;
    if existing.is_some() {
        let saved_model =
            active_model.save(db).await.map_err(|e| e.to_string())?;
        saved_model.try_into_model().map_err(|e| e.to_string())
    } else {
        let saved_model =
            active_model.insert(db).await.map_err(|e| e.to_string())?;
        saved_model.try_into_model().map_err(|e| e.to_string())
    }
}
