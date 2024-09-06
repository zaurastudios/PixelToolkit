use std::process::Command;
#[cfg(target_os = "linux")]
use std::{
    fs,
    path::{Path, PathBuf},
};
use tauri::{Emitter, Manager};

use super::home::{get_projects_vec, Project};

#[tauri::command]
pub fn get_config_dir(app: &tauri::AppHandle) -> String {
    let config_dir = app.path().app_data_dir().unwrap_or(PathBuf::new());
    let config_dir_str = config_dir.to_string_lossy().into_owned();

    config_dir_str
}

#[tauri::command]
pub fn show_in_folder(mut path: String, is_id: Option<bool>, app: tauri::AppHandle) {
    let platform = tauri_plugin_os::platform();

    if is_id.unwrap_or(true) {
        let projects: Vec<Project> = get_projects_vec(&app);
        let filtered_projects: Vec<&Project> = projects.iter().filter(|p| p.id == path).collect();
        if !filtered_projects.is_empty() {
            path = filtered_projects[0].path.clone();
        }
    }

    if platform == "windows" {
        Command::new("explorer")
            .args(["/select,", &path]) // The comma after select is not a typo
            .spawn()
            .unwrap();
    } else if platform == "linux" {
        let new_path = match fs::metadata(&path).unwrap().is_dir() {
            true => path,
            false => {
                let mut path2 = PathBuf::from(path);
                path2.pop();
                path2.into_os_string().into_string().unwrap()
            }
        };
        Command::new("xdg-open").arg(&new_path).spawn().unwrap();
    } else if platform == "macos" {
        Command::new("open").args(["-R", &path]).spawn().unwrap();
    }
}

pub fn simple_toast(message: String, app: tauri::AppHandle) {
    app.emit("simple-toast", message).unwrap();
}

pub fn try_create_directory(base_path: &Path, subdirs: &[&str]) {
    let full_path = subdirs
        .iter()
        .fold(base_path.to_path_buf(), |mut acc, &dir| {
            acc.push(dir);
            acc
        });

    if let Err(e) = fs::create_dir_all(&full_path) {
        eprintln!(
            "Failed to create directory: {:?}, error: {:?}",
            full_path, e
        );
    }
}
