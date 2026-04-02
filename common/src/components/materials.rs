use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MaterialConfig {
    pub base_color: [f32; 4],
    pub texture_path: Option<String>,
    pub unlit: bool,
    #[serde(default)]
    pub roughness: f32,
    #[serde(default)]
    pub metallic: f32,
    #[serde(default = "default_alpha_mode")]
    pub alpha_mode: String,
}


fn default_alpha_mode() -> String { "Blend".to_string() }