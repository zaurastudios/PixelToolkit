pub mod structs;

use std::{fs, path::Path, str::FromStr};

use base64::{engine::general_purpose, Engine};
use regex::Regex;
use structs::MatYml;
use tauri::Emitter;

use super::utils::simple_toast;

#[tauri::command]
pub fn select_texture(material_path: String, app: tauri::AppHandle) -> Result<String, String> {
    let path = Path::new(&material_path);
    if !path.exists() {
        return Err(String::from("Selected path does not exist."));
    }

    let mat_yml = fs::read_to_string(path)
        .map_err(|e| {
            println!("Failed to read mat.yml: {}", e);
            return format!("Failed to read mat.yml: {}", e).to_string();
        })
        .unwrap();
    let _: MatYml = serde_yaml::from_str(&mat_yml)
        .map_err(|e| {
            println!("Failed to serialise mat.yml: {}", e);
            return format!("Failed to serialise mat.yml: {}", e).to_string();
        })
        .unwrap();

    let _ = app.emit(
        "selected-texture",
        path.to_string_lossy().replace("mat.yml", "").to_string(),
    );

    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn select_texture_file(
    material_path: String,
    texture: String,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let path = Path::new(&material_path);
    if !path.exists() {
        return Err(String::from("Selected path does not exist."));
    }

    let texture_file: &TextureFile = match TEXTURE_FILES.iter().find(|t| t.name == texture) {
        Some(tf) => tf,
        None => {
            return Err(format!(
                "Failed to find any texture file with that name: {}",
                texture
            ));
        }
    };

    let matching_file = fs::read_dir(path)
        .map_err(|e| format!("Failed to read directory: {}", e))?
        .filter_map(Result::ok)
        .find(|entry| {
            if let Some(file_name) = entry.file_name().to_str() {
                Regex::from_str(texture_file.pattern)
                    .unwrap()
                    .is_match(file_name)
            } else {
                false
            }
        });

    match matching_file {
        Some(file) => {
            let img = image::open(file.path())
                .map_err(|e| {
                    simple_toast(format!("Failed to open image: {}", e), app.clone());
                    return format!("Failed to open image: {}", e);
                })
                .unwrap();

            let mut buf = vec![];
            let mut cursor = std::io::Cursor::new(&mut buf);

            if texture_file.greyscale {
                img.into_luma8()
                    .write_to(&mut cursor, image::ImageFormat::Png)
                    .unwrap();
            } else {
                img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
            }

            let res_base64 = general_purpose::STANDARD.encode(&buf);

            let _ = app.emit("selected-texture-file", res_base64.clone());
            return Ok(res_base64);
        }

        None => {
            return Err(format!(
                "No file matching the pattern for '{}' found in the directory.",
                texture
            ))
        }
    };
}

struct TextureFile {
    name: &'static str,
    pattern: &'static str,
    greyscale: bool,
}

const TEXTURE_FILES: [TextureFile; 13] = [
    TextureFile {
        name: "color",
        pattern: r".*(albedo|color).*\.png$",
        greyscale: false,
    },
    TextureFile {
        name: "opacity",
        pattern: r".*opacity.*\.png$",
        greyscale: true,
    },
    TextureFile {
        name: "height",
        pattern: r".*height.*\.png$",
        greyscale: true,
    },
    TextureFile {
        name: "normal",
        pattern: r".*normal.*\.png$",
        greyscale: false,
    },
    TextureFile {
        name: "occlusion",
        pattern: r".*(occlusion|ao).*\.png$",
        greyscale: true,
    },
    TextureFile {
        name: "smooth",
        pattern: r".*smooth.*\.png$",
        greyscale: true,
    },
    TextureFile {
        name: "rough",
        pattern: r".*rough.*\.png$",
        greyscale: true,
    },
    TextureFile {
        name: "metal",
        pattern: r".*metal.*\.png$",
        greyscale: true,
    },
    TextureFile {
        name: "hcm",
        pattern: r".*hcm.*\.png$",
        greyscale: true,
    },
    TextureFile {
        name: "f0",
        pattern: r".*f0.*\.png$",
        greyscale: true,
    },
    TextureFile {
        name: "porosity",
        pattern: r".*porosity.*\.png$",
        greyscale: true,
    },
    TextureFile {
        name: "sss",
        pattern: r".*sss.*\.png$",
        greyscale: true,
    },
    TextureFile {
        name: "emissive",
        pattern: r".*emissive.*\.png$",
        greyscale: true,
    },
];
