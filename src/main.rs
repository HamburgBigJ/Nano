mod elements;
mod render;
mod script;
mod world;
mod input;

use crate::elements::elements::ElementsPlugin;
use crate::render::WorldRenderPlugin;
use crate::world::WorldPlugin;
use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup};
use bevy::image::{ImagePlugin};
use crate::input::controller::GameController;


// Do this and scale to screen
pub const SCALE: f32 = 6.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldPlugin)
        .add_plugins(WorldRenderPlugin)
        .add_plugins(ElementsPlugin)
        .add_plugins(GameController)
        .run();
}
