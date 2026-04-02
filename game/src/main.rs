mod player;
mod assets;
mod tests;

use std::f32::consts::PI;
use bevy::asset::AssetMetaCheck;
use bevy::ecs::error::info;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use rust_embed::Embed;
use serde_json::Value;
use common::components::scene::{GameObject, GameScene};
use crate::assets::game_assets::{GameAssets, ResourcesRegistry};
use crate::player::player_plugin::PlayerPlugin;

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
        .add_plugins(EguiPlugin::default())

        .add_systems(Startup, setup)
        .add_systems(PreStartup, init_app)


        .run();
}

fn init_app(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {

    // Embeddeed asset regestry
    let registry = GameAssets::init_registry(&mut images);
    commands.insert_resource(registry);


}

fn setup(mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut images: ResMut<Assets<Image>>,
         regestry: Res<ResourcesRegistry>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 8.0),
    ));



    info!("{:?}", GameAssets::get_struct::<GameScene>("level/map_test.json"));

    GameAssets::get_objects("level/map_test.json").iter().for_each(|o| {
        info!("{:?}", o);
    });


    let texture_handle = regestry.image_handles.get("textures/rock.png").unwrap();

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

}

