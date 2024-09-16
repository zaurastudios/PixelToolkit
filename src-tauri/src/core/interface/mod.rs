pub mod normal;
pub mod structs;
use structs::{Defaults, DefaultsGrayscale, MatYml, PngImage, TextureFile, TEXTURE_FILES};

use base64::{engine::general_purpose, Engine};
use rayon::prelude::*;

use std::{
    fs::{self, File},
    io::Cursor,
    path::Path,
    sync::Arc,
};

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

    // For mutlithreading process of data
    let mat_yml: Arc<MatYml> = Arc::new(load_mat_yml(path)?);

    let img_path = process_image(path, texture_file, mat_yml.clone())?;

    app.emit("selected-texture-file", img_path.clone())
        .map_err(|e| format!("Failed to emit event: {}", e))?;

    let texture_properties = get_texture_properties(&texture_file.name, &mat_yml);

    let res = texture_properties.unwrap_or(DefaultsGrayscale {
        value: Some(0.0),
        shift: Some(0.0),
        scale: Some(1.0),
    });

    let result = serde_json::to_string(&res).map_err(|e| {
        let err = format!("Failed to serialize extended grayscale: {}", e);
        eprintln!("{}", err);
        err
    });

    result
}

fn process_image(
    path: &Path,
    texture_file: &TextureFile,
    mat_yml: Arc<MatYml>,
) -> Result<String, String> {
    let img = match find_matching_file(path, texture_file.pattern) {
        Some(file) => {
            // Using the `png` crate to read the image. So much faster compared to `image` crate.
            let decoder = png::Decoder::new(File::open(file.path()).unwrap());
            let mut reader = decoder.read_info().unwrap();
            let mut buf = vec![0; reader.output_buffer_size()];
            let info = reader.next_frame(&mut buf).unwrap();

            let img = PngImage {
                buf,
                info: Defaults {
                    bit_depth: info.bit_depth,
                    color_type: info.color_type,
                    width: info.width as usize,
                    height: info.height as usize,
                    default_color: None,
                },
                palette: Some(reader.info().palette.clone()).expect("msg"),
            };

            img
        }
        None => create_default_image(texture_file),
    };

    let processed_img = if texture_file.grayscale {
        process_grayscale_image(&img, texture_file, &mat_yml)
    } else {
        img
    };

    let base64_image = image_to_base64(&processed_img)?;

    Ok(base64_image)
}

fn load_mat_yml(path: &Path) -> Result<MatYml, String> {
    let mat_yml_str = fs::read_to_string(path.join("mat.yml")).map_err(|e| {
        let err = format!("Failed to read mat.yml file: {}", e);
        eprintln!("{}", err);
        err
    })?;

    let result = serde_yaml::from_str(&mat_yml_str).map_err(|e| {
        let err = format!("Failed to deserialise mat.yml file: {}", e);
        eprintln!("{}", err);
        err
    });

    result
}

// Create a plain 16x16 image with the filled default colour
fn create_default_image(texture_file: &TextureFile) -> PngImage {
    let def = texture_file.defaults.clone();
    let color = def.default_color.unwrap_or([0, 0, 0]);

    let bytes_per_pixel = match def.color_type {
        png::ColorType::Rgb => 3,
        png::ColorType::Rgba => 4,
        png::ColorType::Grayscale => 1,
        png::ColorType::GrayscaleAlpha => 2,
        png::ColorType::Indexed => 1,
    };

    let mut buf = Vec::with_capacity(def.width * def.height * bytes_per_pixel);
    for _ in 0..def.width * def.height {
        match def.color_type {
            png::ColorType::Rgb => buf.extend_from_slice(&color),
            png::ColorType::Rgba => buf.extend_from_slice(&[color[0], color[1], color[2], 255]),
            png::ColorType::Grayscale | png::ColorType::Indexed => buf.push(color[0]),
            png::ColorType::GrayscaleAlpha => buf.extend_from_slice(&[color[0], 255]),
        }
    }

    PngImage {
        buf,
        info: def,
        palette: None,
    }
}

fn process_grayscale_image(
    img: &PngImage,
    texture_file: &TextureFile,
    mat_yml: &MatYml,
) -> PngImage {
    let info = img.info.clone();

    // Convert the image to grayscale
    let luma_img = if info.color_type != png::ColorType::Grayscale {
        img.buf
            .chunks(4)
            .map(|pixel| {
                ((pixel[0] as u16 * 299 + pixel[1] as u16 * 587 + pixel[2] as u16 * 114) / 1000)
                    as u8
            })
            .collect()
    } else {
        img.buf.to_vec()
    };

    // Textures sharing the same properties, so the processing will be the same
    if [
        "opacity", "smooth", "rough", "porosity", "metal", "f0", "sss", "emissive",
    ]
    .contains(&texture_file.name)
    {
        if let Some(texture) = get_texture_properties(&texture_file.name, mat_yml) {
            let processed =
                process_pixels_grayscale_common(&luma_img, info.width, info.height, &texture);

            return PngImage {
                buf: processed,
                info: info.clone(),
                palette: img.palette.clone(),
            };
        }
    }

    PngImage {
        buf: img.buf.clone(),
        info: info.clone(),
        palette: img.palette.clone(),
    }
}

fn process_pixels_grayscale_common(
    img: &[u8],
    width: usize,
    height: usize,
    texture: &DefaultsGrayscale,
) -> Vec<u8> {
    let value = texture.value.unwrap_or(0.0);
    let shift = texture.shift.unwrap_or(0.0);
    let scale = texture.scale.unwrap_or(1.0);

    let pixels: Arc<[u8]> = Arc::from(img);
    let chunk_size = (width * height / rayon::current_num_threads()).max(1);

    let processed: Vec<u8> = pixels
        .par_chunks(chunk_size as usize)
        .flat_map(|chunk| {
            chunk
                .iter()
                .map(|&pixel| {
                    if value > 0.0 {
                        // No need to modify the pixels if there is a value present since it overwrites the image
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

    processed
}

fn find_matching_file(path: &Path, pattern: &str) -> Option<fs::DirEntry> {
    // Filter all the files/paths in the specified dir and returns false or the path to file
    fs::read_dir(path)
        .ok()?
        .filter_map(Result::ok)
        .find(|entry| {
            entry.file_name().to_str().map_or(false, |file_name| {
                regex::Regex::new(pattern).ok().unwrap().is_match(file_name)
            })
        })
}

fn get_texture_properties(texture_name: &str, mat_yml: &MatYml) -> Option<DefaultsGrayscale> {
    let result = match texture_name {
        "opacity" => mat_yml.opacity.clone(),
        "rough" => mat_yml.rough.clone(),
        "smooth" => mat_yml.smooth.clone(),
        "metal" => mat_yml.metal.clone(),
        "f0" => mat_yml.f0.clone(),
        "porosity" => mat_yml.porosity.clone(),
        "sss" => mat_yml.sss.clone(),
        "emissive" => mat_yml.emissive.clone(),
        _ => None,
    };

    result
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

fn image_to_base64(img: &PngImage) -> Result<String, String> {
    let mut png_data = Vec::new();
    {
        let mut encoder = png::Encoder::new(
            Cursor::new(&mut png_data),
            img.info.width as u32,
            img.info.height as u32,
        );
        encoder.set_color(img.info.color_type);
        encoder.set_depth(img.info.bit_depth);
        if let Some(ref palette) = img.palette {
            encoder.set_palette(palette.clone());
        }
        let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
        writer
            .write_image_data(&img.buf)
            .map_err(|e| e.to_string())?;
    }

    Ok(general_purpose::STANDARD.encode(&png_data))
}
