use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::prelude::{Cuboid, Plane3d, Rectangle, Sphere};
use serde::{Deserialize, Serialize};

// Ai notice needet help because MeshConfig with difrent style asked gemeai
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "object")]
pub enum MeshConfig {
    #[serde(rename = "Box")]
    Box { x: f32, y: f32, z: f32 },

    #[serde(rename = "Rectangle")]
    Rectangle { width: f32, height: f32 },

    /*#[serde(rename = "Sphere")]
    Sphere { radius: f32, sectors: usize, stacks: usize },

    #[serde(rename = "Plane")]
    Plane { size: f32 },*/
}

impl From<MeshConfig> for Mesh {
    fn from(config: MeshConfig) -> Self {
        match config {
            MeshConfig::Box { x, y, z } => {
                Mesh::from(Cuboid::new(x, y, z))
            }
            MeshConfig::Rectangle { width, height } => {
                Mesh::from(Rectangle::new(width, height))
            }
            /*MeshConfig::Sphere { radius, sectors, stacks } => {
                Mesh::from(Sphere::new(radius).mesh().uv(sectors, stacks))
            }
            MeshConfig::Plane { size } => {
                Mesh::from(Plane3d::default().mesh().size(size, size))
            }*/
        }
    }
}