// Modified from https://crates.io/crates/normal-heights

use image::{DynamicImage, GrayImage, RgbImage};

pub const DEFAULT_STRENGTH: f32 = 1.0;

#[derive(Clone, Copy)]
pub enum KernelSize {
    Three,
    Five,
    Nine,
    Low,
    High,
}

struct AdjPixels {
    pixels: Vec<Vec<f32>>,
    size: usize,
}

impl AdjPixels {
    fn new(x: u32, y: u32, img: &GrayImage, kernel_size: KernelSize) -> Self {
        let size = match kernel_size {
            KernelSize::Three | KernelSize::Low | KernelSize::High => 3,
            KernelSize::Five => 5,
            KernelSize::Nine => 9,
        };
        let mut pixels = vec![vec![0.0; size]; size];
        let offset = size / 2;

        for dy in 0..size {
            for dx in 0..size {
                let px = x.saturating_add(dx as u32).saturating_sub(offset as u32);
                let py = y.saturating_add(dy as u32).saturating_sub(offset as u32);
                pixels[dy][dx] = fetch_pixel(px, py, img);
            }
        }
        AdjPixels { pixels, size }
    }

    fn x_normals(&self, kernel_size: KernelSize) -> f32 {
        let kernel = self.get_x_kernel(kernel_size);
        self.apply_kernel(&kernel)
    }

    fn y_normals(&self, kernel_size: KernelSize) -> f32 {
        let kernel = self.get_y_kernel(kernel_size);
        self.apply_kernel(&kernel)
    }

    fn get_x_kernel(&self, kernel_size: KernelSize) -> Vec<Vec<f32>> {
        match kernel_size {
            KernelSize::Three => vec![
                vec![-1.0, 0.0, 1.0],
                vec![-2.0, 0.0, 2.0],
                vec![-1.0, 0.0, 1.0],
            ],
            KernelSize::Five => vec![
                vec![-1.0, -2.0, 0.0, 2.0, 1.0],
                vec![-4.0, -8.0, 0.0, 8.0, 4.0],
                vec![-6.0, -12.0, 0.0, 12.0, 6.0],
                vec![-4.0, -8.0, 0.0, 8.0, 4.0],
                vec![-1.0, -2.0, 0.0, 2.0, 1.0],
            ],
            KernelSize::Nine => vec![
                vec![-1.0, -2.0, -3.0, -4.0, 0.0, 4.0, 3.0, 2.0, 1.0],
                vec![-2.0, -4.0, -6.0, -8.0, 0.0, 8.0, 6.0, 4.0, 2.0],
                vec![-3.0, -6.0, -9.0, -12.0, 0.0, 12.0, 9.0, 6.0, 3.0],
                vec![-4.0, -8.0, -12.0, -16.0, 0.0, 16.0, 12.0, 8.0, 4.0],
                vec![-5.0, -10.0, -15.0, -20.0, 0.0, 20.0, 15.0, 10.0, 5.0],
                vec![-4.0, -8.0, -12.0, -16.0, 0.0, 16.0, 12.0, 8.0, 4.0],
                vec![-3.0, -6.0, -9.0, -12.0, 0.0, 12.0, 9.0, 6.0, 3.0],
                vec![-2.0, -4.0, -6.0, -8.0, 0.0, 8.0, 6.0, 4.0, 2.0],
                vec![-1.0, -2.0, -3.0, -4.0, 0.0, 4.0, 3.0, 2.0, 1.0],
            ],
            KernelSize::Low => vec![
                vec![1.0, 2.0, 1.0],
                vec![0.0, 0.0, 0.0],
                vec![-1.0, -2.0, -1.0],
            ],
            KernelSize::High => vec![
                vec![3.0, 10.0, 3.0],
                vec![0.0, 0.0, 0.0],
                vec![-3.0, -10.0, -3.0],
            ],
        }
    }

    fn get_y_kernel(&self, kernel_size: KernelSize) -> Vec<Vec<f32>> {
        match kernel_size {
            KernelSize::Three => vec![
                vec![-1.0, -2.0, -1.0],
                vec![0.0, 0.0, 0.0],
                vec![1.0, 2.0, 1.0],
            ],
            KernelSize::Five => vec![
                vec![-1.0, -4.0, -6.0, -4.0, -1.0],
                vec![-2.0, -8.0, -12.0, -8.0, -2.0],
                vec![0.0, 0.0, 0.0, 0.0, 0.0],
                vec![2.0, 8.0, 12.0, 8.0, 2.0],
                vec![1.0, 4.0, 6.0, 4.0, 1.0],
            ],
            KernelSize::Nine => vec![
                vec![-1.0, -2.0, -3.0, -4.0, -5.0, -4.0, -3.0, -2.0, -1.0],
                vec![-2.0, -4.0, -6.0, -8.0, -10.0, -8.0, -6.0, -4.0, -2.0],
                vec![-3.0, -6.0, -9.0, -12.0, -15.0, -12.0, -9.0, -6.0, -3.0],
                vec![-4.0, -8.0, -12.0, -16.0, -20.0, -16.0, -12.0, -8.0, -4.0],
                vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                vec![4.0, 8.0, 12.0, 16.0, 20.0, 16.0, 12.0, 8.0, 4.0],
                vec![3.0, 6.0, 9.0, 12.0, 15.0, 12.0, 9.0, 6.0, 3.0],
                vec![2.0, 4.0, 6.0, 8.0, 10.0, 8.0, 6.0, 4.0, 2.0],
                vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0],
            ],
            KernelSize::Low => vec![
                vec![1.0, 0.0, -1.0],
                vec![2.0, 0.0, -2.0],
                vec![1.0, 0.0, -1.0],
            ],
            KernelSize::High => vec![
                vec![3.0, 0.0, -3.0],
                vec![10.0, 0.0, -10.0],
                vec![3.0, 0.0, -3.0],
            ],
        }
    }

    fn apply_kernel(&self, kernel: &[Vec<f32>]) -> f32 {
        let mut sum = 0.0;
        for y in 0..self.size {
            for x in 0..self.size {
                sum += self.pixels[y][x] * kernel[y][x];
            }
        }
        sum
    }
}

fn fetch_pixel(x: u32, y: u32, img: &GrayImage) -> f32 {
    let x = x.min(img.width() - 1);
    let y = y.min(img.height() - 1);
    (img.get_pixel(x, y)[0] as f32) / 255.0
}

pub fn map_normals(img: &DynamicImage, kernel_size: KernelSize) -> RgbImage {
    map_normals_with_strength(img, DEFAULT_STRENGTH, kernel_size)
}

pub fn map_normals_with_strength(
    img: &DynamicImage,
    strength: f32,
    kernel_size: KernelSize,
) -> RgbImage {
    let img = img.clone().into_luma8();
    let mut normal_map = RgbImage::new(img.width(), img.height());

    for (x, y, p) in normal_map.enumerate_pixels_mut() {
        let mut new_p = [0.0, 0.0, 0.0];
        let s = AdjPixels::new(x, y, &img, kernel_size);

        new_p[0] = s.x_normals(kernel_size);
        new_p[1] = -s.y_normals(kernel_size);
        new_p[2] = 1.0 / strength;

        let new_p = scale_normalized_to_0_to_1(&normalize(new_p));

        p[0] = (new_p[0] * 255.0) as u8;
        p[1] = (new_p[1] * 255.0) as u8;
        p[2] = (new_p[2] * 255.0) as u8;
    }
    normal_map
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let v_mag = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / v_mag, v[1] / v_mag, v[2] / v_mag]
}

fn scale_normalized_to_0_to_1(v: &[f32; 3]) -> [f32; 3] {
    [v[0] * 0.5 + 0.5, v[1] * 0.5 + 0.5, v[2] * 0.5 + 0.5]
}
