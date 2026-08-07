mod channel;
pub mod mapping;
pub mod math_ex;

pub use channel::{ColorChannel, EncodingChannel, TextureTag};

pub fn quantize(v: f32) -> u8 {
    (v * 255.0 + 0.5).clamp(0.0, 255.0) as u8
}

pub fn quantize16(v: f32) -> u16 {
    (v * 65535.0 + 0.5).clamp(0.0, 65535.0) as u16
}
