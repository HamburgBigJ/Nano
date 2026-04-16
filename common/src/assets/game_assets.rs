use std::io::Cursor;
use std::marker::PhantomData;
use bevy::app::{App, Plugin, PreStartup};
use bevy::asset::{Asset, Assets, Handle, RenderAssetUsages};
use bevy::audio::AudioSource;
use bevy::camera::ImageRenderTarget;
use bevy::ecs::storage::Resources;
use bevy::ecs::system::SystemParam;
use bevy::image::Image;
use bevy::log::{debug, info};
use bevy::math::Affine2;
use bevy::mesh::{Indices, Mesh, Mesh3d, PrimitiveTopology};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{AlphaMode, AssetApp, Color, ColorMaterial, Commands, Component, Entity, Rectangle, Res, ResMut, Resource, Transform};
use bevy::render::batching::sort_binned_render_phase;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite_render::AlphaMode2d;
use bevy::text::Font;
use bevy::utils::default;
use bevy_egui::egui::ahash::HashMap;
use mlua::{Function, Lua, Table};
use rust_embed::{Embed, EmbeddedFile};
use serde::de::DeserializeOwned;
use serde_json::Value::Null;
use crate::assets;
use crate::components::game::GameConfig;
use crate::components::lua_script::LuaScript;
use crate::components::materials::MaterialConfig;
use crate::components::mesh::MeshConfig;
use crate::components::model::ModelConfig;
use crate::components::scene::{GameObject, GameScene, SceneObject};
use crate::scritping::{get_lua};

// Ai insperation; i asked how i coud load the assets with this functions in the libary and how i coud mitigate the asset function in the app becasue it will beneedet in the edtir;
// T isn' a fild in the struct and we need PahndomData<T> to store the generic
// The main idea for there changes arte ot make the common lbary more acessabel for other project like the edtior and the asset system
// We alsow need for the plguin the generic system with the stuct for the assets in the app
pub struct GameAssetPlugin<T: EmbedHelper> {
    _marker: PhantomData<T>,
}

impl<T: EmbedHelper> Default for GameAssetPlugin<T> {
    fn default() -> GameAssetPlugin<T> {
        Self { _marker: PhantomData }
    }
}


impl<T: EmbedHelper + 'static + Send + Sync> Plugin for GameAssetPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_asset::<GameScene>();
        app.init_asset::<GameObject>();

        app.init_resource::<LevelManager>();

        // dubble regestry
        if !app.world().contains_resource::<ResourcesRegistry>() {
            app.init_resource::<ResourcesRegistry>();
        }

        app.add_systems(PreStartup, init_app_generics::<T>);
    }
}


fn init_app_generics<T: EmbedHelper>(
    mut commands: Commands,
    mut storages: AssetStorages,
    mut existing_registry: Option<ResMut<ResourcesRegistry>>,
) {
    let registry_data = T::init_registry(&mut storages);
    if let Some(mut registry) = existing_registry {
        registry.image.extend(registry_data.image);
        registry.mesh.extend(registry_data.mesh);
        registry.standard_material.extend(registry_data.standard_material);
        registry.color_materials.extend(registry_data.color_materials);
        registry.audio.extend(registry_data.audio);
        registry.font.extend(registry_data.font);
        registry.scene.extend(registry_data.scene);
        registry.game_object.extend(registry_data.game_object);
        registry.models.extend(registry_data.models);
        registry.scripts.extend(registry_data.scripts);

        if let Some(new_config) = registry_data.game_config {
            registry.game_config = Some(new_config);
        }
    } else {
        commands.insert_resource(registry_data);
    }
}

// To get all assets without repeting yourself
#[derive(SystemParam)]
pub struct AssetStorages<'w> {
    pub images: ResMut<'w, Assets<Image>>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
    pub color_materials: ResMut<'w, Assets<ColorMaterial>>,
    pub audio: ResMut<'w, Assets<AudioSource>>,
    pub fonts: ResMut<'w, Assets<Font>>,
    pub scenes: ResMut<'w, Assets<GameScene>>,
    pub objects: ResMut<'w, Assets<GameObject>>,
}

#[derive(Resource, Default)]
pub struct LevelManager {
    pub current_level: Option<String>,
    pub spawned_entities: HashMap<String, Vec<Entity>>,
}

#[derive(SystemParam)]
pub struct LevelSpawner<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub registry: Res<'w, ResourcesRegistry>,
    pub manager: ResMut<'w, LevelManager>,
    pub scene_assets: Res<'w, Assets<GameScene>>,
    pub object_assets: Res<'w, Assets<GameObject>>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
}

#[derive(SystemParam)]
pub struct ScriptSystem<'w> {
    pub registry: Res<'w, ResourcesRegistry>,
}


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
    pub scripts: HashMap<String, LuaScript>,

    pub game_config: Option<GameConfig>,
}
pub fn debug_registry(registry: &Res<ResourcesRegistry>) {
    info!("{:?}", *registry);
}

impl<'w> ScriptSystem<'w> {

    pub fn init_lua(self: &Self) {
        let lua = get_lua().lock().unwrap();
        lua.load(self.registry.scripts.get("src/nano.lua").unwrap().script.clone()).exec().unwrap();
    }


    pub fn execute_load_functions(self: &Self, path: &str) {
        let lua = get_lua().lock().unwrap();
        lua.load(self.registry.scripts.get(path).unwrap().script.clone()).exec(); // BROKEN !!!
        println!("{}", self.registry.scripts.get(path).unwrap().script.clone());
        let globals = lua.globals();
        let nano_table: Table = globals.get("Nano").expect("Nano table");
        let on_load_fn: Function = nano_table.get("onLoad").expect("onLoad");
        let result: String = on_load_fn.call(()).expect("onLoad");
        println!("{}", result);
    }
}

impl<'w, 's> LevelSpawner<'w, 's> {
    pub fn spawn(&mut self, level_path: &str) {
        let scene_handle = self.registry.scene.get(level_path)
            .expect("level not in registry");

        let scene = self.scene_assets.get(scene_handle)
            .expect("scene json not found");

        let mut spawned = Vec::new();

        for entity_ref in &scene.entities {
            let obj_handle = self.registry.game_object.get(&entity_ref.file)
                .expect("game obejct json not foud");

            let obj_data = self.object_assets.get(obj_handle).unwrap();

            let texture_handle = self.registry.image.get(&obj_data.assets)
                .expect("Texture for object not found!");

            let mesh_handle = self.meshes.add(Rectangle::new(obj_data.scale.width, obj_data.scale.height));
            let mat_handle = self.materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            });

            let entity = self.commands.spawn((
                Mesh3d(mesh_handle),
                MeshMaterial3d(mat_handle),
                Transform::from_xyz(entity_ref.position.x, entity_ref.position.y, entity_ref.position.z),
                LevelEntity(level_path.to_string()),
            )).id();

            spawned.push(entity);
        }

        self.manager.current_level = Some(level_path.to_string());
        self.manager.spawned_entities.insert(level_path.to_string(), spawned);
    }

    pub fn despawn_current(&mut self) {
        if let Some(level) = self.manager.current_level.take() {
            if let Some(entities) = self.manager.spawned_entities.remove(&level) {
                for entity in entities {
                    self.commands.entity(entity).despawn();
                }
            }
        }
    }
}

#[derive(Component)]
pub struct LevelEntity(pub String);


pub trait EmbedHelper: Embed {

    fn get_struct<T>(path: &str) -> Result<T, Box<dyn std::error::Error>> where T: DeserializeOwned { // Box needet for error / can not only contain std::error::Error
        let asset = Self::get(path).ok_or_else(|| format!("asset not found: {}", path))?;
        let value = serde_json::from_slice::<T>(&asset.data)?;
        Ok(value)
    }

    fn get_objects(path: &str) -> Result<Vec<GameObject>, Box<dyn std::error::Error>> {
        let assets = Self::get_struct::<GameScene>(path)?;
        let mut scene_object = vec![];
        for entities in assets.entities {
            scene_object.push(Self::get_struct::<GameObject>(&entities.file)?);
        }
        Ok(scene_object)
    }

    fn get_image_struct(path: &str) -> Result<Image, Box<dyn std::error::Error>> {
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

    fn init_registry(
        storages: &mut AssetStorages,
    ) -> ResourcesRegistry {
        let mut registry = ResourcesRegistry::default(); // TODO: maby procces insted of fodler defualt_box.mesh.json or so

        for file_path in Self::iter() { // hevy proccesng maby in ohter thred when window there
            let path_str = file_path.as_ref();

            // Textures
            if path_str.ends_with(".png") || path_str.ends_with(".jpg") {
                if let Ok(img) = Self::get_image_struct(path_str) {
                    registry.image.insert(path_str.to_string(), storages.images.add(img));
                }
            }

            // Audio
            else if path_str.ends_with(".ogg") || path_str.ends_with(".mp3") || path_str.ends_with(".wav") {
                if let Some(asset) = Self::get(path_str) {
                    let source = AudioSource { bytes: asset.data.into() };
                    registry.audio.insert(path_str.to_string(), storages.audio.add(source));
                }
            }

            // Fonts
            else if path_str.ends_with(".ttf") || path_str.ends_with(".otf") {
                if let Some(asset) = Self::get(path_str) {
                    if let Ok(fon) = Font::try_from_bytes(asset.data.into()) {
                        registry.font.insert(path_str.to_string(), storages.fonts.add(fon));
                    }
                }
            }

            // Game scene
            else if path_str.starts_with("level/") && path_str.ends_with(".json") {
                if let Ok(level) = Self::get_struct::<GameScene>(path_str) {
                    registry.scene.insert(path_str.to_string(), storages.scenes.add(level));
                }
            }

            // Game object
            else if path_str.starts_with("objects/") && path_str.ends_with(".json") {
                if let Ok(obj) = Self::get_struct::<GameObject>(path_str) {
                    registry.game_object.insert(path_str.to_string(), storages.objects.add(obj));
                }
            }


            if path_str.starts_with("mesh/") {
                if path_str.ends_with(".json") {
                    if let Ok(config) = Self::get_struct::<MeshConfig>(path_str) {
                        let mesh: Mesh = config.into();
                        registry.mesh.insert(path_str.to_string(), storages.meshes.add(mesh));
                    }
                }
                else if path_str.ends_with(".obj") {
                    if let Some(asset) = Self::get(path_str) {
                        if let Ok(mesh) = Self::load_obj_from_bytes(&asset.data) {
                            registry.mesh.insert(path_str.to_string(), storages.meshes.add(mesh));
                        }
                    }
                }
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
                    registry.standard_material.insert(path_str.to_string(), storages.materials.add(mat));
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

                    registry.color_materials.insert(path_str.to_string(), storages.color_materials.add(mat));
                }
            }

            // Models 3d
            else if path_str.starts_with("models/") && path_str.ends_with(".json") {
                if let Ok(model_config) = Self::get_struct::<ModelConfig>(path_str) {
                    registry.models.insert(path_str.to_string(), model_config);
                }
            }

            if path_str.starts_with("game.json") {
                if let Ok(game_config) = Self::get_struct::<GameConfig>(path_str) {
                    registry.game_config = game_config.into();
                }
            }

            // Scripts
            if path_str.ends_with(".lua") {
                if let Some(lua_script_data) = Self::get(path_str) {
                    if let Ok(script) = std::str::from_utf8(&lua_script_data.data) {
                        let mut lua = LuaScript::default();
                        lua.script = script.to_string();
                        registry.scripts.insert(path_str.to_string(), lua);
                    }
                }
            }


        }
        registry
    }

    // Ai notice help with ai to advanced mathematic themes with verteces and indecies
    fn load_obj_from_bytes(bytes: &[u8]) -> Result<Mesh, Box<dyn std::error::Error>> {
        let mut reader = Cursor::new(bytes);
        let (models, _) = tobj::load_obj_buf(&mut reader, &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        }, |_| Ok(Default::default()))?;

        let mesh_data = &models[0].mesh;
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());

        let positions: Vec<[f32; 3]> = mesh_data.positions.chunks(3)
            .map(|c| [c[0], c[1], c[2]]).collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

        if !mesh_data.normals.is_empty() {
            let normals: Vec<[f32; 3]> = mesh_data.normals.chunks(3)
                .map(|c| [c[0], c[1], c[2]]).collect();
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        }

        if !mesh_data.texcoords.is_empty() {
            let uvs: Vec<[f32; 2]> = mesh_data.texcoords.chunks(2)
                .map(|c| [c[0], c[1]]).collect();
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        }

        mesh.insert_indices(Indices::U32(mesh_data.indices.clone()));

        Ok(mesh)
    }

    #[deprecated(since = "0.0.1", note = "use LevelSpawner struct")]
    fn spawn_level(
        spawner: &mut LevelSpawner,
        path: &str,
    ) {
        let level = Self::get_struct::<GameScene>(path).unwrap();
        let objects = Self::get_objects(path).unwrap();

        for object in level.entities {
            let level_object = Self::get_struct::<GameObject>(&object.file).unwrap();
            info!("{}", &level_object.assets);
            let texture_handle = spawner.registry.image.get(&level_object.assets).unwrap();

            let quad_handle = spawner.meshes.add(Rectangle::new(level_object.scale.width, level_object.scale.height));
            let material_handle = spawner.materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            });

            spawner.commands.spawn((
                Mesh3d(quad_handle.clone()),
                MeshMaterial3d(material_handle),
                Transform::from_xyz(object.position.x, object.position.y, object.position.z)//.with_rotation(Quat::from_rotation_x(-PI / 5.0)),
            ));

        }

        info!("spawned level {}", path);
        info!("objects: {:?}", objects);

    }
}


impl<T: Embed> EmbedHelper for T {}
