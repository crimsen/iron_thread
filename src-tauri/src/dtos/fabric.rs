use std::{fs, path::Path};

use migration::OnConflict;
use sea_orm::{
    prelude::Date,
    ActiveValue::{NotSet, Set},
    DbConn, EntityTrait, FromQueryResult, QueryFilter,
};
use sea_orm::{ActiveModelTrait, ColumnTrait};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError, NoneAsEmptyString, PickFirst};

use crate::entities::fabric;
use crate::entities::{
    fabric_x_project,
    prelude::{Fabric, FabricXProject},
};
use crate::{
    entities::fabric::{ActiveModel, Model},
    error::MyError,
};

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Copy)]
#[serde(rename_all = "camelCase")]
pub struct ProjectArrayWithLength {
    pub project_id: i32,
    #[serde_as(as = "PickFirst<(_, NoneAsEmptyString)>")]
    #[serde(default)]
    pub length: Option<f32>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, FromQueryResult, Default)]
#[serde(rename_all = "camelCase")]
pub struct FabricDTO {
    pub id: i32,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde_as(as = "PickFirst<(_,NoneAsEmptyString)>")]
    #[serde(default)]
    pub length: Option<f32>,
    #[serde_as(as = "PickFirst<(_,NoneAsEmptyString)>")]
    #[serde(default)]
    pub width: Option<f32>,
    #[serde_as(as = "PickFirst<(_,NoneAsEmptyString)>")]
    #[serde(default)]
    pub costs: Option<f32>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default)]
    pub foto_path: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default)]
    pub producer: Option<String>,
    #[serde_as(as = "PickFirst<(_,NoneAsEmptyString)>")]
    #[serde(default)]
    pub kind_of_fabric_id: Option<i32>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default)]
    pub date_of_purchase: Option<Date>,
    #[sea_orm(skip)]
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub projects: Vec<ProjectArrayWithLength>,
}

impl From<FabricDTO> for ActiveModel {
    fn from(value: FabricDTO) -> Self {
        Self {
            id: if value.id < 0 { NotSet } else { Set(value.id) },
            name: Set(value.name),
            length: Set(value.length),
            width: Set(value.width),
            costs: Set(value.costs),
            foto_path: Set(value.foto_path),
            producer: Set(value.producer),
            kind_of_fabric_id: Set(value.kind_of_fabric_id),
            date_of_purchase: Set(value.date_of_purchase),
        }
    }
}

impl From<Model> for FabricDTO {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            length: value.length,
            width: value.width,
            costs: value.costs,
            foto_path: value.foto_path,
            producer: value.producer,
            kind_of_fabric_id: value.kind_of_fabric_id,
            date_of_purchase: value.date_of_purchase,
            ..Default::default()
        }
    }
}

impl FabricDTO {
    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Self, MyError> {
        let mut fabric_base = Fabric::find_by_id(id)
            .into_model::<FabricDTO>()
            .one(db)
            .await?
            .ok_or_else(|| MyError::Validation("fabric not found".into()))?;
        let relations = FabricXProject::find()
            .filter(fabric_x_project::Column::FabricId.eq(fabric_base.id))
            .all(db)
            .await?;
        fabric_base.projects = relations
            .into_iter()
            .map(|rel| ProjectArrayWithLength {
                project_id: rel.project_id,
                length: rel.fabric_length,
            })
            .collect();
        Ok(fabric_base)
    }

    pub async fn find_all(db: &DbConn) -> Result<Vec<Self>, MyError> {
        let fabrics_with_projects: Vec<(Model, Vec<fabric_x_project::Model>)> =
            Fabric::find()
                .find_with_related(FabricXProject)
                .all(db)
                .await?;
        Ok(fabrics_with_projects
            .into_iter()
            .map(|(_fabric, _fabric_x_project)| {
                let mut fabric = FabricDTO::from(_fabric);
                fabric.projects = _fabric_x_project
                    .into_iter()
                    .map(|_rel| ProjectArrayWithLength {
                        project_id: _rel.project_id,
                        length: _rel.fabric_length,
                    })
                    .collect();
                fabric
            })
            .collect())
    }

    pub async fn save(self, db: &DbConn) -> Result<Self, MyError> {
        let active_model: fabric::ActiveModel = self.clone().into();
        let saved_model = if active_model.id == NotSet {
            active_model.insert(db).await?
        } else {
            // TODO: logic to manage fotos must be outer. not only fabrics use fotos
            match (
                self.clone().foto_path,
                FabricDTO::find_by_id(db, self.id).await?.foto_path,
            ) {
                (None, Some(path)) => {
                    let _ = fs::remove_file(Path::new(&path));
                }
                (Some(path_a), Some(path_b)) => {
                    if path_a != path_b {
                        let _ = fs::remove_file(Path::new(&path_b));
                    }
                }
                (_, _) => {}
            }
            active_model.update(db).await?
        };
        let final_id = saved_model.id;
        if self.id >= 0 {
            let current_relations = FabricXProject::find()
                .filter(fabric_x_project::Column::FabricId.eq(self.id))
                .all(db)
                .await?;
            let to_delete_relations: Vec<i32> = current_relations
                .iter()
                .filter_map(|f| {
                    if !self
                        .clone()
                        .projects
                        .into_iter()
                        .map(|_f| _f.project_id)
                        .collect::<Vec<i32>>()
                        .contains(&f.project_id)
                    {
                        Some(f.project_id)
                    } else {
                        None
                    }
                })
                .collect();
            if !to_delete_relations.is_empty() {
                FabricXProject::delete_many()
                    .filter(fabric_x_project::Column::FabricId.eq(self.id))
                    .filter(
                        fabric_x_project::Column::ProjectId
                            .is_in(to_delete_relations),
                    )
                    .exec(db)
                    .await?;
            }
        }

        for p in self.projects {
            let model = fabric_x_project::ActiveModel {
                fabric_id: Set(final_id),
                project_id: Set(p.project_id),
                fabric_length: Set(p.length),
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

        FabricDTO::find_by_id(db, final_id).await
    }
}
