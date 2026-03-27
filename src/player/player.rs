use bevy::{ecs::{component::Component, system::Commands}, math::Vec3};


#[derive(Component)]
struct Player {
    name: String,
    id: i32
}




pub fn setup_player(_commands: Commands) {
    // TODO: implementation for spanwing of player 

        
}
// TODO: implementation for contolls etc