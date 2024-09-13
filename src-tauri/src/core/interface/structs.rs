use image::Rgba;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MatYml {
    pub color: Option<DefaultsGreyscale>,
    pub rough: Option<DefaultsGreyscale>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DefaultsGreyscale {
    pub value: Option<f32>,
    pub scale: Option<f32>,
    pub shift: Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Normal {
    pub curve_x: Option<usize>,
    pub curve_y: Option<usize>,
}

pub struct TextureFile {
    pub name: &'static str,
    pub pattern: &'static str,
    pub greyscale: bool,
    pub alternate: Option<&'static str>,
    pub default_color: Rgba<u8>,
}

pub const TEXTURE_FILES: [TextureFile; 13] = [
    TextureFile {
        name: "color",
        pattern: r".*(albedo|color).*\.png$",
        greyscale: false,
        alternate: None,
        default_color: Rgba([0, 0, 0, 255]),
    },
    TextureFile {
        name: "opacity",
        pattern: r".*opacity.*\.png$",
        greyscale: true,
        alternate: None,
        default_color: Rgba([255, 255, 255, 255]),
    },
    TextureFile {
        name: "height",
        pattern: r".*height.*\.png$",
        greyscale: true,
        alternate: None,
        default_color: Rgba([255, 255, 255, 255]),
    },
    TextureFile {
        name: "normal",
        pattern: r".*normal.*\.png$",
        greyscale: false,
        alternate: None,
        default_color: Rgba([132, 131, 255, 255]),
    },
    TextureFile {
        name: "occlusion",
        pattern: r".*(occlusion|ao).*\.png$",
        greyscale: true,
        alternate: None,
        default_color: Rgba([0, 0, 0, 0]),
    },
    TextureFile {
        name: "smooth",
        pattern: r".*smooth.*\.png$",
        greyscale: true,
        alternate: Some(r".*rough.*\.png$"),
        default_color: Rgba([0, 0, 0, 255]),
    },
    TextureFile {
        name: "rough",
        pattern: r".*rough.*\.png$",
        greyscale: true,
        alternate: Some(r".*smooth.*\.png$"),
        default_color: Rgba([255, 255, 255, 255]),
    },
    TextureFile {
        name: "metal",
        pattern: r".*metal.*\.png$",
        greyscale: true,
        alternate: None,
        default_color: Rgba([0, 0, 0, 255]),
    },
    TextureFile {
        name: "hcm",
        pattern: r".*hcm.*\.png$",
        greyscale: true,
        alternate: None,
        default_color: Rgba([0, 0, 0, 255]),
    },
    TextureFile {
        name: "f0",
        pattern: r".*f0.*\.png$",
        greyscale: true,
        alternate: None,
        default_color: Rgba([10, 10, 10, 255]),
    },
    TextureFile {
        name: "porosity",
        pattern: r".*porosity.*\.png$",
        greyscale: true,
        alternate: None,
        default_color: Rgba([0, 0, 0, 255]),
    },
    TextureFile {
        name: "sss",
        pattern: r".*sss.*\.png$",
        greyscale: true,
        alternate: None,
        default_color: Rgba([0, 0, 0, 255]),
    },
    TextureFile {
        name: "emissive",
        pattern: r".*emissive.*\.png$",
        greyscale: true,
        alternate: None,
        default_color: Rgba([0, 0, 0, 255]),
    },
];
