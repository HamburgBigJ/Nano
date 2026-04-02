use serde::{Deserialize, Serialize};
use crate::components::materials::MaterialConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelConfig {
    pub material: String, // path to materials/3d/rock_material_3d.json
    pub mesh: String, // path to mesh/Rock1.obj

}