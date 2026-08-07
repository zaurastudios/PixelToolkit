use super::math_ex;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PixelMapping {
    pub input_value: Option<f32>,
    pub input_min_value: f32,
    pub input_max_value: f32,
    pub input_range_min: u8,
    pub input_range_max: u8,
    pub input_channel_shift: i32,
    pub input_channel_power: f32,
    pub input_channel_inverted: bool,
    pub input_value_scale: f32,
    pub input_value_shift: f32,
    pub input_enable_clipping: bool,

    pub output_min_value: f32,
    pub output_max_value: f32,
    pub output_range_min: u8,
    pub output_range_max: u8,
    pub output_channel_shift: i32,
    pub output_channel_power: f32,
    pub output_channel_inverted: bool,
    pub output_value_scale: f32,
    pub output_value_shift: f32,
    pub output_enable_clipping: bool,
}

impl Default for PixelMapping {
    fn default() -> Self {
        Self {
            input_value: None,
            input_min_value: 0.0,
            input_max_value: 1.0,
            input_range_min: 0,
            input_range_max: 255,
            input_channel_shift: 0,
            input_channel_power: 1.0,
            input_channel_inverted: false,
            input_value_scale: 1.0,
            input_value_shift: 0.0,
            input_enable_clipping: false,

            output_min_value: 0.0,
            output_max_value: 1.0,
            output_range_min: 0,
            output_range_max: 255,
            output_channel_shift: 0,
            output_channel_power: 1.0,
            output_channel_inverted: false,
            output_value_scale: 1.0,
            output_value_shift: 0.0,
            output_enable_clipping: false,
        }
    }
}

impl PixelMapping {
    fn pixel_input_range(&self) -> i32 {
        self.input_range_max as i32 - self.input_range_min as i32
    }

    fn value_input_range(&self) -> f32 {
        self.input_max_value - self.input_min_value
    }

    fn pixel_output_range(&self) -> i32 {
        self.output_range_max as i32 - self.output_range_min as i32
    }

    fn value_output_range(&self) -> f32 {
        self.output_max_value - self.output_min_value
    }

    pub fn unmap(&self, raw: f32) -> Option<f32> {
        if let Some(v) = self.input_value {
            return Some(v);
        }

        let range_min = self.input_range_min as f32 / 255.0;
        let range_max = self.input_range_max as f32 / 255.0;

        if self.input_enable_clipping && (raw < range_min || raw > range_max) {
            return None;
        }

        let mut value = raw;

        if self.input_channel_shift != 0 {
            math_ex::cycle(&mut value, -self.input_channel_shift, range_min, range_max);
        }

        value -= self.input_range_min as f32 / 255.0;

        let pixel_input_range = self.pixel_input_range();
        if pixel_input_range > 0 {
            let pixel_range = pixel_input_range as f32 / 255.0;
            value *= self.value_input_range() / pixel_range;
        } else {
            value = 0.0;
        }

        value += self.input_min_value;

        if self.input_channel_inverted {
            math_ex::invert(&mut value, self.input_min_value, self.input_max_value);
        }

        if self.input_channel_power != 1.0 {
            value = value.powf(1.0 / self.input_channel_power);
        }

        // WARN: Which should come first?!
        // Directly from `Textures/PixelMapping.cs:161`
        // Following the same order in the original code
        value *= self.input_value_scale;
        value += self.input_value_shift;

        Some(value)
    }

    pub fn map(&self, value: f32) -> Option<f32> {
        let range_min = self.output_range_min as f32 / 255.0;
        let range_max = self.output_range_max as f32 / 255.0;

        let final_value = self.map_value(value);

        if self.output_enable_clipping && (final_value < range_min || final_value > range_max) {
            return None;
        }

        Some(self.finish_map(final_value, range_min, range_max))
    }

    pub fn map_or_clamp(&self, value: f32) -> f32 {
        let range_min = self.output_range_min as f32 / 255.0;
        let range_max = self.output_range_max as f32 / 255.0;

        let final_value = self.map_value(value);
        self.finish_map(final_value, range_min, range_max)
    }

    fn map_value(&self, value: f32) -> f32 {
        self.rescale_to_pixel_space(self.shift_scale_power_invert(value))
    }

    fn shift_scale_power_invert(&self, mut value: f32) -> f32 {
        value += self.output_value_shift;
        value *= self.output_value_scale;

        if self.output_channel_power != 1.0 {
            value = value.powf(self.output_channel_power);
        }

        if self.output_channel_inverted {
            math_ex::invert(&mut value, self.output_min_value, self.output_max_value);
        }

        value
    }

    fn rescale_to_pixel_space(&self, value: f32) -> f32 {
        let pixel_output_range = self.pixel_output_range();
        let value_output_range = self.value_output_range();
        let pixel_range = pixel_output_range as f32 / 255.0;

        let mut final_value = value - self.output_min_value;

        if pixel_range == 0.0 || value_output_range == 0.0 {
            final_value = 0.0;
        } else if !math_ex::near_equal(value_output_range, pixel_range) {
            final_value *= pixel_range / value_output_range;
        }

        final_value += self.output_range_min as f32 / 255.0;
        final_value
    }

    pub fn map_with_intermediate(&self, value: f32) -> Option<(f32, f32)> {
        let range_min = self.output_range_min as f32 / 255.0;
        let range_max = self.output_range_max as f32 / 255.0;

        let intermediate = self.shift_scale_power_invert(value);
        let final_value = self.rescale_to_pixel_space(intermediate);

        if self.output_enable_clipping && (final_value < range_min || final_value > range_max) {
            return None;
        }

        Some((
            intermediate,
            self.finish_map(final_value, range_min, range_max),
        ))
    }

    fn finish_map(&self, mut final_value: f32, range_min: f32, range_max: f32) -> f32 {
        final_value = final_value.clamp(range_min, range_max);

        if self.output_channel_shift != 0 {
            math_ex::cycle(
                &mut final_value,
                self.output_channel_shift,
                range_min,
                range_max,
            );
        }

        final_value
    }
}

pub fn convert_hcm_to_metal(unmapped: f32, output: &PixelMapping) -> f32 {
    if unmapped >= 230.0 - f32::EPSILON {
        output.output_max_value
    } else {
        output.output_min_value
    }
}

pub fn convert_metal_to_hcm(
    unmapped: f32,
    input: &PixelMapping,
    output: &PixelMapping,
) -> Option<f32> {
    let threshold = (input.input_min_value + input.input_max_value) * 0.5;
    if unmapped < threshold {
        None
    } else {
        Some(output.output_max_value)
    }
}
