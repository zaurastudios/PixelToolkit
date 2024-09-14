pub mod structs;

use std::{
    fs::{self, DirEntry},
    path::Path,
};

use base64::{engine::general_purpose, Engine};
use image::{DynamicImage, ImageBuffer, Luma, Pixel};
use regex::Regex;
use structs::{DefaultsGrayscale, ExtendedGrayscale, MatYml, TextureFile, TEXTURE_FILES};
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

    let mat_yml_str = fs::read_to_string(path.join("mat.yml")).map_err(|e| {
        let err = format!("Failed to read mat.yml file {}\n {}", material_path, e);
        eprintln!("{}", err);
        return err;
    })?;
    let mat_yml: MatYml = serde_yaml::from_str(&mat_yml_str).map_err(|e| {
        let err = format!("Failed to deserialise mat.yml file {}", e);
        eprintln!("{}", err);
        return err;
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

        let _ = send_base64_image(file, texture_file, true, mat_yml.clone(), app);
    } else if !original_exists && texture_file.alternate.is_some() {
        let file = find_matching_file(texture_file.alternate.unwrap());
        if file.is_some() {
            let _ = send_base64_image(file.unwrap(), texture_file, false, mat_yml.clone(), app);
        } else {
            let _ = send_base64_image_default(texture_file, true, mat_yml.clone(), app);
        }
    } else {
        let _ = send_base64_image_default(texture_file, true, mat_yml.clone(), app);
    }

    let texture_properties = match texture_file.name {
        "opacity" => mat_yml.opacity,
        "rough" => mat_yml.rough,
        "smooth" => mat_yml.smooth,
        "metal" => mat_yml.metal,
        "f0" => mat_yml.f0,
        "porosity" => mat_yml.porosity,
        "sss" => mat_yml.sss,
        "emissive" => mat_yml.emissive,
        _ => None,
    };

    if let Some(values) = texture_properties {
        let unwrapped_values = DefaultsGrayscale {
            value: Some(values.value.unwrap_or(0.0)),
            shift: Some(values.shift.unwrap_or(0.0)),
            scale: Some(values.scale.unwrap_or(1.0)),
        };
        let extended_res = ExtendedGrayscale {
            use_og: !original_exists && texture_file.alternate.is_some(),
            values: unwrapped_values,
        };
        let serialized_values = serde_json::to_string(&extended_res).map_err(|e| {
            let err = format!("Failed to deserialise mat.yml file {}", e);
            eprintln!("{}", err);
            return err;
        });

        match serialized_values {
            Ok(values) => return Ok(values),
            Err(e) => return Err(e),
        };
    } else {
        let values = DefaultsGrayscale {
            value: Some(0.0),
            shift: Some(0.0),
            scale: Some(1.0),
        };
        let extended_res = ExtendedGrayscale {
            use_og: original_exists && texture_file.alternate.is_some(),
            values,
        };
        let serialized_values = serde_json::to_string(&extended_res).map_err(|e| {
            let err = format!("Failed to deserialise mat.yml file {}", e);
            eprintln!("{}", err);
            return err;
        });

        match serialized_values {
            Ok(values) => return Ok(values),
            Err(e) => return Err(e),
        };
    }
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
        if [
            "opacity", "smooth", "rough", "porosity", "metal", "f0", "sss", "emissive",
        ]
        .contains(&texture_file.name)
        {
            let texture_properties = match texture_file.name {
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
                "metal" => mat_yml.metal,
                "f0" => mat_yml.f0,
                "porosity" => mat_yml.porosity,
                "sss" => mat_yml.sss,
                "emissive" => mat_yml.emissive,
                _ => None,
            };

            if let Some(texture) = &texture_properties {
                let value = texture.value.unwrap_or(0.0);
                let shift = texture.shift.unwrap_or(0.0);
                let scale = texture.scale.unwrap_or(1.0);

                for pixel in luma_img.pixels_mut() {
                    if (value >= 0.0 || value < 0.0) && value != 0.0 {
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
    mat_yml: MatYml,
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

    if texture_file.grayscale {
        let mut luma_img = dynamic_img.into_luma8();
        if [
            "opacity", "smooth", "rough", "porosity", "metal", "f0", "sss", "emissive",
        ]
        .contains(&texture_file.name)
        {
            let texture_properties = match texture_file.name {
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
                "metal" => mat_yml.metal,
                "f0" => mat_yml.f0,
                "porosity" => mat_yml.porosity,
                "sss" => mat_yml.sss,
                "emissive" => mat_yml.emissive,
                _ => None,
            };

            if let Some(texture) = &texture_properties {
                let value = texture.value.unwrap_or(0.0);
                let shift = texture.shift.unwrap_or(0.0);
                let scale = texture.scale.unwrap_or(1.0);

                for pixel in luma_img.pixels_mut() {
                    if (value >= 0.0 || value < 0.0) && value != 0.0 {
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
        let _ = final_img.write_to(&mut cursor, image::ImageFormat::Png);
    } else {
        dynamic_img
            .write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|e| format!("Failed to write default image: {}", e))?;
    }

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
        return Err(String::from("Selected path does not exist."));
    }

    let mat_yml_str = fs::read_to_string(path.join("mat.yml")).map_err(|e| {
        let err = format!("Failed to read mat.yml file {}\n {}", material_path, e);
        eprintln!("{}", err);
        return err;
    })?;
    let mut mat_yml: MatYml = serde_yaml::from_str(&mat_yml_str).map_err(|e| {
        let err = format!("Failed to deserialise mat.yml file {}", e);
        eprintln!("{}", err);
        return err;
    })?;

    let parsed_value = value.parse::<f32>().unwrap_or(0.0);
    let parsed_shift = shift.parse::<f32>().unwrap_or(0.0);
    let parsed_scale = scale.parse::<f32>().unwrap_or(0.0);

    match texture.as_str() {
        "opacity" => {
            if let Some(opacity) = mat_yml.opacity.as_mut() {
                opacity.value = Some(parsed_value);
                opacity.shift = Some(parsed_shift);
                opacity.scale = Some(parsed_scale);
            } else {
                mat_yml.opacity = Some(DefaultsGrayscale {
                    value: Some(0.0),
                    shift: Some(0.0),
                    scale: Some(1.0),
                })
            }
        }
        "rough" => {
            if let Some(rough) = mat_yml.rough.as_mut() {
                rough.value = Some(parsed_value);
                rough.shift = Some(parsed_shift);
                rough.scale = Some(parsed_scale);
            } else {
                mat_yml.rough = Some(DefaultsGrayscale {
                    value: Some(0.0),
                    shift: Some(0.0),
                    scale: Some(1.0),
                })
            }
        }
        "smooth" => {
            if let Some(smooth) = mat_yml.smooth.as_mut() {
                smooth.value = Some(parsed_value);
                smooth.shift = Some(parsed_shift);
                smooth.scale = Some(parsed_scale);
            } else {
                mat_yml.smooth = Some(DefaultsGrayscale {
                    value: Some(0.0),
                    shift: Some(0.0),
                    scale: Some(1.0),
                })
            }
        }
        "metal" => {
            if let Some(porosity) = mat_yml.metal.as_mut() {
                porosity.value = Some(parsed_value);
                porosity.shift = Some(parsed_shift);
                porosity.scale = Some(parsed_scale);
            } else {
                mat_yml.porosity = Some(DefaultsGrayscale {
                    value: Some(0.0),
                    shift: Some(0.0),
                    scale: Some(1.0),
                })
            }
        }
        "f0" => {
            if let Some(porosity) = mat_yml.f0.as_mut() {
                porosity.value = Some(parsed_value);
                porosity.shift = Some(parsed_shift);
                porosity.scale = Some(parsed_scale);
            } else {
                mat_yml.porosity = Some(DefaultsGrayscale {
                    value: Some(0.0),
                    shift: Some(0.0),
                    scale: Some(1.0),
                })
            }
        }
        "porosity" => {
            if let Some(porosity) = mat_yml.porosity.as_mut() {
                porosity.value = Some(parsed_value);
                porosity.shift = Some(parsed_shift);
                porosity.scale = Some(parsed_scale);
            } else {
                mat_yml.porosity = Some(DefaultsGrayscale {
                    value: Some(0.0),
                    shift: Some(0.0),
                    scale: Some(1.0),
                })
            }
        }
        "sss" => {
            if let Some(sss) = mat_yml.sss.as_mut() {
                sss.value = Some(parsed_value);
                sss.shift = Some(parsed_shift);
                sss.scale = Some(parsed_scale);
            } else {
                mat_yml.sss = Some(DefaultsGrayscale {
                    value: Some(0.0),
                    shift: Some(0.0),
                    scale: Some(1.0),
                });
            }
        }
        "emissive" => {
            if let Some(emissive) = mat_yml.emissive.as_mut() {
                emissive.value = Some(parsed_value);
                emissive.shift = Some(parsed_shift);
                emissive.scale = Some(parsed_scale);
            } else {
                mat_yml.emissive = Some(DefaultsGrayscale {
                    value: Some(0.0),
                    shift: Some(0.0),
                    scale: Some(1.0),
                })
            }
        }
        _ => (),
    }

    let updated_mat_yml = serde_yaml::to_string(&mat_yml).map_err(|e| {
        let err = format!("Failed to deserialise mat.yml file {}", e);
        eprintln!("{}", err);
        return err;
    })?;

    match fs::write(&path.join("mat.yml"), updated_mat_yml) {
        Ok(()) => return Ok(true),
        Err(e) => return Err(e.to_string()),
    }
}
