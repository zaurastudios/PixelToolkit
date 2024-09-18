// Modified from https://crates.io/crates/normal-heights

use image::{DynamicImage, ImageBuffer, Luma, RgbImage};
use rayon::prelude::*;
use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
pub enum KernelSize {
    Three,
    Five,
    Nine,
    Low,
    High,
}

struct Kernel {
    x: Arc<Vec<Vec<f32>>>,
    y: Arc<Vec<Vec<f32>>>,
    size: usize,
}

impl Kernel {
    fn new(kernel_size: KernelSize) -> Self {
        let (x, y, size) = match kernel_size {
            KernelSize::Three | KernelSize::Low => {
                let x = Arc::new(vec![
                    vec![-1.0, 0.0, 1.0],
                    vec![-2.0, 0.0, 2.0],
                    vec![-1.0, 0.0, 1.0],
                ]);
                let y = Arc::new(vec![
                    vec![-1.0, -2.0, -1.0],
                    vec![0.0, 0.0, 0.0],
                    vec![1.0, 2.0, 1.0],
                ]);
                (x, y, 3)
            }
            KernelSize::Five => {
                let x = Arc::new(vec![
                    vec![-1.0, -2.0, 0.0, 2.0, 1.0],
                    vec![-4.0, -8.0, 0.0, 8.0, 4.0],
                    vec![-6.0, -12.0, 0.0, 12.0, 6.0],
                    vec![-4.0, -8.0, 0.0, 8.0, 4.0],
                    vec![-1.0, -2.0, 0.0, 2.0, 1.0],
                ]);
                let y = Arc::new(vec![
                    vec![-1.0, -4.0, -6.0, -4.0, -1.0],
                    vec![-2.0, -8.0, -12.0, -8.0, -2.0],
                    vec![0.0, 0.0, 0.0, 0.0, 0.0],
                    vec![2.0, 8.0, 12.0, 8.0, 2.0],
                    vec![1.0, 4.0, 6.0, 4.0, 1.0],
                ]);
                (x, y, 5)
            }
            KernelSize::Nine => {
                let x = Arc::new(vec![
                    vec![-1.0, -2.0, -3.0, -4.0, 0.0, 4.0, 3.0, 2.0, 1.0],
                    vec![-2.0, -4.0, -6.0, -8.0, 0.0, 8.0, 6.0, 4.0, 2.0],
                    vec![-3.0, -6.0, -9.0, -12.0, 0.0, 12.0, 9.0, 6.0, 3.0],
                    vec![-4.0, -8.0, -12.0, -16.0, 0.0, 16.0, 12.0, 8.0, 4.0],
                    vec![-5.0, -10.0, -15.0, -20.0, 0.0, 20.0, 15.0, 10.0, 5.0],
                    vec![-4.0, -8.0, -12.0, -16.0, 0.0, 16.0, 12.0, 8.0, 4.0],
                    vec![-3.0, -6.0, -9.0, -12.0, 0.0, 12.0, 9.0, 6.0, 3.0],
                    vec![-2.0, -4.0, -6.0, -8.0, 0.0, 8.0, 6.0, 4.0, 2.0],
                    vec![-1.0, -2.0, -3.0, -4.0, 0.0, 4.0, 3.0, 2.0, 1.0],
                ]);
                let y = Arc::new(vec![
                    vec![-1.0, -2.0, -3.0, -4.0, -5.0, -4.0, -3.0, -2.0, -1.0],
                    vec![-2.0, -4.0, -6.0, -8.0, -10.0, -8.0, -6.0, -4.0, -2.0],
                    vec![-3.0, -6.0, -9.0, -12.0, -15.0, -12.0, -9.0, -6.0, -3.0],
                    vec![-4.0, -8.0, -12.0, -16.0, -20.0, -16.0, -12.0, -8.0, -4.0],
                    vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                    vec![4.0, 8.0, 12.0, 16.0, 20.0, 16.0, 12.0, 8.0, 4.0],
                    vec![3.0, 6.0, 9.0, 12.0, 15.0, 12.0, 9.0, 6.0, 3.0],
                    vec![2.0, 4.0, 6.0, 8.0, 10.0, 8.0, 6.0, 4.0, 2.0],
                    vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0],
                ]);
                (x, y, 9)
            }
            KernelSize::High => {
                let x = Arc::new(vec![
                    vec![-3.0, 0.0, 3.0],
                    vec![-10.0, 0.0, 10.0],
                    vec![-3.0, 0.0, 3.0],
                ]);
                let y = Arc::new(vec![
                    vec![-3.0, -10.0, -3.0],
                    vec![0.0, 0.0, 0.0],
                    vec![3.0, 10.0, 3.0],
                ]);
                (x, y, 3)
            }
        };
        Kernel { x, y, size }
    }
}

fn apply_kernel(pixels: &[Vec<f32>], kernel: &[Vec<f32>], size: usize) -> f32 {
    pixels
        .iter()
        .zip(kernel.iter())
        .take(size)
        .flat_map(|(row_p, row_k)| row_p.iter().zip(row_k.iter()).take(size))
        .map(|(p, k)| p * k)
        .sum()
}

pub fn map_normals_with_strength(
    img: &DynamicImage,
    strength: f32,
    kernel_size: KernelSize,
) -> RgbImage {
    let image = img.to_luma8();
    let (width, height) = image.dimensions();
    let kernel = Arc::new(Kernel::new(kernel_size));
    let strength = strength;

    let normal_map: Vec<(u32, u32, [u8; 3])> = match img.color() {
        image::ColorType::Rgb8 => process_image(&img.to_luma8(), width, height, &kernel, strength),
        image::ColorType::Rgb16 => {
            process_image_16bit(&img.to_luma16(), width, height, &kernel, strength)
        }
        _ => {
            let luma8 = img.to_luma8();
            process_image(&luma8, width, height, &kernel, strength)
        }
    };

    let mut normal_map_img = RgbImage::new(width, height);
    for (x, y, pixel) in normal_map {
        normal_map_img.put_pixel(x, y, image::Rgb(pixel));
    }
    normal_map_img
}

#[inline]
fn normalize(v: [f32; 3]) -> [f32; 3] {
    let v_mag = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / v_mag, v[1] / v_mag, v[2] / v_mag]
}

#[inline]
fn scale_normalized_to_0_to_1(v: &[f32; 3]) -> [f32; 3] {
    [v[0] * 0.5 + 0.5, v[1] * 0.5 + 0.5, v[2] * 0.5 + 0.5]
}

fn process_image(
    img: &ImageBuffer<Luma<u8>, Vec<u8>>,
    width: u32,
    height: u32,
    kernel: &Arc<Kernel>,
    strength: f32,
) -> Vec<(u32, u32, [u8; 3])> {
    (0..height)
        .into_par_iter()
        .flat_map(move |y| {
            let kernel = kernel.clone();
            (0..width).into_par_iter().map({
                let img = img.clone();
                move |x| {
                    let mut pixels = vec![vec![0.0; kernel.size]; kernel.size];
                    for dy in 0..kernel.size {
                        for dx in 0..kernel.size {
                            let px = x
                                .saturating_add(dx as u32)
                                .saturating_sub((kernel.size / 2) as u32);
                            let py = y
                                .saturating_add(dy as u32)
                                .saturating_sub((kernel.size / 2) as u32);
                            pixels[dy][dx] = img.get_pixel(px.min(width - 1), py.min(height - 1))[0]
                                as f32
                                / 255.0;
                        }
                    }

                    let x_normal = apply_kernel(&pixels, &kernel.x, kernel.size);
                    let y_normal = -apply_kernel(&pixels, &kernel.y, kernel.size);
                    let z_normal = 1.0 / strength;

                    let normal = normalize([x_normal, y_normal, z_normal]);
                    let rgb = scale_normalized_to_0_to_1(&normal);

                    (
                        x,
                        y,
                        [
                            (rgb[0] * 255.0) as u8,
                            (rgb[1] * 255.0) as u8,
                            (rgb[2] * 255.0) as u8,
                        ],
                    )
                }
            })
        })
        .collect()
}

fn process_image_16bit(
    img: &ImageBuffer<Luma<u16>, Vec<u16>>,
    width: u32,
    height: u32,
    kernel: &Arc<Kernel>,
    strength: f32,
) -> Vec<(u32, u32, [u8; 3])> {
    (0..height)
        .into_par_iter()
        .flat_map(move |y| {
            let kernel = kernel.clone();
            (0..width).into_par_iter().map({
                let img = img.clone();
                move |x| {
                    let mut pixels = vec![vec![0.0; kernel.size]; kernel.size];
                    for dy in 0..kernel.size {
                        for dx in 0..kernel.size {
                            let px = x
                                .saturating_add(dx as u32)
                                .saturating_sub((kernel.size / 2) as u32);
                            let py = y
                                .saturating_add(dy as u32)
                                .saturating_sub((kernel.size / 2) as u32);
                            pixels[dy][dx] = img.get_pixel(px.min(width - 1), py.min(height - 1))[0]
                                as f32
                                / u16::MAX as f32;
                        }
                    }

                    let x_normal = apply_kernel(&pixels, &kernel.x, kernel.size);
                    let y_normal = -apply_kernel(&pixels, &kernel.y, kernel.size);
                    let z_normal = 1.0 / strength;

                    let normal = normalize([x_normal, y_normal, z_normal]);
                    let rgb = scale_normalized_to_0_to_1(&normal);

                    (
                        x,
                        y,
                        [
                            (rgb[0] * 255.0) as u8,
                            (rgb[1] * 255.0) as u8,
                            (rgb[2] * 255.0) as u8,
                        ],
                    )
                }
            })
        })
        .collect()
}
