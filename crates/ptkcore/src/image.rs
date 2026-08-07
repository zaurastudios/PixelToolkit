//! Minimal ImageBuf/pixel typings.
//! TODO: Have a bridge between `image` crate and this

use crate::pixel::{quantize, quantize16};
use glam::Vec4;

pub trait Pixel: Copy + Default {
    fn to_vec4(&self) -> Vec4;
    fn from_vec4(v: Vec4) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Luma8(pub u8);

impl Pixel for Luma8 {
    fn to_vec4(&self) -> Vec4 {
        let l = self.0 as f32 / 255.0;
        Vec4::new(l, l, l, 1.0)
    }

    fn from_vec4(v: Vec4) -> Self {
        Luma8(quantize(v.x))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rgb8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel for Rgb8 {
    fn to_vec4(&self) -> Vec4 {
        Vec4::new(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            1.0,
        )
    }

    fn from_vec4(v: Vec4) -> Self {
        Rgb8 {
            r: quantize(v.x),
            g: quantize(v.y),
            b: quantize(v.z),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rgba8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel for Rgba8 {
    fn to_vec4(&self) -> Vec4 {
        Vec4::new(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
    }

    fn from_vec4(v: Vec4) -> Self {
        Rgba8 {
            r: quantize(v.x),
            g: quantize(v.y),
            b: quantize(v.z),
            a: quantize(v.w),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Luma16(pub u16);

impl Pixel for Luma16 {
    fn to_vec4(&self) -> Vec4 {
        let l = self.0 as f32 / 65535.0;
        Vec4::new(l, l, l, 1.0)
    }

    fn from_vec4(v: Vec4) -> Self {
        Luma16(quantize16(v.x))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LumaAlpha8 {
    pub l: u8,
    pub a: u8,
}

impl Pixel for LumaAlpha8 {
    fn to_vec4(&self) -> Vec4 {
        let l = self.l as f32 / 255.0;
        Vec4::new(l, l, l, self.a as f32 / 255.0)
    }

    fn from_vec4(v: Vec4) -> Self {
        LumaAlpha8 {
            l: quantize(v.x),
            a: quantize(v.w),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LumaAlpha16 {
    pub l: u16,
    pub a: u16,
}

impl Pixel for LumaAlpha16 {
    fn to_vec4(&self) -> Vec4 {
        let l = self.l as f32 / 65535.0;
        Vec4::new(l, l, l, self.a as f32 / 65535.0)
    }

    fn from_vec4(v: Vec4) -> Self {
        LumaAlpha16 {
            l: quantize16(v.x),
            a: quantize16(v.w),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rgb16 {
    pub r: u16,
    pub g: u16,
    pub b: u16,
}

impl Pixel for Rgb16 {
    fn to_vec4(&self) -> Vec4 {
        Vec4::new(
            self.r as f32 / 65535.0,
            self.g as f32 / 65535.0,
            self.b as f32 / 65535.0,
            1.0,
        )
    }

    fn from_vec4(v: Vec4) -> Self {
        Rgb16 {
            r: quantize16(v.x),
            g: quantize16(v.y),
            b: quantize16(v.z),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rgba16 {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub a: u16,
}

impl Pixel for Rgba16 {
    fn to_vec4(&self) -> Vec4 {
        Vec4::new(
            self.r as f32 / 65535.0,
            self.g as f32 / 65535.0,
            self.b as f32 / 65535.0,
            self.a as f32 / 65535.0,
        )
    }

    fn from_vec4(v: Vec4) -> Self {
        Rgba16 {
            r: quantize16(v.x),
            g: quantize16(v.y),
            b: quantize16(v.z),
            a: quantize16(v.w),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageBuf<P: Pixel> {
    width: u32,
    height: u32,
    pixels: Vec<P>,
}

impl<P: Pixel> ImageBuf<P> {
    pub fn new(width: u32, height: u32) -> Self {
        Self::from_pixel(width, height, P::default())
    }

    pub fn from_pixel(width: u32, height: u32, fill: P) -> Self {
        Self {
            width,
            height,
            pixels: vec![fill; (width as usize) * (height as usize)],
        }
    }

    pub fn from_fn(width: u32, height: u32, mut f: impl FnMut(u32, u32) -> P) -> Self {
        let mut pixels = Vec::with_capacity((width as usize) * (height as usize));
        for y in 0..height {
            for x in 0..width {
                pixels.push(f(x, y));
            }
        }

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn index(&self, x: u32, y: u32) -> usize {
        debug_assert!(
            x < self.width && y < self.height,
            "pixel ({x}, {y}) out of bounds ({}, {})",
            self.width,
            self.height
        );
        (y as usize) * (self.width as usize) + (x as usize)
    }

    pub fn get(&self, x: u32, y: u32) -> P {
        self.pixels[self.index(x, y)]
    }

    pub fn set(&mut self, x: u32, y: u32, p: P) {
        let i = self.index(x, y);
        self.pixels[i] = p;
    }

    pub fn crop(&self, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self::from_fn(width, height, |dx, dy| {
            self.get((x + dx as i32) as u32, (y + dy as i32) as u32)
        })
    }

    pub fn row(&self, y: u32) -> &[P] {
        let start = self.index(0, y);
        &self.pixels[start..start + self.width as usize]
    }
}
