use serde_json::json;
use std::fs::{self, File};
use std::path::Path;
use tauri::Emitter;

use crate::core::utils::{get_config_dir, simple_toast};

use super::image_process::{save_channel_map, save_channel_map_split, save_normal};
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
    tauri::async_runtime::spawn(async move {
        let zip_path_str = import_zip_path_clone.unwrap_or("".to_string());
        let zip_path = Path::new(&zip_path_str);

        if zip_path.exists() {
            #[allow(unused_must_use)]
            {
                app_cloned.emit("unzip-started", true);
                let _ = unzip_and_process(zip_path, &path_clone, app_cloned.clone()).await;
                app_cloned.emit("unzip-started", false);
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
    app: tauri::AppHandle,
) -> std::io::Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

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

            let mut out_file = File::create(&out_path)?;
            std::io::copy(&mut file, &mut out_file)?;
        }
    }

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let file_name = file.mangled_name();

        if let Some(ext) = file_name.extension() {
            if file_name.to_str() != Some("pack.png") && ext == "png" {
                if !file_name.to_string_lossy().contains("_n.png")
                    && !file_name.to_string_lossy().contains("_s.png")
                {
                    process_image(&file_name, dest_dir, app.clone())?;
                }
            }
        }
    }

    Ok(())
}

pub fn process_image(
    image_path: &Path,
    dest_dir: &Path,
    app: tauri::AppHandle,
) -> std::io::Result<()> {
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

    let file_stem = image_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let image_dir = dest_dir.join(image_path.with_extension(""));

    if ignored_dirs
        .iter()
        .all(|dir| !image_dir.to_string_lossy().contains(dir))
    {
        fs::create_dir_all(&image_dir)?;

        let new_image_path = image_dir.join("color.png");
        fs::rename(dest_dir.join(&image_path), &new_image_path)?;

        let parent_dir = dest_dir.join(image_path.parent().unwrap());

        let _ = save_channel_map(&image_dir, 3, None, None, false);

        #[allow(unused_must_use)]
        {
            app.emit("unzip-progress", new_image_path);
        }

        for suffix in &["_s.png", "_n.png"] {
            let suffix_name = format!("{}{}", file_stem, suffix);
            let suffix_file = parent_dir.join(&suffix_name);
            let suffix_path = image_dir.join(&suffix_name);
            if suffix_file.exists() {
                fs::rename(suffix_file, &suffix_path)?;
            }

            #[allow(unused_must_use)]
            {
                app.emit("unzip-progress", &suffix_path);
            }

            if suffix == &"_n.png" {
                let _ = save_channel_map(
                    &image_dir,
                    2,
                    Some(suffix_name.to_owned()),
                    Some(String::from("ao.png")),
                    false,
                );
                let _ = save_channel_map(
                    &image_dir,
                    3,
                    Some(suffix_name.to_owned()),
                    Some(String::from("height.png")),
                    false,
                );

                let _ = save_normal(
                    &image_dir,
                    Some(suffix_name.to_owned()),
                    Some(String::from("normal.png")),
                );

                let _ = fs::remove_file(&suffix_path);
            } else if suffix == &"_s.png" {
                let _ = save_channel_map(
                    &image_dir,
                    0,
                    Some(suffix_name.to_owned()),
                    Some(String::from("smooth.png")),
                    true,
                );
                let _ = save_channel_map(
                    &image_dir,
                    3,
                    Some(suffix_name.to_owned()),
                    Some(String::from("emissive.png")),
                    true,
                );

                let _ = save_channel_map_split(
                    &image_dir,
                    1,
                    0,
                    229,
                    Some(suffix_name.to_owned()),
                    Some(String::from("f0.png")),
                    Some(String::from("hcm.png")),
                    true,
                    false,
                );
                let _ = save_channel_map_split(
                    &image_dir,
                    3,
                    0,
                    127,
                    Some(suffix_name.to_owned()),
                    Some(String::from("porosity.png")),
                    Some(String::from("sss.png")),
                    true,
                    false,
                );

                let _ = fs::remove_file(&suffix_path);
            }
        }

        let _ = fs::write(image_dir.join("mat.yml"), "");
    }

    Ok(())
}
