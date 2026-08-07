use crate::pixel::{ColorChannel, EncodingChannel, TextureTag};
use crate::sampler::SamplerKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ChannelDescriptor {
    pub texture: Option<TextureTag>,
    pub color: Option<ColorChannel>,
    pub sampler: Option<SamplerKind>,
    #[serde(rename = "min-value")]
    pub min_value: Option<f32>,
    #[serde(rename = "max-value")]
    pub max_value: Option<f32>,
    #[serde(rename = "range-min")]
    pub range_min: Option<u8>,
    #[serde(rename = "range-max")]
    pub range_max: Option<u8>,
    pub shift: Option<i32>,
    pub power: Option<f32>,
    pub invert: Option<bool>,
    #[serde(rename = "default-value")]
    pub default_value: Option<f32>,
    #[serde(rename = "clip-value")]
    pub clip_value: Option<f32>,
    pub priority: Option<i32>,
    #[serde(rename = "enable-clipping")]
    pub enable_clipping: Option<bool>,
}

impl ChannelDescriptor {
    pub fn new(texture: TextureTag, color: ColorChannel) -> Self {
        ChannelDescriptor {
            texture: Some(texture),
            color: Some(color),
            ..Default::default()
        }
    }

    pub fn is_mapped(&self) -> bool {
        matches!(self.texture, Some(t) if t != TextureTag::None) && self.color.is_some()
    }

    #[must_use]
    pub fn merge(&self, other: &ChannelDescriptor) -> ChannelDescriptor {
        ChannelDescriptor {
            texture: other.texture.or(self.texture),
            color: other.color.or(self.color),
            sampler: other.sampler.or(self.sampler),
            min_value: other.min_value.or(self.min_value),
            max_value: other.max_value.or(self.max_value),
            range_min: other.range_min.or(self.range_min),
            range_max: other.range_max.or(self.range_max),
            shift: other.shift.or(self.shift),
            power: other.power.or(self.power),
            invert: other.invert.or(self.invert),
            default_value: other.default_value.or(self.default_value),
            clip_value: other.clip_value.or(self.clip_value),
            priority: other.priority.or(self.priority),
            enable_clipping: other.enable_clipping.or(self.enable_clipping),
        }
    }

    #[must_use]
    pub fn min(mut self, v: f32) -> Self {
        self.min_value = Some(v);
        self
    }
    #[must_use]
    pub fn max(mut self, v: f32) -> Self {
        self.max_value = Some(v);
        self
    }
    #[must_use]
    pub fn range(mut self, min: u8, max: u8) -> Self {
        self.range_min = Some(min);
        self.range_max = Some(max);
        self
    }
    #[must_use]
    pub fn shift(mut self, v: i32) -> Self {
        self.shift = Some(v);
        self
    }
    #[must_use]
    pub fn power(mut self, v: f32) -> Self {
        self.power = Some(v);
        self
    }
    #[must_use]
    pub fn invert(mut self) -> Self {
        self.invert = Some(true);
        self
    }
    #[must_use]
    pub fn default_value(mut self, v: f32) -> Self {
        self.default_value = Some(v);
        self
    }
    #[must_use]
    pub fn clip_value(mut self, v: f32) -> Self {
        self.clip_value = Some(v);
        self
    }
    #[must_use]
    pub fn priority(mut self, v: i32) -> Self {
        self.priority = Some(v);
        self
    }
    #[must_use]
    pub fn clipping(mut self) -> Self {
        self.enable_clipping = Some(true);
        self
    }
    #[must_use]
    pub fn no_clipping(mut self) -> Self {
        self.enable_clipping = Some(false);
        self
    }
    #[must_use]
    pub fn sampler(mut self, s: SamplerKind) -> Self {
        self.sampler = Some(s);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PackEncoding {
    channels: [Option<ChannelDescriptor>; EncodingChannel::COUNT],
}

impl PackEncoding {
    pub fn get(&self, channel: EncodingChannel) -> Option<&ChannelDescriptor> {
        self.channels[channel.index()].as_ref()
    }

    pub fn set(&mut self, channel: EncodingChannel, descriptor: ChannelDescriptor) {
        self.channels[channel.index()] = Some(descriptor);
    }

    #[must_use]
    pub fn merge(&self, other: &PackEncoding) -> PackEncoding {
        let mut result = PackEncoding::default();
        for c in EncodingChannel::ALL {
            let i = c.index();
            result.channels[i] = match (&self.channels[i], &other.channels[i]) {
                (Some(a), Some(b)) => Some(a.merge(b)),
                (Some(a), None) => Some(*a),
                (None, Some(b)) => Some(*b),
                (None, None) => None,
            };
        }

        result
    }

    pub fn mapped(&self) -> impl Iterator<Item = (EncodingChannel, &ChannelDescriptor)> {
        EncodingChannel::ALL
            .into_iter()
            .filter_map(move |c| self.get(c).filter(|d| d.is_mapped()).map(|d| (c, d)))
    }

    pub fn has_any_mapped(&self) -> bool {
        self.mapped().next().is_some()
    }
}

/// `project.yml`'s `input:` and project profile's `encoding:`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct PackEncodingWire {
    opacity: Option<ChannelDescriptor>,
    color_red: Option<ChannelDescriptor>,
    color_green: Option<ChannelDescriptor>,
    color_blue: Option<ChannelDescriptor>,
    height: Option<ChannelDescriptor>,
    occlusion: Option<ChannelDescriptor>,
    normal_x: Option<ChannelDescriptor>,
    normal_y: Option<ChannelDescriptor>,
    normal_z: Option<ChannelDescriptor>,
    specular: Option<ChannelDescriptor>,
    smooth: Option<ChannelDescriptor>,
    rough: Option<ChannelDescriptor>,
    metal: Option<ChannelDescriptor>,
    #[serde(rename = "hcm")]
    hcm: Option<ChannelDescriptor>,
    #[serde(rename = "f0")]
    f0: Option<ChannelDescriptor>,
    porosity: Option<ChannelDescriptor>,
    #[serde(rename = "sss")]
    sss: Option<ChannelDescriptor>,
    emissive: Option<ChannelDescriptor>,
}

impl From<PackEncodingWire> for PackEncoding {
    fn from(w: PackEncodingWire) -> Self {
        let mut e = PackEncoding::default();
        macro_rules! set {
            ($channel:expr, $field:expr) => {
                if let Some(d) = $field {
                    e.set($channel, d);
                }
            };
        }

        set!(EncodingChannel::Opacity, w.opacity);
        set!(EncodingChannel::ColorRed, w.color_red);
        set!(EncodingChannel::ColorGreen, w.color_green);
        set!(EncodingChannel::ColorBlue, w.color_blue);
        set!(EncodingChannel::Height, w.height);
        set!(EncodingChannel::Occlusion, w.occlusion);
        set!(EncodingChannel::NormalX, w.normal_x);
        set!(EncodingChannel::NormalY, w.normal_y);
        set!(EncodingChannel::NormalZ, w.normal_z);
        set!(EncodingChannel::Specular, w.specular);
        set!(EncodingChannel::Smooth, w.smooth);
        set!(EncodingChannel::Rough, w.rough);
        set!(EncodingChannel::Metal, w.metal);
        set!(EncodingChannel::Hcm, w.hcm);
        set!(EncodingChannel::F0, w.f0);
        set!(EncodingChannel::Porosity, w.porosity);
        set!(EncodingChannel::Sss, w.sss);
        set!(EncodingChannel::Emissive, w.emissive);

        e
    }
}

impl From<&PackEncoding> for PackEncodingWire {
    fn from(e: &PackEncoding) -> Self {
        PackEncodingWire {
            opacity: e.get(EncodingChannel::Opacity).copied(),
            color_red: e.get(EncodingChannel::ColorRed).copied(),
            color_green: e.get(EncodingChannel::ColorGreen).copied(),
            color_blue: e.get(EncodingChannel::ColorBlue).copied(),
            height: e.get(EncodingChannel::Height).copied(),
            occlusion: e.get(EncodingChannel::Occlusion).copied(),
            normal_x: e.get(EncodingChannel::NormalX).copied(),
            normal_y: e.get(EncodingChannel::NormalY).copied(),
            normal_z: e.get(EncodingChannel::NormalZ).copied(),
            specular: e.get(EncodingChannel::Specular).copied(),
            smooth: e.get(EncodingChannel::Smooth).copied(),
            rough: e.get(EncodingChannel::Rough).copied(),
            metal: e.get(EncodingChannel::Metal).copied(),
            hcm: e.get(EncodingChannel::Hcm).copied(),
            f0: e.get(EncodingChannel::F0).copied(),
            porosity: e.get(EncodingChannel::Porosity).copied(),
            sss: e.get(EncodingChannel::Sss).copied(),
            emissive: e.get(EncodingChannel::Emissive).copied(),
        }
    }
}

impl Serialize for PackEncoding {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        PackEncodingWire::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PackEncoding {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(PackEncodingWire::deserialize(deserializer)?.into())
    }
}

pub struct FormatDef {
    pub id: &'static str,
    pub aliases: &'static [&'static str],
    pub description: &'static str,
    pub hidden: bool,
    pub build: fn() -> PackEncoding,
}

/// Resolves a format id, case-insensitively, including deprecated aliases.
pub fn find_format(id: &str) -> Option<&'static FormatDef> {
    FORMATS.iter().find(|f| {
        f.id.eq_ignore_ascii_case(id) || f.aliases.iter().any(|a| a.eq_ignore_ascii_case(id))
    })
}

fn ch(texture: TextureTag, color: ColorChannel) -> ChannelDescriptor {
    ChannelDescriptor::new(texture, color)
}

fn raw() -> PackEncoding {
    use EncodingChannel as E;
    use TextureTag as T;
    let mut e = PackEncoding::default();
    e.set(
        E::Opacity,
        ch(T::Opacity, ColorChannel::Red).default_value(1.0),
    );
    e.set(E::ColorRed, ch(T::Color, ColorChannel::Red));
    e.set(E::ColorGreen, ch(T::Color, ColorChannel::Green));
    e.set(E::ColorBlue, ch(T::Color, ColorChannel::Blue));
    e.set(E::Height, ch(T::Height, ColorChannel::Red).invert());
    e.set(E::Occlusion, ch(T::Occlusion, ColorChannel::Red).invert());
    e.set(
        E::NormalX,
        ch(T::Normal, ColorChannel::Red).min(-1.0).max(1.0),
    );
    e.set(
        E::NormalY,
        ch(T::Normal, ColorChannel::Green).min(-1.0).max(1.0),
    );
    e.set(
        E::NormalZ,
        ch(T::Normal, ColorChannel::Blue).min(0.0).max(1.0),
    );
    e.set(E::Specular, ch(T::Specular, ColorChannel::Red));
    e.set(E::Smooth, ch(T::Smooth, ColorChannel::Red));
    e.set(E::Rough, ch(T::Rough, ColorChannel::Red));
    e.set(E::Metal, ch(T::Metal, ColorChannel::Red));
    e.set(
        E::Hcm,
        ch(T::Hcm, ColorChannel::Red)
            .sampler(SamplerKind::Nearest)
            .min(230.0)
            .max(255.0)
            .range(230, 255)
            .clipping(),
    );
    e.set(E::F0, ch(T::F0, ColorChannel::Red));
    e.set(E::Porosity, ch(T::Porosity, ColorChannel::Red));
    e.set(E::Sss, ch(T::Sss, ColorChannel::Red));
    e.set(E::Emissive, ch(T::Emissive, ColorChannel::Red));

    e
}
const RAW: FormatDef = FormatDef {
    id: "raw",
    aliases: &[],
    description: "Separates all encoding channels as separate textures.",
    hidden: false,
    build: raw,
};

fn color() -> PackEncoding {
    use EncodingChannel as E;
    let mut e = PackEncoding::default();
    e.set(
        E::Opacity,
        ch(TextureTag::Color, ColorChannel::Alpha).default_value(1.0),
    );
    e.set(E::ColorRed, ch(TextureTag::Color, ColorChannel::Red));
    e.set(E::ColorGreen, ch(TextureTag::Color, ColorChannel::Green));
    e.set(E::ColorBlue, ch(TextureTag::Color, ColorChannel::Blue));

    e
}
const COLOR: FormatDef = FormatDef {
    id: "color",
    aliases: &["albedo", "diffuse"],
    description: "Only the albedo color map and alpha channel.",
    // INFO: Not a priority for now. Should be accessible it w/ CLI (when I get to writing it).
    // I will not be bothering with the tests for this on the initial release. Old/LabPBR and both
    // Bedrock edition formats will be prioritized.
    hidden: true,
    build: color,
};

fn specular() -> PackEncoding {
    use EncodingChannel as E;
    use TextureTag as T;
    let mut e = PackEncoding::default();
    e.set(
        E::Opacity,
        ch(T::Color, ColorChannel::Alpha).default_value(1.0),
    );
    e.set(E::ColorRed, ch(T::Color, ColorChannel::Red));
    e.set(E::ColorGreen, ch(T::Color, ColorChannel::Green));
    e.set(E::ColorBlue, ch(T::Color, ColorChannel::Blue));
    e.set(
        E::NormalX,
        ch(T::Normal, ColorChannel::Red).min(-1.0).max(1.0),
    );
    e.set(
        E::NormalY,
        ch(T::Normal, ColorChannel::Green).min(-1.0).max(1.0),
    );
    e.set(
        E::NormalZ,
        ch(T::Normal, ColorChannel::Blue).min(0.0).max(1.0),
    );
    e.set(E::Specular, ch(T::Specular, ColorChannel::Red));

    e
}
const SPECULAR: FormatDef = FormatDef {
    id: "specular",
    aliases: &[],
    description: "Uses diffuse color map, normal XYZ, and legacy grayscale specular channels.",
    hidden: true, // INFO: See COLOR
    build: specular,
};

fn old_pbr() -> PackEncoding {
    use EncodingChannel as E;
    use TextureTag as T;
    let mut e = PackEncoding::default();
    e.set(
        E::Opacity,
        ch(T::Color, ColorChannel::Alpha).default_value(1.0),
    );
    e.set(E::ColorRed, ch(T::Color, ColorChannel::Red));
    e.set(E::ColorGreen, ch(T::Color, ColorChannel::Green));
    e.set(E::ColorBlue, ch(T::Color, ColorChannel::Blue));
    e.set(
        E::NormalX,
        ch(T::Normal, ColorChannel::Red).min(-1.0).max(1.0),
    );
    e.set(
        E::NormalY,
        ch(T::Normal, ColorChannel::Green).min(-1.0).max(1.0),
    );
    e.set(
        E::NormalZ,
        ch(T::Normal, ColorChannel::Blue).min(0.0).max(1.0),
    );
    e.set(
        E::Height,
        ch(T::Normal, ColorChannel::Alpha)
            .default_value(0.0)
            .min(0.0)
            .max(1.0)
            .invert(),
    );
    e.set(E::Smooth, ch(T::Specular, ColorChannel::Red));
    e.set(E::Metal, ch(T::Specular, ColorChannel::Green));
    e.set(E::Emissive, ch(T::Specular, ColorChannel::Blue));

    e
}
const OLD_PBR: FormatDef = FormatDef {
    id: "old-pbr",
    aliases: &["legacy"],
    description: "The pre-Lab standard for PBR, also known as \"Old PBR\".",
    hidden: false,
    build: old_pbr,
};

fn alpha_pbr() -> PackEncoding {
    use EncodingChannel as E;
    use TextureTag as T;
    let mut e = PackEncoding::default();
    e.set(E::ColorRed, ch(T::Color, ColorChannel::Red));
    e.set(E::ColorGreen, ch(T::Color, ColorChannel::Green));
    e.set(E::ColorBlue, ch(T::Color, ColorChannel::Blue));
    e.set(
        E::Opacity,
        ch(T::Color, ColorChannel::Alpha).range(0, 0).priority(10),
    );
    e.set(
        E::Smooth,
        ch(T::Color, ColorChannel::Alpha)
            .range(73, 157)
            .priority(1)
            .default_value(0.0)
            .clipping(),
    );
    e.set(
        E::Metal,
        ch(T::Color, ColorChannel::Alpha)
            .range(251, 251)
            .priority(2)
            .clip_value(0.0)
            .clipping(),
    );
    e.set(
        E::Sss,
        ch(T::Color, ColorChannel::Alpha)
            .range(22, 47)
            .priority(3)
            .clip_value(0.0)
            .clipping(),
    );
    e.set(
        E::Emissive,
        ch(T::Color, ColorChannel::Alpha)
            .range(48, 72)
            .priority(4)
            .clip_value(0.0)
            .clipping(),
    );

    e
}
const ALPHA_PBR: FormatDef = FormatDef {
    id: "alpha-pbr",
    aliases: &["vanilla-pbr"],
    description: "Uses a diffuse color map, with special encoding of the alpha channel for PBR materials. Proposed by Espen 2021",
    hidden: true, // INFO: See COLOR
    build: alpha_pbr,
};

fn lab_11() -> PackEncoding {
    use EncodingChannel as E;
    use TextureTag as T;
    let mut e = PackEncoding::default();
    e.set(
        E::Opacity,
        ch(T::Color, ColorChannel::Alpha).default_value(1.0),
    );
    e.set(E::ColorRed, ch(T::Color, ColorChannel::Red));
    e.set(E::ColorGreen, ch(T::Color, ColorChannel::Green));
    e.set(E::ColorBlue, ch(T::Color, ColorChannel::Blue));
    e.set(
        E::NormalX,
        ch(T::Normal, ColorChannel::Red).min(-1.0).max(1.0),
    );
    e.set(
        E::NormalY,
        ch(T::Normal, ColorChannel::Green).min(-1.0).max(1.0),
    );
    e.set(
        E::NormalZ,
        ch(T::Normal, ColorChannel::Blue).min(0.0).max(1.0),
    );
    e.set(
        E::Occlusion,
        ch(T::Normal, ColorChannel::Magnitude)
            .range(17, 255)
            .power(0.5)
            .invert(),
    );
    e.set(E::Height, ch(T::Normal, ColorChannel::Alpha).invert());
    e.set(E::Smooth, ch(T::Specular, ColorChannel::Red));
    e.set(
        E::F0,
        ch(T::Specular, ColorChannel::Green)
            .min(0.0)
            .max(0.9)
            .range(0, 229)
            .power(0.5)
            .clipping(),
    );
    e.set(
        E::Hcm,
        ch(T::Specular, ColorChannel::Green)
            .sampler(SamplerKind::Nearest)
            .min(230.0)
            .max(255.0)
            .range(230, 255)
            .clipping(),
    );
    e.set(E::Porosity, ch(T::Specular, ColorChannel::Blue));
    e.set(E::Emissive, ch(T::Specular, ColorChannel::Alpha).shift(-1));

    e
}
const LAB_11: FormatDef = FormatDef {
    id: "lab-1.1",
    aliases: &[],
    description: "The first LabPbr standard.",
    hidden: true, // INFO: See COLOR
    build: lab_11,
};

fn lab_12() -> PackEncoding {
    use EncodingChannel as E;
    use TextureTag as T;
    let mut e = PackEncoding::default();
    e.set(
        E::Opacity,
        ch(T::Color, ColorChannel::Alpha).default_value(1.0),
    );
    e.set(E::ColorRed, ch(T::Color, ColorChannel::Red));
    e.set(E::ColorGreen, ch(T::Color, ColorChannel::Green));
    e.set(E::ColorBlue, ch(T::Color, ColorChannel::Blue));
    e.set(
        E::NormalX,
        ch(T::Normal, ColorChannel::Red).min(-1.0).max(1.0),
    );
    e.set(
        E::NormalY,
        ch(T::Normal, ColorChannel::Green).min(-1.0).max(1.0),
    );
    // No normal-z: occlusion takes the blue channel instead.
    e.set(E::Occlusion, ch(T::Normal, ColorChannel::Blue).invert());
    e.set(E::Height, ch(T::Normal, ColorChannel::Alpha).invert());
    e.set(E::Smooth, ch(T::Specular, ColorChannel::Red));
    e.set(
        E::F0,
        ch(T::Specular, ColorChannel::Green)
            .min(0.0)
            .max(0.9)
            .range(0, 229)
            .power(0.5)
            .clipping(),
    );
    e.set(
        E::Hcm,
        ch(T::Specular, ColorChannel::Green)
            .sampler(SamplerKind::Nearest)
            .min(230.0)
            .max(255.0)
            .range(230, 255)
            .clipping(),
    );
    e.set(
        E::Porosity,
        ch(T::Specular, ColorChannel::Blue).range(0, 64).clipping(),
    );
    e.set(
        E::Sss,
        ch(T::Specular, ColorChannel::Blue)
            .range(65, 255)
            .clipping(),
    );
    e.set(E::Emissive, ch(T::Specular, ColorChannel::Blue).shift(-1));

    e
}
const LAB_12: FormatDef = FormatDef {
    id: "lab-1.2",
    aliases: &[],
    description: "The second LabPBR standard.",
    hidden: true, // INFO: See COLOR
    build: lab_12,
};

fn lab_13() -> PackEncoding {
    use EncodingChannel as E;
    use TextureTag as T;
    let mut e = PackEncoding::default();
    e.set(
        E::Opacity,
        ch(T::Color, ColorChannel::Alpha).default_value(1.0),
    );
    e.set(E::ColorRed, ch(T::Color, ColorChannel::Red));
    e.set(E::ColorGreen, ch(T::Color, ColorChannel::Green));
    e.set(E::ColorBlue, ch(T::Color, ColorChannel::Blue));
    e.set(
        E::NormalX,
        ch(T::Normal, ColorChannel::Red).min(-1.0).max(1.0),
    );
    e.set(
        E::NormalY,
        ch(T::Normal, ColorChannel::Green).min(-1.0).max(1.0),
    );
    e.set(
        E::Occlusion,
        ch(T::Normal, ColorChannel::Blue)
            .min(0.0)
            .max(1.0)
            .default_value(0.0)
            .invert(),
    );
    e.set(
        E::Height,
        ch(T::Normal, ColorChannel::Alpha)
            .default_value(0.0)
            .min(0.0)
            .max(1.0)
            .invert(),
    );
    e.set(E::Smooth, ch(T::Specular, ColorChannel::Red));
    e.set(
        E::F0,
        ch(T::Specular, ColorChannel::Green)
            .min(0.0)
            .max(0.9)
            .default_value(0.04)
            .range(0, 229)
            .clip_value(0.0)
            .clipping(),
    );
    e.set(
        E::Hcm,
        ch(T::Specular, ColorChannel::Green)
            .sampler(SamplerKind::Nearest)
            .min(230.0)
            .max(255.0)
            .range(230, 255)
            .clipping()
            .priority(1),
    );
    e.set(
        E::Porosity,
        ch(T::Specular, ColorChannel::Blue).range(0, 64).clipping(),
    );
    e.set(
        E::Sss,
        ch(T::Specular, ColorChannel::Blue)
            .range(65, 255)
            .clip_value(0.0)
            .no_clipping(),
    );
    e.set(
        E::Emissive,
        ch(T::Specular, ColorChannel::Alpha)
            .shift(-1)
            .default_value(0.0),
    );

    e
}
const LAB_13: FormatDef = FormatDef {
    id: "lab-1.3",
    aliases: &[],
    description: "The latest LabPbr standard.",
    hidden: false,
    build: lab_13,
};

fn rtx() -> PackEncoding {
    use EncodingChannel as E;
    use TextureTag as T;
    let mut e = PackEncoding::default();
    e.set(E::ColorRed, ch(T::Color, ColorChannel::Red));
    e.set(E::ColorGreen, ch(T::Color, ColorChannel::Green));
    e.set(E::ColorBlue, ch(T::Color, ColorChannel::Blue));
    e.set(
        E::Opacity,
        ch(T::Color, ColorChannel::Alpha).default_value(1.0),
    );
    e.set(
        E::Height,
        ch(T::Height, ColorChannel::Red).invert().default_value(0.0),
    );
    e.set(
        E::NormalX,
        ch(T::Normal, ColorChannel::Red).min(-1.0).max(1.0),
    );
    e.set(
        E::NormalY,
        ch(T::Normal, ColorChannel::Green).min(-1.0).max(1.0),
    );
    e.set(
        E::NormalZ,
        ch(T::Normal, ColorChannel::Blue).min(0.0).max(1.0),
    );
    e.set(E::Metal, ch(T::Mer, ColorChannel::Red));
    e.set(E::Emissive, ch(T::Mer, ColorChannel::Green));
    e.set(E::Rough, ch(T::Mer, ColorChannel::Blue));

    e
}
const RTX: FormatDef = FormatDef {
    id: "rtx",
    aliases: &[],
    description: "The NVidia standard for Bedrock RTX PBR.",
    hidden: false,
    build: rtx,
};

fn mers() -> PackEncoding {
    let mut e = rtx();
    // Same as RTX, packed tag "mers" instead of "mer", plus sss in alpha.
    // Retag every mer-tagged channel to mers rather than rebuilding from
    // scratch, so this stays a visible diff against rtx() forever.
    for channel in EncodingChannel::ALL {
        if let Some(d) = e.get(channel)
            && d.texture == Some(TextureTag::Mer)
        {
            let mut d = *d;
            d.texture = Some(TextureTag::Mers);
            e.set(channel, d);
        }
    }
    e.set(
        EncodingChannel::Sss,
        ch(TextureTag::Mers, ColorChannel::Alpha),
    );

    e
}
const MERS: FormatDef = FormatDef {
    id: "mers",
    aliases: &[],
    description: "Standard PBR format for Visual Vibrancy on Bedrock.",
    hidden: false,
    build: mers,
};

pub static FORMATS: &[FormatDef] = &[
    RAW, COLOR, SPECULAR, ALPHA_PBR, OLD_PBR, LAB_11, LAB_12, LAB_13, RTX, MERS,
];
