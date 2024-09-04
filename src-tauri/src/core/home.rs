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
    pack_image: Option<String>,
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
    let mut projects: Vec<Project> = serde_yaml::from_str(&projects_content)
        .map_err(|e| format!("Failed to deserialize project file: {}", e))?;
    projects.sort_by_key(|p| p.date_modified.clone());
    projects.reverse();

    for project in &mut projects {
        let pack_image_path = Path::new(&project.path).join("pack.png");
        if pack_image_path.exists() {
            project.pack_image = Some(pack_image_path.to_str().unwrap_or_default().to_string());
        } else {
            project.pack_image = None;
        }
    }

    Ok(json!(projects))
}

#[tauri::command]
pub fn remove_project(id: String, app: tauri::AppHandle) -> bool {
    let config_dir = get_config_dir(&app);
    let projects_yml_path = Path::new(&config_dir).join("projects.yml");

    let projects_content = match fs::read_to_string(&projects_yml_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read project file: {}", e);
            return false;
        }
    };
    let mut projects: Vec<Project> = match serde_yaml::from_str(&projects_content) {
        Ok(projects) => projects,
        Err(e) => {
            eprintln!("Failed to deserialize project file: {}", e);
            return false;
        }
    };

    projects.retain(|p| p.id != id);

    let updated_content = match serde_yaml::to_string(&projects) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to serialize project file: {}", e);
            return false;
        }
    };

    if let Err(e) = fs::write(&projects_yml_path, updated_content) {
        eprintln!("Failed to write project file: {}", e);
        return false;
    }

    true
}

