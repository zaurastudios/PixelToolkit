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
