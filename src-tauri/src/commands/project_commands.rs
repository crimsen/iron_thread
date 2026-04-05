use log;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter,
    TryIntoModel,
};
use tauri::State;

use crate::dtos::project::ProjectDTO;
use crate::entities::prelude::{Fabric, FabricXProject, Project};
use crate::entities::{fabric, fabric_x_project, project};
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
    let existing_project = Project::find_by_id(project_data.id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;
    let active_model: project::ActiveModel = project_data.clone().into();
    /*TODO: hier müssen noch Fotos rein. Problem: mehrere Fotos? Evtl. extra Table für Fotos?*/
    if let Some(project) = existing_project {
        let current_fabrics = project
            .find_related(Fabric)
            .all(db)
            .await
            .map_err(|e| e.to_string())?;

        let to_delete: Vec<i32> = current_fabrics
            .iter()
            .filter(|f| !project_data.fabric_ids.contains(&f.id))
            .map(|f| f.id)
            .collect();

        if !to_delete.is_empty() {
            FabricXProject::delete_many()
                .filter(fabric_x_project::Column::ProjectId.eq(project.id))
                .filter(fabric_x_project::Column::FabricId.is_in(to_delete))
                .exec(db)
                .await
                .map_err(|e| e.to_string())?;
        }

        let current_ids: Vec<i32> =
            current_fabrics.iter().map(|f| f.id).collect();

        let to_add: Vec<fabric_x_project::ActiveModel> = project_data
            .fabric_ids
            .iter()
            .filter(|id| !current_ids.contains(id))
            .map(|&new_id| fabric_x_project::ActiveModel {
                fabric_id: Set(new_id),
                project_id: Set(project.id),
                ..Default::default()
            })
            .collect();

        if !to_add.is_empty() {
            FabricXProject::insert_many(to_add)
                .exec(db)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    log::debug!("active_model is: {:?}", &active_model);

    let saved_model = active_model.save(db).await.map_err(|e| e.to_string())?;
    log::debug!("saved_model is: {:?}", &saved_model);
    saved_model.try_into_model().map_err(|e| e.to_string())
}
