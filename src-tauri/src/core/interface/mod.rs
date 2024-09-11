pub mod structs;

use std::{
    fs::{self, DirEntry},
    path::Path,
};

use base64::{engine::general_purpose, Engine};
use image::{DynamicImage, ImageBuffer, Luma};
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

    let texture_file = TEXTURE_FILES
        .iter()
        .find(|t| t.name == texture)
        .ok_or_else(|| {
            format!(
                "Failed to find any texture file with that name: {}",
                texture
            )
        })?;

    let find_matching_file = |pattern: &str| -> Option<DirEntry> {
        fs::read_dir(path)
            .ok()?
            .filter_map(Result::ok)
            .find(|entry| {
                entry.file_name().to_str().map_or(false, |file_name| {
                    Regex::new(pattern).ok().unwrap().is_match(file_name)
                })
            })
    };

    let file = find_matching_file(texture_file.pattern)
        .or_else(|| {
            texture_file
                .alternate
                .and_then(|alt| find_matching_file(alt))
        })
        .ok_or_else(|| {
            format!(
                "No file matching the pattern for '{}' found in the directory.",
                texture
            )
        })?;

    send_base64_image(file, texture_file, app)
}

fn send_base64_image(
    file: DirEntry,
    texture_file: &TextureFile,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let img = image::open(file.path()).map_err(|e| {
        simple_toast(format!("Failed to open image: {}", e), app.clone());
        format!("Failed to open image: {}", e)
    })?;

    let mut buf = vec![];
    let mut cursor = std::io::Cursor::new(&mut buf);

    if texture_file.greyscale {
        let luma_img = img.into_luma8();
        let final_img = if texture_file.alternate.is_some() {
            invert_grayscale_image(luma_img)
        } else {
            DynamicImage::ImageLuma8(luma_img)
        };
        final_img.write_to(&mut cursor, image::ImageFormat::Png)
    } else {
        img.write_to(&mut cursor, image::ImageFormat::Png)
    }
    .map_err(|e| format!("Failed to write image: {}", e))?;

    let res_base64 = general_purpose::STANDARD.encode(&buf);

    app.emit("selected-texture-file", res_base64.clone())
        .map_err(|e| format!("Failed to emit event: {}", e))?;

    Ok(res_base64)
}
fn invert_grayscale_image(img: ImageBuffer<Luma<u8>, Vec<u8>>) -> DynamicImage {
    let (width, height) = img.dimensions();
    let inverted = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel = img.get_pixel(x, y);
        Luma([255 - pixel[0]])
    });
    DynamicImage::ImageLuma8(inverted)
}

struct TextureFile {
    name: &'static str,
    pattern: &'static str,
    greyscale: bool,
    alternate: Option<&'static str>,
}

const TEXTURE_FILES: [TextureFile; 13] = [
    TextureFile {
        name: "color",
        pattern: r".*(albedo|color).*\.png$",
        greyscale: false,
        alternate: None,
    },
    TextureFile {
        name: "opacity",
        pattern: r".*opacity.*\.png$",
        greyscale: true,
        alternate: None,
    },
    TextureFile {
        name: "height",
        pattern: r".*height.*\.png$",
        greyscale: true,
        alternate: None,
    },
    TextureFile {
        name: "normal",
        pattern: r".*normal.*\.png$",
        greyscale: false,
        alternate: None,
    },
    TextureFile {
        name: "occlusion",
        pattern: r".*(occlusion|ao).*\.png$",
        greyscale: true,
        alternate: None,
    },
    TextureFile {
        name: "smooth",
        pattern: r".*smooth.*\.png$",
        greyscale: true,
        alternate: Some(r".*rough.*\.png$"),
    },
    TextureFile {
        name: "rough",
        pattern: r".*rough.*\.png$",
        greyscale: true,
        alternate: Some(r".*smooth.*\.png$"),
    },
    TextureFile {
        name: "metal",
        pattern: r".*metal.*\.png$",
        greyscale: true,
        alternate: None,
    },
    TextureFile {
        name: "hcm",
        pattern: r".*hcm.*\.png$",
        greyscale: true,
        alternate: None,
    },
    TextureFile {
        name: "f0",
        pattern: r".*f0.*\.png$",
        greyscale: true,
        alternate: None,
    },
    TextureFile {
        name: "porosity",
        pattern: r".*porosity.*\.png$",
        greyscale: true,
        alternate: None,
    },
    TextureFile {
        name: "sss",
        pattern: r".*sss.*\.png$",
        greyscale: true,
        alternate: None,
    },
    TextureFile {
        name: "emissive",
        pattern: r".*emissive.*\.png$",
        greyscale: true,
        alternate: None,
    },
];
