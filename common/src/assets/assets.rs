use std::{fs, path::PathBuf};

use crate::components::scene::GameScene;

pub struct AssetLoader {
    pub asset_root: PathBuf,
}

impl AssetLoader {
    pub fn new<P: Into<PathBuf>>(asset_root: P) -> Self {
        AssetLoader {
            asset_root: asset_root.into(),
        }
    }

    pub fn load_scene(&self, scene_path: &str) -> Result<GameScene, Box<dyn std::error::Error>> {
        let full_path = self.asset_root.join(scene_path);
        let data = fs::read_to_string(&full_path)?;
        let mut scene: GameScene = serde_json::from_str(&data)?;


        for entity in &mut scene.entities {
            entity.file = self.asset_root.join(&entity.file).to_string_lossy().to_string();
        }

        Ok(scene)
    }
    /*
    pub fn get_asset_path(&self, relative_path: &str) -> PathBuf {
        self.asset_root.join(relative_path)
    }
     */
}