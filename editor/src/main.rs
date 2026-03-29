mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::ui::editor_ui::EditorUi;







fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(EditorUi)

        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default()); // for 3d view of sceene ( palralx unety like )
}