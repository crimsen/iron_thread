use sea_orm::{
    prelude::Date,
    ActiveValue::{NotSet, Set},
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString, PickFirst};

use crate::entities::fabric::ActiveModel;

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FabricDTO {
    pub id: i32,
    #[serde_as(as = "NoneAsEmptyString")]
    pub name: Option<String>,
    #[serde_as(as = "PickFirst<(_,NoneAsEmptyString)>")]
    pub length: Option<f32>,
    #[serde_as(as = "PickFirst<(_,NoneAsEmptyString)>")]
    pub width: Option<f32>,
    #[serde_as(as = "PickFirst<(_,NoneAsEmptyString)>")]
    pub costs: Option<f32>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub foto_path: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub producer: Option<String>,
    #[serde_as(as = "PickFirst<(_,NoneAsEmptyString)>")]
    pub kind_of_fabric_id: Option<i32>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub date_of_purchase: Option<Date>,
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
