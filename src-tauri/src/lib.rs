pub mod db;
pub mod error;

use db::repo::{NewWowInstance, WowInstance};
use db::Database;
use error::{AppError, AppResult};
use tauri::{Manager, State};

#[tauri::command]
fn list_instances(db: State<'_, Database>) -> AppResult<Vec<WowInstance>> {
    db.list_instances()
}

#[tauri::command]
fn add_instance(db: State<'_, Database>, input: NewWowInstance) -> AppResult<WowInstance> {
    let created = db.insert_instance(&input)?;
    let _ = db.log_activity(
        Some(created.id),
        "instance_add",
        "ok",
        &format!("Instance '{}' added", created.name),
    );
    Ok(created)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| AppError::Internal(e.to_string()))?;
            let db_path = data_dir.join("data.sqlite");

            let db = Database::open(&db_path)?;
            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![list_instances, add_instance])
        .run(tauri::generate_context!())
        .expect("Fatal error while launching the Tauri app");
}