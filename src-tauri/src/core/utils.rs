use std::path::PathBuf;
use tauri::Manager;

#[tauri::command]
pub fn get_config_dir(app: &tauri::AppHandle) -> String {
    let config_dir = app.path().app_data_dir().unwrap_or(PathBuf::new());
    let config_dir_str = config_dir.to_string_lossy().into_owned();

    config_dir_str
}
