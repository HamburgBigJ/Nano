mod player;
mod assets;

use bevy::ecs::error::info;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use rust_embed::Embed;
use serde_json::Value;
use common::components::scene::{GameObject, GameScene};
use crate::assets::game_assets::GameAssets;
use crate::player::player_plugin::PlayerPlugin;

fn main() {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                #[cfg(target_arch = "wasm32")]
                canvas: Some("#bevy".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PlayerPlugin)
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default());



    let testjsondata = GameAssets::get("level/map_test.json").unwrap();

    let test_map: GameScene = serde_json::from_slice(&testjsondata.data).unwrap();


    let onject_rock = GameAssets::get(&test_map.entities[0].file).unwrap();
    let object_rock: GameObject = serde_json::from_slice(&onject_rock.data).unwrap();
    info!("{:?}", object_rock);
    info!("{:?}", test_map);

}

