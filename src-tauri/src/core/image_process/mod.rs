use image::{GenericImageView, ImageBuffer, Luma};
use std::error::Error;
use std::path::Path;

pub fn save_alpha(
    parent_dir: &Path,
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
    let mut alpha_map = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let mut alpha = pixel.0[3];
        if invert {
            alpha = 255 - alpha;
        }
        alpha_map.put_pixel(x, y, Luma([alpha]));
    }

    alpha_map
        .save(parent_dir.join(save_file_name))
        .map_err(|e| {
            eprintln!("Error saving alpha map: {}", e);
            e
        })?;

    Ok(())
}
