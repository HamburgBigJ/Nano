use bevy::asset::{Asset, Assets, Handle, RenderAssetUsages};
use bevy::camera::ImageRenderTarget;
use bevy::image::Image;
use bevy::prelude::{Commands, ResMut, Resource};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_egui::egui::ahash::HashMap;
use rust_embed::{Embed, EmbeddedFile};
use serde::de::DeserializeOwned;
use common::components::scene::{GameObject, GameScene};

#[derive(Embed)]
#[folder = "assets/"]
pub struct GameAssets;

#[derive(Resource, Default)]
pub struct ResourcesRegistry {
    pub image_handles: HashMap<String, Handle<Image>>, // String as id
}


impl GameAssets {

    pub fn get_struct<T>(path: &str) -> Result<T, Box<dyn std::error::Error>> where T: DeserializeOwned { // Box needet for error / can not only contain std::error::Error
        let asset = Self::get(path).ok_or_else(|| format!("asset not found: {}", path))?;
        let value = serde_json::from_slice::<T>(&asset.data)?;
        Ok(value)
    }

    pub fn get_objects(path: &str) -> Result<Vec<GameObject>, Box<dyn std::error::Error>> {
        let assets = Self::get_struct::<GameScene>(path)?;
        let mut scene_object = vec![];
        for entities in assets.entities {
            scene_object.push(Self::get_struct::<GameObject>(&entities.file)?);
        }
        Ok(scene_object)
    }

    pub fn get_image_struct(path: &str) -> Result<Image, Box<dyn std::error::Error>> {
        let asset = Self::get(path).ok_or_else(|| format!("asset not found: {}", path))?;
        let dyn_img = image::load_from_memory(&asset.data)?.to_rgba8();
        let (width, height) = dyn_img.dimensions();

        Ok(Image::new(
            Extent3d { width, height, depth_or_array_layers: 1 },
            TextureDimension::D2,
            dyn_img.into_raw(),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        ))
    }

    pub fn init_registry(images: &mut Assets<Image>) -> ResourcesRegistry {
        let mut registry = ResourcesRegistry::default();

        for file_path in Self::iter() {
            let path_str = file_path.as_ref();
            if path_str.ends_with(".png") || path_str.ends_with(".jpg") {
                if let Ok(img) = Self::get_image_struct(path_str) {
                    let handle = images.add(img);
                    registry.image_handles.insert(path_str.to_string(), handle);
                }
            }
        }
        registry
    }
}