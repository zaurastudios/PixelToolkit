use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EncodingChannel {
    Opacity,
    ColorRed,
    ColorGreen,
    ColorBlue,
    Height,
    Occlusion,
    NormalX,
    NormalY,
    NormalZ,
    Specular,
    Smooth,
    Rough,
    Metal,
    #[serde(rename = "hcm")]
    Hcm,
    #[serde(rename = "f0")]
    F0,
    Porosity,
    #[serde(rename = "sss")]
    Sss,
    Emissive,
}

impl EncodingChannel {
    pub const ALL: [EncodingChannel; 18] = [
        EncodingChannel::Opacity,
        EncodingChannel::ColorRed,
        EncodingChannel::ColorGreen,
        EncodingChannel::ColorBlue,
        EncodingChannel::Height,
        EncodingChannel::Occlusion,
        EncodingChannel::NormalX,
        EncodingChannel::NormalY,
        EncodingChannel::NormalZ,
        EncodingChannel::Specular,
        EncodingChannel::Smooth,
        EncodingChannel::Rough,
        EncodingChannel::Metal,
        EncodingChannel::Hcm,
        EncodingChannel::F0,
        EncodingChannel::Porosity,
        EncodingChannel::Sss,
        EncodingChannel::Emissive,
    ];

    pub const COUNT: usize = 18;

    // Position ordering;
    pub const fn index(self) -> usize {
        match self {
            EncodingChannel::Opacity => 0,
            EncodingChannel::ColorRed => 1,
            EncodingChannel::ColorGreen => 2,
            EncodingChannel::ColorBlue => 3,
            EncodingChannel::Height => 4,
            EncodingChannel::Occlusion => 5,
            EncodingChannel::NormalX => 6,
            EncodingChannel::NormalY => 7,
            EncodingChannel::NormalZ => 8,
            EncodingChannel::Specular => 9,
            EncodingChannel::Smooth => 10,
            EncodingChannel::Rough => 11,
            EncodingChannel::Metal => 12,
            EncodingChannel::Hcm => 13,
            EncodingChannel::F0 => 14,
            EncodingChannel::Porosity => 15,
            EncodingChannel::Sss => 16,
            EncodingChannel::Emissive => 17,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorChannel {
    Red,
    Green,
    Blue,
    Alpha,
    Magnitude, // LabPBR 1.1's AO encoding
}

impl ColorChannel {
    pub fn extract(self, v: glam::Vec4) -> f32 {
        match self {
            ColorChannel::Red => v.x,
            ColorChannel::Green => v.y,
            ColorChannel::Blue => v.z,
            ColorChannel::Alpha => v.w,
            ColorChannel::Magnitude => 0.0,
        }
    }

    pub fn write_into(self, v: &mut glam::Vec4, value: f32) {
        match self {
            ColorChannel::Red => v.x = value,
            ColorChannel::Green => v.y = value,
            ColorChannel::Blue => v.z = value,
            ColorChannel::Alpha => v.w = value,
            ColorChannel::Magnitude => {
                panic!("Magnitude is not a valid write target for a color channel")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TextureTag {
    None,
    Opacity,
    Color,
    Height,
    Bump,
    Normal,
    Occlusion,
    Specular,
    Rough,
    Smooth,
    Metal,
    #[serde(rename = "hcm")]
    Hcm,
    #[serde(rename = "f0")]
    F0,
    Porosity,
    #[serde(rename = "sss")]
    Sss,
    Emissive,
    #[serde(rename = "mer")]
    Mer,
    #[serde(rename = "mers")]
    Mers,
    Item,

    // Internal only
    General,
    #[serde(rename = "normal-generated")]
    NormalGenerated,
    #[serde(rename = "magnitude-buffer")]
    MagnitudeBuffer,
    #[serde(rename = "occlusion-generated")]
    OcclusionGenerated,
}
