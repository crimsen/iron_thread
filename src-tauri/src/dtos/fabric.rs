use std::{fs, path::Path};

use migration::OnConflict;
use sea_orm::{
    prelude::Date,
    ActiveValue::{NotSet, Set},
    DbConn, EntityTrait, FromQueryResult, LoaderTrait, QueryFilter,
};
use sea_orm::{ActiveModelTrait, ColumnTrait};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError, NoneAsEmptyString, PickFirst};

use serde_with::base64::Base64;
use tauri::AppHandle;

use crate::entities::{
    fabric_x_project, foto_path,
    prelude::{Fabric, FabricXProject, FotoPath},
};
use crate::{
    entities::fabric,
    utils::fotos::{create_file, FotoType},
};
use crate::{
    entities::fabric::{ActiveModel, Model},
    error::MyError,
};
use itertools::izip;

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
    #[serde_as(as = "PickFirst<(_,NoneAsEmptyString)>")]
    #[serde(default)]
    pub foto_path_id: Option<i32>,
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
    #[sea_orm(skip)]
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub file_data: Option<Vec<u8>>,
}

impl From<FabricDTO> for ActiveModel {
    fn from(value: FabricDTO) -> Self {
        Self {
            id: if value.id < 0 { NotSet } else { Set(value.id) },
            name: Set(value.name),
            length: Set(value.length),
            width: Set(value.width),
            costs: Set(value.costs),
            producer: Set(value.producer),
            kind_of_fabric_id: Set(value.kind_of_fabric_id),
            date_of_purchase: Set(value.date_of_purchase),
            foto_path_id: Set(value.foto_path_id),
        }
    }
}

impl From<Model> for FabricDTO {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            width: value.width,
            costs: value.costs,
            producer: value.producer,
            kind_of_fabric_id: value.kind_of_fabric_id,
            date_of_purchase: value.date_of_purchase,
            foto_path_id: value.foto_path_id,
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
        if let Some(foto_path_id) = fabric_base.foto_path_id {
            let fabric_foto: Option<foto_path::Model> = FotoPath::find()
                .filter(foto_path::Column::Id.eq(foto_path_id))
                .one(db)
                .await?;
            if let Some(fabric_foto) = fabric_foto {
                fabric_base.foto_path = Some(fabric_foto.path);
            }
        }
        let projects = FabricXProject::find()
            .filter(fabric_x_project::Column::FabricId.eq(fabric_base.id))
            .all(db)
            .await?;
        fabric_base.projects = projects
            .into_iter()
            .map(|rel| ProjectArrayWithLength {
                project_id: rel.project_id,
                length: rel.fabric_length,
            })
            .collect();
        Ok(fabric_base)
    }

    pub async fn find_all(db: &DbConn) -> Result<Vec<Self>, MyError> {
        let fabrics: Vec<Model> = Fabric::find().all(db).await?;
        let fabric_fotos: Vec<Option<foto_path::Model>> =
            fabrics.load_one(FotoPath, db).await?;
        let fabric_x_projects: Vec<Vec<fabric_x_project::Model>> =
            fabrics.load_many(FabricXProject, db).await?;
        let mut result: Vec<Self> = vec![];
        for (fabric, fabric_foto, _fabric_x_project) in
            izip!(&fabrics, &fabric_fotos, &fabric_x_projects)
        {
            let mut fabric: FabricDTO = FabricDTO::from(fabric.clone());
            fabric.foto_path = if let Some(fabric_foto) = fabric_foto.clone() {
                Some(fabric_foto.path)
            } else {
                None
            };
            fabric.projects = _fabric_x_project
                .iter()
                .map(|project| ProjectArrayWithLength {
                    project_id: project.project_id,
                    length: project.fabric_length,
                })
                .collect();
            result.push(fabric);
        }
        Ok(result)
    }

    pub async fn save(
        self,
        db: &DbConn,
        app: &AppHandle,
    ) -> Result<Self, MyError> {
        let mut active_model: fabric::ActiveModel = self.clone().into();
        if let Some(file_data) = self.clone().file_data {
            let foto =
                create_file(FotoType::Fabric, &file_data, db, app).await?;
            active_model.foto_path_id = Set(Some(foto.id));
        }
        let saved_model = if active_model.id == NotSet {
            active_model.insert(db).await?
        } else {
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
