pub mod structs;

use std::{
    fs::{self, File},
    path::Path,
};

use structs::FileTree;

use super::{
    home::{get_projects_vec, process_image, remove_project},
    utils::get_config_dir,
};

#[derive(serde::Serialize)]
struct GetDirsResponse {
    file_tree: Option<FileTree>,
    redirect: bool,
    project_path: String,
}

fn build_file_tree(path: &Path) -> Option<FileTree> {
    let name = path.file_name().unwrap().to_str().unwrap().to_string();
    let mut is_mat = None;
    let mut children = Vec::new();

    if path.is_dir() {
        let mat_files = ["mat.yml", "mat.yaml", "material.yml", "material.yaml"];
        let has_mat_file = mat_files.iter().any(|&file| path.join(file).exists());

        if has_mat_file {
            is_mat = Some(true);
            Some(FileTree {
                name,
                is_mat,
                children,
            })
        } else {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.filter_map(Result::ok) {
                    let child_path = entry.path();
                    if child_path.is_dir() {
                        if let Some(child_tree) = build_file_tree(&child_path) {
                            children.push(child_tree);
                        }
                    }
                }
            }

            children.sort_by(|a, b| a.name.cmp(&b.name));

            if children.is_empty() {
                None
            } else {
                Some(FileTree {
                    name,
                    is_mat,
                    children,
                })
            }
        }
    } else {
        None
    }
}

#[tauri::command]
pub fn get_dirs(project_id: String, app: tauri::AppHandle) -> Result<String, String> {
    let mut projects = get_projects_vec(&app);
    let project = projects
        .iter_mut()
        .find(|p| p.id == project_id)
        .ok_or_else(|| {
            remove_project(project_id.clone(), app.clone());
            "Project not found".to_string()
        })?;

    update_project_modified_date(project_id.clone(), &app)?;

    let path = Path::new(&project.path);
    let project_yml_path = path.join("project.yml");
    if !project_yml_path.exists() {
        remove_project(project_id, app);
        return Err("Project file not found".to_string());
    }

    let file_tree = build_file_tree(path).unwrap_or_else(|| FileTree {
        name: "".to_string(),
        is_mat: None,
        children: Vec::new(),
    });
    let response = GetDirsResponse {
        file_tree: Some(file_tree),
        redirect: false,
        project_path: project.path.clone(),
    };

    serde_json::to_string(&response).map_err(|_| "Error serializing response".to_string())
}

fn update_project_modified_date(project_id: String, app: &tauri::AppHandle) -> Result<(), String> {
    let current_date = chrono::Utc::now();

    let config_dir = get_config_dir(app);
    let projects_yml_path = Path::new(&config_dir).join("projects.yml");
    let mut projects = get_projects_vec(app);
    for p in projects.iter_mut() {
        if p.id == project_id {
            p.date_modified = current_date.to_string();
        }
    }

    let updated_content = serde_yaml::to_string(&projects)
        .map_err(|e| format!("Failed to serialize project file: {}", e))?;

    fs::write(&projects_yml_path, updated_content)
        .map_err(|e| format!("Failed to write project file: {}", e))
}

#[tauri::command]
pub fn update_dirs(project_id: String, app: tauri::AppHandle) -> Result<String, String> {
    let mut projects = get_projects_vec(&app);
    let project = projects
        .iter_mut()
        .find(|p| p.id == project_id)
        .ok_or_else(|| {
            remove_project(project_id.clone(), app.clone());
            "Project not found".to_string()
        })?;

    let path = Path::new(&project.path);

    let file_tree = build_file_tree(path).unwrap_or_else(|| FileTree {
        name: "".to_string(),
        is_mat: None,
        children: Vec::new(),
    });

    let response = GetDirsResponse {
        file_tree: Some(file_tree),
        redirect: false,
        project_path: project.path.clone(),
    };

    serde_json::to_string(&response).map_err(|_| "Error serializing response".to_string())
}

#[tauri::command]
pub async fn import_archive_overwrite(
    zip_path: String,
    project_id: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let file = File::open(zip_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    let mut projects = get_projects_vec(&app);
    let project = projects
        .iter_mut()
        .find(|p| p.id == project_id)
        .ok_or_else(|| {
            remove_project(project_id.clone(), app.clone());
            "Project not found".to_string()
        })?;
    let dest_dir = Path::new(&project.path);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let file_name = file.mangled_name();
        let out_path = dest_dir.join(&file_name);
        if file_name.to_string_lossy().ends_with('/') {
            fs::create_dir_all(&out_path).map_err(|e| e.to_string())?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut out_file = File::create(&out_path).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut out_file).map_err(|e| e.to_string())?;
        }
    }

    for i in 0..archive.len() {
        let file = archive.by_index(i).map_err(|e| e.to_string())?;
        let file_name = file.mangled_name();
        if let Some(ext) = file_name.extension() {
            if file_name.to_str() != Some("pack.png") && ext == "png" {
                if !file_name.to_string_lossy().contains("_n.png")
                    && !file_name.to_string_lossy().contains("_s.png")
                {
                    let _ = process_image(&file_name, dest_dir, app.clone());
                }
            }
        }
    }

    Ok(())
}
