use image::{GenericImageView, ImageBuffer, Luma, Rgba};
use rayon::prelude::*;
use std::error::Error;
use std::path::Path;

pub fn save_channel_map(
    material_dir: &Path,
    channel: usize,
    image_path: &Path,
    save_name: String,
    invert: bool,
) -> Result<(), Box<dyn Error>> {
    let img = image::open(image_path).map_err(|e| {
        eprintln!("Error opening image: {}", e);
        eprintln!(
            "Error opening image path and file name: {}\n{}",
            image_path.to_string_lossy(),
            save_name
        );
        e
    })?;

    if channel == 3 {
        let has_transparency = if invert {
            img.pixels().any(|(_, _, pixel)| 255 - pixel[3] > 0)
        } else {
            img.pixels().any(|(_, _, pixel)| pixel[3] < 255)
        };

        if !has_transparency {
            return Ok(());
        }
    }

    let (width, height) = img.dimensions();
    let mut channel_map: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    channel_map
        .par_chunks_exact_mut(width as usize)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, pixel) in row.iter_mut().enumerate() {
                let src_pixel = img.get_pixel(x as u32, y as u32);
                let value = if invert {
                    255 - src_pixel[channel]
                } else {
                    src_pixel[channel]
                };
                *pixel = value;
            }
        });

    channel_map
        .save(material_dir.join(save_name))
        .map_err(|e| {
            eprintln!("Error saving channel map: {}", e);
            e
        })?;

    Ok(())
}

pub fn save_f0_hcm(material_dir: &Path, image_path: &Path) -> Result<(), Box<dyn Error>> {
    let img = image::open(image_path).map_err(|e| {
        eprintln!("Error opening image: {}", e);
        eprintln!("Error opening image path: {}", image_path.to_string_lossy(),);
        e
    })?;

    let (width, height) = img.dimensions();
    let mut f0_map: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    let mut hcm_map: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let mut process_images = || {
        f0_map
            .par_chunks_exact_mut(width as usize * 4)
            .zip(hcm_map.par_chunks_exact_mut(width as usize * 4))
            .enumerate()
            .for_each(|(y, (f0_row, hcm_row))| {
                for (x, (f0_chunk, hcm_chunk)) in f0_row
                    .chunks_exact_mut(4)
                    .zip(hcm_row.chunks_exact_mut(4))
                    .enumerate()
                {
                    let pixel = img.get_pixel(x as u32, y as u32);
                    let value = pixel[1];

                    if value < 230 {
                        f0_chunk.copy_from_slice(&[value, value, value, 255]);
                        hcm_chunk.copy_from_slice(&[0, 0, 0, 255]);
                    } else {
                        f0_chunk.copy_from_slice(&[0, 0, 0, 255]);
                        hcm_chunk.copy_from_slice(&[value, value, value, 255]);
                    }
                }
            });
    };

    process_images();

    f0_map.save(material_dir.join("f0.png")).map_err(|e| {
        eprintln!("Error saving f0 map: {}", e);
        e
    })?;

    hcm_map.save(material_dir.join("hcm.png")).map_err(|e| {
        eprintln!("Error saving hcm map: {}", e);
        e
    })?;

    Ok(())
}

pub fn save_porosity_sss(material_dir: &Path, image_path: &Path) -> Result<(), Box<dyn Error>> {
    let img = image::open(image_path).map_err(|e| {
        eprintln!("Error opening image: {}", e);
        eprintln!("Error opening image path: {}", image_path.to_string_lossy(),);
        e
    })?;

    let (width, height) = img.dimensions();
    let mut porosity_map: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    let mut sss_map: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let mut process_images = || {
        porosity_map
            .par_chunks_exact_mut(width as usize * 4)
            .zip(sss_map.par_chunks_exact_mut(width as usize * 4))
            .enumerate()
            .for_each(|(y, (porosity_row, sss_row))| {
                for (x, (porosity_chunk, sss_chunk)) in porosity_row
                    .chunks_exact_mut(4)
                    .zip(sss_row.chunks_exact_mut(4))
                    .enumerate()
                {
                    let pixel = img.get_pixel(x as u32, y as u32);
                    let value = pixel[2];

                    if value <= 127 {
                        let transformed_value = value.saturating_mul(2);
                        porosity_chunk.copy_from_slice(&[
                            transformed_value,
                            transformed_value,
                            transformed_value,
                            255,
                        ]);
                        sss_chunk.copy_from_slice(&[0, 0, 0, 255]);
                    } else {
                        let transformed_value = value.saturating_sub(128).saturating_mul(2);
                        porosity_chunk.copy_from_slice(&[0, 0, 0, 255]);
                        sss_chunk.copy_from_slice(&[
                            transformed_value,
                            transformed_value,
                            transformed_value,
                            255,
                        ]);
                    }
                }
            });
    };

    process_images();

    porosity_map
        .save(material_dir.join("porosity.png"))
        .map_err(|e| {
            eprintln!("Error saving porosity map: {}", e);
            e
        })?;

    sss_map.save(material_dir.join("sss.png")).map_err(|e| {
        eprintln!("Error saving sss map: {}", e);
        e
    })?;

    Ok(())
}

pub fn save_normal(
    material_dir: &Path,
    image_path: &Path,
    save_name: String,
) -> Result<(), Box<dyn Error>> {
    let save_file_name = save_name;
    let img = image::open(&image_path).map_err(|e| {
        eprintln!("Error opening image: {}", e);
        eprintln!(
            "Error opening image path and file name: {}\n{}",
            image_path.to_string_lossy(),
            save_file_name
        );
        e
    })?;

    let (width, height) = img.dimensions();
    let mut normal_map: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    normal_map
        .par_chunks_mut(width as usize * 4)
        .enumerate()
        .for_each(|(y, row)| {
            for x in 0..width {
                let pixel = img.get_pixel(x, y as u32);
                let idx = (x as usize) * 4;
                row[idx] = pixel[0];
                row[idx + 1] = pixel[1];
                row[idx + 2] = 255;
                row[idx + 3] = 255;
            }
        });

    normal_map
        .save(material_dir.join(save_file_name))
        .map_err(|e| {
            eprintln!("Error saving normal map: {}", e);
            e
        })?;

    Ok(())
}
