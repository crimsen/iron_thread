use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString, PickFirst};

use crate::entities::fabric_x_project::ActiveModel;

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FabricXProjectDTO {
    pub fabric_id: i32,
    pub project_id: i32,
    #[serde_as(as = "PickFirst<(_, NoneAsEmptyString)>")]
    #[serde(default)]
    pub fabric_length: Option<f32>,
}

impl From<FabricXProjectDTO> for ActiveModel {
    fn from(value: FabricXProjectDTO) -> Self {
        Self {
            fabric_id: Set(value.fabric_id),
            project_id: Set(value.project_id),
            fabric_length: Set(value.fabric_length),
        }
    }
}
