use crate::entities::prelude::FabricXProject;
use crate::entities::prelude::Project;
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
}

impl From<ProjectDTO> for ActiveModel {
    fn from(value: ProjectDTO) -> Self {
        Self {
            id: if value.id < 0 { NotSet } else { Set(value.id) },
            name: NotSet,
            size: Set(value.size),
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
}
