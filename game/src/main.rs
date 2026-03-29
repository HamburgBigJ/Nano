mod player;



use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use common::assets::assets::AssetLoader;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

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

    #[cfg(target_arch = "wasm32")]
    {
        let loader = AssetLoader::new("assets");
        spawn_local(async move {
            match loader.load_scene_async("level/map_test.json").await {
                Ok(scene) => console::log_1(&format!("Scene loaded: {:?}", scene).into()),
                Err(e) => console::log_1(&format!("Failed to load scene: {}", e).into()),
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let loader = AssetLoader::new("game/assets");
        match loader.load_scene("level/map_test.json") {
            Ok(scene) => info!("Scene loaded: {:?}", scene),
            Err(e) => info!("Failed to load scene: {}", e),
        }
    }
}