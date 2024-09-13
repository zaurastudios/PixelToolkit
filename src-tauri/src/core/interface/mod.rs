pub mod structs;

use std::{
    fs::{self, DirEntry},
    path::Path,
};

use base64::{engine::general_purpose, Engine};
use image::{DynamicImage, ImageBuffer, Luma, Pixel};
use regex::Regex;
use structs::{MatYml, TextureFile, TEXTURE_FILES};
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

    let original_exists = find_matching_file(texture_file.pattern).is_some();

    if original_exists {
        let file = find_matching_file(&texture_file.pattern).ok_or(format!(
            "No file matching the pattern for '{}' found in the directory.",
            texture
        ))?;
        return send_base64_image(file, texture_file, true, app);
    } else if !original_exists && texture_file.alternate.is_some() {
        let file = find_matching_file(texture_file.alternate.unwrap());
        match file {
            Some(file) => return send_base64_image(file, texture_file, false, app),
            None => return send_base64_image_default(texture_file, original_exists, app),
        }
    } else {
        Err(String::from("Failed to get texture."))
    }
}

fn send_base64_image(
    file: DirEntry,
    texture_file: &TextureFile,
    use_og: bool,
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
        let final_img = if !use_og {
            invert_img(luma_img)
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

fn send_base64_image_default(
    texture_file: &TextureFile,
    use_og: bool,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let color = if texture_file.alternate.is_some() && !use_og {
        texture_file.default_color.map_without_alpha(|e| 255 - e)
    } else {
        texture_file.default_color
    };
    let img = image::ImageBuffer::from_pixel(16, 16, color);
    let dynamic_img = image::DynamicImage::ImageRgba8(img);

    let mut buf = vec![];
    let mut cursor = std::io::Cursor::new(&mut buf);
    dynamic_img
        .write_to(&mut cursor, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to write default image: {}", e))?;

    let res_base64 = general_purpose::STANDARD.encode(&buf);

    app.emit("selected-texture-file", res_base64.clone())
        .map_err(|e| format!("Failed to emit event: {}", e))?;

    Ok(res_base64)
}

fn invert_img(img: ImageBuffer<Luma<u8>, Vec<u8>>) -> DynamicImage {
    let (width, height) = img.dimensions();
    let inverted = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel = img.get_pixel(x, y);
        Luma([255 - pixel[0]])
    });
    DynamicImage::ImageLuma8(inverted)
}