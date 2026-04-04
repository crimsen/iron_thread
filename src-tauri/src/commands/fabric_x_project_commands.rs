use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, TryIntoModel};
use tauri::{command, State};

use crate::{
    entities::{fabric_x_project, prelude::FabricXProject},
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
    fabric_x_project_data: fabric_x_project::Model,
) -> Result<fabric_x_project::Model, String> {
    log::debug!("fabric_x_project_data is {:?}", &fabric_x_project_data);
    let db = &state.db;
    let active_model = fabric_x_project_data.into_active_model();
    log::debug!("active_model is: {:?}", &active_model);
    let saved_model = active_model.save(db).await.map_err(|e| e.to_string())?;
    log::debug!("saved_model is: {:?}", &saved_model);
    saved_model.try_into_model().map_err(|e| e.to_string())
}
