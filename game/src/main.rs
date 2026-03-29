mod player;
mod tests;

use bevy::{prelude::*};
use bevy_egui::{EguiPlugin};
use common::assets::assets::AssetLoader;

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
    // For camera
    commands.spawn(Camera3d::default()); // for paralax resons 


    /*
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::BLACK),
    ));
     */



    let loader = AssetLoader::new("game/assets");

    match loader.load_scene("level/map_test.json") {
        Ok(scene) => {
            info!("Scene loaded: {:?}", scene);

            for entity in scene.entities {
                info!("Entity at {:?} with file {:?}", entity.position, entity.file);
            }
        }
        Err(e) => info!("Failed to load scene: {}", e),
    }
}