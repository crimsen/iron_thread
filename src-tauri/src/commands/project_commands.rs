use log;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    sea_query::OnConflict, ActiveModelTrait, ColumnTrait, EntityTrait,
    ModelTrait, QueryFilter, TryIntoModel,
};
use tauri::State;

use crate::dtos::project::{FabricsArrayWithLength, ProjectDTO};
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
) -> Result<ProjectDTO, String> {
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
            .filter(|f| {
                !project_data
                    .fabrics
                    .iter()
                    .map(|f| f.fabric_id)
                    .collect::<Vec<i32>>()
                    .contains(&f.id)
            })
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

        let to_upsert: Vec<fabric_x_project::ActiveModel> = project_data
            .fabrics
            .iter()
            .map(|f| fabric_x_project::ActiveModel {
                project_id: Set(project.id),
                fabric_id: Set(f.fabric_id),
                fabric_length: Set(f.length),
            })
            .collect();

        for model in to_upsert {
            FabricXProject::insert(model)
                .on_conflict(
                    OnConflict::columns([
                        fabric_x_project::Column::ProjectId,
                        fabric_x_project::Column::FabricId,
                    ])
                    .update_column(fabric_x_project::Column::FabricLength)
                    .to_owned(),
                )
                .exec(db)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    log::debug!("active_model is: {:?}", &active_model);

    let saved_model = active_model
        .save(db)
        .await
        .map_err(|e| e.to_string())?
        .try_into_model();
    log::debug!("saved_model is: {:?}", &saved_model);
    if let Ok(model) = saved_model {
        let existing_project = Project::find_by_id(model.id)
            .one(db)
            .await
            .map_err(|e| e.to_string())?;
        if let Some(project) = existing_project {
            let relations: Vec<(fabric_x_project::Model, Vec<fabric::Model>)> =
                FabricXProject::find()
                    .filter(fabric_x_project::Column::ProjectId.eq(project.id))
                    .find_with_related(Fabric)
                    .all(db)
                    .await
                    .map_err(|e| e.to_string())?;
            let fabrics: Vec<FabricsArrayWithLength> = relations
                .into_iter()
                .filter_map(|(rel, mut fabrics)| {
                    fabrics.pop().map(|f| FabricsArrayWithLength {
                        fabric_id: f.id,
                        length: rel.fabric_length,
                    })
                })
                .collect();
            let mut project: ProjectDTO = project.into();
            project.fabrics = fabrics;
            return Ok(project);
        }
    }
    Err("Something went wrong".into())
    // saved_model.try_into_model().map_err(|e| e.to_string())
}
