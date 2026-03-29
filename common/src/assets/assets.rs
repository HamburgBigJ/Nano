use std::path::{Path, PathBuf};
use serde::Deserialize;
use crate::components::scene::GameScene;

#[derive(Debug)]
pub struct AssetLoader {
    pub asset_root: PathBuf,
}

impl AssetLoader {
    pub fn new<P: Into<PathBuf>>(asset_root: P) -> Self {
        AssetLoader {
            asset_root: asset_root.into(),
        }
    }

    // DESKTOP
    pub fn load_scene(&self, scene_path: &str) -> Result<GameScene, Box<dyn std::error::Error>> {
        let full_path = self.asset_root.join(scene_path);
        let data = std::fs::read_to_string(&full_path)?;
        let mut scene: GameScene = serde_json::from_str(&data)?;
        for entity in &mut scene.entities {
            entity.file = self.asset_root.join(&entity.file).to_string_lossy().to_string();
        }
        Ok(scene)
    }

    // for WSAM
    #[cfg(target_arch = "wasm32")]
    pub async fn load_scene_async(&self, scene_path: &str) -> Result<GameScene, Box<dyn std::error::Error>> {
        use gloo_net::http::Request;

        let url = format!("{}/{}", self.asset_root.to_string_lossy(), scene_path);
        let data = Request::get(&url).send().await?.text().await?;
        let mut scene: GameScene = serde_json::from_str(&data)?;
        for entity in &mut scene.entities {
            entity.file = format!("{}/{}", self.asset_root.to_string_lossy(), entity.file);
        }
        Ok(scene)
    }
}