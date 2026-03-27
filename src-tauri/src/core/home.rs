use std::{fs, path::Path};

use log;
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
pub fn get_projects<R: Runtime>(app: AppHandle<R>) -> Result<serde_json::Value, String> {
    log::info!("Home: Getting projects");
    let config_dir = match get_config_dir(&app) {
        Ok(dir) => {
            log::debug!("Home: Config dir found at {:?}", dir);
            dir
        }
        Err(e) => {
            let err = format!("Home: Failed to get config dir: {:?}", e);
            log::error!("{}", err);
            return Err(err);
        }
    };

    let projects_yml_path = config_dir.join("projects.yml");
    log::debug!("Home: projects.yml path: {:?}", projects_yml_path);

    if !projects_yml_path.exists() {
        if let Err(e) = fs::write(&projects_yml_path, "") {
            let err = format!("Home: Failed to create project: {}", e);
            log::error!("{}", err);
            return Err(err);
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
