mod player;
mod tests;

use bevy::prelude::*;

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
        .run();
}


