use crate::entities::prelude::FabricXProject;
use crate::entities::prelude::Project;
use crate::entities::project;
use migration::OnConflict;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    DbConn, FromQueryResult,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError, NoneAsEmptyString, PickFirst};

use crate::{
    entities::{
        fabric_x_project,
        project::{ActiveModel, Model},
    },
    error::MyError,
};

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Copy)]
#[serde(rename_all = "camelCase")]
pub struct FabricsArrayWithLength {
    pub fabric_id: i32,
    #[serde_as(as = "PickFirst<(_, NoneAsEmptyString)>")]
    #[serde(default)]
    pub length: Option<f32>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, Default, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDTO {
    pub id: i32,
    pub name: String,
    #[serde_as(as = "PickFirst<(_, NoneAsEmptyString)>")]
    #[serde(default)]
    pub size: Option<i32>,
    #[sea_orm(skip)]
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub fabrics: Vec<FabricsArrayWithLength>,
    #[serde_as(as = "PickFirst<(_, NoneAsEmptyString)>")]
    #[serde(default)]
    pub foto_path_id: Option<i32>,
}

impl From<ProjectDTO> for ActiveModel {
    fn from(value: ProjectDTO) -> Self {
        Self {
            id: if value.id < 0 { NotSet } else { Set(value.id) },
            name: Set(value.name),
            size: Set(value.size),
            foto_path_id: Set(value.foto_path_id),
        }
    }
}

impl From<Model> for ProjectDTO {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            size: value.size,
            ..Default::default()
        }
    }
}

impl ProjectDTO {
    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Self, MyError> {
        let mut project_base = Project::find_by_id(id)
            .into_model::<ProjectDTO>()
            .one(db)
            .await?
            .ok_or_else(|| MyError::Validation("project not found".into()))?;
        let relations: Vec<fabric_x_project::Model> = FabricXProject::find()
            .filter(fabric_x_project::Column::ProjectId.eq(project_base.id))
            .all(db)
            .await?;
        let fabrics: Vec<FabricsArrayWithLength> = relations
            .into_iter()
            .map(|rel| FabricsArrayWithLength {
                fabric_id: rel.fabric_id,
                length: rel.fabric_length,
            })
            .collect();
        project_base.fabrics = fabrics;
        Ok(project_base)
    }

    pub async fn find_all(db: &DbConn) -> Result<Vec<Self>, MyError> {
        let projects_with_fabrics: Vec<(Model, Vec<fabric_x_project::Model>)> =
            Project::find()
                .find_with_related(FabricXProject)
                .all(db)
                .await?;
        Ok(projects_with_fabrics
            .into_iter()
            .map(|(_project, _fabric_x_project)| {
                let mut project = ProjectDTO::from(_project);
                project.fabrics = _fabric_x_project
                    .into_iter()
                    .map(|_rel| FabricsArrayWithLength {
                        fabric_id: _rel.fabric_id,
                        length: _rel.fabric_length,
                    })
                    .collect();
                project
            })
            .collect::<Vec<ProjectDTO>>())
    }

    // TODO: handle multiple fotos:
    // Need to set main foto and other fotos
    pub async fn save_project(self, db: &DbConn) -> Result<Self, MyError> {
        let active_model: project::ActiveModel = self.clone().into();
        log::debug!("save project with active_model: {:?}", active_model);
        let saved_project = if active_model.id == NotSet {
            active_model.insert(db).await?
        } else {
            active_model.update(db).await?
        };
        log::debug!("saved_project: {:?}", saved_project);
        let final_id = saved_project.id;
        if self.id >= 0 {
            let current_relations = FabricXProject::find()
                .filter(fabric_x_project::Column::ProjectId.eq(self.id))
                .all(db)
                .await?;
            let to_delete_relations: Vec<i32> = current_relations
                .iter()
                .filter_map(|f| {
                    if !self
                        .clone()
                        .fabrics
                        .into_iter()
                        .map(|_f| _f.fabric_id)
                        .collect::<Vec<i32>>()
                        .contains(&f.fabric_id)
                    {
                        Some(f.fabric_id)
                    } else {
                        None
                    }
                })
                .collect();
            if !to_delete_relations.is_empty() {
                FabricXProject::delete_many()
                    .filter(fabric_x_project::Column::ProjectId.eq(self.id))
                    .filter(
                        fabric_x_project::Column::FabricId
                            .is_in(to_delete_relations),
                    )
                    .exec(db)
                    .await?;
            }
        }

        for f in self.fabrics {
            let model = fabric_x_project::ActiveModel {
                fabric_id: Set(f.fabric_id),
                project_id: Set(final_id),
                fabric_length: Set(f.length),
            };
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
        ProjectDTO::find_by_id(db, final_id).await
    }
}
