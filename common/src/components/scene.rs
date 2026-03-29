use serde::Deserialize;

use crate::components::position::{Position, Position2D};

// docs/scense/SCENE.md

#[derive(Deserialize, Debug)]
pub struct GameScene {
    pub name: String,
    pub id: String,

    pub entities: Vec<SceneObject>,
}


#[derive(Deserialize, Debug)]
pub struct SceneObject {
    pub file: String,
    pub position: Position,
}


#[derive(Deserialize, Debug)]
pub struct GameObject {
    pub id: String,
    pub assets: String,
    pub collision: CollisionShape,
}

// Collision enum
#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CollisionShape {
    Spline { points: Vec<Position2D> },
    Box { width: f32, height: f32 },
}