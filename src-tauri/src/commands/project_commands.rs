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
    let existing_project = Project::find_by_id(project_data.id).one(db).await?;
    let active_model: project::ActiveModel = project_data.clone().into();
    /*TODO: hier müssen noch Fotos rein. Problem: mehrere Fotos? Evtl. extra Table für Fotos?*/
    if let Some(project) = existing_project {
        let current_fabrics = project.find_related(Fabric).all(db).await?;

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
                .await?;
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
                .await?;
        }
    }

    log::debug!("active_model is: {:?}", &active_model);

    let saved_model = active_model.save(db).await?.try_into_model();
    log::debug!("saved_model is: {:?}", &saved_model);
    if let Ok(model) = saved_model {
        return ProjectDTO::find_by_id(&db, model.id).await;
    }
    Err(MyError::Validation("Something went wrong".into()))
    // saved_model.try_into_model().map_err(|e| e.to_string())
}
