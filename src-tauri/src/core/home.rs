use anyhow::Result;
use serde_json::json;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use tauri::Emitter;

use crate::core::image_process::{save_f0_hcm, save_porosity_sss};
use crate::core::utils::{get_config_dir, simple_toast};

use super::image_process::{save_channel_map, save_normal};
use super::project::structs::{Input, ProjectYml};
use super::project::update_project;
use super::utils::try_create_directory;

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
    import_zip_path: Option<String>,
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
    }

    if path.read_dir().unwrap().count() != 0 {
        simple_toast(
            "Error creating project: Selected folder is not empty".to_string(),
            app,
        );
        return serde_json::to_string(&CreateProject {
            success: false,
            id: None,
            message: Some("Selected folder is not empty".to_string()),
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

    let project_id = format!("{}", uuid::Uuid::new_v4());
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

        let dirs = [
            "blockstates",
            "font",
            "models/block",
            "models/item",
            "particles",
            "shaders",
            "sounds",
            "texts",
            "textures/block",
            "textures/colormap",
            "textures/effect",
            "textures/entity",
            "textures/environment",
            "textures/font",
            "textures/gui",
            "textures/item",
            "textures/map",
            "textures/misc",
            "textures/mob_effect",
            "textures/models",
            "textures/painting",
            "textures/particle",
        ];

        for dir in &dirs {
            try_create_directory(&mc_path, &dir.split('/').collect::<Vec<_>>());
        }
    }
    if create_realms_dirs {
        try_create_directory(&path, &["assets", "realms"]);
        let realms_path = path.join("assets").join("minecraft");

        try_create_directory(&realms_path, &["textures"]);
    }
    if create_of_dirs {
        try_create_directory(&path, &["assets", "optifine"]);
        let of_path = path.join("assets").join("optifine");

        let dirs = [
            "anim", "cem", "cit", "colormap", "ctm", "font", "gui", "lightmap", "mob", "random",
            "sky",
        ];
        for dir in &dirs {
            try_create_directory(&of_path, &[dir]);
        }
    }

    let path_clone = path.to_path_buf();
    let import_zip_path_clone = import_zip_path.clone();
    let app_cloned = app.clone();
    let project_id_clone = project_id.clone();
    let project_name_clone = name.clone();
    tauri::async_runtime::spawn(async move {
        let zip_path_str = import_zip_path_clone.unwrap_or("".to_string());
        let zip_path = Path::new(&zip_path_str);

        if zip_path.exists() {
            #[allow(unused_must_use)]
            {
                app_cloned.emit("unzip-started", true);
                let _ = unzip_and_process(
                    zip_path,
                    &path_clone,
                    project_id_clone,
                    project_name_clone,
                    app_cloned.clone(),
                )
                .await;
                app_cloned.emit("unzip-started", false);
                app_cloned.emit("resync_dir_fe", false);
            }
        }
    });

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
        id: Some(project_id),
        message: Some(format!("Created project: {}", name).to_string()),
    })
    .unwrap_or_else(|_| "Error serializing response".to_string())
}

async fn unzip_and_process(
    zip_path: &Path,
    dest_dir: &Path,
    project_id: String,
    project_name: String,
    app: tauri::AppHandle,
) -> Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    let mut tasks = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.mangled_name();
        let out_path = dest_dir.join(&file_name);

        if file_name.to_string_lossy().ends_with('/') {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            let out_path_clone = out_path.clone();
            let dest_dir_clone = dest_dir.to_path_buf();
            let app_clone = app.clone();
            let project_id_clone = project_id.clone();
            let project_name_clone = project_name.clone();

            let task = tokio::spawn(async move {
                let mut out_file = File::create(&out_path_clone)?;
                out_file.write_all(&buffer)?;

                if out_path_clone.to_string_lossy().contains("pack.mcmeta") {
                    let mc_meta_str = fs::read_to_string(&out_path_clone).unwrap();

                    let mc_meta: McMeta = serde_json::from_str(&mc_meta_str)
                        .expect("Failed to deserialize project file");

                    let mut name = mc_meta.pack.name.unwrap_or(project_name_clone.clone());
                    if name != "My New Project" {
                        name = project_name_clone;
                    }

                    let new_project_yml = ProjectYml {
                        name,
                        description: Some(mc_meta.pack.description.unwrap_or("".to_string())),
                        input: Input {
                            format: "labpbr-1.3".to_string(),
                        },
                        profiles: None,
                        tags: None,
                    };
                    update_project(project_id_clone, new_project_yml, app_clone.clone());
                }

                if let Some(ext) = out_path_clone.extension() {
                    if out_path_clone.file_name() != Some(std::ffi::OsStr::new("pack.png"))
                        && ext == "png"
                    {
                        let _ = process_image(&out_path_clone, &dest_dir_clone, app_clone);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });

            tasks.push(task);
        }
    }

    for task in tasks {
        task.await??;
    }

    Ok(())
}

fn process_image(image_path: &Path, dest_dir: &Path, app: tauri::AppHandle) -> Result<()> {
    let ignored_dirs = [
        "colormap",
        "effect",
        "environment",
        "font",
        "gui",
        "map",
        "misc",
        "mob_effect",
        "models",
    ];

    let file_name = image_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let image_path_str = image_path.to_string_lossy().to_string();

    if ignored_dirs.iter().any(|dir| image_path_str.contains(dir)) {
        return Ok(());
    }

    let remove_suffix = |s: &str| s.replace("_n.png", ".png").replace("_s.png", ".png");
    let material_dir = dest_dir.join(remove_suffix(&image_path_str).replace(".png", ""));

    fs::create_dir_all(&material_dir)?;
    fs::write(material_dir.join("mat.yml"), "")?;

    let image_path_combined = dest_dir.join(image_path);

    app.emit("unzip-progress", material_dir.to_string_lossy().to_string())?;

    match file_name.as_str() {
        name if name.ends_with("_n.png") => {
            let _ = save_channel_map(
                &material_dir,
                2,
                &image_path_combined,
                String::from("ao.png"),
                false,
            );
            let _ = save_channel_map(
                &material_dir,
                3,
                &image_path_combined,
                String::from("height.png"),
                false,
            );
            let _ = save_normal(
                &material_dir,
                &image_path_combined,
                String::from("normal.png"),
            );

            fs::remove_file(&image_path_combined)?;
        }
        name if name.ends_with("_s.png") => {
            let _ = save_channel_map(
                &material_dir,
                0,
                &image_path_combined,
                String::from("smooth.png"),
                false,
            );
            let _ = save_channel_map(
                &material_dir,
                3,
                &image_path_combined,
                String::from("emissive.png"),
                true,
            );
            let _ = save_f0_hcm(&material_dir, &image_path_combined);
            let _ = save_porosity_sss(&material_dir, &image_path_combined);

            fs::remove_file(&image_path_combined)?;
        }
        _ => {
            fs::rename(&image_path_combined, material_dir.join("color.png"))?;
            let _ = save_channel_map(
                &material_dir,
                3,
                &material_dir.join("color.png"),
                String::from("opacity.png"),
                false,
            );
        }
    }

    Ok(())
}

#[tauri::command]
pub fn create_project_existing(
    project_yml_path: String,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let path = Path::new(&project_yml_path);
    if !path.exists() {
        return Err(serde_json::to_string(&CreateExistingRes {
            success: false,
            id: None,
            message: String::from("Failed to create project. Selected project.yml does not exist."),
        })
        .unwrap_or_else(|_| "Error serializing response".to_string()));
    }

    let project_yml_str = fs::read_to_string(path).map_err(|e| {
        eprintln!("Failed to read project.yml: {}", e);
        return serde_json::to_string(&CreateExistingRes {
            success: false,
            id: None,
            message: String::from(format!("Failed to read project.yml: {}", e)),
        })
        .unwrap_or_else(|_| "Error serializing response".to_string());
    })?;

    let project_yml: ProjectYml = serde_yaml::from_str(&project_yml_str)
        .map_err(|e| format!("Failed to deserialize project file: {}", e))?;
    let project_path = project_yml_path.replace("/project.yml", "");
    let project_id = String::from(uuid::Uuid::new_v4());
    let project_name = project_yml.name;
    let project_desc = project_yml.description.unwrap_or("".to_string());
    let date_modified = chrono::prelude::Utc::now();

    let config_dir = get_config_dir(&app);
    let projects_yml_path = Path::new(&config_dir).join("projects.yml");

    let mut projects: Vec<Project> = get_projects_vec(&app);
    if projects.iter().any(|p| p.path == project_path) {
        return Err(serde_json::to_string(&CreateProject {
            success: false,
            id: None,
            message: Some("Selected path already exists in your projects".to_string()),
        })
        .unwrap_or_else(|_| "Error serializing response".to_string()));
    }

    projects.push(Project {
        id: project_id.clone(),
        path: project_path,
        name: project_name,
        description: Some(project_desc),
        date_modified: date_modified.to_string(),
        pack_image: None,
    });

    let updated_content = match serde_yaml::to_string(&projects) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to serialize project file: {}", e);
            return Err(serde_json::to_string(&CreateExistingRes {
                success: false,
                id: None,
                message: e.to_string(),
            })
            .unwrap_or_else(|_| "Error serializing response".to_string()));
        }
    };

    if let Err(e) = fs::write(&projects_yml_path, updated_content) {
        eprintln!("Failed to write project file: {}", e);
        return Err(serde_json::to_string(&CreateExistingRes {
            success: false,
            id: None,
            message: format!("Failed to write to your projects: {}", e.to_string()).to_string(),
        })
        .unwrap_or_else(|_| "Error serializing response".to_string()));
    }

    Ok(serde_json::to_string(&CreateExistingRes {
        success: true,
        id: Some(project_id),
        message: String::from("Successfully added project"),
    })
    .unwrap_or_else(|_| "Error serializing response".to_string()))
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct McMeta {
    pack: Pack,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Pack {
    name: Option<String>,
    description: Option<String>,
    pack_format: Option<u8>,
}

#[derive(serde::Serialize)]
struct CreateExistingRes {
    success: bool,
    id: Option<String>,
    message: String,
}
