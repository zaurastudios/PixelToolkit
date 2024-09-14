use image::Rgba;

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
    pub alternate: Option<&'static str>,
    pub default_color: Rgba<u8>,
}

pub const TEXTURE_FILES: [TextureFile; 13] = [
    TextureFile {
        name: "color",
        pattern: r".*(albedo|color).*\.png$",
        grayscale: false,
        alternate: None,
        default_color: Rgba([0, 0, 0, 255]),
    },
    TextureFile {
        name: "opacity",
        pattern: r".*opacity.*\.png$",
        grayscale: true,
        alternate: None,
        default_color: Rgba([255, 255, 255, 255]),
    },
    TextureFile {
        name: "height",
        pattern: r".*height.*\.png$",
        grayscale: true,
        alternate: None,
        default_color: Rgba([255, 255, 255, 255]),
    },
    TextureFile {
        name: "normal",
        pattern: r".*normal.*\.png$",
        grayscale: false,
        alternate: None,
        default_color: Rgba([132, 131, 255, 255]),
    },
    TextureFile {
        name: "occlusion",
        pattern: r".*(occlusion|ao).*\.png$",
        grayscale: true,
        alternate: None,
        default_color: Rgba([0, 0, 0, 0]),
    },
    TextureFile {
        name: "smooth",
        pattern: r".*smooth.*\.png$",
        grayscale: true,
        alternate: Some(r".*rough.*\.png$"),
        default_color: Rgba([0, 0, 0, 255]),
    },
    TextureFile {
        name: "rough",
        pattern: r".*rough.*\.png$",
        grayscale: true,
        alternate: Some(r".*smooth.*\.png$"),
        default_color: Rgba([255, 255, 255, 255]),
    },
    TextureFile {
        name: "metal",
        pattern: r".*metal.*\.png$",
        grayscale: true,
        alternate: None,
        default_color: Rgba([0, 0, 0, 255]),
    },
    TextureFile {
        name: "hcm",
        pattern: r".*hcm.*\.png$",
        grayscale: true,
        alternate: None,
        default_color: Rgba([0, 0, 0, 255]),
    },
    TextureFile {
        name: "f0",
        pattern: r".*f0.*\.png$",
        grayscale: true,
        alternate: None,
        default_color: Rgba([10, 10, 10, 255]),
    },
    TextureFile {
        name: "porosity",
        pattern: r".*porosity.*\.png$",
        grayscale: true,
        alternate: None,
        default_color: Rgba([0, 0, 0, 255]),
    },
    TextureFile {
        name: "sss",
        pattern: r".*sss.*\.png$",
        grayscale: true,
        alternate: None,
        default_color: Rgba([0, 0, 0, 255]),
    },
    TextureFile {
        name: "emissive",
        pattern: r".*emissive.*\.png$",
        grayscale: true,
        alternate: None,
        default_color: Rgba([0, 0, 0, 255]),
    },
];
