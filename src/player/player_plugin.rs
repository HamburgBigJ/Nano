use bevy::prelude::*;

use crate::player::player;


struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player::setup_player);
    }

}