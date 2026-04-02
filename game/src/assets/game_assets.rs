use bevy::asset::{Asset, Assets, Handle, RenderAssetUsages};
use bevy::audio::AudioSource;
use bevy::camera::ImageRenderTarget;
use bevy::image::Image;
use bevy::log::{debug, info};
use bevy::math::Affine2;
use bevy::mesh::Mesh;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Color, ColorMaterial, Commands, Res, ResMut, Resource};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite_render::AlphaMode2d;
use bevy::text::Font;
use bevy::utils::default;
use bevy_egui::egui::ahash::HashMap;
use rust_embed::{Embed, EmbeddedFile};
use serde::de::DeserializeOwned;
use common::components::materials::MaterialConfig;
use common::components::model::ModelConfig;
use common::components::scene::{GameObject, GameScene};

#[derive(Embed)]
#[folder = "assets/"]
pub struct GameAssets;

#[derive(Resource, Default)]
#[derive(Debug)]
pub struct ResourcesRegistry {
    // Bevy stuff
    pub image: HashMap<String, Handle<Image>>,
    pub mesh: HashMap<String, Handle<Mesh>>,
    pub standard_material: HashMap<String, Handle<StandardMaterial>>,
    pub color_materials: HashMap<String, Handle<ColorMaterial>>,
    pub audio: HashMap<String, Handle<AudioSource>>,
    pub font: HashMap<String, Handle<Font>>,

    // Custom stuff
    pub scene: HashMap<String, Handle<GameScene>>,
    pub game_object: HashMap<String, Handle<GameObject>>,
    pub models: HashMap<String, ModelConfig>,
}
pub fn debug_registry(registry: Res<ResourcesRegistry>) {
    info!("{:?}", *registry);
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

    pub fn init_registry(
        images: &mut Assets<Image>,
        mesh: &mut Assets<Mesh>,
        standard_materials: &mut Assets<StandardMaterial>,
        color_materials: &mut Assets<ColorMaterial>,
        audio: &mut Assets<AudioSource>,
        font: &mut Assets<Font>,
        game_scenes: &mut Assets<GameScene>,
        game_objects: &mut Assets<GameObject>,
    ) -> ResourcesRegistry {
        let mut registry = ResourcesRegistry::default();

        for file_path in Self::iter() { // hevy proccesng maby in ohter thred when window there
            let path_str = file_path.as_ref();

            // Textures
            if path_str.ends_with(".png") || path_str.ends_with(".jpg") {
                if let Ok(img) = Self::get_image_struct(path_str) {
                    registry.image.insert(path_str.to_string(), images.add(img));
                }
            }

            // Audio
            else if path_str.ends_with(".ogg") || path_str.ends_with(".mp3") || path_str.ends_with(".wav") {
                if let Some(asset) = Self::get(path_str) {
                    let source = AudioSource { bytes: asset.data.into() };
                    registry.audio.insert(path_str.to_string(), audio.add(source));
                }
            }

            // Fonts
            else if path_str.ends_with(".ttf") || path_str.ends_with(".otf") {
                if let Some(asset) = Self::get(path_str) {
                    if let Ok(fon) = Font::try_from_bytes(asset.data.into()) {
                        registry.font.insert(path_str.to_string(), font.add(fon));
                    }
                }
            }

            // Game scene
            else if path_str.starts_with("level/") && path_str.ends_with(".json") {
                if let Ok(level) = Self::get_struct::<GameScene>(path_str) {
                    registry.scene.insert(path_str.to_string(), game_scenes.add(level));
                }
            }

            // Game object
            else if path_str.starts_with("objects/") && path_str.ends_with(".json") {
                if let Ok(obj) = Self::get_struct::<GameObject>(path_str) {
                    registry.game_object.insert(path_str.to_string(), game_objects.add(obj));
                }
            }

            // byte to mesh not supportet
            else if path_str.ends_with(".obj") { // maby .gltf
                /* Let asset = Self::get(path_str).unwrap();
                   // tobj::load_obj_buf to create bevy::Mesh
                */

            }

            // Materials 3d
            else if path_str.starts_with("materials/3d/") {
                if let Ok(config) = Self::get_struct::<MaterialConfig>(path_str) {
                    let texture_handle = config.texture_path.and_then(|p| registry.image.get(&p).cloned());

                    let mat = StandardMaterial {
                        base_color: Color::srgba(config.base_color[0], config.base_color[1], config.base_color[2], config.base_color[3]),
                        base_color_texture: texture_handle,
                        perceptual_roughness: config.roughness,
                        metallic: config.metallic,
                        unlit: config.unlit,
                        ..Default::default()
                    };
                    registry.standard_material.insert(path_str.to_string(), standard_materials.add(mat));
                }
            }

            // Materials 2d
            else if path_str.starts_with("materials/2d/") {
                if let Ok(config) = Self::get_struct::<MaterialConfig>(path_str) {
                    let texture_handle = config.texture_path.and_then(|p| registry.image.get(&p).cloned());

                    let alpha_mode = match config.alpha_mode.as_str() {
                        "Mask" => AlphaMode2d::Mask(0.5),
                        "Opaque" => AlphaMode2d::Opaque,
                        _ => AlphaMode2d::Blend,
                    };

                    let mat = ColorMaterial {
                        color: Color::srgba(
                            config.base_color[0],
                            config.base_color[1],
                            config.base_color[2],
                            config.base_color[3]
                        ),
                        alpha_mode,
                        uv_transform: Affine2::IDENTITY,
                        texture: texture_handle,
                    };

                    registry.color_materials.insert(path_str.to_string(), color_materials.add(mat));
                }
            }

            // Models 3d
            else if path_str.starts_with("models/") && path_str.ends_with(".json") {
                if let Ok(model_config) = Self::get_struct::<ModelConfig>(path_str) {
                    registry.models.insert(path_str.to_string(), model_config);
                }
            }




        }
        registry
    }
}