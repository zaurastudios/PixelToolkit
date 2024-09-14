pub mod structs;
use rayon::prelude::*;

use std::{
    fs::{self},
    path::Path,
    sync::Arc,
};

use base64::{engine::general_purpose, Engine};
use image::{DynamicImage, ImageBuffer, Luma, Pixel};
use structs::{DefaultsGrayscale, ExtendedGrayscale, MatYml, TextureFile, TEXTURE_FILES};
use tauri::Emitter;

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

    let mat_yml: Arc<MatYml> = Arc::new(load_mat_yml(path)?);

    let (base64_image, original_exists) = process_image(path, texture_file, mat_yml.clone())?;

    app.emit("selected-texture-file", base64_image.clone())
        .map_err(|e| format!("Failed to emit event: {}", e))?;

    let texture_properties = get_texture_properties(&texture_file.name, &mat_yml);
    let extended_res = ExtendedGrayscale {
        use_og: original_exists || texture_file.alternate.is_none(),
        values: texture_properties.unwrap_or(DefaultsGrayscale {
            value: Some(0.0),
            shift: Some(0.0),
            scale: Some(1.0),
        }),
    };

    serde_json::to_string(&extended_res).map_err(|e| {
        let err = format!("Failed to serialize extended grayscale: {}", e);
        eprintln!("{}", err);
        err
    })
}

fn load_mat_yml(path: &Path) -> Result<MatYml, String> {
    let mat_yml_str = fs::read_to_string(path.join("mat.yml")).map_err(|e| {
        let err = format!("Failed to read mat.yml file: {}", e);
        eprintln!("{}", err);
        err
    })?;
    serde_yaml::from_str(&mat_yml_str).map_err(|e| {
        let err = format!("Failed to deserialise mat.yml file: {}", e);
        eprintln!("{}", err);
        err
    })
}

fn process_image(
    path: &Path,
    texture_file: &TextureFile,
    mat_yml: Arc<MatYml>,
) -> Result<(String, bool), String> {
    let img = match find_matching_file(path, texture_file.pattern) {
        Some(file) => {
            image::open(file.path()).map_err(|e| format!("Failed to open image: {}", e))?
        }
        None => {
            if let Some(alt_pattern) = texture_file.alternate {
                if let Some(alt_file) = find_matching_file(path, alt_pattern) {
                    image::open(alt_file.path())
                        .map_err(|e| format!("Failed to open image: {}", e))?
                } else {
                    create_default_image(texture_file, false)
                }
            } else {
                create_default_image(texture_file, true)
            }
        }
    };

    let original_exists = img.width() > 16;
    let processed_img = if texture_file.grayscale {
        process_grayscale_image(&img, texture_file, original_exists, &mat_yml)
    } else {
        img
    };

    let base64 = encode_image_to_base64(&processed_img)?;
    Ok((base64, original_exists))
}

fn find_matching_file(path: &Path, pattern: &str) -> Option<fs::DirEntry> {
    fs::read_dir(path)
        .ok()?
        .filter_map(Result::ok)
        .find(|entry| {
            entry.file_name().to_str().map_or(false, |file_name| {
                regex::Regex::new(pattern).ok().unwrap().is_match(file_name)
            })
        })
}

fn create_default_image(texture_file: &TextureFile, use_og: bool) -> DynamicImage {
    let color = if !use_og && texture_file.alternate.is_some() {
        texture_file.default_color.map_without_alpha(|e| 255 - e)
    } else {
        texture_file.default_color
    };
    DynamicImage::ImageRgba8(ImageBuffer::from_pixel(16, 16, color))
}

fn process_grayscale_image(
    img: &DynamicImage,
    texture_file: &TextureFile,
    use_og: bool,
    mat_yml: &MatYml,
) -> DynamicImage {
    let luma_img = img.to_luma8();
    let (width, height) = luma_img.dimensions();

    if [
        "opacity", "smooth", "rough", "porosity", "metal", "f0", "sss", "emissive",
    ]
    .contains(&texture_file.name)
    {
        if let Some(texture) = get_texture_properties(&texture_file.name, mat_yml) {
            let processed = process_pixels_parallel(luma_img, width, height, &texture);
            return if !use_og {
                DynamicImage::ImageLuma8(invert_img_parallel(processed))
            } else {
                DynamicImage::ImageLuma8(processed)
            };
        }
    }

    if !use_og {
        DynamicImage::ImageLuma8(invert_img_parallel(luma_img))
    } else {
        DynamicImage::ImageLuma8(luma_img)
    }
}

fn process_pixels_parallel(
    img: ImageBuffer<Luma<u8>, Vec<u8>>,
    width: u32,
    height: u32,
    texture: &DefaultsGrayscale,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let value = texture.value.unwrap_or(0.0);
    let shift = texture.shift.unwrap_or(0.0);
    let scale = texture.scale.unwrap_or(1.0);

    let pixels: Arc<[u8]> = Arc::from(img.into_raw());
    let chunk_size = (width * height / rayon::current_num_threads() as u32).max(1);

    let processed: Vec<u8> = pixels
        .par_chunks(chunk_size as usize)
        .flat_map(|chunk| {
            chunk
                .iter()
                .map(|&pixel| {
                    if value != 0.0 {
                        value.clamp(0.0, 255.0) as u8
                    } else {
                        let current_value = pixel as f32 / 255.0;
                        let new_value = ((current_value + shift) * scale).clamp(0.0, 1.0);
                        (new_value * 255.0) as u8
                    }
                })
                .collect::<Vec<u8>>()
        })
        .collect();

    ImageBuffer::from_raw(width, height, processed).unwrap()
}

fn invert_img_parallel(img: ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let pixels: Arc<[u8]> = Arc::from(img.into_raw());
    let chunk_size = (width * height / rayon::current_num_threads() as u32).max(1);

    let inverted: Vec<u8> = pixels
        .par_chunks(chunk_size as usize)
        .flat_map(|chunk| chunk.iter().map(|&pixel| 255 - pixel).collect::<Vec<u8>>())
        .collect();

    ImageBuffer::from_raw(width, height, inverted).unwrap()
}

fn encode_image_to_base64(img: &DynamicImage) -> Result<String, String> {
    let mut buf = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
        .map_err(|e| format!("Failed to write image: {}", e))?;

    Ok(general_purpose::STANDARD.encode(&buf))
}

fn get_texture_properties(texture_name: &str, mat_yml: &MatYml) -> Option<DefaultsGrayscale> {
    match texture_name {
        "opacity" => mat_yml.opacity.clone(),
        "rough" => mat_yml.rough.clone(),
        "smooth" => mat_yml.smooth.clone(),
        "metal" => mat_yml.metal.clone(),
        "f0" => mat_yml.f0.clone(),
        "porosity" => mat_yml.porosity.clone(),
        "sss" => mat_yml.sss.clone(),
        "emissive" => mat_yml.emissive.clone(),
        _ => None,
    }
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
        err
    })?;
    let mut mat_yml: MatYml = serde_yaml::from_str(&mat_yml_str).map_err(|e| {
        let err = format!("Failed to deserialise mat.yml file {}", e);
        eprintln!("{}", err);
        err
    })?;

    let parsed_value = value.parse::<f32>().unwrap_or(0.0).clamp(0.0, 255.0);
    let parsed_shift = shift.parse::<f32>().unwrap_or(0.0);
    let parsed_scale = scale.parse::<f32>().unwrap_or(1.0);

    let new_defaults = DefaultsGrayscale {
        value: Some(parsed_value),
        shift: Some(parsed_shift),
        scale: Some(parsed_scale),
    };

    match texture.as_str() {
        "opacity" => mat_yml.opacity = Some(new_defaults),
        "rough" => mat_yml.rough = Some(new_defaults),
        "smooth" => mat_yml.smooth = Some(new_defaults),
        "metal" => mat_yml.metal = Some(new_defaults),
        "f0" => mat_yml.f0 = Some(new_defaults),
        "porosity" => mat_yml.porosity = Some(new_defaults),
        "sss" => mat_yml.sss = Some(new_defaults),
        "emissive" => mat_yml.emissive = Some(new_defaults),
        _ => (),
    }

    let updated_mat_yml = serde_yaml::to_string(&mat_yml).map_err(|e| {
        let err = format!("Failed to serialize mat.yml file {}", e);
        eprintln!("{}", err);
        err
    })?;

    fs::write(&path.join("mat.yml"), updated_mat_yml).map_err(|e| e.to_string())?;

    Ok(true)
}
