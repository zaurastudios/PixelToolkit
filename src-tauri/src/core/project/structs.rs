// use serde::{Deserialize, Deserializer};

// For unwrapping optional values elegantly
// fn null_to_default<'de, D, T>(d: D) -> Result<T, D::Error>
// where
//     D: Deserializer<'de>,
//     T: Default + Deserialize<'de>,
// {
//     let opt = Option::deserialize(d)?;
//     let val = opt.unwrap_or_else(T::default);
//     Ok(val)
// }

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProjectYml {
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub input: Input,
    // #[serde(deserialize_with = "null_to_default")]
    pub profiles: Option<Vec<Profile>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Input {
    pub format: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Profile {
    pub edition: GameEdition,
    pub format: i8,
    pub name: String,
    pub description: Option<String>,
    pub block_texture_size: Option<i16>,
    pub item_texture_size: Option<i16>,
    pub texture_scale: Option<i8>,
    pub auto_level_height: Option<bool>,
    pub bake_occlusion_texture: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum GameEdition {
    Java,
    Bedrock,
}
#[derive(serde::Serialize)]
pub struct FileTree {
    pub name: String,
    pub is_mat: Option<bool>,
    pub children: Vec<FileTree>,
}
