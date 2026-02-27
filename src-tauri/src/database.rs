use migration::MigratorTrait;
use sea_orm::{Database, DatabaseConnection, DbErr};
use tauri::Manager;

pub async fn init(app: &tauri::App) -> Result<DatabaseConnection, DbErr> {
    let app_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir");
    std::fs::create_dir_all(&app_dir).ok();

    let db_path = app_dir.join("iron_thread.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let db = Database::connect(db_url).await?;
    migration::Migrator::up(&db, None).await?;
    Ok(db)
}
