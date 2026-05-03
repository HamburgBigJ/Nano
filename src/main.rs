mod elements;
mod render;
mod script;
mod world;

use crate::elements::elements::ElementsPlugin;
use crate::render::WorldRenderPlugin;
use crate::world::WorldPlugin;
use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup, Update};
use bevy::image::{Image, ImagePlugin};
use bevy::prelude::{Commands, Component, Res, ResMut, Resource, Startup, Transform};

pub const W: u32 = 300;
pub const H: u32 = 180;
pub const SCALE: f32 = 2.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldPlugin)
        .add_plugins(WorldRenderPlugin)
        .add_plugins(ElementsPlugin)
        .run();
}
