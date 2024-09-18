use std::path::Path;

use super::structs::PngImage;
use crate::core::{
    interface::structs::Defaults,
    normal_map::{self, KernelSize},
};

use image::{DynamicImage, GenericImageView, ImageBuffer};

// const SOBEL_3X3: [[[i32; 3]; 3]; 2] = [
//     [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]], // Sobel X
//     [[-1, -2, -1], [0, 0, 0], [1, 2, 1]], // Sobel Y
// ];

pub fn generate_normal_map(path: &Path, size: KernelSize, strength: f32) -> PngImage {
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
        buf: normal_map,
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
