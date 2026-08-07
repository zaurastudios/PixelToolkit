//! Math helper fns ported from `Extensions/MathEx.cs`

pub fn cos_d(degrees: f32) -> f32 {
    (degrees * std::f32::consts::PI / 180.0).cos()
}

pub fn sin_d(degrees: f32) -> f32 {
    (degrees * std::f32::consts::PI / 180.0).sin()
}

pub fn asin_d(value: f32) -> f32 {
    value.asin() * (180.0 / std::f32::consts::PI)
}

pub fn near_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < f32::EPSILON
}

pub fn invert(value: &mut f32, min_value: f32, max_value: f32) {
    *value = max_value - (*value - min_value);
}

pub fn lerp(min: f32, max: f32, mix: f32) -> f32 {
    min * (1.0 - mix) + max * mix
}

pub fn wrap(value: &mut f32, min: f32, max: f32) {
    while *value < min {
        *value += max - min;
    }
    while *value > max {
        *value -= max - min;
    }
}

pub fn wrap_exclusive(value: &mut i32, min: i32, max: i32) {
    while *value < min {
        *value += max - min;
    }
    while *value >= max {
        *value -= max - min;
    }
}

pub fn cycle(value: &mut f32, offset: i32, min: f32, max: f32) {
    *value += offset as f32 / 255.0;
    let step = max - min + (1.0 / 255.0);
    while *value < min {
        *value += step;
    }
    while *value > max {
        *value -= step;
    }
}
