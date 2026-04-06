// mod entities;

use sea_orm::DatabaseConnection;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

pub struct DbState {
    pub db: DatabaseConnection,
}

mod commands;
mod database;
mod dtos;
mod entities;
mod error;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    #[cfg(not(target_os = "android"))]
                    Target::new(TargetKind::Webview),
                ])
                .level(log::LevelFilter::Debug)
                // .level_for("iron_thread", log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let db = tauri::async_runtime::block_on(database::init(app))?;
            app.manage(DbState { db });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::fabric_commands::get_fabrics,
            commands::fabric_commands::save_fabric,
            commands::fabric_commands::upload_fabric_image,
            commands::fabric_commands::delete_fabric,
            commands::kind_of_fabric_commands::get_kind_of_fabrics,
            commands::kind_of_fabric_commands::save_kind_of_fabric,
            commands::project_commands::get_projects,
            commands::project_commands::save_project,
            commands::fabric_x_project_commands::save_fabric_x_project,
            commands::fabric_x_project_commands::get_fabric_x_projects
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::entities::*;
    use ts_rs::{Config, TS};

    #[test]
    fn export_types() {
        let config = Config::from_env();
        let _ = fabric::Model::export_all(&config);
        let _ = project::Model::export_all(&config);
        let _ = fabric_x_project::Model::export_all(&config);
        let _ = pattern::Model::export_all(&config);
        let _ = pattern_x_project::Model::export_all(&config);
        let _ = kind_of_fabric::Model::export_all(&config);
    }
}
