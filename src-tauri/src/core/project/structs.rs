pub enum GameEdition {
    Java,
    Bedrock,
}
pub struct Profile {
    pub game_edition: GameEdition,
    pub fomart: i8,
    pub name: String,
    pub description: Option<String>,
    pub block_texture_size: Option<i16>,
    pub item_texture_size: Option<i16>,
    pub texture_scale: Option<i8>,
    pub auto_level_height: Option<bool>,
    pub bake_occlusion_texture: Option<bool>,
}

pub struct Input {
    pub format: String,
}

pub struct ProjectYml {
    pub name: String,
    pub description: String,
    pub tags: String,
    pub input: Input,
    pub profiles: Vec<Profile>,
}

#[derive(serde::Serialize)]
pub struct FileTree {
    pub name: String,
    pub is_mat: Option<bool>,
    pub children: Vec<FileTree>,
}
