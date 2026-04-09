mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use rust_embed::RustEmbed;
use common::assets::game_assets::GameAssetPlugin;
use crate::ui::editor_ui::EditorUi;


#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct EditorAssets;



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(EditorUi)
        .add_plugins(GameAssetPlugin::<EditorAssets>::default())

        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default()); // for 3d view of sceene ( palralx unety like )
}