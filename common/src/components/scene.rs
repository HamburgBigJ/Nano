use bevy::{ecs::component::Component};

use crate::components::position::Position;

// docs/scense/SCENE.md

#[derive(Component)]
pub struct Game_scene {
    name: String,

}

#[derive(Component)]
pub struct Scene_object {
    position: Position,

}