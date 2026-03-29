use serde::Deserialize;



#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
}