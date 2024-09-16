use std::borrow::Cow;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct MatYml {
    // pub color: Option<DefaultsGrayscale>,
    pub normal: Option<Normal>,
    pub opacity: Option<DefaultsGrayscale>,
    pub smooth: Option<DefaultsGrayscale>,
    pub rough: Option<DefaultsGrayscale>,
    pub porosity: Option<DefaultsGrayscale>,
    pub metal: Option<DefaultsGrayscale>,
    pub f0: Option<DefaultsGrayscale>,
    pub sss: Option<DefaultsGrayscale>,
    pub emissive: Option<DefaultsGrayscale>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DefaultsGrayscale {
    pub value: Option<f32>,
    pub scale: Option<f32>,
    pub shift: Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Normal {
    pub curve_x: Option<usize>,
    pub curve_y: Option<usize>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ExtendedGrayscale {
    pub use_og: bool,
    pub values: DefaultsGrayscale,
}

pub struct TextureFile {
    pub name: &'static str,
    pub pattern: &'static str,
    pub grayscale: bool,
    pub defaults: Defaults,
}

pub const TEXTURE_FILES: [TextureFile; 13] = [
    TextureFile {
        name: "color",
        pattern: r".*(?i)(albedo|color).*\.png$",
        grayscale: false,
        defaults: Defaults {
            bit_depth: png::BitDepth::Eight,
            color_type: png::ColorType::Rgb,
            width: 16,
            height: 16,
            default_color: Some([0, 0, 0]),
        },
    },
    TextureFile {
        name: "opacity",
        pattern: r".*(?i)opacity.*\.png$",
        grayscale: true,
        defaults: Defaults {
            bit_depth: png::BitDepth::Eight,
            color_type: png::ColorType::Grayscale,
            width: 16,
            height: 16,
            default_color: Some([255, 255, 255]),
        },
    },
    TextureFile {
        name: "height",
        pattern: r".*(?i)height.*\.png$",
        grayscale: true,
        defaults: Defaults {
            bit_depth: png::BitDepth::Eight,
            color_type: png::ColorType::Grayscale,
            width: 16,
            height: 16,
            default_color: Some([255, 255, 255]),
        },
    },
    TextureFile {
        name: "normal",
        pattern: r".*(?i)normal.*\.png$",
        grayscale: false,
        defaults: Defaults {
            bit_depth: png::BitDepth::Eight,
            color_type: png::ColorType::Rgb,
            width: 16,
            height: 16,
            default_color: Some([128, 128, 255]),
        },
    },
    TextureFile {
        name: "occlusion",
        pattern: r".*(?i)(occlusion|ao).*\.png$",
        grayscale: true,
        defaults: Defaults {
            bit_depth: png::BitDepth::Eight,
            color_type: png::ColorType::Grayscale,
            width: 16,
            height: 16,
            default_color: Some([0, 0, 0]),
        },
    },
    TextureFile {
        name: "smooth",
        pattern: r".*(?i)smooth.*\.png$",
        grayscale: true,
        defaults: Defaults {
            bit_depth: png::BitDepth::Eight,
            color_type: png::ColorType::Grayscale,
            width: 16,
            height: 16,
            default_color: Some([0, 0, 0]),
        },
    },
    TextureFile {
        name: "rough",
        pattern: r".*(?i)rough.*\.png$",
        grayscale: true,
        defaults: Defaults {
            bit_depth: png::BitDepth::Eight,
            color_type: png::ColorType::Grayscale,
            width: 16,
            height: 16,
            default_color: Some([255, 255, 255]),
        },
    },
    TextureFile {
        name: "metal",
        pattern: r".*(?i)metal.*\.png$",
        grayscale: true,
        defaults: Defaults {
            bit_depth: png::BitDepth::Eight,
            color_type: png::ColorType::Grayscale,
            width: 16,
            height: 16,
            default_color: Some([0, 0, 0]),
        },
    },
    TextureFile {
        name: "hcm",
        pattern: r".*(?i)hcm.*\.png$",
        grayscale: true,
        defaults: Defaults {
            bit_depth: png::BitDepth::Eight,
            color_type: png::ColorType::Grayscale,
            width: 16,
            height: 16,
            default_color: Some([0, 0, 0]),
        },
    },
    TextureFile {
        name: "f0",
        pattern: r".*(?i)f0.*\.png$",
        grayscale: true,
        defaults: Defaults {
            bit_depth: png::BitDepth::Eight,
            color_type: png::ColorType::Grayscale,
            width: 16,
            height: 16,
            default_color: Some([10, 10, 10]),
        },
    },
    TextureFile {
        name: "porosity",
        pattern: r".*(?i)porosity.*\.png$",
        grayscale: true,
        defaults: Defaults {
            bit_depth: png::BitDepth::Eight,
            color_type: png::ColorType::Grayscale,
            width: 16,
            height: 16,
            default_color: Some([0, 0, 0]),
        },
    },
    TextureFile {
        name: "sss",
        pattern: r".*(?i)sss.*\.png$",
        grayscale: true,
        defaults: Defaults {
            bit_depth: png::BitDepth::Eight,
            color_type: png::ColorType::Grayscale,
            width: 16,
            height: 16,
            default_color: Some([0, 0, 0]),
        },
    },
    TextureFile {
        name: "emissive",
        pattern: r".*(?i)emissive.*\.png$",
        grayscale: true,
        defaults: Defaults {
            bit_depth: png::BitDepth::Eight,
            color_type: png::ColorType::Grayscale,
            width: 16,
            height: 16,
            default_color: Some([0, 0, 0]),
        },
    },
];

#[derive(Clone)]
pub struct Defaults {
    pub bit_depth: png::BitDepth,
    pub color_type: png::ColorType,
    pub width: usize,
    pub height: usize,
    pub default_color: Option<[u8; 3]>,
}

pub struct PngImage {
    pub buf: Vec<u8>,
    pub info: Defaults,
    pub palette: Option<Cow<'static, [u8]>>,
}
