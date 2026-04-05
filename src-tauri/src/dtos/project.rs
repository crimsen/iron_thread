use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError, NoneAsEmptyString, PickFirst};

use crate::entities::project::{ActiveModel, Model};

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
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDTO {
    pub id: i32,
    pub name: String,
    #[serde_as(as = "PickFirst<(_, NoneAsEmptyString)>")]
    #[serde(default)]
    pub size: Option<i32>,
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
