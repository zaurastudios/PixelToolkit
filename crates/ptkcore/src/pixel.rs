pub mod channel;
pub mod mapping;
pub mod math_ex;

pub fn quantize(v: f32) -> u8 {
    (v * 255.0 + 0.5).clamp(0.0, 255.0) as u8
}
