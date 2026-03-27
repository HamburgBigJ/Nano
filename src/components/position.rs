use bevy::ecs::component::Component;



#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32
}


impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }


}