use log;
use sea_orm::{ActiveModelTrait, EntityTrait, TryIntoModel};
use tauri::State;

use crate::dtos::project::ProjectDTO;
use crate::entities::prelude::Project;
use crate::entities::project;
use crate::DbState;

#[tauri::command]
pub async fn get_projects(
    db: State<'_, DbState>,
) -> Result<Vec<project::Model>, String> {
    log::debug!("get_all projects");
    Project::find().all(&db.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_project(
    state: State<'_, DbState>,
    project_data: ProjectDTO,
) -> Result<project::Model, String> {
    log::debug!("prodec_data is: {:?}", &project_data);
    let db = &state.db;
    let _ = Project::find_by_id(project_data.id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;
    let active_model: project::ActiveModel = project_data.clone().into();
    /*TODO: hier müssen noch Fotos rein. Problem: mehrere Fotos? Evtl. extra Table für Fotos?*/
    // if let Some(existing_project) = existings_project {
    //
    // }
    log::debug!("active_model is: {:?}", &active_model);

    let saved_model = active_model.save(db).await.map_err(|e| e.to_string())?;
    log::debug!("saved_model is: {:?}", &saved_model);
    saved_model.try_into_model().map_err(|e| e.to_string())
}
