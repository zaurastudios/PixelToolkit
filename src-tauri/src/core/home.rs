use serde_json::json;
use std::fs;
use std::path::Path;

use crate::core::utils::{get_config_dir, simple_toast};

use super::utils::try_create_directory;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Project {
    pub id: String,
    pub path: String,
    pub name: String,
    pub description: Option<String>,
    pub pack_image: Option<String>,
    pub date_modified: String,
}

#[derive(serde::Serialize)]
pub struct CreateProject {
    pub success: bool,
    pub id: Option<String>,
    pub message: Option<String>,
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

pub fn get_projects_vec(app: &tauri::AppHandle) -> Vec<Project> {
    let config_dir = get_config_dir(&app);
    let projects_yml_path = Path::new(&config_dir).join("projects.yml");

    let projects_content = match fs::read_to_string(&projects_yml_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read project file: {}", e);
            return vec![];
        }
    };
    let projects: Vec<Project> = match serde_yaml::from_str(&projects_content) {
        Ok(projects) => projects,
        Err(e) => {
            eprintln!("Failed to deserialize project file: {}", e);
            return vec![];
        }
    };

    projects
}

#[tauri::command]
pub fn remove_project(id: String, app: tauri::AppHandle) -> bool {
    let config_dir = get_config_dir(&app);
    let projects_yml_path = Path::new(&config_dir).join("projects.yml");

    let mut projects: Vec<Project> = get_projects_vec(&app);

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

#[tauri::command]
pub fn create_project(
    dir_path: String,
    name: String,
    description: Option<String>,
    create_mc_dirs: bool,
    create_realms_dirs: bool,
    create_of_dirs: bool,
    app: tauri::AppHandle,
) -> String {
    let path = Path::new(&dir_path);

    if !path.exists() {
        simple_toast(
            "Error creating project: Selected folder does not exist".to_string(),
            app,
        );
        return serde_json::to_string(&CreateProject {
            success: false,
            id: None,
            message: Some("Selected folder does not exist".to_string()),
        })
        .unwrap_or_else(|_| "Error serializing response".to_string());
    } else if !path
        .read_dir()
        .map(|mut i| i.next().is_none())
        .unwrap_or(false)
    {
        simple_toast(
            "Error creating project: Select an empty folder".to_string(),
            app,
        );
        return serde_json::to_string(&CreateProject {
            success: false,
            id: None,
            message: Some("Select an empty folder".to_string()),
        })
        .unwrap_or_else(|_| "Error serializing response".to_string());
    }

    let config_dir = get_config_dir(&app);
    let projects_yml_path = Path::new(&config_dir).join("projects.yml");

    let mut projects: Vec<Project> = get_projects_vec(&app);

    if projects.iter().any(|p| p.path == dir_path) {
        simple_toast(
            "Error creating project: Selected path already exists in your projects".to_string(),
            app,
        );
        return serde_json::to_string(&CreateProject {
            success: false,
            id: None,
            message: Some("Selected path already exists in your projects".to_string()),
        })
        .unwrap_or_else(|_| "Error serializing response".to_string());
    }

    let project_id = format!("{}_{}", name, uuid::Uuid::new_v4());
    let project_description = description.unwrap_or("".to_string());
    let current_date = chrono::prelude::Utc::now();

    projects.push(Project {
        id: project_id.clone(),
        path: dir_path.clone(),
        name: name.clone(),
        description: Some(project_description.clone()),
        date_modified: current_date.to_string(),
        pack_image: None,
    });

    let updated_content = match serde_yaml::to_string(&projects) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to serialize project file: {}", e);
            simple_toast(format!("Error creating project: {}", e).to_string(), app);
            return serde_json::to_string(&CreateProject {
                success: false,
                id: None,
                message: Some(e.to_string()),
            })
            .unwrap_or_else(|_| "Error serializing response".to_string());
        }
    };

    let _ = fs::write(
        path.join("project.yml"),
        format!(
            "name: {}\ndesription: {}\ninput:\n\tformat: raw",
            name, project_description
        ),
    );

    if create_mc_dirs {
        try_create_directory(&path, &["assets", "minecraft"]);
        let mc_path = path.join("assets").join("minecraft");

        try_create_directory(&mc_path, &["blockstates"]);
        try_create_directory(&mc_path, &["font"]);
        try_create_directory(&mc_path, &["models", "block"]);
        try_create_directory(&mc_path, &["models", "item"]);
        try_create_directory(&mc_path, &["particles"]);
        try_create_directory(&mc_path, &["shaders"]);
        try_create_directory(&mc_path, &["sounds"]);
        try_create_directory(&mc_path, &["texts"]);
        try_create_directory(&mc_path, &["textures", "block"]);
        try_create_directory(&mc_path, &["textures", "colormap"]);
        try_create_directory(&mc_path, &["textures", "effect"]);
        try_create_directory(&mc_path, &["textures", "entity"]);
        try_create_directory(&mc_path, &["textures", "environment"]);
        try_create_directory(&mc_path, &["textures", "font"]);
        try_create_directory(&mc_path, &["textures", "gui"]);
        try_create_directory(&mc_path, &["textures", "item"]);
        try_create_directory(&mc_path, &["textures", "map"]);
        try_create_directory(&mc_path, &["textures", "misc"]);
        try_create_directory(&mc_path, &["textures", "mob_effect"]);
        try_create_directory(&mc_path, &["textures", "models"]);
        try_create_directory(&mc_path, &["textures", "painting"]);
        try_create_directory(&mc_path, &["textures", "particle"]);
    }
    if create_realms_dirs {
        try_create_directory(&path, &["assets", "realms"]);
        let realms_path = path.join("assets").join("minecraft");

        try_create_directory(&realms_path, &["textures"]);
    }
    if create_of_dirs {
        try_create_directory(&path, &["assets", "optifine"]);
        let of_path = path.join("assets").join("optifine");

        try_create_directory(&of_path, &["anim"]);
        try_create_directory(&of_path, &["cem"]);
        try_create_directory(&of_path, &["cit"]);
        try_create_directory(&of_path, &["colormap"]);
        try_create_directory(&of_path, &["ctm"]);
        try_create_directory(&of_path, &["font"]);
        try_create_directory(&of_path, &["gui"]);
        try_create_directory(&of_path, &["lightmap"]);
        try_create_directory(&of_path, &["mob"]);
        try_create_directory(&of_path, &["random"]);
        try_create_directory(&of_path, &["sky"]);
    }

    if let Err(e) = fs::write(&projects_yml_path, updated_content) {
        eprintln!("Failed to write project file: {}", e);
        simple_toast(format!("Error creating project: {}", e).to_string(), app);
        return serde_json::to_string(&CreateProject {
            success: false,
            id: None,
            message: Some(e.to_string()),
        })
        .unwrap_or_else(|_| "Error serializing response".to_string());
    }

    serde_json::to_string(&CreateProject {
        success: true,
        id: None,
        message: Some(format!("Created project: {}", name).to_string()),
    })
    .unwrap_or_else(|_| "Error serializing response".to_string())
}