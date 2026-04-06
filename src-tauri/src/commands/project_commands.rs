use log;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    sea_query::OnConflict, ActiveModelTrait, ColumnTrait, EntityTrait,
    ModelTrait, QueryFilter, TryIntoModel,
};
use tauri::State;

use crate::dtos::project::ProjectDTO;
use crate::entities::prelude::{Fabric, FabricXProject, Project};
use crate::entities::{fabric_x_project, project};
use crate::error::MyError;
use crate::DbState;

#[tauri::command]
pub async fn get_projects(
    db: State<'_, DbState>,
) -> Result<Vec<ProjectDTO>, MyError> {
    log::debug!("get_all projects");
    // Project::find().all(&db.db).await.map_err(|e| e.to_string())
    ProjectDTO::find_all(&db.db).await
}

#[tauri::command]
pub async fn save_project(
    state: State<'_, DbState>,
    project_data: ProjectDTO,
) -> Result<ProjectDTO, MyError> {
    log::debug!("prodec_data is: {:?}", &project_data);
    let db = &state.db;
    project_data.save_project(db).await
}
