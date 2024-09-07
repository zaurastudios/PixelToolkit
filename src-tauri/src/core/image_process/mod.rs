use image::{GenericImageView, ImageBuffer, Luma, Rgba};
use std::error::Error;
use std::path::Path;

pub fn save_channel_map(
    parent_dir: &Path,
    channel: usize, // 0: red, 1: green, 2: blue, 3: alpha
    file_name: Option<String>,
    save_name: Option<String>,
    invert: bool,
) -> Result<(), Box<dyn Error>> {
    let image_path = parent_dir.join(file_name.unwrap_or(String::from("color.png")));
    let save_file_name = save_name.unwrap_or(String::from("opacity.png"));

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
    let mut channel_map = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let mut value = pixel.0[channel];
        if invert {
            value = 255 - value;
        }
        channel_map.put_pixel(x, y, Luma([value]));
    }

    channel_map
        .save(parent_dir.join(save_file_name))
        .map_err(|e| {
            eprintln!("Error saving alpha map: {}", e);
            e
        })?;

    Ok(())
}

pub fn save_channel_map_split(
    parent_dir: &Path,
    channel: usize, // 0: red, 1: green, 2: blue, 3: alpha
    start: u8,
    end: u8,
    file_name: Option<String>,
    save_name_1: Option<String>,
    save_name_2: Option<String>,
    scale: bool,
    invert: bool,
) -> Result<(), Box<dyn Error>> {
    let image_path = parent_dir.join(file_name.unwrap_or(String::from("color.png")));
    let save_file_name_1 = save_name_1.unwrap_or(String::from("opacity.png"));
    let save_file_name_2 = save_name_2.unwrap_or(String::from("opacity.png"));

    let img = image::open(&image_path).map_err(|e| {
        eprintln!("Error opening image: {}", e);
        eprintln!(
            "Error opening image path and file name: {}\n{}\n{}",
            image_path.to_string_lossy(),
            save_file_name_1,
            save_file_name_2
        );
        e
    })?;

    let (width, height) = img.dimensions();
    let mut channel_map_1 = ImageBuffer::new(width, height);
    let mut channel_map_2 = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let mut value = pixel.0[channel];
        if invert {
            value = 255 - value;
        }

        if value >= start && value <= end {
            let scaled_value = if scale {
                ((value - start) as f32 / (end - start) as f32 * 255.0) as u8
            } else {
                value
            };

            channel_map_1.put_pixel(x, y, Luma([scaled_value]));
            channel_map_2.put_pixel(x, y, Luma([0]));
        } else {
            let scaled_value = if scale {
                if value < start {
                    ((value as f32 / start as f32) * 255.0) as u8
                } else {
                    (((value - end) as f32 / (255 - end) as f32) * 255.0) as u8
                }
            } else {
                value
            };

            channel_map_1.put_pixel(x, y, Luma([0]));
            channel_map_2.put_pixel(x, y, Luma([scaled_value]));
        }
    }

    channel_map_1
        .save(parent_dir.join(save_file_name_1))
        .map_err(|e| {
            println!(
                "Failed to save split channel map 1: {}\n{}",
                e,
                image_path.to_string_lossy()
            );
            e
        })?;
    channel_map_2
        .save(parent_dir.join(save_file_name_2))
        .map_err(|e| {
            println!(
                "Failed to save split channel map 2: {}\n{}",
                e,
                image_path.to_string_lossy()
            );
            e
        })?;

    Ok(())
}

pub fn save_normal(
    parent_dir: &Path,
    file_name: Option<String>,
    save_name: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let image_path = parent_dir.join(file_name.unwrap_or(String::from("color.png")));
    let save_file_name = save_name.unwrap_or(String::from("opacity.png"));

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
    let mut normal_map = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let mut value = pixel.0;
        value[2] = 255;
        value[3] = 255;

        normal_map.put_pixel(x, y, Rgba(value));
    }

    normal_map
        .save(parent_dir.join(save_file_name))
        .map_err(|e| {
            eprintln!("Error saving alpha map: {}", e);
            e
        })?;

    Ok(())
}
