use crate::DbState;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel};
use tauri::State;

use crate::entities::kind_of_fabric;
use crate::entities::prelude::KindOfFabric;
use log;

#[tauri::command]
pub async fn get_kind_of_fabrics(
    db: State<'_, DbState>,
) -> Result<Vec<kind_of_fabric::Model>, String> {
    log::debug!("get_all kind_of_fabrics");
    KindOfFabric::find()
        .all(&db.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_kind_of_fabric(
    state: State<'_, DbState>,
    kind_of_fabric_data: kind_of_fabric::Model,
) -> Result<kind_of_fabric::Model, String> {
    log::debug!("kind_of_fabric_data is: {:?}", &kind_of_fabric_data);
    let db = &state.db;
    let mut active_model: kind_of_fabric::ActiveModel =
        kind_of_fabric_data.into_active_model();

    log::debug!("active_model is: {:?}", &active_model);
    if active_model.id.is_set()
        && (if let ActiveValue::Set(val) | ActiveValue::Unchanged(val) =
            active_model.id
        {
            val
        } else {
            -1
        }) > 0
    {
        log::debug!("update active model");
        active_model.update(db).await.map_err(|e| e.to_string())
    } else {
        log::debug!("set no model");
        active_model.id = NotSet;
        active_model.insert(db).await.map_err(|e| e.to_string())
    }
}
