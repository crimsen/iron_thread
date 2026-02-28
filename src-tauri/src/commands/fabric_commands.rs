use std::ops::Deref;

use crate::DbState;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{
    ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, TryIntoModel,
};
use tauri::State;

use crate::entities::fabric;
use crate::entities::prelude::Fabric;
use log;

#[tauri::command]
pub async fn get_fabrics(
    db: State<'_, DbState>,
) -> Result<Vec<fabric::Model>, String> {
    log::debug!("get_all fabrics");
    Fabric::find().all(&db.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_fabric(
    state: State<'_, DbState>,
    fabric_data: fabric::Model,
) -> Result<fabric::Model, String> {
    log::debug!("fabric_data is: {:?}", &fabric_data);
    let db = &state.db;
    let existing_fabric = Fabric::find_by_id(fabric_data.id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;
    let active_model = if let Some(existing_fabric) = existing_fabric {
        let mut active_model = existing_fabric.into_active_model();
        active_model.name = Set(fabric_data.name.clone());
        active_model.producer = Set(fabric_data.producer.clone());
        active_model.length = Set(fabric_data.length);
        active_model.width = Set(fabric_data.width);
        active_model.costs = Set(fabric_data.costs);
        active_model.foto_path = Set(fabric_data.foto_path.clone());
        active_model.kind_of_fabric_id = Set(fabric_data.kind_of_fabric_id);
        active_model.date_of_purchase = Set(fabric_data.date_of_purchase);
        active_model
    } else {
        let mut active_model = fabric_data.into_active_model();
        active_model.id = NotSet;
        active_model
    };

    log::debug!("active_model is: {:?}", &active_model);

    let saved_model = active_model.save(db).await.map_err(|e| e.to_string())?;
    log::debug!("saved_model is: {:?}", &saved_model);
    saved_model.try_into_model().map_err(|e| e.to_string())
}
