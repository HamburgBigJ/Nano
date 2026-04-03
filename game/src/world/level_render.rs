use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, Res};
use common::assets::game_assets::ResourcesRegistry;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {

    }
}

fn load_default_level(
    mut commands: Commands,
    resources: Res<ResourcesRegistry>
) {
    
}