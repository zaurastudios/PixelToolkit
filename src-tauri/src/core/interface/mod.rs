pub mod structs;

use std::{
    fs::{self, DirEntry},
    path::Path,
};

use super::utils::simple_toast;
use base64::{engine::general_purpose, Engine};
use image::{DynamicImage, ImageBuffer, Luma, Pixel};
use regex::Regex;
use structs::{DefaultsGrayscale, ExtendedGrayscale, MatYml, TextureFile, TEXTURE_FILES};
use tauri::Emitter;

#[tauri::command]
pub fn select_texture(material_path: String, app: tauri::AppHandle) -> Result<String, String> {
    let path = Path::new(&material_path);
    if !path.exists() {
        return Err("Selected path does not exist.".into());
    }

    let mat_yml = fs::read_to_string(path).map_err(|e| format!("Failed to read mat.yml: {}", e))?;

    let _: MatYml = serde_yaml::from_str(&mat_yml)
        .map_err(|e| format!("Failed to serialize mat.yml: {}", e))?;

    app.emit(
        "selected-texture",
        path.to_string_lossy().replace("mat.yml", "").to_string(),
    )
    .map_err(|e| format!("Failed to emit event: {}", e))?;

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
        return Err("Selected path does not exist.".into());
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

    let mat_yml_str = fs::read_to_string(path.join("mat.yml"))
        .map_err(|e| format!("Failed to read mat.yml file {}: {}", material_path, e))?;

    let mat_yml: MatYml = serde_yaml::from_str(&mat_yml_str)
        .map_err(|e| format!("Failed to deserialize mat.yml file: {}", e))?;

    let find_matching_file = |pattern: &str| -> Option<DirEntry> {
        fs::read_dir(path)
            .ok()?
            .filter_map(Result::ok)
            .find(|entry| {
                entry.file_name().to_str().map_or(false, |file_name| {
                    Regex::new(pattern)
                        .ok()
                        .map_or(false, |regex| regex.is_match(file_name))
                })
            })
    };

    let original_exists = find_matching_file(&texture_file.pattern).is_some();

    if original_exists {
        let file = find_matching_file(&texture_file.pattern).ok_or(format!(
            "No file matching the pattern for '{}' found in the directory.",
            texture
        ))?;
        send_base64_image(file, texture_file, true, mat_yml.clone(), app)?;
    } else if let Some(alternate) = &texture_file.alternate {
        if let Some(file) = find_matching_file(alternate) {
            send_base64_image(file, texture_file, false, mat_yml.clone(), app)?;
        } else {
            send_base64_image_default(texture_file, original_exists, app)?;
        }
    } else {
        send_base64_image_default(texture_file, original_exists, app)?;
    }

    let texture_properties = match texture_file.name {
        "opacity" => mat_yml.opacity,
        "rough" => mat_yml.rough,
        "smooth" => mat_yml.smooth,
        "porosity" => mat_yml.porosity,
        "sss" => mat_yml.sss,
        "emissive" => mat_yml.emissive,
        _ => None,
    };

    let unwrapped_values = DefaultsGrayscale {
        value: Some(
            texture_properties
                .as_ref()
                .and_then(|v| v.value)
                .unwrap_or(0.0),
        ),
        shift: Some(
            texture_properties
                .as_ref()
                .and_then(|v| v.shift)
                .unwrap_or(0.0),
        ),
        scale: Some(
            texture_properties
                .as_ref()
                .and_then(|v| v.scale)
                .unwrap_or(1.0),
        ),
    };

    let extended_res = ExtendedGrayscale {
        use_og: !original_exists && texture_file.alternate.is_some(),
        values: unwrapped_values,
    };

    let serialized_values = serde_json::to_string(&extended_res)
        .map_err(|e| format!("Failed to serialize texture properties: {}", e))?;

    Ok(serialized_values)
}

fn send_base64_image(
    file: DirEntry,
    texture_file: &TextureFile,
    use_og: bool,
    mat_yml: MatYml,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let img = image::open(file.path()).map_err(|e| {
        simple_toast(format!("Failed to open image: {}", e), app.clone());
        format!("Failed to open image: {}", e)
    })?;

    let mut buf = vec![];
    let mut cursor = std::io::Cursor::new(&mut buf);
    let mut luma_img = img.clone().into_luma8();

    if texture_file.grayscale {
        if let Some(texture_properties) = match texture_file.name {
            "opacity" => mat_yml.opacity,
            "rough" => {
                if use_og {
                    mat_yml.rough
                } else {
                    mat_yml.smooth
                }
            }
            "smooth" => {
                if use_og {
                    mat_yml.smooth
                } else {
                    mat_yml.rough
                }
            }
            "porosity" => mat_yml.porosity,
            "sss" => mat_yml.sss,
            "emissive" => mat_yml.emissive,
            _ => None,
        } {
            let value = texture_properties.value.unwrap_or(0.0);
            let shift = texture_properties.shift.unwrap_or(0.0);
            let scale = texture_properties.scale.unwrap_or(1.0);

            for pixel in luma_img.pixels_mut() {
                if value != 0.0 {
                    pixel.0[0] = value.clamp(0.0, 255.0) as u8;
                } else {
                    let current_value = pixel.0[0] as f32 / 255.0;
                    let new_value = ((current_value + shift) * scale).clamp(0.0, 1.0);
                    pixel.0[0] = (new_value * 255.0) as u8;
                }
            }
        }
    }

    let final_img = if !use_og {
        invert_img(luma_img)
    } else {
        DynamicImage::ImageLuma8(luma_img)
    };

    final_img
        .write_to(&mut cursor, image::ImageFormat::Png)
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
        texture_file.default_color.map(|e| 255 - e)
    } else {
        texture_file.default_color
    };

    let img = ImageBuffer::from_pixel(16, 16, color);
    let dynamic_img = DynamicImage::ImageRgba8(img);
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

#[tauri::command]
pub fn update_defaults_grayscale(
    material_path: String,
    texture: String,
    value: String,
    shift: String,
    scale: String,
) -> Result<bool, String> {
    let path = Path::new(&material_path);
    if !path.exists() {
        return Err("Selected path does not exist.".into());
    }

    let mat_yml_str = fs::read_to_string(path.join("mat.yml"))
        .map_err(|e| format!("Failed to read mat.yml file {}: {}", material_path, e))?;

    let mut mat_yml: MatYml = serde_yaml::from_str(&mat_yml_str)
        .map_err(|e| format!("Failed to deserialize mat.yml file: {}", e))?;

    let parsed_value = value.parse::<f32>().unwrap_or(0.0);
    let parsed_shift = shift.parse::<f32>().unwrap_or(0.0);
    let parsed_scale = scale.parse::<f32>().unwrap_or(0.0);

    match texture.as_str() {
        "opacity" => update_texture_property(
            &mut mat_yml.opacity,
            parsed_value,
            parsed_shift,
            parsed_scale,
        ),
        "rough" => {
            update_texture_property(&mut mat_yml.rough, parsed_value, parsed_shift, parsed_scale)
        }
        "smooth" => update_texture_property(
            &mut mat_yml.smooth,
            parsed_value,
            parsed_shift,
            parsed_scale,
        ),
        "porosity" => update_texture_property(
            &mut mat_yml.porosity,
            parsed_value,
            parsed_shift,
            parsed_scale,
        ),
        "sss" => {
            update_texture_property(&mut mat_yml.sss, parsed_value, parsed_shift, parsed_scale)
        }
        "emissive" => update_texture_property(
            &mut mat_yml.emissive,
            parsed_value,
            parsed_shift,
            parsed_scale,
        ),
        _ => {}
    }

    let updated_mat_yml = serde_yaml::to_string(&mat_yml)
        .map_err(|e| format!("Failed to serialize mat.yml file: {}", e))?;

    fs::write(&path.join("mat.yml"), updated_mat_yml).map_err(|e| e.to_string())?;

    Ok(true)
}

fn update_texture_property(
    property: &mut Option<DefaultsGrayscale>,
    value: f32,
    shift: f32,
    scale: f32,
) {
    if let Some(prop) = property.as_mut() {
        prop.value = Some(value);
        prop.shift = Some(shift);
        prop.scale = Some(scale);
    } else {
        *property = Some(DefaultsGrayscale {
            value: Some(value),
            shift: Some(shift),
            scale: Some(scale),
        });
    }
}
