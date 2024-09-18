use std::path::Path;

use super::structs::PngImage;
use crate::core::{
    interface::structs::Defaults,
    normal_map::{self, KernelSize},
};

use image::{DynamicImage, GenericImageView, ImageBuffer};
use nalgebra::{Rotation3, Vector3};

// const SOBEL_3X3: [[[i32; 3]; 3]; 2] = [
//     [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]], // Sobel X
//     [[-1, -2, -1], [0, 0, 0], [1, 2, 1]], // Sobel Y
// ];

pub fn generate_normal_map(
    path: &Path,
    size: KernelSize,
    strength: f32,
    curve_x: f32,
    curve_y: f32,
    radius_x: f32,
    radius_y: f32,
) -> PngImage {
    let img = image::open(path).unwrap();
    let (width, height) = img.dimensions();

    // Create a 3x3 tiled image
    let tiled_width = width * 3;
    let tiled_height = height * 3;

    let tiled_img = match img.color() {
        image::ColorType::Rgb16 | image::ColorType::L16 | image::ColorType::Rgba16 => {
            &DynamicImage::ImageRgb16({
                let img_16 = img.to_rgb16();
                let mut image =
                    ImageBuffer::<image::Rgb<u16>, Vec<u16>>::new(tiled_width, tiled_height);

                for y in 0..tiled_height {
                    for x in 0..tiled_width {
                        let src_x = x % width;
                        let src_y = y % height;
                        let pixel = img_16.get_pixel(src_x, src_y);
                        image.put_pixel(x, y, *pixel);
                    }
                }

                image
            })
        }
        _ => &DynamicImage::ImageRgb8({
            let img_8 = img.to_rgb8();
            let mut image = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::new(tiled_width, tiled_height);

            for y in 0..tiled_height {
                for x in 0..tiled_width {
                    let src_x = x % width;
                    let src_y = y % height;
                    let pixel = img_8.get_pixel(src_x, src_y);
                    image.put_pixel(x, y, *pixel);
                }
            }

            image
        }),
    };

    let tiled_normal_map = normal_map::map_normals_with_strength(tiled_img, strength, size);
    let tiled_normal_map_buf = tiled_normal_map.into_raw();

    // Extract the middle tile
    let mut normal_map = vec![0u8; (width * height * 3) as usize];
    for y in 0..height {
        for x in 0..width {
            let src_index = (((height + y) * tiled_width + (width + x)) * 3) as usize;
            let dst_index = ((y * width + x) * 3) as usize;
            normal_map[dst_index] = tiled_normal_map_buf[src_index];
            normal_map[dst_index + 1] = tiled_normal_map_buf[src_index + 1];
            normal_map[dst_index + 2] = tiled_normal_map_buf[src_index + 2];
        }
    }

    PngImage {
        buf: apply_curved_normals(
            &mut normal_map,
            width as usize,
            height as usize,
            curve_x,
            curve_y,
            radius_x,
            radius_y,
        ),
        info: Defaults {
            bit_depth: png::BitDepth::Eight,
            color_type: png::ColorType::Rgb,
            width: width as usize,
            height: height as usize,
            default_color: Some([128, 128, 255]),
        },
        palette: None,
    }
}

pub fn apply_curved_normals(
    normal_map: &mut Vec<u8>,
    width: usize,
    height: usize,
    curve_y_deg: f32,
    curve_x_deg: f32,
    radius_y: f32,
    radius_x: f32,
) -> Vec<u8> {
    let curve_x_rad = curve_x_deg.to_radians();
    let curve_y_rad = curve_y_deg.to_radians();

    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) * 3;

            // Convert RGB to normal vector
            let mut normal = Vector3::new(
                (normal_map[index] as f32 / 255.0) * 2.0 - 1.0,
                (normal_map[index + 1] as f32 / 255.0) * 2.0 - 1.0,
                (normal_map[index + 2] as f32 / 255.0) * 2.0 - 1.0,
            );

            // Apply curvature
            let rotation_y = Rotation3::from_axis_angle(
                &Vector3::x_axis(),
                curve_x_rad * (y as f32 / height as f32 - 0.5)
                    / (radius_x + (if radius_x == 0.0 { 1.0 } else { 0.0 })),
            );
            let rotation_x = Rotation3::from_axis_angle(
                &Vector3::y_axis(),
                curve_y_rad * (x as f32 / width as f32 - 0.5)
                    / (radius_y + (if radius_y == 0.0 { 1.0 } else { 0.0 })),
            );

            normal = rotation_y * rotation_x * normal;
            normal = normal.normalize();

            // Convert back to RGB
            normal_map[index] = ((normal.x + 1.0) * 0.5 * 255.0) as u8;
            normal_map[index + 1] = ((normal.y + 1.0) * 0.5 * 255.0) as u8;
            normal_map[index + 2] = ((normal.z + 1.0) * 0.5 * 255.0) as u8;
        }
    }

    normal_map.to_vec()
}
