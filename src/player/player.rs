use bevy::{ecs::{component::Component, system::{Commands}}, *};


#[derive(Component)]
struct Player {
    name: String,
    id: i32
}




pub fn setupPlayer(mut commands: Commands) {
    // TODO: implementation for spanwing of player 
}

// TODO: implementation for contolls etc