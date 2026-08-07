//! `nearest`, `bilinear`, `bicubic`, `average`, `weighted-average`

use crate::image::{ImageBuf, Pixel};
use crate::pixel::ColorChannel;
use glam::Vec4;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SamplerKind {
    Nearest,
    Bilinear,
    Bicubic,
    Average,
    WeightedAverage,
    // TODO: Implement Lanczos
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct UvRegion {
    pub left: f64,
    pub top: f64,
    /// Renamed from width
    pub right: f64,
    /// Renamed from height
    pub bottom: f64,
}

impl UvRegion {
    pub const FULL: UvRegion = UvRegion {
        left: 0.0,
        top: 0.0,
        right: 1.0,
        bottom: 1.0,
    };
    pub const EMPTY: UvRegion = UvRegion {
        left: 0.0,
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
    };

    pub fn width(&self) -> f64 {
        self.right - self.left
    }

    pub fn height(&self) -> f64 {
        self.bottom - self.top
    }

    pub fn scale_to(&self, width: u32, height: u32) -> PixelBounds {
        PixelBounds {
            left: (self.left * width as f64).round_ties_even() as i32,
            top: (self.top * height as f64).round_ties_even() as i32,
            width: (self.width() * width as f64).round_ties_even() as i32,
            height: (self.height() * height as f64).round_ties_even() as i32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PixelBounds {
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

impl PixelBounds {
    pub fn right(&self) -> i32 {
        self.left + self.width
    }

    pub fn bottom(&self) -> i32 {
        self.top + self.height
    }
}

fn wrap_coord_x(mut x: i32, b: PixelBounds) -> i32 {
    if b.width <= 0 {
        return b.left;
    }
    while x < b.left {
        x += b.width;
    }
    while x >= b.right() {
        x -= b.width;
    }

    x
}

fn wrap_coord_y(mut y: i32, b: PixelBounds) -> i32 {
    if b.height <= 0 {
        return b.top;
    }
    while y < b.top {
        y += b.height;
    }
    while y >= b.bottom() {
        y -= b.height;
    }

    y
}

fn clamp_coord_x(x: i32, b: PixelBounds) -> i32 {
    x.clamp(b.left, b.right() - 1)
}

fn clamp_coord_y(y: i32, b: PixelBounds) -> i32 {
    y.clamp(b.top, b.bottom() - 1)
}

pub enum RowSampler<'a, P: Pixel> {
    Nearest {
        row: &'a [P],
        bounds: PixelBounds,
        wrap_x: bool,
    },
    Bilinear {
        row_min: &'a [P],
        row_max: &'a [P],
        bounds: PixelBounds,
        wrap_x: bool,
        range_x: f32,
        /// The vertical lerp weight, precomputed once at `ForRow` time from
        /// the fractional part of `fy`; `row_min`/`row_max` were already
        /// chosen using the same `fy`, so only the horizontal weight is
        /// recomputed per pixel.
        frac_y: f32,
    },
    Average {
        rows: Vec<&'a [P]>,
        bounds: PixelBounds,
        wrap_x: bool,
        range_x: f32,
    },
    Bicubic {
        rows: [&'a [P]; 4],
        bounds: PixelBounds,
        wrap_x: bool,
        /// Same as Bilinear
        frac_y: f32,
    },
    WeightedAverage {
        rows: Vec<&'a [P]>,
        bounds: PixelBounds,
        wrap_x: bool,
        range_x: f32,
        nearest_weight: f32,
    },
}

fn cubic_hermite(row: [Vec4; 4], t: f32) -> Vec4 {
    let a = row[0] * -0.5 + row[1] * 1.5 - row[2] * 1.5 + row[3] * 0.5;
    let b = row[0] - row[1] * 2.5 + row[2] * 2.0 - row[3] * 0.5;
    let c = row[0] * -0.5 + row[2] * 0.5;
    a * (t * t * t) + b * (t * t) + c * t + row[1]
}

impl<'a, P: Pixel> RowSampler<'a, P> {
    pub fn sample_scaled(&self, x: f64) -> Vec4 {
        match self {
            RowSampler::Nearest {
                row,
                bounds,
                wrap_x,
            } => {
                let fx = bounds.left as f64 + x * bounds.width as f64;
                let px = fx.floor() as i32;
                let px = if *wrap_x {
                    wrap_coord_x(px, *bounds)
                } else {
                    clamp_coord_x(px, *bounds)
                };
                row[(px - bounds.left) as usize].to_vec4()
            }

            RowSampler::Bilinear {
                row_min,
                row_max,
                bounds,
                wrap_x,
                range_x,
                frac_y,
            } => {
                let min_range_x = range_x.ceil() as i32;
                let fx = bounds.left as f64 + x * bounds.width as f64 - 0.5 * min_range_x as f64;

                let px_min = fx.floor() as i32;
                let px_max = px_min + min_range_x;
                let frac_x = (fx - px_min as f64) as f32;

                let (px_min, px_max) = if *wrap_x {
                    (wrap_coord_x(px_min, *bounds), wrap_coord_x(px_max, *bounds))
                } else {
                    (
                        clamp_coord_x(px_min, *bounds),
                        clamp_coord_x(px_max, *bounds),
                    )
                };

                let a = row_min[(px_min - bounds.left) as usize].to_vec4();
                let b = row_min[(px_max - bounds.left) as usize].to_vec4();
                let c = row_max[(px_min - bounds.left) as usize].to_vec4();
                let d = row_max[(px_max - bounds.left) as usize].to_vec4();

                let z_min = a.lerp(b, frac_x);
                let z_max = c.lerp(d, frac_x);
                z_min.lerp(z_max, *frac_y)
            }

            RowSampler::Average {
                rows,
                bounds,
                wrap_x,
                range_x,
            } => {
                let fx = bounds.left as f64 + x * bounds.width as f64;
                let step_x = range_x.max(1.0).ceil() as i32;
                let step_y = rows.len() as i32;
                let weight = (step_x * step_y) as f32;

                let px_min = (fx - 0.5 * step_x as f64).floor() as i32;

                let mut sum = Vec4::ZERO;
                for row in rows {
                    for i in 0..step_x {
                        let px = px_min + i;
                        let px = if *wrap_x {
                            wrap_coord_x(px, *bounds)
                        } else {
                            clamp_coord_x(px, *bounds)
                        };
                        sum += row[(px - bounds.left) as usize].to_vec4();
                    }
                }
                sum / weight
            }

            RowSampler::Bicubic {
                rows,
                bounds,
                wrap_x,
                frac_y,
            } => {
                let fx = bounds.left as f64 + x * bounds.width as f64 - 1.5;
                let px_min = fx.floor() as i32;
                let frac_x = (fx - px_min as f64) as f32;

                let col: [Vec4; 4] = std::array::from_fn(|ky| {
                    let row = rows[ky];
                    let k: [Vec4; 4] = std::array::from_fn(|kx| {
                        let lx = px_min + kx as i32;
                        let lx = if *wrap_x {
                            wrap_coord_x(lx, *bounds)
                        } else {
                            clamp_coord_x(lx, *bounds)
                        };
                        row[(lx - bounds.left) as usize].to_vec4()
                    });
                    cubic_hermite(k, frac_x)
                });
                cubic_hermite(col, *frac_y)
            }

            RowSampler::WeightedAverage {
                rows,
                bounds,
                wrap_x,
                range_x,
                nearest_weight,
            } => {
                let fx = bounds.left as f64 + x * bounds.width as f64;
                let step_x = range_x.max(1.0).ceil() as i32;
                let step_y = rows.len() as i32;
                let weight = (step_x * step_y) as f32;

                let px_min = (fx - 0.5 * step_x as f64).floor() as i32;

                let mut avg_weight = 0.0f32;
                if weight > 1.0 {
                    avg_weight = 1.0 / (weight - 1.0);
                }
                avg_weight *= 1.0 - nearest_weight;
                let near_weight = if weight > 1.0 { *nearest_weight } else { 1.0 };

                let cx = (step_x as f32 * 0.5 + f32::EPSILON).floor() as i32;
                let cy = (step_y as f32 * 0.5 + f32::EPSILON).floor() as i32;

                let mut sum = Vec4::ZERO;
                for (iy, row) in rows.iter().enumerate() {
                    for ix in 0..step_x {
                        let px = px_min + ix;
                        let px = if *wrap_x {
                            wrap_coord_x(px, *bounds)
                        } else {
                            clamp_coord_x(px, *bounds)
                        };
                        let v = row[(px - bounds.left) as usize].to_vec4();
                        if iy as i32 == cy && ix == cx {
                            sum += v * near_weight;
                        } else {
                            sum += v * avg_weight;
                        }
                    }
                }
                sum
            }
        }
    }

    pub fn sample_channel(&self, x: f64, color: ColorChannel) -> f32 {
        color.extract(self.sample_scaled(x))
    }
}

pub struct Sampler<'a, P: Pixel> {
    kind: SamplerKind,
    image: &'a ImageBuf<P>,
    bounds: PixelBounds,
    wrap_x: bool,
    wrap_y: bool,
    range_x: f32,
    range_y: f32,
    nearest_weight: f32,
}

impl<'a, P: Pixel> Sampler<'a, P> {
    pub fn new(kind: SamplerKind, image: &'a ImageBuf<P>) -> Self {
        Self {
            kind,
            image,
            bounds: PixelBounds {
                left: 0,
                top: 0,
                width: image.width() as i32,
                height: image.height() as i32,
            },
            wrap_x: true,
            wrap_y: true,
            range_x: 1.0,
            range_y: 1.0,
            nearest_weight: 0.5,
        }
    }

    pub fn with_wrap(mut self, wrap_x: bool, wrap_y: bool) -> Self {
        self.wrap_x = wrap_x;
        self.wrap_y = wrap_y;
        self
    }

    pub fn with_range(mut self, range_x: f32, range_y: f32) -> Self {
        self.range_x = range_x;
        self.range_y = range_y;
        self
    }

    /// Defaults to 0.5
    pub fn with_nearest_weight(mut self, nearest_weight: f32) -> Self {
        self.nearest_weight = nearest_weight;
        self
    }

    pub fn set_bounds(&mut self, region: UvRegion) {
        self.bounds = region.scale_to(self.image.width(), self.image.height());
    }

    fn get_texcoord_y(&self, y: f64) -> f64 {
        self.bounds.top as f64 + y * self.bounds.height as f64
    }

    pub fn for_row(&self, y: f64) -> RowSampler<'a, P> {
        match self.kind {
            SamplerKind::Nearest => self.for_row_nearest(y),
            SamplerKind::Bilinear => self.for_row_bilinear(y),
            SamplerKind::Average => self.for_row_average(y),
            SamplerKind::Bicubic => self.for_row_bicubic(y),
            SamplerKind::WeightedAverage => self.for_row_weighted_average(y),
        }
    }

    pub fn sample_scaled(&self, x: f64, y: f64) -> Vec4 {
        self.for_row(y).sample_scaled(x)
    }

    pub fn sample_channel(&self, x: f64, y: f64, color: ColorChannel) -> f32 {
        color.extract(self.sample_scaled(x, y))
    }

    fn for_row_nearest(&self, y: f64) -> RowSampler<'a, P> {
        let fy = self.get_texcoord_y(y);
        let py = fy.floor() as i32;
        let py = if self.wrap_y {
            wrap_coord_y(py, self.bounds)
        } else {
            clamp_coord_y(py, self.bounds)
        };

        RowSampler::Nearest {
            row: &self.image.row(py as u32)
                [self.bounds.left as usize..self.bounds.right() as usize],
            bounds: self.bounds,
            wrap_x: self.wrap_x,
        }
    }

    fn for_row_bilinear(&self, y: f64) -> RowSampler<'a, P> {
        let fy = self.get_texcoord_y(y);
        let min_range_y = self.range_y.ceil();
        let fy = fy - 0.5 * min_range_y as f64;

        let py_min = fy.floor() as i32;
        let py_max = py_min + min_range_y as i32;
        let frac_y = (fy - py_min as f64) as f32;

        let (py_min, py_max) = if self.wrap_y {
            (
                wrap_coord_y(py_min, self.bounds),
                wrap_coord_y(py_max, self.bounds),
            )
        } else {
            (
                clamp_coord_y(py_min, self.bounds),
                clamp_coord_y(py_max, self.bounds),
            )
        };

        RowSampler::Bilinear {
            row_min: &self.image.row(py_min as u32)
                [self.bounds.left as usize..self.bounds.right() as usize],
            row_max: &self.image.row(py_max as u32)
                [self.bounds.left as usize..self.bounds.right() as usize],
            bounds: self.bounds,
            wrap_x: self.wrap_x,
            range_x: self.range_x,
            frac_y,
        }
    }

    fn for_row_average(&self, y: f64) -> RowSampler<'a, P> {
        let fy = self.get_texcoord_y(y);
        let min_range_y = self.range_y.max(1.0);
        let step_y = min_range_y.ceil() as i32;
        let py_min = (fy - 0.5 * step_y as f64).floor() as i32;

        let rows = (0..step_y)
            .map(|i| {
                let py = if self.wrap_y {
                    wrap_coord_y(py_min + i, self.bounds)
                } else {
                    clamp_coord_y(py_min + i, self.bounds)
                };
                &self.image.row(py as u32)[self.bounds.left as usize..self.bounds.right() as usize]
            })
            .collect();

        RowSampler::Average {
            rows,
            bounds: self.bounds,
            wrap_x: self.wrap_x,
            range_x: self.range_x,
        }
    }

    fn for_row_bicubic(&self, y: f64) -> RowSampler<'a, P> {
        let fy = self.get_texcoord_y(y) - 1.5;
        let py_min = fy.floor() as i32;
        let frac_y = (fy - py_min as f64) as f32;

        let rows: [&[P]; 4] = std::array::from_fn(|i| {
            let py = py_min + i as i32;
            let py = if self.wrap_y {
                wrap_coord_y(py, self.bounds)
            } else {
                clamp_coord_y(py, self.bounds)
            };
            &self.image.row(py as u32)[self.bounds.left as usize..self.bounds.right() as usize]
        });

        RowSampler::Bicubic {
            rows,
            bounds: self.bounds,
            wrap_x: self.wrap_x,
            frac_y,
        }
    }

    fn for_row_weighted_average(&self, y: f64) -> RowSampler<'a, P> {
        let fy = self.get_texcoord_y(y);
        let min_range_y = self.range_y.max(1.0);
        let step_y = min_range_y.ceil() as i32;
        let py_min = (fy - 0.5 * step_y as f64).floor() as i32;

        let rows = (0..step_y)
            .map(|i| {
                let py = if self.wrap_y {
                    wrap_coord_y(py_min + i, self.bounds)
                } else {
                    clamp_coord_y(py_min + i, self.bounds)
                };
                &self.image.row(py as u32)[self.bounds.left as usize..self.bounds.right() as usize]
            })
            .collect();

        RowSampler::WeightedAverage {
            rows,
            bounds: self.bounds,
            wrap_x: self.wrap_x,
            range_x: self.range_x,
            nearest_weight: self.nearest_weight,
        }
    }
}
