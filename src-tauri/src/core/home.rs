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

impl Project {
    fn resolve_pack_image(&mut self) {
        let pack_image_path = Path::new(&self.path).join("pack.png");
        self.pack_image = pack_image_path
            .exists()
            .then(|| pack_image_path.to_str().unwrap_or_default().to_string());
    }
}

#[tauri::command]
pub fn get_projects<R: Runtime>(app: AppHandle<R>) -> Result<serde_json::Value, String> {
    log::info!("Home: Getting projects");

    let config_dir = get_config_dir(&app).map_err(|e| {
        let err = format!("Home: Failed to get config dir: {:?}", e);
        log::error!("{}", err);
        err
    })?;
    log::debug!("Home: Config dir found at {:?}", config_dir);

    let projects_yml_path = config_dir.join("projects.yml");
    log::debug!("Home: projects.yml path: {:?}", projects_yml_path);

    if !projects_yml_path.exists() {
        fs::write(&projects_yml_path, "").map_err(|e| {
            let err = format!("Home: Failed to create projects.yml: {}", e);
            log::error!("{}", err);
            err
        })?;
    }

    let content = fs::read_to_string(&projects_yml_path).map_err(|e| {
        let err = format!("Home: Failed to read projects.yml: {}", e);
        log::error!("{}", err);
        err
    })?;

    let mut projects: Vec<Project> = serde_yaml::from_str(&content).map_err(|e| {
        let err = format!("Home: Failed to deserialize projects.yml: {}", e);
        log::error!("{}", err);
        err
    })?;

    projects.sort_by(|a, b| b.date_modified.cmp(&a.date_modified));

    for project in &mut projects {
        project.resolve_pack_image();
    }

    Ok(json!(projects))
}
