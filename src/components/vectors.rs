use bevy::ecs::{component::Component, *};

#[derive(Component)]
struct Vec2 {
    x: f32,
    y: f32
}

#[derive(Component)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}