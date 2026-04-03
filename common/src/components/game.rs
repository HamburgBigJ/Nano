use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GameConfig {
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default = "default_version")]
    pub version: String,

    #[serde(default = "default_level")]
    pub default_level: String,
}


fn default_name() -> String { "Nano-Game".to_string() }
fn default_version() -> String { "0.0.0".to_string() }
fn default_level() -> String { "level/default.json".to_string() }