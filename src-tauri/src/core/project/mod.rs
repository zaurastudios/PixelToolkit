pub mod structs;

use std::{fs, path::Path};

use structs::FileTree;

use super::home::{get_projects_vec, remove_project, Project};

#[derive(serde::Serialize)]
struct GetDirsResponse {
    file_tree: Option<FileTree>,
    redirect: bool,
}

fn build_file_tree(path: &Path) -> FileTree {
    let name = path.file_name().unwrap().to_str().unwrap().to_string();
    let mut is_mat = None;
    let mut children = Vec::new();

    if path.is_dir() {
        let mat_files = ["mat.yml", "mat.yaml", "material.yml", "material.yaml"];
        let has_mat_file = mat_files.iter().any(|&file| path.join(file).exists());

        if has_mat_file {
            is_mat = Some(true);
        } else {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.filter_map(Result::ok) {
                    let child_path = entry.path();
                    if child_path.is_dir() {
                        children.push(build_file_tree(&child_path));
                    }
                }
            }
        }
    }

    FileTree {
        name,
        is_mat,
        children,
    }
}

#[tauri::command]
pub fn get_dirs(project_id: String, app: tauri::AppHandle) -> String {
    let projects: Vec<Project> = get_projects_vec(&app);
    let filtered_projects: Vec<&Project> = projects.iter().filter(|p| p.id == project_id).collect();

    if filtered_projects.is_empty() {
        // remove_project(project_id, app);
        return serde_json::to_string(&GetDirsResponse {
            file_tree: None,
            redirect: false,
        })
        .unwrap_or_else(|_| "Error serializing response".to_string());
    }

    let project = filtered_projects[0];
    let path = Path::new(&project.path);

    let project_yml_path = path.join("project.yml");
    if project_yml_path.try_exists().is_err() {
        // remove_project(project_id, app);
        return serde_json::to_string(&GetDirsResponse {
            file_tree: None,
            redirect: false,
        })
        .unwrap_or_else(|_| "Error serializing response".to_string());
    }

    let file_tree = build_file_tree(path);

    serde_json::to_string(&GetDirsResponse {
        file_tree: Some(file_tree),
        redirect: false,
    })
    .unwrap_or_else(|_| "Error serializing response".to_string())
}
