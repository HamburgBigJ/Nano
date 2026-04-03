mod player;
mod tests;
mod world;

use std::f32::consts::PI;
use bevy::asset::AssetMetaCheck;
use bevy::ecs::error::info;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use rust_embed::Embed;
use serde_json::Value;
use common::components::scene::{GameObject, GameScene};
use common::assets::game_assets::{debug_registry, GameAssetPlugin, ResourcesRegistry};
use crate::player::player_plugin::PlayerPlugin;
use crate::world::level_render::LevelPlugin;


#[derive(Embed)]
#[folder = "assets/"]
pub struct GameAssets;

fn main() {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .add_plugins(
            DefaultPlugins.set(
            WindowPlugin {
                    primary_window: Some(Window {
                        #[cfg(target_arch = "wasm32")]
                        canvas: Some("#bevy".into()),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                },

            )
            .set(AssetPlugin {
                #[cfg(target_arch = "wasm32")]
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
        )
        .add_plugins(PlayerPlugin)
        .add_plugins(LevelPlugin)

        .add_plugins(GameAssetPlugin::<GameAssets>::default())
        .add_plugins(EguiPlugin::default())

        .add_systems(Startup, setup)

        .run();
}


fn setup(mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
         registry: Res<ResourcesRegistry>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 8.0),
    ));

    info!("Running: {} Version: {}", registry.game_config.name, registry.game_config.version);

    //info!("{:?}", GameAssets::get_struct::<GameScene>("level/map_test.json"));

    info!("{:?}", registry.game_config);

    /*GameAssets::get_objects("level/map_test.json").iter().for_each(|o| {
        info!("{:?}", o);
    });*/


    let texture_handle = registry.image.get("textures/rock.png").unwrap();

    let quad_handle = meshes.add(Rectangle::new(1.0, 1.0));
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    commands.spawn((
        Mesh3d(quad_handle.clone()),
        MeshMaterial3d(material_handle),
        Transform::from_xyz(0.0, 0.0, 1.5)//.with_rotation(Quat::from_rotation_x(-PI / 5.0)),
    ));

    debug_registry(registry);


}

