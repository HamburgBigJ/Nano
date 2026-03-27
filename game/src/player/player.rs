use bevy::{ecs::{component::Component, system::Commands}, math::Vec3};
use common::components::position::Position;

#[derive(Component)]
pub struct Player {
    name: String,
    id: i32,
    position: Position
}




pub fn setup_player(_commands: Commands) {
    // TODO: implementation for spanwing of player 

        
}
// TODO: implementation for contolls etc