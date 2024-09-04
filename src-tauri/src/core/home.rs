use serde_json::json;
use std::fs;
use std::path::Path;

use crate::core::utils::get_config_dir;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Project {
    id: String,
    path: String,
    name: String,
    description: Option<String>,
    date_modified: String,
}

#[tauri::command]
pub fn get_projects(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let config_dir = get_config_dir(&app);
    let projects_yml_path = Path::new(&config_dir).join("projects.yml");

    if !projects_yml_path.exists() {
        if let Err(e) = fs::write(&projects_yml_path, "") {
            return Err(format!("Failed to create project file: {}", e));
        }
    }

    let projects_content = fs::read_to_string(&projects_yml_path)
        .map_err(|e| format!("Failed to read project file: {}", e))?;
    let projects: Option<Vec<Project>> = serde_yaml::from_str(&projects_content)
        .map_err(|e| format!("Failed to deserialize project file: {}", e))?;

    Ok(json!(projects.unwrap_or(vec![])))
}
