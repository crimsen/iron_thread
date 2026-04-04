use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString, PickFirst};

use crate::entities::project::ActiveModel;

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDTO {
    pub id: i32,
    pub name: String,
    #[serde_as(as = "PickFirst<(_, NoneAsEmptyString)>")]
    #[serde(default)]
    pub size: Option<i32>,
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
