use serde_json::json;
use tauri::{AppHandle, Runtime};

use crate::core::utils::get_config_dir;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Project {
    pub id: String,
    pub path: String,
    pub name: String,
    pub description: Option<String>,
    pub pack_image: Option<String>,
    pub date_modified: String,
}

#[tauri::command]
pub fn get_projects<R: Runtime>(app: AppHandle<R>) {
    let config_dir = get_config_dir(&app);
}
